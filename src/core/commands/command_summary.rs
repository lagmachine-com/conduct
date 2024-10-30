use clap::Args;
use log::info;

use serde::{Deserialize, Serialize};
use serde_json::json;
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
}

impl Command for SummaryArgs {
    fn execute(
        self,
        project: &mut Project,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Project Summary:");
        info!("Identifier: {}", project.get_identifier());
        info!("Display Name: {}", project.get_display_name());

        let result = SummaryResponse {
            display_name: project.get_display_name(),
            identifier: project.get_identifier(),
        };

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
