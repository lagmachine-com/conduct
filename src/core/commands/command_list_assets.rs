use std::sync::RwLock;

use clap::{command, Args};
use ts_rs::TS;

use crate::core::project::Project;
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ListAssetsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/list_assets_result.ts")]
pub struct ListAssetsResult {
    pub assets: Vec<String>,
}

impl Command for ListAssetsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();
        let assets = project.get_assets_flattened();

        let mut result = ListAssetsResult { assets: Vec::new() };

        for pair in assets.iter() {
            if let Some(filter_dept) = self.common.department.clone() {
                if pair.1.departments.contains_key(&filter_dept) == false {
                    continue;
                }
            }

            result.assets.push(pair.1.name.clone());
        }

        result.assets.sort();

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
