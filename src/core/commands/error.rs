use std::{error::Error, fmt};

#[derive(Debug)]
pub enum CommandError {
    InvalidArguments,
}

impl Error for CommandError {}

impl fmt::Display for CommandError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Oh no, something bad went down")
    }
}
