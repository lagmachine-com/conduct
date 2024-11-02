use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use log::{info, warn};
use matchit::{Params, Router};
use wry::{
    http::{response::Builder, Request, Response},
    RequestAsyncResponder,
};

use crate::{core::project::Project, gui::embedded_files};

use super::api_result::ApiResult;

pub type ApiRequestHandler = fn(&Request<Vec<u8>>, Params, RequestContext) -> Option<ApiResult>;

pub struct RequestContext {
    pub project: Arc<Mutex<Project>>,
}

pub fn route(
    request: Request<Vec<u8>>,
    router: Arc<Mutex<Router<ApiRequestHandler>>>,
    context: RequestContext,
    responder: RequestAsyncResponder,
) {
    let path = request.uri().path();

    info!("Received request: {}", request.uri().path());

    let response_builder = Response::builder();

    #[cfg(debug_assertions)]
    let response_builder =
        response_builder.header("Access-Control-Allow-Origin", "http://localhost:3000");

    if path.starts_with("/api") {
        handle_api(request, router, context, response_builder, responder);
    } else {
        embedded_files::get(path.to_string(), response_builder, responder);
    }
}

fn handle_api(
    request: Request<Vec<u8>>,
    api_router: Arc<Mutex<Router<ApiRequestHandler>>>,
    context: RequestContext,
    response_builder: Builder,
    responder: RequestAsyncResponder,
) {
    // Some api interactions may have lots of IO interactions, so we run in a different thread so as to not freeze the UI
    std::thread::spawn(move || {
        let router = api_router.lock().unwrap();
        let m = router.at(request.uri().path());

        let result = match m {
            Ok(m) => {
                let handler = m.value;
                match handler(&request, m.params, context) {
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
                            warn!("Api Error: {}", msg);
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
            Err(_) => response_builder
                .status(404)
                .body(Cow::Owned("Not found".into()))
                .unwrap(),
        };
        responder.respond(result);
    });
}
