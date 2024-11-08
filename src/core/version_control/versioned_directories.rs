use crate::core::{commands::ExportArgs, project};

use super::{ExportError, ExportResult, VersionControl};
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigVersionedDirectories {}

impl VersionControl for VersionControlConfigVersionedDirectories {
    fn export(
        &self,
        _project: &project::Project,
        _args: &ExportArgs,
    ) -> Result<ExportResult, ExportError> {
        info!("Exporting using versioned directories version control");

        Err(ExportError::NotImplemented)
    }
}
