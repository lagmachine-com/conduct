use crate::core::{
    commands::ExportArgs,
    context::Context,
    element::{self, element_resolver::ElementResolver},
    project,
    version_control::common::resolve_element_path,
};

use super::{ExportError, ExportResult, VersionControl};
use log::{error, info, warn};
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

        let (path, file_name) = match resolve_element_path(project, dept, asset_name, element_name)
        {
            Ok(val) => val,
            Err(err) => {
                error!("Failed to resolve path");
                return Err(err);
            }
        };

        let mut dir = project.get_root_directory();
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
}
