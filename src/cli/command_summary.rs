use clap::Args;
use log::info;

use crate::core::project::Project;

use super::CliResult;

#[derive(Debug, Args)]
pub struct SummaryArgs {}

impl SummaryArgs {
    pub fn execute(self, project: &mut Project) -> CliResult {
        info!("Project Summary:");
        info!("Identifier: {}", project.get_identifier());
        info!("Display Name: {}", project.get_display_name());
        CliResult::Success
    }
}
