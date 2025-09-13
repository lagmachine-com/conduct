use std::{
    collections::{BTreeMap, HashMap},
    sync::RwLock,
};

use argmap::new;
use clap::{command, Args};
use ts_rs::TS;

use crate::core::{
    context::{Context, ContextMode},
    element::{
        self, element_resolver::ElementResolver, resolved_element_data::ResolvedElementData,
    },
    project::Project,
    version_control::{VersionControl, VersionControlFile},
};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct ResolveElementsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Serialize, Deserialize, TS)]
pub struct ResolvedElementResult {
    info: ResolvedElementData,
    versions: Vec<VersionControlFile>,
}

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct ResolveElementsResult {
    pub result: BTreeMap<String, ResolvedElementResult>,
}

impl Command for ResolveElementsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        if self.common.asset.is_none() {
            return Err(CommandError::InvalidArguments);
        }

        let context = Context {
            department: self.common.department,
            shot: self.common.shot,
            mode: ContextMode::Export,
        };

        let project = project.read().unwrap();

        let asset = self.common.asset.unwrap();
        let asset = asset.split("/").last().unwrap();

        let elements = project.get_elements(asset.to_string(), &context);
        let elements = match elements {
            Ok(elements) => elements,
            Err(err) => return Err(CommandError::Message(format!("{}", err))),
        };

        let mut result: BTreeMap<String, ResolvedElementResult> = BTreeMap::new();

        for element in elements.iter() {
            let files = VersionControl::get_element_files(
                &project.version_control,
                &project,
                element.0.clone(),
                element.1,
            );

            result.insert(
                element.0.clone(),
                ResolvedElementResult {
                    info: element.1.clone(),
                    versions: files.clone(),
                },
            );
        }

        Ok(Some(
            serde_json::to_value(ResolveElementsResult { result: result }).unwrap(),
        ))
    }
}
