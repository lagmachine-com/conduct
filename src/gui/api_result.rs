#[derive(Debug)]
pub enum ApiResult {
    Ok(Option<serde_json::Value>),
    Error(String),
    OkExit,
}
