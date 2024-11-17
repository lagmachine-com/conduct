#[derive(Clone, Debug)]
pub struct ResolvedElementData {
    scene_local: bool,
    dependencies: Option<Vec<String>>,
    asset: Option<String>,
    shot: Option<String>,
}

impl ResolvedElementData {
    pub fn new() -> Self {
        ResolvedElementData {
            scene_local: false,
            dependencies: None,
            asset: None,
            shot: None,
        }
    }

    pub fn set_shot_local(&mut self, value: bool) {
        self.scene_local = value;
    }

    pub fn is_shot_local(&self) -> bool {
        self.scene_local
    }

    pub fn set_shot(&mut self, value: &String) {
        self.shot = Some(value.to_lowercase());
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
}
