use log::info;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub exports: BTreeMap<String, String>,

    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub imports: BTreeMap<String, String>,
}

pub fn to_yaml(departments: BTreeMap<String, Program>) -> serde_yaml::Mapping {
    let result = serde_yaml::to_value(departments).unwrap();

    return result.as_mapping().unwrap().clone();
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> BTreeMap<String, Program> {
    let result = serde_yaml::from_value(serde_yaml::Value::Mapping(map.clone())).unwrap();

    info!("Read programs: {:#?}", result);

    result
}
