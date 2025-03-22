use std::{collections::BTreeMap, sync::RwLock};

use clap::{command, Args};
use log::info;

use crate::core::{
    asset::{Asset, AssetEntry},
    project::Project,
};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SaveArgs {}

impl Command for SaveArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Saving Project!");

        let project = project.read().unwrap();
        project.save();

        Ok(None)
    }
}
