use crate::core::{commands::ExportArgs, project};

use super::{ExportError, ExportResult, VersionControl};
use log::{info, warn};
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
        let element = args.common.element.clone().unwrap();

        let asset = project.get_asset_by_name(asset_name.clone());

        let (_asset, path) = match &asset {
            Some(asset) => asset,
            None => {
                warn!("Could not find asset entry!");
                return Err(ExportError::NotImplemented);
            }
        };

        info!("Found asset at path: {}", path);

        let mut dir = project.get_root_directory();
        dir.push("export");
        dir.push("asset");

        for part in path.split("/").into_iter() {
            dir.push(part);
        }

        dir.push(&asset_name);
        dir.push(&dept);
        dir.push(&element);

        let file_name = format!("{}_{}_{}", asset_name, dept, element);
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
