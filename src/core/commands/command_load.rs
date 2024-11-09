use std::sync::RwLock;

use clap::Args;
use log::debug;

use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::core::{
    load::{LoadConfigEntry, LoadOp, LoadOperator},
    project::Project,
};

use super::{args::CommonArgs, error::CommandError, Command};

#[derive(Debug, Clone, Args, Serialize, Deserialize)]
pub struct LoadArgs {
    #[command(flatten)]
    #[serde(flatten)]
    pub common: CommonArgs,
}

fn handle_entry(
    entry: &LoadConfigEntry,
    project: &Project,
    args: &LoadArgs,
    current: &mut Vec<String>,
) {
    match entry {
        LoadConfigEntry::Element(element) => current.push(element.to_string()),
        LoadConfigEntry::Operator(load_operator_entry) => match &load_operator_entry.operator {
            Some(op) => {
                if LoadOperator::matches(op, project, args) {
                    LoadOperator::apply(op, project, args, current);

                    for child in load_operator_entry.children.iter() {
                        handle_entry(child, project, args, current);
                    }
                }
            }
            None => debug!("Got no operator: {:#?}", load_operator_entry),
        },
    }
}

impl Command for LoadArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let mut elements = Vec::<String>::new();

        let proj = project.read().unwrap();

        let entries = &proj.load_config.entries;

        for entry in entries.iter() {
            handle_entry(entry, &proj, &self, &mut elements);
        }

        let result = json!({
            "result": elements
        });

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
