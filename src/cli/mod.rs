mod args;
mod command_create;
mod command_summary;
mod commands;
mod result;
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

pub fn cli() -> CliResult {
    let args = CLI::parse();

    if let Some(project_dir) = args.global_args.project_dir {
        info!("Overriding project directory with: '{}'", project_dir)
    }

    let mut project = crate::core::project::create_project();

    match args.command {
        Some(command) => {
            info!("Running command: {:?}", command);
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
