use clap::Parser;

#[derive(Debug, Parser)]
pub struct GlobalArgs {
    #[arg(short, long, help = "Override the targeted project directory")]
    pub project_dir: Option<String>,
}
