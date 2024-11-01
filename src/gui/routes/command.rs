use log::{debug, info};
use matchit::Params;
use serde_json::json;
use url::Url;
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
        .insert("/api/v1/command/{command_name}", do_command)
        .unwrap();
}

fn do_command(
    request: &Request<Vec<u8>>,
    params: Params,
    context: RequestContext,
) -> Option<ApiResult> {
    let command = params.get("command_name");
    if command.is_none() {
        return Some(ApiResult::Error(
            "Missing parameter 'command_name'".to_string(),
        ));
    }

    let url = Url::parse(request.uri().to_string().as_str()).unwrap();

    let id = command.unwrap();

    let mut value = json!({
        "type": id
    });

    for pair in url.query_pairs().into_iter() {
        let obj = value.as_object_mut().unwrap();
        match obj.insert(
            pair.0.to_string(),
            serde_json::Value::String(pair.1.to_string()),
        ) {
            Some(_) => return Some(ApiResult::Error("Conflicting arguments".to_string())),
            None => (),
        }
    }

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
