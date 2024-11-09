use std::collections::BTreeMap;

use log::info;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepartmentProgramEntry {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub exports: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    pub default_elements: Vec<String>,
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub programs: BTreeMap<String, DepartmentProgramEntry>,
}

pub fn to_yaml(departments: BTreeMap<String, Department>) -> serde_yaml::Mapping {
    let result = serde_yaml::to_value(departments).unwrap();

    return result.as_mapping().unwrap().clone();
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> BTreeMap<String, Department> {
    let result = serde_yaml::from_value(serde_yaml::Value::Mapping(map.clone())).unwrap();

    info!("Read departments: {:#?}", result);

    result
}
