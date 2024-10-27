use super::VersionControl;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigVersionedDirectories {}

impl VersionControl for VersionControlConfigVersionedDirectories {
    fn export(&self) {
        info!("Exporting using versioned directories version control");
    }
}
