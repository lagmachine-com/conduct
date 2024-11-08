use std::sync::RwLock;

use clap::{command, Args};
use log::{info, warn};

use serde::{Deserialize, Serialize};

use crate::core::{project::Project, version_control::VersionControl};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ExportArgs {
    #[command(flatten)]
    #[serde(flatten)]
    pub common: CommonArgs,

    #[arg(long, help = "Which program is being used to export this asset")]
    pub from: String,

    #[arg(long, help = "Which file format to export with")]
    pub file_format: String,
}

impl Command for ExportArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Exporting Asset!");

        if self.common.asset.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        if self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        if self.common.element.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let project = project.read().unwrap();

        let result = VersionControl::export(&project.version_control, &project, &self);

        if result.is_err() {
            warn!("VersionControl export result was an error");
            return Err(CommandError::Unknown);
        }

        let mut result = result.unwrap();

        let dept = match project.departments.get(&self.common.department.unwrap()) {
            Some(dept) => dept,
            None => {
                warn!("Could not find department");
                return Err(CommandError::InvalidArguments);
            }
        };

        let program = dept.programs.as_ref().unwrap().get(&self.from);

        let program = match program {
            Some(program) => program,
            None => {
                warn!("Could not find program");
                return Err(CommandError::InvalidArguments);
            }
        };

        let script_name = match program.exports.as_ref() {
            Some(exports) => match exports.get(&self.file_format) {
                Some(script) => script,
                None => {
                    warn!("Could not find program export");
                    return Err(CommandError::InvalidArguments);
                }
            },
            None => {
                warn!("Program did not have any exports");
                return Err(CommandError::InvalidArguments);
            }
        };

        let mut script_path = project.get_root_directory();
        script_path.push("scripts");
        script_path.push(self.from);
        script_path.push(script_name);

        result.script = script_path.to_str().unwrap().to_string();

        match std::fs::read_to_string(script_path) {
            Ok(text) => {
                result.script = text;
                Ok(Some(serde_json::to_value(result).unwrap()))
            }
            Err(_) => {
                warn!("Script file not found!");
                Err(CommandError::ScriptNotFound)
            }
        }
    }
}
