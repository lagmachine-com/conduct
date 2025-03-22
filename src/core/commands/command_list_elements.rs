use std::sync::RwLock;

use clap::{command, Args};
use ts_rs::TS;

use crate::core::{
    context::{Context, ContextMode},
    element::element_resolver::ElementResolver,
    project::Project,
};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ListElementsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,

    #[clap(
        help = "List elements in load context instead of export",
        long,
        short,
        action,
        default_value_t = false
    )]
    load: bool,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct ListElementsResult {
    pub elements: Vec<String>,
}

impl Command for ListElementsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let context = Context {
            department: self.common.department,
            shot: self.common.shot,
            mode: if self.load {
                ContextMode::Load
            } else {
                ContextMode::Export
            },
        };

        let project = project.read().unwrap();
        let mut result = ListElementsResult {
            elements: project
                .get_elements(self.common.asset.unwrap(), &context)
                .keys()
                .into_iter()
                .map(|f| f.to_string())
                .collect(),
        };

        result.elements.sort();

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
