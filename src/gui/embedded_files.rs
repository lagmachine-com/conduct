use std::borrow::Cow;

use include_directory::include_directory;
use log::debug;
use wry::http::Response;

static UI_FILES: include_directory::Dir = include_directory!("$CARGO_MANIFEST_DIR/ui/dist");

pub fn get(path: String) -> Option<Response<Cow<'static, [u8]>>> {
    let path = match path.as_str() {
        "/" => "index.html",
        _ => path.as_str().trim_start_matches("/"),
    }
    .to_string();

    debug!("Looking for file: {}", path);

    let file = UI_FILES.get_file(path);

    match file {
        Some(file) => {
            debug!("Found file! mime: {}", file.mimetype_as_string());
            Some(
                Response::builder()
                    .status(200)
                    .header("Content-Type", file.mimetype_as_string())
                    .body(Cow::Owned(file.contents().into()))
                    .unwrap(),
            )
        }
        None => None,
    }
}
