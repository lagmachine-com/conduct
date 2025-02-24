use std::sync::RwLock;

use clap::{command, Args};
use log::{debug, error, info, trace, warn};
use ts_rs::TS;

use crate::core::{
    context::Context, element::element_resolver::ElementResolver, project::Project,
    version_control::VersionControl,
};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command};

pub enum LoadReason {
    Requested,
    Dependency(String),
}

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct LoadAssetsArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,

    #[arg(long, help = "Which program is being used to load these assets")]
    pub program: String,

    /// Comma seperated list of asset names to load
    assets_list: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct AssetLoadStep {
    pub asset: String,
    pub element: String,
    pub script: String,
    pub file: String,
    pub file_type: String,
    pub version: String,
}

#[derive(Debug, Serialize, Clone, Deserialize, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct LoadAssetsResult {
    results: Vec<AssetLoadStep>,
}

impl Command for LoadAssetsArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();

        let assets: Vec<String> = self.assets_list.split(",").map(|f| f.to_string()).collect();

        let c = Context {
            department: self.common.department.clone(),
            shot: self.common.shot.clone(),
            mode: crate::core::context::ContextMode::Load,
        };

        let assets = get_required_assets(&assets, &project, &c);

        info!("Loading assets: {:?}", assets);

        let department = project
            .departments
            .get(&self.common.department.clone().unwrap())
            .unwrap();

        let program = department.programs.get(&self.program).unwrap();

        let mut import_formats = program.imports.clone();
        import_formats.sort_by(|a, b| b.len().cmp(&a.len()));

        let scripts = project.programs.get(&self.program).unwrap().imports.clone();

        let mut result = LoadAssetsResult {
            results: Vec::new(),
        };

        for asset in assets.into_iter() {
            info!("Loading asset: {}", asset);

            let elements = project.get_elements(asset.to_string(), &c);
            info!("Resolved elements: {:#?}", elements);

            for element in elements.iter() {
                let files = VersionControl::get_element_files(
                    &project.version_control,
                    &project,
                    element.0.clone(),
                    element.1,
                );

                for file in files.iter() {
                    for format in import_formats.iter() {
                        if file.path.ends_with(format) {
                            trace!("Getting script for format: {}", format);

                            let mut script_path = project.get_root_directory();
                            script_path.push("scripts");
                            script_path.push(self.program.clone());
                            script_path.push(scripts.get(format).unwrap().clone());

                            result.results.push(AssetLoadStep {
                                asset: asset.clone(),
                                element: element.0.clone(),
                                script: script_path.to_str().unwrap().to_string(),
                                file_type: format.clone(),
                                file: file.path.clone(),
                                version: file.version.clone(),
                            });

                            break;
                        }
                    }
                }

                info!("Checking import formats {:?}", import_formats)
            }
        }

        info!("Resuling load steps: {:#?}", result);

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}

// Recursively get all required assets, checking for dependencies
fn get_required_assets(
    asset_names: &Vec<String>,
    project: &Project,
    context: &Context,
) -> Vec<String> {
    warn!("TODO: Check for dependency loops");
    let mut result = Vec::new();
    for asset in asset_names.into_iter() {
        info!("Loading asset: {}", asset);

        let elements = project.get_elements(asset.to_string(), context);
        for (element, data) in elements.iter() {
            match data.get_dependencies() {
                Some(dependencies) => {
                    let assets = get_required_assets(&dependencies, project, context);

                    for required_asset in assets {
                        info!(
                            "Also loading {} because it's a dependency of {}/{}",
                            required_asset, asset, element
                        );
                        result.insert(0, required_asset.clone());
                    }
                }
                None => (),
            }
        }
    }

    for name in asset_names.iter() {
        result.push(name.clone());
    }

    result
}
