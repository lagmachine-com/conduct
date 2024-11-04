use std::sync::RwLock;

use clap::Args;
use log::info;

use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::core::project::Project;

use super::{error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SummaryArgs {}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/summary_response.ts")]
pub struct SummaryResponse {
    pub display_name: String,
    pub identifier: String,
    pub assets_flat: Vec<String>,
    pub departments: Vec<String>,
}

impl Command for SummaryArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();

        info!("Project Summary:");
        info!("Identifier: {}", project.get_identifier());
        info!("Display Name: {}", project.get_display_name());

        let mut result = SummaryResponse {
            display_name: project.get_display_name(),
            identifier: project.get_identifier(),
            assets_flat: project
                .get_assets_flattened()
                .keys()
                .into_iter()
                .map(|f| f.to_string())
                .collect(),
            departments: project
                .departments
                .keys()
                .into_iter()
                .map(|f| f.to_string())
                .collect(),
        };

        result.assets_flat.sort();
        result.departments.sort();

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
