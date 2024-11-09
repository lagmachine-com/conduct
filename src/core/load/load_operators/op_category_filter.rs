use log::info;
use serde::{Deserialize, Serialize};

use crate::core::{
    commands::LoadArgs,
    load::{add_entries_from_yaml_value, LoadConfigEntry, LoadOp},
    project::Project,
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LoadOpCategoryFilter {
    pub path: String,
}

impl LoadOp for LoadOpCategoryFilter {
    fn matches(&self, project: &Project, args: &LoadArgs) -> bool {
        let asset = project.get_asset_by_name(args.common.asset.clone().unwrap());

        match asset {
            Some(pair) => {
                let result = pair.1.starts_with(&self.path);
                info!("Checking asset category: {} <-> {}", self.path, pair.1);
                result
            }
            None => false,
        }
    }

    fn apply(&self, _project: &Project, _args: &LoadArgs, _elements: &mut Vec<String>) {}

    fn get_children(
        &self,
        value: &serde_yaml::Value,
        current_path: String,
    ) -> Vec<LoadConfigEntry> {
        let mut result = Vec::new();
        add_entries_from_yaml_value(&mut result, value, current_path);
        return result;
    }
}

pub fn from_path(key: String) -> LoadOpCategoryFilter {
    return LoadOpCategoryFilter { path: key };
}
