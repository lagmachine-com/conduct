use clap::Subcommand;

use super::{command_create::CreateArgs, command_summary::SummaryArgs};

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Create a new entry in the project
    Create(CreateArgs),

    /// Summarize the project in computer-readable output
    Summary(SummaryArgs),
}
