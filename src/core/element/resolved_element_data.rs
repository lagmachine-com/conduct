#[derive(Clone, Debug)]
pub struct ResolvedElementData {
    scene_local: bool,
    dependencies: Option<Vec<String>>,
    asset: Option<String>,
}

impl ResolvedElementData {
    pub fn new() -> Self {
        ResolvedElementData {
            scene_local: false,
            dependencies: None,
            asset: None,
        }
    }

    pub fn set_scene_local(&mut self, value: bool) {
        self.scene_local = value;
    }

    pub fn is_scene_local(&self) -> bool {
        self.scene_local
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
}
