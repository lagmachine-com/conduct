use std::{
    borrow::Cow,
    sync::{Arc, Mutex},
};

use log::info;
use wry::{http::Request, http::Response};

use crate::{
    core::project::Project,
    gui::{api, embedded_files},
};

pub struct RequestContext {
    pub project: Arc<Mutex<Project>>,
}

pub fn route(
    _id: &str,
    request: Request<Vec<u8>>,
    context: RequestContext,
) -> Response<Cow<'static, [u8]>> {
    let path = request.uri().path();

    info!("Received request: {}", request.uri().path());

    if path.starts_with("/api") {
        if let Some(response) = api::handle(&request) {
            return response;
        }
    }

    if let Some(response) = embedded_files::get(path.to_string()) {
        return response;
    }

    Response::builder()
        .status(404)
        .header("Content-Type", "text/html; charset=utf-8")
        .body(Cow::Owned(
            context.project.lock().unwrap().get_display_name().into(),
        ))
        .unwrap()
}
