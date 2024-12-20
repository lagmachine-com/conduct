mod common;
mod direct;
mod symlink;
mod versioned_directories;

use direct::VersionControlConfigDirect;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use symlink::VersionControlConfigSymlink;
use versioned_directories::VersionControlConfigVersionedDirectories;

use super::{commands::ExportArgs, element::resolved_element_data::ResolvedElementData, project};

#[derive(Debug)]
pub enum ExportError {
    NotImplemented,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportResult {
    pub directory: String,

    pub recommended_file_name: String,

    pub file_format: String,

    pub script: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlFile {
    pub path: String,
    pub version: String,
}

#[enum_dispatch]
pub trait VersionControl {
    fn export(
        &self,
        project: &project::Project,
        args: &ExportArgs,
    ) -> Result<ExportResult, ExportError>;

    fn get_element_files(
        &self,
        project: &project::Project,
        element_name: String,
        element_data: &ResolvedElementData,
    ) -> Vec<VersionControlFile>;
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
#[enum_dispatch(VersionControl)]
pub enum VersionControlConfig {
    Direct(VersionControlConfigDirect),
    VersionedDirectories(VersionControlConfigVersionedDirectories),
    Symlink(VersionControlConfigSymlink),
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> VersionControlConfig {
    serde_yaml::from_value::<VersionControlConfig>(serde_yaml::Value::Mapping(map.clone()))
        .expect("Could not parse version control config")
}
