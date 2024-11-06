use std::sync::RwLock;

use clap::{command, Args};
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
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct SetupResult {
    pub asset: String,
    pub department: String,
    pub path: String,
    pub file_name: String,
}

impl Command for SetupArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() || self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let department = self.common.department.clone().unwrap();
        let asset = self.common.asset.clone().unwrap();
        let mut dir_path = project.read().unwrap().get_root_directory();
        dir_path.push("setup");
        dir_path.push(&department);
        dir_path.push(&asset);

        _ = std::fs::create_dir_all(&dir_path);

        let file_name = format!("{}_{}", asset, department);

        Ok(Some(
            serde_json::to_value(SetupResult {
                asset: self.common.asset.unwrap(),
                department: self.common.department.unwrap(),
                path: dir_path.to_str().unwrap().to_string(),
                file_name: file_name,
            })
            .unwrap(),
        ))
    }
}
