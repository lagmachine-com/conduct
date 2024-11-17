use std::sync::RwLock;

use clap::Args;

use crate::{core::project::Project, gui};
use serde::{Deserialize, Serialize};

use super::{error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct DialogArgs {
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
        gui::gui(
            project.read().unwrap().clone(),
            Some(DialogOptions {
                path: "/dialogs/".to_string() + self.kind.as_str(),
                title: "Conduct Dialogue".to_string(),
                width: 600.0,
                height: 350.0,
            }),
        );

        Ok(None)
    }
}
