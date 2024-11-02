use std::sync::RwLock;

use clap::{command, Args};
use log::info;

use serde::{Deserialize, Serialize};

use crate::core::project::Project;

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ExportArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

impl Command for ExportArgs {
    fn execute(
        self,
        _project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Exporting Asset!");

        if self.common.asset.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        if self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        Ok(None)
    }
}
