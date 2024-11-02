use std::{sync::RwLock, thread, time::Duration};

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
        _project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        thread::sleep(Duration::from_secs(5));
        info!("Returning result from command create!");
        Ok(None)
    }
}
