use std::sync::RwLock;

use clap::{command, Args};
use log::warn;
use ts_rs::TS;

use crate::core::{element::ElementFinder, project::Project};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ListElementsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct ListElementsResult {
    pub elements: Vec<String>,
}

impl Command for ListElementsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();

        let asset_name = match self.common.asset {
            Some(asset) => asset,
            None => {
                warn!("No asset specified");
                return Err(CommandError::InvalidArguments);
            }
        };

        let _asset = match project.get_asset_by_name(asset_name.clone()) {
            Some(asset) => asset,
            None => {
                warn!("Asset does not exist");
                return Err(CommandError::InvalidArguments);
            }
        };

        let result = ListElementsResult {
            elements: project.get_elements_for_asset(asset_name, self.common.department),
        };

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
