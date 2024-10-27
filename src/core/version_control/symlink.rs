use super::VersionControl;
use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigSymlink {
    relative: bool,
    pool: String,
}

impl VersionControl for VersionControlConfigSymlink {
    fn export(&self) {
        info!("Exporting using symlink version control");
    }
}
