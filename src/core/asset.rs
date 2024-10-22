use std::collections::HashMap;

use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

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

pub fn parse_category_assets(value: &Vec<serde_yaml::Value>, key: String) -> AssetEntry {
    let mut result = AssetCategory {
        name: key,
        children: HashMap::new(),
    };

    for asset_entry in value.iter() {
        let mapping = asset_entry
            .as_mapping()
            .expect("Unable to read asset as mapping");

        let key = mapping
            .keys()
            .next()
            .expect("Unable to get a key for the asset entry")
            .as_str()
            .expect("Asset key was not a valid string");

        debug!("Found asset key: {}", key);

        let mut data = mapping.clone();
        data.insert(
            Value::String("name".to_string()),
            Value::String(key.to_string()),
        );

        let asset = serde_yaml::from_value::<Asset>(Value::Mapping(data))
            .expect("Unable to parse asset form yaml data");

        result
            .children
            .insert(key.to_string(), AssetEntry::Asset(asset));
    }

    return AssetEntry::Category(result);
}

pub fn parse_category(value: &serde_yaml::Mapping, key: String) -> AssetEntry {
    let mut result = AssetCategory {
        name: key.clone(),
        children: HashMap::new(),
    };

    debug!("Parsing asset category: {}", key);

    for entry in value.iter() {
        let key = entry
            .0
            .as_str()
            .expect("Asset category key was not valid string")
            .to_string()
            .clone();

        let data = entry.1;

        result.children.insert(key.clone(), parse_entry(data, key));
    }

    return AssetEntry::Category(result);
}

pub fn parse_entry(value: &serde_yaml::Value, key: String) -> AssetEntry {
    let result = match value {
        Value::Sequence(vec) => parse_category_assets(vec, key),
        Value::Mapping(mapping) => parse_category(mapping, key),
        _ => panic!("Invalid entry in asset entries"),
    };

    result
}

pub fn from_yaml(map: &serde_yaml::Mapping) -> AssetCategory {
    let value = serde_yaml::Value::Mapping(map.clone());

    let entry = parse_entry(&value, "assets".to_string());

    match entry {
        AssetEntry::Asset(_asset) => {
            panic!("Top level asset entry as an asset itself! this should not be posible!")
        }
        AssetEntry::Category(asset_category) => asset_category,
    }
}
