use std::path::{Path, PathBuf};

use crate::core::{commands::ExportArgs, project, version_control::common::resolve_element_path};

use super::{ExportError, ExportResult, VersionControl};
use clap::error;
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigVersionedDirectories {}

impl VersionControl for VersionControlConfigVersionedDirectories {
    fn export(
        &self,
        project: &project::Project,
        args: &ExportArgs,
    ) -> Result<ExportResult, ExportError> {
        info!("Exporting using versioned directories version control");

        let asset_name = args.common.asset.clone().unwrap();
        let dept = args.common.department.clone().unwrap();
        let element_name = args.common.element.clone().unwrap();

        let (path, file_name) = match resolve_element_path(project, dept, asset_name, element_name)
        {
            Ok(val) => val,
            Err(err) => {
                error!("Failed to resolve path");
                return Err(err);
            }
        };

        let mut dir = project.get_root_directory();
        dir.push("export");
        dir.push(path);

        let dir = match get_next_version(dir) {
            Ok(dir) => dir,
            Err(_) => {
                error!("Could not create versioned directory");
                return Err(ExportError::NotImplemented);
            }
        };

        info!("Exporting to: {}", dir.to_str().unwrap());

        _ = std::fs::create_dir_all(&dir);

        Ok(ExportResult {
            directory: dir.to_str().unwrap().to_string(),
            recommended_file_name: file_name,
            file_format: args.file_format.clone(),
            script: "".to_string(),
        })
    }
}

pub fn get_next_version(directory: PathBuf) -> Result<PathBuf, std::io::Error> {
    _ = std::fs::create_dir_all(&directory);
    let paths = std::fs::read_dir(&directory)?;

    info!("Checking dir {} for versions", directory.to_str().unwrap());

    let mut version = 1;

    for path in paths {
        match path {
            Ok(path) => {
                let file_name = path.file_name().into_string().unwrap();
                let version_str = file_name.strip_prefix("v");
                match version_str {
                    Some(version_str) => {
                        let ver = version_str.parse::<i32>().unwrap();

                        if ver >= version {
                            version = ver + 1
                        }
                    }
                    None => continue,
                }

                info!("Got path: {:?}", version_str)
            }
            Err(err) => {
                error!("Got error: {:?}", err)
            }
        }
    }

    let version_name = format!("v{:0>3}", version);
    info!("Next version: {}", version_name);
    let mut directory = directory.clone();
    directory.push(version_name);
    return Ok(directory);
}
