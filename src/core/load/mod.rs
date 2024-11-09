use enum_dispatch::enum_dispatch;
use load_operators::{
    op_category_filter::{self, LoadOpCategoryFilter},
    op_department_switch::LoadOpDepartmentSwitch,
};
use log::{info, warn};
use serde::{Deserialize, Serialize};

use super::{commands::LoadArgs, project::Project};
mod load_operators;

#[enum_dispatch]
pub trait LoadOp {
    fn matches(&self, project: &Project, args: &LoadArgs) -> bool;

    fn apply(&self, project: &Project, args: &LoadArgs, elements: &mut Vec<String>);

    fn get_children(&self, value: &serde_yaml::Value, current_path: String)
        -> Vec<LoadConfigEntry>;
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[enum_dispatch(LoadOp)]
#[serde(tag = "type")]
#[serde(rename_all = "snake_case")]
pub enum LoadOperator {
    CategoryFilter(LoadOpCategoryFilter),
    DepartmentSwitch(LoadOpDepartmentSwitch),
}

#[derive(Debug, Clone)]
pub struct LoadOperatorEntry {
    pub operator: Option<LoadOperator>,
    pub children: Vec<LoadConfigEntry>,
}

#[derive(Debug, Clone)]
pub enum LoadConfigEntry {
    Element(String),
    Operator(LoadOperatorEntry),
}

#[derive(Debug, Clone)]
pub struct LoadConfig {
    pub entries: Vec<LoadConfigEntry>,
}

pub fn entry_from_yaml_pair(
    pair: (&serde_yaml::Value, &serde_yaml::Value),
    current_path: String,
) -> LoadConfigEntry {
    let key = pair.0.as_str().unwrap();
    let value = pair.1;

    let mut entry = LoadOperatorEntry {
        operator: None,
        children: Vec::new(),
    };

    let path_element = key.trim_start_matches("/");
    info!("Path element: {}", path_element);
    let mut path = current_path.clone();
    if path.is_empty() == false {
        path.push('/');
    }

    path.push_str(path_element);

    if key.starts_with("/") {
        entry.operator = Some(LoadOperator::CategoryFilter(op_category_filter::from_path(
            path.clone(),
        )));
    }

    if entry.operator.is_none() {
        let value = value.clone();
        if let Some(mapping) = value.as_mapping() {
            let mut mapping = mapping.clone();

            mapping.insert(
                serde_yaml::Value::String("type".to_string()),
                serde_yaml::Value::String(key.to_string()),
            );

            let value =
                serde_yaml::from_value::<LoadOperator>(serde_yaml::Value::Mapping(mapping.clone()));

            match value {
                Ok(op) => {
                    entry.operator = Some(op);
                }
                Err(err) => warn!("Failed to automatically load operator: {:?}", err),
            }
        }
    }

    if let Some(op) = &entry.operator {
        entry.children = LoadOperator::get_children(&op, value, path.clone())
    }

    return LoadConfigEntry::Operator(entry);
}

pub fn add_entries_from_yaml_value(
    current: &mut Vec<LoadConfigEntry>,
    value: &serde_yaml::Value,
    current_path: String,
) {
    match value {
        serde_yaml::Value::String(element) => {
            current.push(LoadConfigEntry::Element(element.clone()))
        }
        serde_yaml::Value::Sequence(elements) => {
            for entry in elements {
                add_entries_from_yaml_value(current, entry, current_path.clone());
            }
        }
        serde_yaml::Value::Mapping(mapping) => {
            for pair in mapping.iter() {
                current.push(entry_from_yaml_pair(pair, current_path.clone()));
            }
        }
        _ => {
            info!("Unknown yaml entry: {:#?}", value);
        }
    }
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> LoadConfig {
    let mut result = LoadConfig {
        entries: Vec::new(),
    };

    for pair in map.into_iter() {
        let path = "".to_string();
        let entry: LoadConfigEntry = entry_from_yaml_pair(pair, path);
        info!("Got entry: {:#?}", entry);
        result.entries.push(entry);
    }

    return result;
}
