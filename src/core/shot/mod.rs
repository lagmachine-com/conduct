use std::collections::BTreeMap;
pub mod shot_resolver;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ShotEntry {
    Subcategory(BTreeMap<String, ShotEntry>),
    Scenes(Vec<String>),
}
