use serde_json::json;
use std::borrow::Cow;
use wry::http::{response::Builder, Request, Response};

use super::router::RequestContext;

pub fn handle(
    request: &Request<Vec<u8>>,
    response_builder: Builder,
    context: RequestContext,
) -> Response<Cow<'static, [u8]>> {
    match handle_request(request, context) {
        Some(response) => response_builder
            .header("Content-Type", "text/json")
            .status(200)
            .body(Cow::Owned::<[u8]>(
                serde_json::to_string(&response).unwrap().into(),
            ))
            .unwrap(),
        None => response_builder
            .status(404)
            .body(Cow::Owned("Not found".into()))
            .unwrap(),
    }
}

fn handle_request(
    request: &Request<Vec<u8>>,
    context: RequestContext,
) -> Option<serde_json::Value> {
    let project = context.project.lock().unwrap();

    match request.uri().path() {
        "/api/json" => Some(json!({
            "name": project.get_display_name(),
            "id": project.get_identifier()
        })),
        _ => None,
    }
}
