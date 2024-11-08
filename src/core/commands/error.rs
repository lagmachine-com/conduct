use std::{
    error::Error,
    fmt::{self},
};

#[derive(Debug)]
pub enum CommandError {
    InvalidArguments,
    Unknown,
    ScriptNotFound,
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::InvalidArguments => write!(f, "Invalid arguments"),
            CommandError::Unknown => write!(f, "Unknown error occurred"),
            CommandError::ScriptNotFound => {
                write!(f, "Script file not found, or could not be read")
            }
        }
    }
}
