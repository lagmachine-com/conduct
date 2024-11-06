use std::sync::RwLock;

use clap::{command, Args};
use log::info;

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
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Returning result from command create!");

        project.read().unwrap().save();

        Ok(None)
    }
}
