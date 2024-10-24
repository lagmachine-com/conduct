use std::collections::HashMap;

use log::{debug, info};

use super::asset::AssetCategory;
use super::department::Department;

#[derive(Clone)]
pub struct Project {
    identifier: String,
    display_name: String,
    pub departments: HashMap<String, Department>,
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

    info!(" --- Reading Departments --- ");
    let dept_data = map
        .get("departments")
        .expect("Could not read departments")
        .as_mapping()
        .expect("Departments was not a valid mapping");

    let departments = crate::core::department::from_yaml(dept_data);

    info!(" --- Reading Assets ---");
    let asset_data = map
        .get("assets")
        .expect("Could not read assets")
        .as_mapping()
        .expect("Assets was not a valid mapping");

    let assets = crate::core::asset::from_yaml(asset_data);

    info!(" --- Reading Version Control ---");
    let config = map
        .get("version_control")
        .expect("Could not read version control config")
        .as_mapping()
        .expect("Version control config was not a valid mapping");

    let config = crate::core::version_control::from_yaml(config);

    debug!("Using version control config: {:?}", config);

    Project {
        identifier: identifier.to_string(),
        display_name: display_name.to_string(),
        assets: assets,
        departments: departments,
    }
}
