use std::collections::BTreeMap;

use log::info;
use serde::{Deserialize, Serialize};

use super::{element::element::Element, project::Project};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DepartmentProgramEntry {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub exports: Vec<String>,

    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub imports: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub default_elements: Vec<Element>,
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

pub trait DepartmentFinder {
    fn get_departments_for_asset(&self, asset_name: String) -> Vec<String>;
}

impl DepartmentFinder for Project {
    fn get_departments_for_asset(&self, asset_name: String) -> Vec<String> {
        let mut result = Vec::new();

        let asset = self.get_asset_by_name(asset_name.clone());

        let asset = match asset {
            Some(asset) => asset,
            None => return result,
        };

        info!("found asset {} at path: {}", asset_name, asset.1);

        for dept in asset.0.departments.iter() {
            result.push(dept.0.clone());
        }

        if let Some(category) = self.get_category_by_path(asset.1) {
            info!("Found category: {:#?}", category);
            if let Some(template) = &category.template {
                for template in template.departments.iter() {
                    if result.contains(template.0) {
                        continue;
                    }

                    result.push(template.0.clone());
                }
            }
        }

        return result;
    }
}
