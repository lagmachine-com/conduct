use std::sync::RwLock;

use clap::{command, Args};
use log::warn;
use ts_rs::TS;

use crate::core::project::Project;
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ListExportFormatsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,

    #[arg(long, help = "Which program is being used to export this asset")]
    pub from: String,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct ListExportFormatsResult {
    pub formats: Vec<String>,
}

impl Command for ListExportFormatsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let project = project.read().unwrap();

        let dept = project.departments.get(&self.common.department.unwrap());

        let dept = match dept {
            Some(dept) => dept,
            None => {
                warn!("The specified department was not found");
                return Err(CommandError::InvalidArguments);
            }
        };

        let program = dept.programs.get(&self.from);

        let program = match program {
            Some(prog) => prog,
            None => {
                warn!("The specified program was not found in the department");
                return Err(CommandError::InvalidArguments);
            }
        };

        let mut result = ListExportFormatsResult {
            formats: Vec::new(),
        };

        result.formats = program.exports.clone();

        result.formats.sort();

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
