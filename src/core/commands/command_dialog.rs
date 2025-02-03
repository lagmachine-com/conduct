use std::sync::RwLock;

use clap::Args;
use query_string_builder::QueryString;

use crate::{core::project::Project, gui};
use serde::{Deserialize, Serialize};

use super::{error::CommandError, Command};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct DialogArgs {
    kind: String,

    extras: Vec<String>,
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
        let (_, argv) = argmap::parse(self.extras.iter());
        log::debug!("Got extras: {:?}", argv);

        let mut args = QueryString::dynamic();
        for pair in argv.iter() {
            let value = pair.1.get(0).unwrap();
            _ = args.push(pair.0.clone(), value.clone())
        }

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
