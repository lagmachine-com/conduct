use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
pub struct ResolvedElementData {
    shot_local: bool,
    dependencies: Option<Vec<String>>,
    asset: Option<String>,
    shot: Option<String>,
    owning_department: Option<String>,
}

impl ResolvedElementData {
    pub fn new() -> Self {
        ResolvedElementData {
            shot_local: false,
            dependencies: None,
            asset: None,
            shot: None,
            owning_department: None,
        }
    }

    pub fn set_shot_local(&mut self, value: bool) {
        self.shot_local = value;
    }

    pub fn is_shot_local(&self) -> bool {
        self.shot_local
    }

    pub fn set_shot(&mut self, value: &String) {
        self.shot = Some(value.clone());
    }

    pub fn get_shot(&self) -> Option<String> {
        self.shot.clone()
    }

    pub fn set_asset(&mut self, asset_name: &String) {
        self.asset = Some(asset_name.clone())
    }

    pub fn add_dependency(&mut self, asset_name: &String) {
        match &mut self.dependencies {
            Some(vec) => vec.push(asset_name.clone()),
            None => {
                let mut vec = Vec::new();
                vec.push(asset_name.clone());
                self.dependencies = Some(vec)
            }
        }
    }

    pub fn get_asset_name(&self) -> Option<String> {
        self.asset.clone()
    }

    pub fn set_owning_department(&mut self, value: String) {
        self.owning_department = Some(value);
    }

    pub fn get_owning_department(&self) -> Option<String> {
        self.owning_department.clone()
    }

    pub fn get_dependencies(&self) -> Option<Vec<String>> {
        self.dependencies.clone()
    }
}
