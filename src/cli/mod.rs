use clap::{Args, Parser, Subcommand};
use log::*;
#[derive(Debug, Parser)]
#[command(name = "conduct")]
#[command(about = "Generic asset and version management for creative projects", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(short, long, help = "Override the targeted project directory")]
    project_dir: Option<String>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Create(CommonArgs),
}

#[derive(Debug, Args)]
struct CommonArgs {
    #[arg(short, long)]
    department: Option<String>,

    #[arg(short, long)]
    asset: Option<String>,

    #[arg(short, long)]
    element: Option<String>,

    #[arg(short, long)]
    scene: Option<String>,
}

pub enum CliResult {
    ShowUI,
    Success,
}

pub fn cli() -> CliResult {
    let args = CLI::parse();

    if let Some(project_dir) = args.project_dir {
        info!("Overriding project directory with: '{}'", project_dir)
    }

    match args.command {
        Some(command) => {
            info!("Running command: {:?}", command);
            match command {
                Commands::Create(_common_args) => {
                    return CliResult::Success;
                }
            }
        }
        None => {
            info!("Missing subcommand, showing UI");
            return CliResult::ShowUI;
        }
    }
}
