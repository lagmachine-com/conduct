use serde::{Deserialize, Serialize};

use crate::core::{
    commands::LoadArgs,
    element::ElementFinder,
    load::{LoadConfigEntry, LoadOp},
    project::Project,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "String")]
#[serde(into = "String")]
pub struct LoadOpDepartmentLoad {
    pub department: String,
}

impl From<String> for LoadOpDepartmentLoad {
    fn from(value: String) -> Self {
        return LoadOpDepartmentLoad { department: value };
    }
}

impl Into<String> for LoadOpDepartmentLoad {
    fn into(self) -> String {
        return self.department;
    }
}

impl LoadOp for LoadOpDepartmentLoad {
    fn matches(&self, _project: &Project, _args: &LoadArgs) -> bool {
        true
    }

    fn apply(&self, project: &Project, args: &LoadArgs, elements: &mut Vec<String>) {
        let els = project.get_elements_for_asset(
            args.common.asset.clone().unwrap(),
            Some(self.department.clone()),
        );

        for element in els.iter() {
            elements.push(element.clone());
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
