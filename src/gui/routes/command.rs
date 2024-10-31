use log::debug;
use matchit::Params;
use serde_json::json;
use wry::http::Request;

use crate::{
    core::commands::{Command, CommandType},
    gui::{
        api_result::ApiResult,
        router::{ApiRequestHandler, RequestContext},
    },
};

pub fn register_routes(router: &mut matchit::Router<ApiRequestHandler>) {
    router
        .insert("/api/command/{command_name}", do_command)
        .unwrap();
}

fn do_command(
    _request: &Request<Vec<u8>>,
    params: Params,
    context: RequestContext,
) -> Option<ApiResult> {
    let command = params.get("command_name");
    if command.is_none() {
        return Some(ApiResult::Error(
            "Missing parameter 'command_name'".to_string(),
        ));
    }

    let id = command.unwrap();

    let value = json!({
        "type": id
    });

    let command = serde_json::from_value::<CommandType>(value);
    match command {
        Ok(command) => {
            debug!("Got command: {:?}", command);
            let mut m = context.project.lock().unwrap();
            let command_result = CommandType::execute(command, &mut m);

            match command_result {
                Ok(value) => Some(ApiResult::Ok(value)),
                Err(err) => Some(ApiResult::Error(err.to_string())),
            }
        }
        Err(_) => {
            debug!("Missing command type: {}", id);
            Some(ApiResult::Error("Unknown command".to_string()))
        }
    }
}
