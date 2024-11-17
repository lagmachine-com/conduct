use std::sync::RwLock;

use clap::{command, Args};
use log::warn;
use ts_rs::TS;

use crate::core::{project::Project, shot::shot_resolver::ShotResolver};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SetupArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,

    #[arg(short, long)]
    pub dry: bool,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct SetupResult {
    pub asset: String,
    pub department: String,
    pub path: String,
    pub file_name: String,
    pub shot: Option<String>,
}

impl Command for SetupArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() || self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let department = self.common.department.clone().unwrap();
        let asset = self.common.asset.clone().unwrap();
        let mut dir_path = project.read().unwrap().get_root_directory();
        dir_path.push("setup");

        match &self.common.shot {
            Some(shot) => {
                if project.read().unwrap().shot_exists(shot) {
                    dir_path.push("shot");
                    for part in shot.split("/").into_iter() {
                        dir_path.push(part);
                    }
                } else {
                    warn!("Invalid shot: {}", shot);
                    return Err(CommandError::InvalidArguments);
                }
            }
            None => {
                dir_path.push("asset");
            }
        }

        let mut shot_code: Option<String> = None;

        let file_name = match &self.common.shot {
            Some(shot) => {
                shot_code = project.read().unwrap().get_shot_formatted(shot);
                format!(
                    "{}_{}_{}",
                    asset,
                    department,
                    shot_code.clone().unwrap().replace("/", "-")
                )
            }
            None => format!("{}_{}", asset, department),
        };

        dir_path.push(&department);
        dir_path.push(&asset);

        if self.dry == false {
            _ = std::fs::create_dir_all(&dir_path);
        }

        let files = std::fs::read_dir(&dir_path);

        if self.dry {
            match files {
                Ok(files) => {
                    for file in files.into_iter() {
                        match file {
                            Ok(file) => {
                                let name = file
                                    .path()
                                    .file_stem()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();

                                let file_name_with_ext =
                                    file.file_name().to_str().unwrap().to_string();

                                if name == file_name {
                                    return Err(CommandError::Message(format!(
                                    "File {file_name_with_ext} already exists! Continuing may overwrite this file and result in a loss of work"
                                )));
                                }
                            }
                            Err(_) => (),
                        }
                    }
                }
                Err(_) => (),
            }
        }

        Ok(Some(
            serde_json::to_value(SetupResult {
                asset: self.common.asset.unwrap(),
                department: self.common.department.unwrap(),
                path: dir_path.to_str().unwrap().to_string(),
                file_name: file_name,
                shot: shot_code,
            })
            .unwrap(),
        ))
    }
}
