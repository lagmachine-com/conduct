use log::{error, info};
use serde_json::json;
use std::borrow::Cow;
use wry::http::{response::Builder, Method, Request, Response};

use super::router::RequestContext;
use crate::core::commands::{Command, CommandType};

enum ApiResult {
    Ok(Option<serde_json::Value>),
    Error(String),
}

pub fn handle(
    request: &Request<Vec<u8>>,
    response_builder: Builder,
    context: RequestContext,
) -> Response<Cow<'static, [u8]>> {
    match handle_request(request, context) {
        Some(response) => match response {
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

fn handle_request(request: &Request<Vec<u8>>, context: RequestContext) -> Option<ApiResult> {
    match request.method() {
        &Method::GET => handle_get_request(request, context),
        &Method::POST => handle_post_request(request, context),
        _ => todo!(),
    }
}

fn handle_get_request(request: &Request<Vec<u8>>, context: RequestContext) -> Option<ApiResult> {
    let project = context.project.lock().unwrap();

    match request.uri().path() {
        "/api/json" => Some(ApiResult::Ok(Some(json!({
            "name": project.get_display_name(),
            "id": project.get_identifier()
        })))),
        _ => None,
    }
}

fn handle_post_request(request: &Request<Vec<u8>>, context: RequestContext) -> Option<ApiResult> {
    match request.uri().path() {
        "/api/command" => Some(handle_post_command(request, context)),
        _ => None,
    }
}

fn handle_post_command(request: &Request<Vec<u8>>, context: RequestContext) -> ApiResult {
    let body = request.body();

    let s = match std::str::from_utf8(body) {
        Ok(v) => v,
        Err(_) => return ApiResult::Error("Invalid UTF-8 sequence".to_string()),
    };

    let value = serde_json::from_str::<CommandType>(s);
    let mut project = context.project.lock().unwrap();

    info!("received request body: {}", s);

    match value {
        Ok(command) => {
            info!("Executing command: {:?}", command);
            _ = Command::execute(command, &mut project);
            ApiResult::Ok(None)
        }
        Err(_) => ApiResult::Error("Invalid json".to_string()),
    }
}
