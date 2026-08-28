use std::{collections::HashMap, sync::RwLock};

use clap::{command, Args};
use log::{info, warn};
use ts_rs::TS;

use crate::core::{project::Project, shot::shot_resolver::ShotResolver};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct SetupArgs {
    #[command(flatten)]
    #[serde(flatten)]
    pub common: CommonArgs,

    #[arg(short, long)]
    pub file_format: String,

    #[arg(long)]
    pub dry: bool,
}

#[derive(Debug, Args, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct SetupResult {
    pub asset: String,
    pub department: String,
    pub folder: String,
    pub file_name: String,
    pub path: String,
    pub shot: Option<String>,
}

impl Command for SetupArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() || self.common.department.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        // shot is swapped for 'asset' if not in a shot

        const DEFAULT_PATH: &str = "setup/${shot}/${department}/${asset}";

        let department = self.common.department.clone().unwrap();
        let asset = self.common.asset.clone().unwrap();

        let mut map = HashMap::<&str, String>::new();
        map.insert("department", department.clone());
        map.insert("asset", asset.clone());

        match &self.common.shot {
            Some(shot) => {
                map.insert("shot", "shot/".to_owned() + &shot.clone());
            }
            None => {
                map.insert("shot", "asset".into());
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

        let mut resolved_path: String = match &project.read().unwrap().setup_path_template {
            Some(s) => s.clone(),
            None => DEFAULT_PATH.to_string(),
        };

        for (key, value) in map.iter() {
            let replace = "${".to_string() + &key.to_string() + "}";
            resolved_path = resolved_path.replace(&replace, value);
        }

        let mut new_dir_path = project.read().unwrap().get_root_directory();
        for entry in resolved_path.split("/") {
            new_dir_path.push(entry);
        }

        if self.dry == false {
            _ = std::fs::create_dir_all(&new_dir_path);
        }

        let file_name_with_ext = file_name.clone() + &self.file_format;

        let mut path = new_dir_path.clone();
        path.push(&file_name_with_ext);

        if self.dry {
            let exists = std::fs::exists(&path);
            match exists {
                Ok(exists) => match exists {
                    true => {
                        return Err(CommandError::Message(format!("{file_name_with_ext} already exists! Continuing may result in loss of work")));
                    }
                    false => (),
                },
                Err(_) => (),
            }
        }

        Ok(Some(
            serde_json::to_value(SetupResult {
                asset: self.common.asset.unwrap(),
                department: self.common.department.unwrap(),
                folder: new_dir_path.to_str().unwrap().to_string(),
                path: path.to_str().unwrap().to_string(),
                file_name: file_name,
                shot: shot_code,
            })
            .unwrap(),
        ))
    }
}
