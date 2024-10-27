use clap::Args;
use log::info;

use serde::{Deserialize, Serialize};

use crate::core::project::Project;

use super::{error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SummaryArgs {}

impl Command for SummaryArgs {
    fn execute(self, project: &mut Project) -> Result<(), CommandError> {
        info!("Project Summary:");
        info!("Identifier: {}", project.get_identifier());
        info!("Display Name: {}", project.get_display_name());
        Ok(())
    }
}
