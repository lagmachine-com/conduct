use std::process::Command;

use log::{info, warn};
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
            "/os/execute",
            ApiEntry {
                handler: execute,
                threaded: true,
            },
        )
        .unwrap();
}

fn execute(
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

    info!("Executing os command: {}", s);

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd").args(["/C", s]).output()
    } else {
        Command::new("sh").arg("-c").arg(s).output()
    };

    match output {
        Ok(output) => {
            info!("status: {}", output.status);
            info!("stdout: {}", String::from_utf8_lossy(&output.stdout));
            info!("stderr: {}", String::from_utf8_lossy(&output.stderr));
        }
        Err(_) => {
            warn!("Failed to execute command! :(")
        }
    }

    Some(ApiResult::Ok(None))
}
