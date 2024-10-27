use super::VersionControl;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigDirect {}

impl VersionControl for VersionControlConfigDirect {
    fn export(&self) {
        info!("Exporting using direct version control");
    }
}
