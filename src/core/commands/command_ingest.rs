use std::{fmt::format, fs::OpenOptions, path::PathBuf, sync::RwLock, time::SystemTime};

use clap::{command, Args};
use log::{error, info, warn};
use time::OffsetDateTime;
use ts_rs::TS;

use super::{
    args::CommonArgs, command_setup::SetupArgs, error::CommandError, Command, CommandContext,
};
use crate::{
    core::{program, project::Project, shot::shot_resolver::ShotResolver},
    utils,
};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct IngestArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,

    #[arg(long)]
    target_format: Option<String>,

    #[clap(long)]
    file: Option<String>,

    #[clap(long)]
    license: Option<String>,

    #[clap(long)]
    source: Option<String>,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct IngestResult {
    original_file: Option<String>,
    new_file: Option<String>,
    new_license_file: Option<String>,
    script: Option<String>,
}

impl Command for IngestArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() || self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let setup = SetupArgs {
            common: self.common,
            file_format: "".to_string(),
            dry: true,
        };

        let project_path = project.try_read().unwrap().get_root_directory();

        let result = Command::execute(setup, project, context);

        let folder = match result {
            Ok(val) => match val {
                Some(result) => match result {
                    serde_json::Value::Object(map) => Some(
                        map.get("folder")
                            .unwrap()
                            .clone()
                            .as_str()
                            .unwrap()
                            .to_string(),
                    ),
                    _ => None,
                },
                None => None,
            },
            Err(_) => None,
        };

        let mut result = IngestResult {
            original_file: self.file.clone(),
            new_file: None,
            script: None,
            new_license_file: None,
        };

        match folder {
            Some(folder) => {
                let mut path = PathBuf::from(folder);
                path.push("ingest");

                _ = std::fs::create_dir_all(&path);

                match self.file.clone() {
                    Some(file) => {
                        let mut file_path = path.clone();
                        let original = PathBuf::from(file.clone());
                        file_path.push(original.file_name().unwrap());

                        match std::fs::copy(&original, &file_path) {
                            Ok(_) => info!("Copied file to {}", file_path.to_str().unwrap()),
                            Err(err) => {
                                return Err(CommandError::Message(format!(
                                    "Failed to copy file!: {:?}",
                                    err
                                )))
                            }
                        }

                        result.new_file = Some(file_path.to_str().unwrap().to_string());

                        let file_relative = file_path.strip_prefix(&project_path).unwrap();

                        match self.source {
                            Some(source) => {
                                let mut sources_path = project_path.clone();
                                sources_path.push("logs");
                                std::fs::create_dir_all(&sources_path);

                                sources_path.push("ingest_sources.csv");

                                let exists = std::fs::exists(&sources_path).unwrap();

                                let mut file = OpenOptions::new()
                                    .write(true)
                                    .append(true)
                                    .create(true)
                                    .open(&sources_path)
                                    .unwrap();

                                if !exists {
                                    writeln!(file, "FILE,ORIGINAL_FILE_NAME,ORIGINAL_FILE_PATH,SOURCE,TIMESTAMP,USER");
                                }

                                let now = OffsetDateTime::now_local().unwrap();

                                write!(file, "{},", file_relative.to_str().unwrap());
                                write!(file, "{},", original.file_name().unwrap().to_str().unwrap());
                                write!(file, "{},", original.to_str().unwrap());
                                write!(file, "{},", source);
                                write!(file, "{},", now);
                                writeln!(file, "{},", whoami::username());

                            }
                            None => warn!("No source for this ingest has been specified! continuing regardless"),
                        }
                    }
                    None => (),
                }

                match self.license {
                    Some(license) => {
                        let mut license_path = path.clone();
                        license_path.push("license");
                        _ = std::fs::create_dir_all(&license_path);

                        let original = PathBuf::from(license.clone());
                        let file_name = PathBuf::from(self.file.unwrap());
                        let file_name = file_name.file_stem().unwrap().to_str().unwrap();
                        let license_file_name = original.file_name().unwrap().to_str().unwrap();
                        license_path.push(format!("{} - {}", file_name, license_file_name));

                        info!(
                            "Copying license from {} to: {}",
                            license,
                            license_path.to_str().unwrap()
                        );

                        match std::fs::copy(&original, &license_path) {
                            Ok(_) => {
                                info!("Copied license file to {}", license_path.to_str().unwrap());
                                result.new_license_file =
                                    Some(license_path.to_str().unwrap().to_string());
                            }
                            Err(err) => {
                                return Err(CommandError::Message(format!(
                                    "Failed to copy license file!: {:?}",
                                    err
                                )))
                            }
                        }
                    }
                    None => (),
                }

                let project = project.read().unwrap();
                let program = project.programs.get("ingest");

                let program = match program {
                    Some(program) => program,
                    None => {
                        return Err(CommandError::Message(
                            "No 'ingest' program specified".to_string(),
                        ))
                    }
                };

                match self.target_format {
                    Some(file_format) => {
                        let script = program.exports.get(&file_format);

                        match script {
                            Some(script) => {
                                info!("Ingesting with script: {}", script);

                                let mut script_path = project.get_root_directory();
                                script_path.push("scripts");
                                script_path.push("ingest");
                                script_path.push(script);

                                match std::fs::read_to_string(script_path) {
                                    Ok(text) => {
                                        result.script = Some(text);
                                    }
                                    Err(_) => {
                                        warn!("Script file not found!");
                                    }
                                }
                            }
                            None => {
                                return Err(CommandError::Message(format!(
                                    "No script has been specified for the format {}",
                                    file_format
                                )))
                            }
                        }
                    }
                    None => warn!("No target format was specified, so we cant run any script! continuing on regardless"),
                }
            }
            None => {
                return Err(CommandError::Message(
                    "Failed to get setup folder".to_string(),
                ))
            }
        }

        return Ok(Some(serde_json::to_value(result).unwrap()));
    }
}
