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

    let builder = Response::builder();

    #[cfg(debug_assertions)]
    let builder = builder.header("Access-Control-Allow-Origin", "http://localhost:3000");

    if path.starts_with("/api") {
        return api::handle(&request, builder, context);
    }

    return embedded_files::get(path.to_string(), builder);
}
