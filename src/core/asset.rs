use std::collections::HashMap;

use log::{debug, info};
use serde::{Deserialize, Serialize};

pub enum AssetEntry {
    Asset(Asset),
    Category(AssetCategory),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    #[serde(skip_serializing)]
    pub name: String,

    pub departments: HashMap<String, Vec<String>>,
}

pub struct AssetCategory {
    pub name: String,
    pub children: HashMap<String, AssetEntry>,
}

fn is_mapping_asset(map: &serde_yaml::Mapping) -> bool {
    return map.contains_key("departments");
}

pub fn parse_entry_as_asset(map: &serde_yaml::Mapping, key: String) -> Asset {
    let mut data = map.clone();
    data.insert(
        serde_yaml::Value::String("name".to_string()),
        serde_yaml::Value::String(key),
    );

    let asset = serde_yaml::from_value::<Asset>(serde_yaml::Value::Mapping(data))
        .expect("Unable to parse asset");

    debug!("\tReading asset: {}", asset.name);

    asset
}

pub fn parse_entry(map: &serde_yaml::Mapping, key: String) -> AssetEntry {
    if is_mapping_asset(map) {
        return AssetEntry::Asset(parse_entry_as_asset(map, key));
    }

    debug!("Reading asset category: {}", key);

    let mut category = AssetCategory {
        name: key.to_owned(),
        children: HashMap::new(),
    };

    for key in map.keys() {
        let value = map.get(key).unwrap();

        if !value.is_mapping() {
            debug!(
                "Invalid value found in assets at key '{}'",
                key.as_str().unwrap()
            );
        }

        let key = key.as_str().expect("Asset key was not a valid string");
        let map = value.as_mapping().expect("Asset was not a valid mapping");
        let entry = parse_entry(map, key.to_string());
        category.children.insert(key.to_string(), entry);
    }

    return AssetEntry::Category(category);
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> AssetCategory {
    let entry = parse_entry(map, "assets".to_string());

    match entry {
        AssetEntry::Asset(_asset) => {
            panic!("Top level asset entry as an asset itself! this should not be posible!")
        }
        AssetEntry::Category(asset_category) => asset_category,
    }
}
