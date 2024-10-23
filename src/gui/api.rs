use std::borrow::Cow;
use wry::{http::Request, http::Response};

pub fn handle(_path: &Request<Vec<u8>>) -> Option<Response<Cow<'static, [u8]>>> {
    None
}
