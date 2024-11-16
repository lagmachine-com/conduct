use std::path::PathBuf;

use crate::core::{
    commands::ExportArgs,
    project,
    version_control::{common::resolve_element_path, versioned_directories::get_next_version},
};

use super::{ExportError, ExportResult, VersionControl};
use log::{error, info};
use path_absolutize::Absolutize;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigSymlink {
    relative: bool,
    pool: String,
}

impl VersionControl for VersionControlConfigSymlink {
    fn export(
        &self,
        project: &project::Project,
        args: &ExportArgs,
    ) -> Result<ExportResult, ExportError> {
        info!("Exporting using symlink version control");

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

        let pool_path = project.path_absolutize_string(self.pool.clone());
        let pool_path = pool_path.absolutize().unwrap();
        info!("Pool Dir: {}", pool_path.to_str().unwrap());

        let mut links_path = project.get_root_directory();
        links_path.push("link");
        links_path.push(&path);

        info!("Links path: {}", links_path.to_str().unwrap());

        let mut export_dir = PathBuf::from(pool_path.clone());
        export_dir.push(&path);

        info!("Export Dir: {}", export_dir.to_str().unwrap());

        let export_dir = get_next_version(export_dir.clone());
        let export_dir = match export_dir {
            Ok(export_dir) => {
                _ = std::fs::create_dir_all(&export_dir);
                export_dir
            }
            Err(_) => {
                error!("Failed to get next directory version");
                return Err(ExportError::NotImplemented);
            }
        };

        let relative = pathdiff::diff_paths(export_dir, &links_path.parent().unwrap());
        let relative = match relative {
            Some(relative) => {
                info!("Relative path: {}", relative.to_str().unwrap());
                relative
            }
            None => todo!(),
        };

        if std::fs::exists(&links_path).unwrap() {
            info!("Removing old link at: {}", links_path.to_str().unwrap());
            std::fs::remove_file(&links_path).unwrap()
        } else {
            info!(
                "old link at: {} did not exist",
                links_path.to_str().unwrap()
            );
        }

        let link_parent = links_path.parent().unwrap();

        info!("Creating parent dir: {}", link_parent.to_str().unwrap());

        let _ = std::fs::create_dir_all(link_parent);

        #[cfg(not(any(target_os = "windows", target_os = "android")))]
        std::os::unix::fs::symlink(relative, &links_path).unwrap();

        #[cfg(any(target_os = "windows", target_os = "android"))]
        {
            warn!("TODO: Implement symlinks on windows");
            panic!()
        }

        Ok(ExportResult {
            directory: links_path.to_str().unwrap().to_string(),
            recommended_file_name: file_name,
            file_format: args.file_format.clone(),
            script: "".to_string(),
        })
    }
}
