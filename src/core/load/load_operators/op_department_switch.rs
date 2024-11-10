use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::{
    commands::LoadArgs,
    load::{LoadConfigEntry, LoadOp},
    project::Project,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadOpDepartmentSwitch {
    #[serde(flatten)]
    options: BTreeMap<String, serde_yaml::Value>,
}

impl LoadOp for LoadOpDepartmentSwitch {
    fn matches(&self, _project: &Project, args: &LoadArgs) -> bool {
        match &args.common.department {
            Some(dept) => self.options.contains_key(dept) || self.options.contains_key("*"),
            None => false,
        }
    }

    fn apply(&self, _project: &Project, args: &LoadArgs, elements: &mut Vec<String>) {
        match &args.common.department {
            Some(dept) => {
                let result: Option<&serde_yaml::Value> = match self.options.get(dept) {
                    Some(element) => Some(element),
                    None => match self.options.get("*") {
                        Some(default_element) => Some(default_element),
                        None => None,
                    },
                };

                match result {
                    Some(element) => match element {
                        serde_yaml::Value::String(value) => elements.push(value.clone()),
                        _ => (),
                    },
                    None => (),
                }
            }
            None => (),
        }
    }

    fn get_children(
        &self,
        _value: &serde_yaml::Value,
        _current_path: String,
    ) -> Vec<LoadConfigEntry> {
        return Vec::new();
    }
}
