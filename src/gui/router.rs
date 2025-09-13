use std::{
    borrow::Cow,
    sync::{Arc, RwLock},
};

use log::{info, warn};
use matchit::{Params, Router};
use serde_json::json;
use wry::{
    http::{response::Builder, Request, Response},
    RequestAsyncResponder,
};

use crate::{
    core::project::Project,
    gui::{api_result::ApiResultType, embedded_files},
};

use super::api_result::ApiResult;

pub type ApiRequestHandler = fn(&Request<Vec<u8>>, Params, RequestContext) -> Option<ApiResult>;

pub struct ApiEntry {
    pub handler: ApiRequestHandler,
    pub threaded: bool,
}

#[derive(Debug)]
pub enum ApiControlFlowResult {
    Close,
}

pub struct RequestContext {
    pub project: Arc<RwLock<Project>>,
}

pub fn route(
    request: Request<Vec<u8>>,
    router: Arc<RwLock<Router<ApiEntry>>>,
    context: RequestContext,
    responder: RequestAsyncResponder,
) -> Option<ApiControlFlowResult> {
    let path = request.uri().path();

    info!(
        "Received request: {}",
        request.uri().path_and_query().unwrap().as_str()
    );

    let response_builder = Response::builder();

    #[cfg(debug_assertions)]
    let response_builder =
        response_builder.header("Access-Control-Allow-Origin", "http://localhost:3000");

    let use_thread = match router.read() {
        Ok(router) => {
            let entry = router.at(path);
            match entry {
                Ok(entry) => Some(entry.value.threaded),
                Err(_) => None,
            }
        }
        Err(_) => None,
    };

    info!("Use thread: {:?}", use_thread);

    match use_thread {
        Some(use_thread) => {
            info!("Executing api request! (threaded: {})", use_thread);
            match use_thread {
                true => handle_api_threaded(request, router, context, response_builder, responder),
                false => handle_api(request, router, context, response_builder, responder),
            }
        }
        None => {
            embedded_files::get(path.to_string(), response_builder, responder);
            None
        }
    }
}

fn handle_api_threaded(
    request: Request<Vec<u8>>,
    api_router: Arc<RwLock<Router<ApiEntry>>>,
    context: RequestContext,
    response_builder: Builder,
    responder: RequestAsyncResponder,
) -> Option<ApiControlFlowResult> {
    // Some api interactions may have lots of IO interactions, so we run in a different thread so as to not freeze the UI
    std::thread::spawn(move || {
        handle_api(request, api_router, context, response_builder, responder);
    });

    None
}

fn handle_api(
    request: Request<Vec<u8>>,
    api_router: Arc<RwLock<Router<ApiEntry>>>,
    context: RequestContext,
    response_builder: Builder,
    responder: RequestAsyncResponder,
) -> Option<ApiControlFlowResult> {
    let router = api_router.read().unwrap();
    let m = router.at(request.uri().path());

    let mut control_flow_result: Option<ApiControlFlowResult> = None;

    let result = match m {
        Ok(m) => {
            let handler = m.value;
            match (handler.handler)(&request, m.params, context) {
                Some(result) => match result {
                    ApiResult::Ok(value) => match value {
                        ApiResultType::Json(value) => response_builder
                            .status(200)
                            .header("Content-Type", "text/json")
                            .body(Cow::Owned::<[u8]>(
                                serde_json::to_string(&value).unwrap().into(),
                            ))
                            .unwrap(),
                        ApiResultType::Binary(value) => response_builder
                            .status(200)
                            .body(Cow::Owned::<[u8]>(value))
                            .unwrap(),
                        ApiResultType::None => response_builder
                            .status(200)
                            .body(Cow::Owned("Ok".into()))
                            .unwrap(),
                    },
                    ApiResult::Error(msg) => {
                        let result = json!({"error": serde_json::Value::String(msg.clone())});
                        warn!("Api Error: {}", msg);
                        response_builder
                            .status(400)
                            .body(Cow::Owned::<[u8]>(
                                serde_json::to_string(&result).unwrap().into(),
                            ))
                            .unwrap()
                    }
                    ApiResult::OkExit => {
                        control_flow_result = Some(ApiControlFlowResult::Close);
                        response_builder
                            .status(200)
                            .body(Cow::Owned("Ok".into()))
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

    return control_flow_result;
}
