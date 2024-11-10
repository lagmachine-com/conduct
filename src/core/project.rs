use std::collections::{BTreeMap, HashSet};
use std::path::PathBuf;

use log::{debug, info, warn};
use serde_yaml::Mapping;

use crate::core::{asset, format};

use super::asset::{Asset, AssetCategory, AssetEntry};
use super::department::{self, Department};
use super::load::{self, LoadConfig};
use super::program::{self, Program};
use super::version_control::VersionControlConfig;

#[derive(Clone, Debug)]
pub struct Project {
    identifier: String,
    display_name: String,
    manifest_file: PathBuf,
    pub programs: BTreeMap<String, Program>,
    pub departments: BTreeMap<String, Department>,
    pub assets: AssetCategory,
    pub version_control: VersionControlConfig,
    pub load_config: LoadConfig,
}

impl Project {
    pub fn get_identifier(&self) -> String {
        self.identifier.clone()
    }

    pub fn get_display_name(&self) -> String {
        self.display_name.clone()
    }

    pub fn save(&self) {
        let serialized = to_yaml(self);
        let str = serde_yaml::to_string(&serialized).unwrap();
        let str = format::pretty_format_yaml(str);
        info!("Rewrote yaml file: \n{str}");
        std::fs::write(&self.manifest_file, str).unwrap();
    }

    pub fn get_assets_flattened(&self) -> BTreeMap<String, &Asset> {
        let mut result = BTreeMap::new();
        insert_assets_to_map(&self.assets, "".to_string(), &mut result, false);
        return result;
    }

    pub fn get_root_directory(&self) -> PathBuf {
        let mut path = PathBuf::from(self.manifest_file.clone());
        path.pop();

        return path;
    }

    fn get_asset_child_by_name(
        category: &AssetCategory,
        name: String,
        current_path: String,
    ) -> Option<(&Asset, String)> {
        for child in category.children.iter() {
            let mut path = current_path.clone();
            if path.is_empty() == false {
                path += "/";
            }

            path += child.0;

            match child.1 {
                AssetEntry::Asset(asset) => {
                    if child.0 == &name {
                        return Some((asset, current_path));
                    } else {
                        continue;
                    }
                }
                AssetEntry::Category(asset_category) => {
                    match Self::get_asset_child_by_name(&asset_category, name.clone(), path) {
                        Some(result) => return Some(result),
                        None => continue,
                    }
                }
            }
        }

        None
    }

    pub fn get_asset_by_name(&self, name: String) -> Option<(&Asset, String)> {
        let current = &self.assets;

        Self::get_asset_child_by_name(current, name, "".to_string())
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

    pub fn get_category_by_path(&self, path: String) -> Option<&AssetCategory> {
        let parts = path.split('/');

        let mut current = &self.assets;

        for part in parts.into_iter() {
            info!("Looking for part: {}", part);
            let result = current.children.get(part);
            match result {
                Some(result) => match result {
                    AssetEntry::Asset(_) => return None,
                    AssetEntry::Category(asset_category) => current = asset_category,
                },
                None => return None,
            }
        }

        Some(current)
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
    current: &mut BTreeMap<String, &'a Asset>,
    full_path: bool,
) {
    let mut path = current_path.to_owned();

    if path.is_empty() == false {
        path.push('/');
    }

    for child in category.children.iter() {
        let mut path = path.clone();

        let name = match child.1 {
            AssetEntry::Asset(_asset) => &child.0,
            AssetEntry::Category(asset_category) => &asset_category.name,
        };

        path.push_str(&name);

        match child.1 {
            AssetEntry::Asset(asset) => {
                current.insert(if full_path { path } else { child.0.clone() }, asset);
            }
            AssetEntry::Category(asset_category) => {
                insert_assets_to_map(asset_category, path, current, full_path);
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
        if asset_names.insert(entry.0.clone()) == false {
            warn!("Invalid project structure, found duplicate asset entry");
            return false;
        }
    }

    return true;
}

pub fn to_yaml(project: &Project) -> serde_yaml::Value {
    let mut mapping = Mapping::new();
    mapping.insert(
        "identifier".into(),
        serde_yaml::Value::String(project.get_identifier()),
    );

    mapping.insert(
        "display_name".into(),
        serde_yaml::Value::String(project.get_display_name()),
    );

    mapping.insert(
        "programs".into(),
        serde_yaml::Value::Mapping(program::to_yaml(project.programs.clone())),
    );

    mapping.insert(
        "departments".into(),
        serde_yaml::Value::Mapping(department::to_yaml(project.departments.clone())),
    );

    mapping.insert("assets".into(), asset::to_yaml(&project.assets));

    mapping.insert("load_order".into(), load::to_yaml(&project.load_config));

    mapping.insert(
        "version_control".into(),
        serde_yaml::to_value(&project.version_control).unwrap(),
    );

    return serde_yaml::Value::Mapping(mapping);
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

    let program_data = map
        .get("programs")
        .expect("Could not read programs")
        .as_mapping()
        .expect("Programs was not a valid mapping");

    let programs = crate::core::program::from_yaml(program_data);

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

    info!(" --- Reading Load Order ---");
    let load_order = map
        .get("load_order")
        .expect("Could not read load order config");

    let load_order = crate::core::load::from_yaml(load_order);

    let result = Project {
        manifest_file: file_path,
        identifier: identifier.to_string(),
        display_name: display_name.to_string(),
        assets: assets,
        departments: departments,
        version_control: config,
        load_config: load_order,
        programs: programs,
    };

    to_yaml(&result);

    if is_valid_project_structure(&result) {
        return result;
    }

    panic!("Project structure was not valid")
}
