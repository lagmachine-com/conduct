use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]

pub struct VersionControlConfigDirect {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigVersionedDirectories {}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VersionControlConfigSymlink {
    relative: bool,
    pool: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum VersionControlConfig {
    Direct(VersionControlConfigDirect),
    VersionedDirectories(VersionControlConfigVersionedDirectories),
    Symlink(VersionControlConfigSymlink),
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> VersionControlConfig {
    serde_yaml::from_value::<VersionControlConfig>(serde_yaml::Value::Mapping(map.clone()))
        .expect("Could not parse version control config")
}
