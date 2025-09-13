use log::{debug, error};
use matchit::Params;
use serde_json::json;
use url::Url;
use wry::http::Request;

use crate::{
    core::commands::{Command, CommandContext, CommandType},
    gui::{
        api_result::{ApiResult, ApiResultType},
        router::{ApiEntry, RequestContext},
    },
};

pub fn register_routes(router: &mut matchit::Router<ApiEntry>) {
    router
        .insert(
            "/api/v1/command/{command_name}",
            ApiEntry {
                handler: do_command,
                threaded: true,
            },
        )
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

        let mut value = serde_json::Value::String(pair.1.to_string());

        if pair.1 == "true" {
            value = serde_json::Value::Bool(true)
        }

        if pair.1 == "false" {
            value = serde_json::Value::Bool(false)
        }

        match obj.insert(pair.0.to_string(), value) {
            Some(_) => return Some(ApiResult::Error("Conflicting arguments".to_string())),
            None => (),
        }
    }

    let command = serde_json::from_value::<CommandType>(value);
    match command {
        Ok(command) => {
            debug!("Got command: {:?}", command);
            let command_result =
                CommandType::execute(command, &context.project, CommandContext { is_cli: false });

            match command_result {
                Ok(value) => match value {
                    Some(value) => Some(ApiResult::Ok(ApiResultType::Json(value))),
                    None => Some(ApiResult::Ok(ApiResultType::None)),
                },
                Err(err) => Some(ApiResult::Error(err.to_string())),
            }
        }
        Err(err) => {
            debug!("Missing command type: {}", id);
            error!("{:?}", err);
            Some(ApiResult::Error("Unknown command".to_string()))
        }
    }
}
