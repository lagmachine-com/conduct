use std::sync::RwLock;

use clap::Args;
use query_string_builder::QueryString;

use crate::{core::project::Project, gui};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct DialogArgs {
    #[command(flatten)]
    #[serde(flatten)]
    pub common: CommonArgs,

    kind: String,
}

pub struct DialogOptions {
    pub path: String,
    pub title: String,
    pub width: f64,
    pub height: f64,
}

impl Command for DialogArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let args = QueryString::dynamic()
            .with_opt_value("department", self.common.department)
            .with_opt_value("asset", self.common.asset)
            .with_opt_value("shot", self.common.shot);

        gui::gui(
            project.read().unwrap().clone(),
            Some(DialogOptions {
                path: "/dialogs/".to_string() + self.kind.as_str() + args.to_string().as_str(),
                title: "Conduct Dialogue".to_string(),
                width: 600.0,
                height: 350.0,
            }),
        );

        Ok(None)
    }
}
