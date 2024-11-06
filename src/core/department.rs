use std::collections::BTreeMap;

use log::info;
use serde::{Deserialize, Serialize};
use serde_yaml::Mapping;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Program {
    #[serde(skip_serializing)]
    pub name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exports: Option<BTreeMap<String, String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<BTreeMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Department {
    #[serde(skip_serializing)]
    pub name: String,
    pub programs: Option<BTreeMap<String, Program>>,
}

pub fn to_yaml(departments: BTreeMap<String, Department>) -> serde_yaml::Mapping {
    let mut result = Mapping::new();

    for entry in departments.iter() {
        let mut dept_entry = Mapping::new();

        if let Some(programs) = &entry.1.programs {
            for program in programs.iter() {
                let program_value = serde_yaml::to_value(program.1).unwrap();

                dept_entry.insert(serde_yaml::Value::String(program.0.clone()), program_value);
            }
        };

        result.insert(
            serde_yaml::Value::String(entry.0.to_string()),
            serde_yaml::Value::Mapping(dept_entry),
        );
    }

    return result;
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> BTreeMap<String, Department> {
    let mut result = BTreeMap::new();

    for entry in map.iter() {
        let mapping = entry
            .1
            .as_mapping()
            .expect("Department entry was not a mapping");

        let key = entry
            .0
            .as_str()
            .expect("Department key was not a valid string");

        info!("Reading department: {}", key);

        let data = mapping.clone();
        let mut programs_map = BTreeMap::<String, Program>::new();

        for entry in data.iter() {
            let entry_key = entry.0.as_str().unwrap().to_string();
            let mut entry_mapping = entry.1.as_mapping().cloned().unwrap();

            entry_mapping.insert(serde_yaml::Value::String("name".into()), entry.0.clone());

            let program =
                serde_yaml::from_value::<Program>(serde_yaml::Value::Mapping(entry_mapping))
                    .unwrap();
            programs_map.insert(entry_key.clone(), program);
            info!("Department has entry key: {}", entry_key)
        }

        let dept = Department {
            name: key.to_string(),
            programs: Some(programs_map),
        };

        result.insert(key.to_string(), dept);
    }

    result
}
