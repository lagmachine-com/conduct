#[derive(Debug)]
pub enum ApiResultType {
    Json(serde_json::Value),
    Binary(Vec<u8>),
    None,
}

#[derive(Debug)]
pub enum ApiResult {
    Ok(ApiResultType),
    Error(String),
    OkExit,
}
