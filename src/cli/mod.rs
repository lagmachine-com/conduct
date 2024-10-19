use clap::{Args, Parser, Subcommand};
use log::*;
#[derive(Debug, Parser)]
#[command(name = "conduct")]
#[command(about = "Generic asset and version management for creative projects", long_about = None)]
struct CLI {
    #[command(subcommand)]
    command: Commands,
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

pub fn cli() {
    info!("Hello from CLI!");
    let args = CLI::parse();
    match args.command {
        Commands::Create(common_args) => todo!(),
    }
}
