mod args;
mod result;
use std::{fs::File, io::Read, path::PathBuf};

use args::GlobalArgs;
use clap::Parser;
use log::*;
pub use result::CliResult;

use crate::core::commands::{Command, CommandType};

#[derive(Debug, Parser)]
#[command(name = "conduct")]
#[command(about = "Generic asset and version management for creative projects", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<CommandType>,

    #[command(flatten)]
    pub global_args: GlobalArgs,
}

fn get_project_directory(cli: &CLI) -> PathBuf {
    if let Some(project_dir) = &cli.global_args.project_dir {
        info!("Overriding project directory with: '{}'", project_dir);
        let path = PathBuf::from(project_dir);
        return path;
    } else {
        let mut path = std::env::current_exe().unwrap();
        path.pop();
        return path;
    }
}

fn get_project_manifest_path(cli: &CLI) -> PathBuf {
    let mut path = get_project_directory(cli);
    path.push("manifest.yaml");

    return path;
}

pub fn cli() -> CliResult {
    let args = CLI::parse();

    let dir = get_project_manifest_path(&args);

    if !std::fs::exists(&dir).expect("Unable to check if manifest file exists") {
        return CliResult::Error("Project manifest file was not found".to_owned());
    }

    let mut file = File::open(&dir).expect("Unable to open project manifest file");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("Unable to read file");

    let mut project = crate::core::project::from_yaml(contents);

    info!("Project directory: {}", dir.to_str().unwrap());

    match args.command {
        Some(command) => {
            debug!("Running command: {:?}", command);
            let result = CommandType::execute(command, &mut project);

            match result {
                Ok(value) => match value {
                    Some(value) => {
                        let str = serde_json::to_string_pretty(&value).unwrap();
                        info!("{}", str);
                        return CliResult::Success;
                    }
                    None => CliResult::Success,
                },
                Err(_) => CliResult::Error("".to_string()),
            }
        }
        None => {
            info!("Missing subcommand, showing UI");
            return CliResult::ShowUI(project);
        }
    }
}
