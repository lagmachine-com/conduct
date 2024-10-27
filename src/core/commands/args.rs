use clap::{Args, Parser};
use serde::{Deserialize, Serialize};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct CommonArgs {
    #[arg(short, long)]
    pub department: Option<String>,

    #[arg(short, long)]
    pub asset: Option<String>,

    #[arg(short, long)]
    pub element: Option<String>,

    #[arg(short, long)]
    pub scene: Option<String>,
}

#[derive(Debug, Parser)]
pub struct GlobalArgs {
    #[arg(short, long, help = "Override the targeted project directory")]
    pub project_dir: Option<String>,
}
