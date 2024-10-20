use clap::{command, Args};

use crate::core::project::Project;

use super::{args::CommonArgs, CliResult};

#[derive(Debug, Args)]
pub struct CreateArgs {
    #[command(flatten)]
    common: CommonArgs,
}

impl CreateArgs {
    pub fn execute(self, _project: &mut Project) -> CliResult {
        CliResult::SaveChanges
    }
}
