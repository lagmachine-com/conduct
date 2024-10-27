mod direct;
mod symlink;
mod versioned_directories;

use direct::VersionControlConfigDirect;
use enum_dispatch::enum_dispatch;
use serde::{Deserialize, Serialize};
use symlink::VersionControlConfigSymlink;
use versioned_directories::VersionControlConfigVersionedDirectories;

#[enum_dispatch]
pub trait VersionControl {
    fn export(&self);
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
