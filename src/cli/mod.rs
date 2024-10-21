mod args;
mod command_create;
mod command_summary;
mod commands;
mod result;
use std::{fs::File, io::Read, path::PathBuf};

use args::GlobalArgs;
use clap::Parser;
use commands::Commands;
use log::*;
pub use result::CliResult;

#[derive(Debug, Parser)]
#[command(name = "conduct")]
#[command(about = "Generic asset and version management for creative projects", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<commands::Commands>,

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
        return CliResult::Error;
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
            match command {
                Commands::Create(args) => args.execute(&mut project),
                Commands::Summary(args) => args.execute(&mut project),
            }
        }
        None => {
            info!("Missing subcommand, showing UI");
            return CliResult::ShowUI(project);
        }
    }
}
