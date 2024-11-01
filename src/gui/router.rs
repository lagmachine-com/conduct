use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use log::{debug, error, info};
use matchit::Params;
use wry::{http::Request, http::Response};

use crate::{
    core::project::Project,
    gui::{embedded_files, routes::register_routes},
};

use super::api_result::ApiResult;

pub type ApiRequestHandler = fn(&Request<Vec<u8>>, Params, RequestContext) -> Option<ApiResult>;

pub struct RequestContext {
    pub project: Arc<Mutex<Project>>,
}

pub fn route(
    _id: &str,
    request: Request<Vec<u8>>,
    context: RequestContext,
) -> Response<Cow<'static, [u8]>> {
    let path = request.uri().path();

    info!("Received request: {}", request.uri().path());

    let response_builder = Response::builder();

    #[cfg(debug_assertions)]
    let response_builder =
        response_builder.header("Access-Control-Allow-Origin", "http://localhost:3000");

    // TODO: Dont create a new router for every request!
    let mut router = matchit::Router::<ApiRequestHandler>::new();
    register_routes(&mut router);
    let m = router.at(path);

    match m {
        Ok(m) => {
            let handler = m.value;
            let result = handler(&request, m.params, context);
            debug!("Got result: {:?}", result);
            match result {
                Some(result) => match result {
                    ApiResult::Ok(value) => match value {
                        Some(response) => response_builder
                            .status(200)
                            .header("Content-Type", "text/json")
                            .body(Cow::Owned::<[u8]>(
                                serde_json::to_string(&response).unwrap().into(),
                            ))
                            .unwrap(),
                        None => response_builder
                            .status(200)
                            .body(Cow::Owned("Ok".into()))
                            .unwrap(),
                    },
                    ApiResult::Error(msg) => {
                        error!("Api Error: {}", msg);

                        response_builder
                            .status(400)
                            .body(Cow::Owned(msg.into()))
                            .unwrap()
                    }
                },
                None => response_builder
                    .status(404)
                    .body(Cow::Owned("Not found".into()))
                    .unwrap(),
            }
        }
        Err(_) => embedded_files::get(path.to_string(), response_builder),
    }
}
