use std::borrow::Cow;

use include_directory::include_directory;
use log::{debug, trace};
use wry::{
    http::{response::Builder, Response},
    RequestAsyncResponder,
};

static UI_FILES: include_directory::Dir = include_directory!("$CARGO_MANIFEST_DIR/ui/dist");

pub fn get(path: String, response_builder: Builder, responder: RequestAsyncResponder) {
    let path = match path.as_str() {
        "/" => "index.html",
        _ => path.as_str().trim_start_matches("/"),
    }
    .to_string();

    trace!("Looking for file: {}", path);

    let file = UI_FILES.get_file(path);

    let file = match file {
        Some(file) => Some(file),
        None => UI_FILES.get_file("index.html"),
    };

    let response: Response<Cow<'static, [u8]>> = match file {
        Some(file) => {
            trace!("Found file! mime: {}", file.mimetype_as_string());

            response_builder
                .status(200)
                .header("Content-Type", file.mimetype_as_string())
                .body(Cow::Owned(file.contents().into()))
                .unwrap()
        }
        None => response_builder
            .status(404)
            .body(Cow::Owned("Not found".into()))
            .unwrap(),
    };

    responder.respond(response);
}
