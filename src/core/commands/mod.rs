mod command_create;
mod command_dialog;
mod command_export;
mod command_summary;

mod error;

use std::{
    io::{self, Write},
    str,
    sync::RwLock,
};

use clap::Subcommand;

pub mod args;

use command_create::CreateArgs;
use command_dialog::DialogArgs;
pub use command_dialog::DialogOptions;
use command_export::ExportArgs;
use command_summary::SummaryArgs;
use log::{info, warn};

use crate::core::project::Project;
use enum_dispatch::enum_dispatch;
use error::CommandError;
use serde::{Deserialize, Serialize};

#[enum_dispatch]
pub trait Command {
    fn execute(self, _project: &RwLock<Project>)
        -> Result<Option<serde_json::Value>, CommandError>;
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

    /// Display a dialog and get the result
    Dialog(DialogArgs),
}

pub fn write_command_result(result: serde_json::Value) {
    let str = serde_json::to_string_pretty(&result).unwrap();
    info!("Writing command result to stdout");
    let mut stdout = io::stdout().lock();
    match stdout.write_all(str.as_bytes()) {
        Ok(_) => (),
        Err(_) => warn!("Failed to write data to stdout"),
    }
}
