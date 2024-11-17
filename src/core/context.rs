#[derive(Debug)]
pub enum ContextMode {
    Load,
    Export,
}

#[derive(Debug)]
pub struct Context {
    pub department: Option<String>,
    pub mode: ContextMode,
    pub shot: Option<String>,
}
