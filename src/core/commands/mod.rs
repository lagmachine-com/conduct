mod command_create;
mod command_dialog;
mod command_export;
mod command_get_asset_tree;
mod command_ingest;
mod command_list_assets;
mod command_list_elements;
mod command_list_export_formats;
mod command_list_shots;
mod command_load_assets;
mod command_resolve_elements;
mod command_save;
mod command_setup;
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
pub use command_export::ExportArgs;

use command_get_asset_tree::GetAssetTreeArgs;
use command_ingest::IngestArgs;
use command_list_assets::ListAssetsArgs;
use command_list_elements::ListElementsArgs;
use command_list_export_formats::ListExportFormatsArgs;
use command_list_shots::ListShotsArgs;
use command_load_assets::LoadAssetsArgs;
use command_resolve_elements::ResolveElementsArgs;
use command_save::SaveArgs;
use command_setup::SetupArgs;
use command_summary::SummaryArgs;
use log::{info, warn};

use crate::core::project::Project;
use enum_dispatch::enum_dispatch;
use error::CommandError;
use serde::{Deserialize, Serialize};

pub struct CommandContext {
    pub is_cli: bool,
}

#[enum_dispatch]
pub trait Command {
    fn execute(
        self,
        _project: &RwLock<Project>,
        context: CommandContext,
    ) -> Result<Option<serde_json::Value>, CommandError>;
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

    /// Configure a new setup file
    Setup(SetupArgs),

    /// Get a list of all assets, optionally filtered by department
    ListAssets(ListAssetsArgs),

    /// Lists all available export file formats for a given program and department
    ListExportFormats(ListExportFormatsArgs),

    /// List all elements of an asset, optionally filtering by department
    ListElements(ListElementsArgs),

    /// List all shots
    ListShots(ListShotsArgs),

    ResolveElements(ResolveElementsArgs),

    GetAssetTree(GetAssetTreeArgs),

    LoadAssets(LoadAssetsArgs),

    Save(SaveArgs),

    Ingest(IngestArgs),
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
