use super::asset::AssetCategory;

pub struct Project {
    identifier: String,
    display_name: String,
    pub assets: AssetCategory,
}

impl Project {
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }
}

pub fn from_yaml(content: String) -> Project {
    let value: serde_yaml::Value = serde_yaml::from_str(&content).expect("Unable to parse yaml");

    assert_eq!(value.is_mapping(), true);

    let map = value.as_mapping().expect("Invalid Yaml");

    let identifier = map
        .get("identifier")
        .expect("Unable to read project identifier")
        .as_str()
        .expect("Project identifier was not a string");

    let display_name = map
        .get("display_name")
        .expect("Unable to read project display name")
        .as_str()
        .expect("Display name was not a string");

    let asset_data = map
        .get("assets")
        .expect("Could not read assets")
        .as_mapping()
        .expect("Assets was not a valid mapping");

    let assets = crate::core::asset::from_yaml(asset_data);

    Project {
        identifier: identifier.to_string(),
        display_name: display_name.to_string(),
        assets: assets,
    }
}
