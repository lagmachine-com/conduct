use matchit::Params;
use serde_json::json;
use wry::http::Request;

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
}

fn exit(
    _request: &Request<Vec<u8>>,
    _params: Params,
    _context: RequestContext,
) -> Option<ApiResult> {
    write_command_result(json!({"result": "ok"}));
    Some(ApiResult::OkExit)
}
