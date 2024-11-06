use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use log::{debug, info, warn};

use super::asset::{Asset, AssetCategory, AssetEntry};
use super::department::Department;
use super::version_control::VersionControlConfig;

#[derive(Clone)]
pub struct Project {
    identifier: String,
    display_name: String,
    manifest_file: PathBuf,
    pub departments: HashMap<String, Department>,
    pub assets: AssetCategory,
    pub version_control: VersionControlConfig,
}

impl Project {
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn get_assets_flattened(&self) -> HashMap<String, &Asset> {
        let mut result = HashMap::new();
        insert_assets_to_map(&self.assets, "".to_string(), &mut result);
        return result;
    }

    pub fn get_root_directory(&self) -> PathBuf {
        let mut path = PathBuf::from(self.manifest_file.clone());
        path.pop();

        return path;
    }

    pub fn get_asset_by_path(&self, path: String) -> Option<&Asset> {
        let parts = path.split('/');

        let mut current = &self.assets;

        for part in parts.into_iter() {
            info!("Looking for part: {}", part);
            let result = current.children.get(part);
            match result {
                Some(result) => match result {
                    AssetEntry::Asset(asset) => return Some(asset),
                    AssetEntry::Category(asset_category) => current = asset_category,
                },
                None => return None,
            }
        }

        None
    }

    pub fn get_mut_asset_by_path(&mut self, path: String) -> Option<&mut Asset> {
        let parts = path.split('/');

        let mut current = &mut self.assets;

        for part in parts.into_iter() {
            info!("Looking for part: {}", part);
            let result = current.children.get_mut(part);
            match result {
                Some(result) => match result {
                    AssetEntry::Asset(asset) => return Some(asset),
                    AssetEntry::Category(asset_category) => current = asset_category,
                },
                None => return None,
            }
        }

        None
    }
}

fn insert_assets_to_map<'a>(
    category: &'a AssetCategory,
    current_path: String,
    current: &mut HashMap<String, &'a Asset>,
) {
    let mut path = current_path.to_owned();

    if path.is_empty() == false {
        path.push('/');
    }

    for child in category.children.iter() {
        let mut path = path.clone();

        let name = match child.1 {
            AssetEntry::Asset(asset) => &asset.name,
            AssetEntry::Category(asset_category) => &asset_category.name,
        };

        path.push_str(&name);

        match child.1 {
            AssetEntry::Asset(asset) => {
                current.insert(path, asset);
            }
            AssetEntry::Category(asset_category) => {
                insert_assets_to_map(asset_category, path, current);
            }
        }
    }
}

// TODO: Actually validate stuff
fn is_valid_project_structure(project: &Project) -> bool {
    let flat = project.get_assets_flattened();

    let mut asset_names = HashSet::<String>::new();

    for entry in flat.iter() {
        debug!("Checking asset: {}", entry.0);
        if asset_names.insert(entry.1.name.clone()) == false {
            warn!("Invalid project structure, found duplicate asset entry");
            return false;
        }
    }

    return true;
}

pub fn from_yaml(content: String, file_path: PathBuf) -> Project {
    let value: serde_yaml::Value = serde_yaml::from_str(&content).expect("Unable to parse yaml");

    info!("Parsing yaml from file: {}", file_path.to_str().unwrap());

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

    let result = Project {
        manifest_file: file_path,
        identifier: identifier.to_string(),
        display_name: display_name.to_string(),
        assets: assets,
        departments: departments,
        version_control: config,
    };

    if is_valid_project_structure(&result) {
        return result;
    }

    panic!("Project structure was not valid")
}
