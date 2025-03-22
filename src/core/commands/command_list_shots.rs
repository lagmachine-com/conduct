use std::sync::RwLock;

use clap::{command, Args};
use ts_rs::TS;

use crate::core::project::Project;
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};
use crate::core::shot::shot_resolver::ShotResolver;

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ListShotsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct ListShotsResult {
    pub shots: Vec<String>,
}

impl Command for ListShotsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();

        let result = ListShotsResult {
            shots: project.get_shots(),
        };

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
