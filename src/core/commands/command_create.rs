use std::time::Duration;

use clap::{command, Args};

use crate::core::project::Project;
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct CreateArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

impl Command for CreateArgs {
    fn execute(
        self,
        project: &mut Project,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        std::thread::sleep(Duration::from_secs(5));
        Ok(None)
    }
}
