mod command_create;
mod command_export;
mod command_summary;

mod error;

use clap::Subcommand;

pub mod args;

use command_create::CreateArgs;
use command_export::ExportArgs;
use command_summary::SummaryArgs;

use crate::core::project::Project;
use enum_dispatch::enum_dispatch;
use error::CommandError;
use serde::{Deserialize, Serialize};

#[enum_dispatch]
pub trait Command {
    fn execute(self, _project: &mut Project) -> Result<(), CommandError>;
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[enum_dispatch(Command)]
pub enum CommandType {
    /// Create a new entry in the project
    Create(CreateArgs),

    /// Summarize the project in computer-readable output
    Summary(SummaryArgs),

    /// Export an asset
    Export(ExportArgs),
}
