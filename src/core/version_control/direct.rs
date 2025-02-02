use std::{collections::BTreeMap, path::PathBuf};

use crate::core::{
    commands::ExportArgs, element::resolved_element_data::ResolvedElementData, project,
    version_control::common::resolve_element_path,
};

use super::{
    common::CommonVersionControlConfig, ExportError, ExportResult, VersionControl,
    VersionControlFile,
};
use log::{error, info};
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigDirect {
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    export_overrides: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub overrides_order: Vec<String>,

    #[serde(flatten)]
    common: CommonVersionControlConfig,
}

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

        let using_path_override = self.export_overrides.contains_key(&args.file_format);

        let config = &mut self.common.clone();
        if using_path_override && !self.overrides_order.is_empty() {
            config.path_order = self.overrides_order.clone()
        }

        let (path, file_name) =
            match resolve_element_path(project, dept, asset_name, element_name, shot, &config) {
                Ok(val) => val,
                Err(err) => {
                    error!("Failed to resolve path");
                    return Err(err);
                }
            };

        info!("Resolved path: {}", &path.to_str().unwrap());

        let mut dir = project.get_root_directory();

        match self.export_overrides.get(&args.file_format) {
            Some(override_str) => {
                info!("Found override path: {}", override_str);
                let override_path = PathBuf::from(override_str);
                if override_path.is_relative() {
                    info!("Override path is relative");
                    dir.push(override_str);
                    dir = dir.absolutize().unwrap().to_path_buf();

                    info!("dir after relative override: {}", dir.to_str().unwrap());
                } else {
                    info!("Override path is absolute")
                }

                dir.push(path);
            }
            None => {
                dir.push("export");
                dir.push(path);
            }
        }

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
    ) -> Vec<VersionControlFile> {
        let asset_name = element_data.get_asset_name().unwrap();
        let dept = element_data.get_owning_department().unwrap();
        let shot = element_data.get_shot();

        let (path, _file_name) =
            match resolve_element_path(project, dept, asset_name, element_name, shot, &self.common)
            {
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
                    .map(|e| VersionControlFile {
                        path: e.unwrap().path().to_str().unwrap().to_string(),
                        version: "current".to_string(),
                    })
                    .collect()
            }
            Err(_) => return Vec::new(),
        }
    }
}
