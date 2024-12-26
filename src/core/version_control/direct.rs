use crate::core::{
    commands::ExportArgs, element::resolved_element_data::ResolvedElementData, project,
    version_control::common::resolve_element_path,
};

use super::{ExportError, ExportResult, VersionControl};
use log::{error, info};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigDirect {}

impl VersionControl for VersionControlConfigDirect {
    fn export(
        &self,
        project: &project::Project,
        args: &ExportArgs,
    ) -> Result<ExportResult, ExportError> {
        info!("Exporting using direct version control");

        let asset_name = args.common.asset.clone().unwrap();
        let dept = args.common.department.clone().unwrap();
        let element_name = args.common.element.clone().unwrap();
        let shot = args.common.shot.clone();

        let (path, file_name) =
            match resolve_element_path(project, dept, asset_name, element_name, shot) {
                Ok(val) => val,
                Err(err) => {
                    error!("Failed to resolve path");
                    return Err(err);
                }
            };

        let mut dir = project.get_root_directory();
        dir.push("export");
        dir.push(path);

        info!("Exporting to: {}", dir.to_str().unwrap());
        info!("Recommended file name: {}", file_name);

        _ = std::fs::create_dir_all(&dir);

        Ok(ExportResult {
            directory: dir.to_str().unwrap().to_string(),
            recommended_file_name: file_name,
            file_format: args.file_format.clone(),
            script: "".to_string(),
        })
    }

    fn get_element_files(
        &self,
        project: &project::Project,
        element_name: String,
        element_data: &ResolvedElementData,
    ) -> Vec<String> {
        let asset_name = element_data.get_asset_name().unwrap();
        let dept = element_data.get_owning_department().unwrap();
        let shot = element_data.get_shot();

        let (path, _file_name) =
            match resolve_element_path(project, dept, asset_name, element_name, shot) {
                Ok(val) => val,
                Err(err) => {
                    error!("Failed to resolve path {:?}", err);
                    return Vec::new();
                }
            };

        let mut dir = project.get_root_directory();
        dir.push("export");
        dir.push(path);

        let files = std::fs::read_dir(dir);
        match files {
            Ok(files) => {
                return files
                    .into_iter()
                    .filter(|e| e.is_ok())
                    .map(|e| e.unwrap().path().to_str().unwrap().to_string())
                    .collect()
            }
            Err(_) => return Vec::new(),
        }
    }
}
