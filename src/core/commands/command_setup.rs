use std::{sync::RwLock, thread, time::Duration};

use clap::{command, Args};
use log::info;
use serde_json::json;
use ts_rs::TS;

use crate::core::project::Project;
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SetupArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/list_assets_result.ts")]
pub struct SetupResult {
    pub asset: String,
    pub department: String,
    pub path: String,
}

impl Command for SetupArgs {
    fn execute(
        self,
        _project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() || self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        Ok(Some(
            serde_json::to_value(SetupResult {
                asset: self.common.asset.unwrap(),
                department: self.common.department.unwrap(),
                path: "/".to_string(),
            })
            .unwrap(),
        ))
    }
}
