use std::sync::RwLock;

use clap::{command, Args};
use ts_rs::TS;

use crate::core::{department::DepartmentFinder, project::Project};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ListAssetsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct ListAssetsResult {
    pub assets: Vec<String>,
}

impl Command for ListAssetsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();
        let assets = project.get_assets_flattened();

        let mut result = ListAssetsResult { assets: Vec::new() };

        for pair in assets.iter() {
            if let Some(filter_dept) = self.common.department.clone() {
                let departments = project.get_departments_for_asset(pair.0.clone());

                if departments.contains(&filter_dept) == false {
                    continue;
                }
            }

            result.assets.push(pair.0.clone());
        }

        result.assets.sort();

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
