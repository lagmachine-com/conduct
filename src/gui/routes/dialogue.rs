use log::info;
use matchit::Params;
use serde_json::json;
use wry::http::{Method, Request};

use crate::{
    core::commands::write_command_result,
    gui::{
        api_result::ApiResult,
        router::{ApiEntry, RequestContext},
    },
};

pub fn register_routes(router: &mut matchit::Router<ApiEntry>) {
    router
        .insert(
            "/api/v1/dialog/exit",
            ApiEntry {
                handler: exit,
                threaded: false,
            },
        )
        .unwrap();

    router
        .insert(
            "/api/v1/dialog/cancel",
            ApiEntry {
                handler: cancel,
                threaded: false,
            },
        )
        .unwrap();
}

fn exit(
    request: &Request<Vec<u8>>,
    _params: Params,
    _context: RequestContext,
) -> Option<ApiResult> {
    if request.method() != Method::POST {
        return Some(ApiResult::Error("Invalid http method".to_string()));
    }

    let body = request.body();

    let s = match std::str::from_utf8(body) {
        Ok(v) => v,
        Err(_) => return Some(ApiResult::Error("Invalid body".to_string())),
    };

    info!("Closing with data: {}", s);

    let value: serde_json::Value = match serde_json::from_str(s) {
        Ok(v) => v,
        Err(_) => return Some(ApiResult::Error("Body was not valid json".to_string())),
    };

    write_command_result(json!({"result": "ok", "data": value}));
    Some(ApiResult::OkExit)
}

fn cancel(
    _request: &Request<Vec<u8>>,
    _params: Params,
    _context: RequestContext,
) -> Option<ApiResult> {
    write_command_result(json!({"result": "cancelled"}));
    Some(ApiResult::OkExit)
}
