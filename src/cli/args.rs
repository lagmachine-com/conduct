use clap::{Args, Parser};

#[derive(Debug, Args)]
pub struct CommonArgs {
    #[arg(short, long)]
    department: Option<String>,

    #[arg(short, long)]
    asset: Option<String>,

    #[arg(short, long)]
    element: Option<String>,

    #[arg(short, long)]
    scene: Option<String>,
}

#[derive(Debug, Parser)]
pub struct GlobalArgs {
    #[arg(short, long, help = "Override the targeted project directory")]
    pub project_dir: Option<String>,
}
