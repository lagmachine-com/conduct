use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CommandError {
    InvalidArguments,
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CommandError::InvalidArguments => write!(f, "Invalid arguments"),
        }
    }
}
