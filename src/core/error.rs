use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug)]
pub enum ProjectError {
    Message(String),
}

impl Error for ProjectError {}

impl fmt::Display for ProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectError::Message(msg) => write!(f, "{}", msg),
        }
    }
}
