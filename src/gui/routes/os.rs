use std::{
    collections::HashMap,
    fs::File,
    io::{BufReader, Read},
    os,
    process::Command,
};

use log::{info, warn};
use matchit::Params;
use serde_json::json;
use url::Url;
use urlencoding::decode;
use wry::http::{Method, Request};

use crate::{
    core::commands::write_command_result,
    gui::{
        api_result::{ApiResult, ApiResultType},
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

    router
        .insert(
            "/os/file",
            ApiEntry {
                handler: file,
                threaded: true,
            },
        )
        .unwrap();
}

fn file(
    request: &Request<Vec<u8>>,
    _params: Params,
    _context: RequestContext,
) -> Option<ApiResult> {
    if request.method() != Method::GET {
        return Some(ApiResult::Error("Invalid http method".to_string()));
    }

    let url = Url::parse(request.uri().to_string().as_str()).unwrap();

    let hash_query: HashMap<_, _> = url.query_pairs().into_owned().collect();

    let path = hash_query.get("path");

    match path {
        Some(path) => {
            let path = decode(path).unwrap();

            info!("Reading file at path: {}", path);

            let f = File::open(path.to_string()).unwrap();
            let mut reader = BufReader::new(f);
            let mut buffer = Vec::new();

            reader.read_to_end(&mut buffer).unwrap();

            info!("Returning {} bytes", buffer.len());
            return Some(ApiResult::Ok(ApiResultType::Binary(buffer)));
        }
        None => todo!(),
    }
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

    Some(ApiResult::Ok(ApiResultType::None))
}
