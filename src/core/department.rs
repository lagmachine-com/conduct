use std::collections::HashMap;

use log::debug;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Department {
    #[serde(skip_serializing)]
    pub name: String,

    pub exports: Option<HashMap<String, String>>,
    pub imports: Option<HashMap<String, String>>,
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> HashMap<String, Department> {
    let mut result = HashMap::new();

    for entry in map.iter() {
        let mapping = entry
            .1
            .as_mapping()
            .expect("Department entry was not a mapping");

        let key = entry
            .0
            .as_str()
            .expect("Department key was not a valid string");

        let mut data = mapping.clone();
        data.insert(
            serde_yaml::Value::String("name".to_string()),
            entry.0.clone(),
        );

        let dept = serde_yaml::from_value::<Department>(serde_yaml::Value::Mapping(data))
            .expect("Unable to parse department");

        debug!("Read department: {}", key);

        match &dept.exports {
            Some(exports) => {
                for entry in exports.iter() {
                    debug!("Export: {} -> {}", entry.0, entry.1)
                }
            }
            None => debug!("Has no exports"),
        }

        match &dept.imports {
            Some(imports) => {
                for entry in imports.iter() {
                    debug!("Import: {} -> {}", entry.0, entry.1)
                }
            }
            None => debug!("Has no imports"),
        }

        result.insert(key.to_string(), dept);
    }
    result
}
