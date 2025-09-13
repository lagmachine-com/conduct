mod args;
mod result;
use std::{fs::File, io::Read, path::PathBuf, sync::RwLock};

use args::GlobalArgs;
use clap::Parser;
use log::*;
pub use result::CliResult;

use crate::core::commands::{write_command_result, Command, CommandContext, CommandType};

#[derive(Debug, Parser)]
#[command(name = "conduct")]
#[command(about = "Generic asset and version management for creative projects", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<CommandType>,

    #[command(flatten)]
    pub global_args: GlobalArgs,
}

fn get_project_manifest_path(cli: &CLI) -> PathBuf {
    let mut paths = Vec::<PathBuf>::new();

    if let Some(project_dir) = &cli.global_args.project_dir {
        info!("Overriding project directory with: '{}'", project_dir);
        let path = PathBuf::from(project_dir);
        paths.push(path);
    } else {
        let arg = std::env::args().into_iter().next().unwrap();
        let mut path = PathBuf::from(arg);

        path.pop();
        paths.push(path);

        let mut path = std::env::current_exe().unwrap();

        path.pop();
        paths.push(path);

        let path = std::env::current_dir().unwrap();
        paths.push(path);
    }

    for path in paths.iter() {
        let mut test_path = PathBuf::from(path);
        test_path.push("manifest.yml");

        let mut test_path_2 = PathBuf::from(path);
        test_path_2.push("manifest.yaml");
        for path in vec![test_path, test_path_2].iter() {
            let path = dunce::canonicalize(path);
            match path {
                Ok(path) => {
                    info!("Checking path: {}", path.to_str().unwrap());
                    match std::fs::exists(path.clone()) {
                        Ok(value) => match value {
                            true => return path.clone(),
                            false => continue,
                        },
                        Err(_) => continue,
                    };
                }
                Err(_) => continue,
            }
        }
    }

    panic!("Could not find project manifest, you might want to use `--project-dir <PATH>`")
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

    let project = crate::core::project::from_yaml(contents, dir.clone());
    let project = RwLock::new(project);
    info!("Project directory: {}", dir.to_str().unwrap());

    match args.command {
        Some(command) => {
            info!("Running command: {:?}", command);
            let result = CommandType::execute(command, &project, CommandContext { is_cli: true });

            match result {
                Ok(value) => match value {
                    Some(value) => {
                        write_command_result(value);
                        return CliResult::Success;
                    }
                    None => CliResult::Success,
                },
                Err(error) => CliResult::Error(error.to_string()),
            }
        }
        None => {
            info!("Missing subcommand, showing UI");
            return CliResult::ShowUI(project.read().unwrap().clone());
        }
    }
}
