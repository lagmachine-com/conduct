use crate::core::{commands::ExportArgs, project};

use super::{ExportError, ExportResult, VersionControl};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigSymlink {
    relative: bool,
    pool: String,
}

impl VersionControl for VersionControlConfigSymlink {
    fn export(
        &self,
        _project: &project::Project,
        _args: &ExportArgs,
    ) -> Result<ExportResult, ExportError> {
        info!("Exporting using symlink version control");

        Err(ExportError::NotImplemented)
    }
}
