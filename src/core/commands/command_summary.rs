use clap::Args;
use log::info;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::project::Project;

use super::{error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SummaryArgs {}

impl Command for SummaryArgs {
    fn execute(
        self,
        project: &mut Project,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Project Summary:");
        info!("Identifier: {}", project.get_identifier());
        info!("Display Name: {}", project.get_display_name());

        Ok(Some(json!({
            "identifier": project.get_identifier(),
            "display_name": project.get_display_name()
        })))
    }
}
