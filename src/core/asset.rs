use std::collections::BTreeMap;

use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_yaml::Value;

#[derive(Clone)]
pub enum AssetEntry {
    Asset(Asset),
    Category(AssetCategory),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    pub departments: BTreeMap<String, Vec<String>>,
}

#[derive(Clone)]
pub struct AssetCategory {
    pub name: String,
    pub children: BTreeMap<String, AssetEntry>,
}

pub fn parse_category_assets(value: &Vec<serde_yaml::Value>, key: String) -> AssetEntry {
    let mut result = AssetCategory {
        name: key.clone(),
        children: BTreeMap::new(),
    };

    debug!("Reading asset category: {}", key);

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

        debug!("  Reading asset: {}", key);

        for entry in mapping.iter() {
            let asset = serde_yaml::from_value::<Asset>(entry.1.clone())
                .expect("Unable to parse asset form yaml data");

            result
                .children
                .insert(key.to_string(), AssetEntry::Asset(asset));
        }
    }

    return AssetEntry::Category(result);
}

pub fn parse_category(value: &serde_yaml::Mapping, key: String) -> AssetEntry {
    let mut result = AssetCategory {
        name: key.clone(),
        children: BTreeMap::new(),
    };

    debug!("Reading asset category: {}", key);

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

fn asset_to_yaml(value: &Asset) -> serde_yaml::Value {
    serde_yaml::to_value(value).unwrap()
}

fn asset_category_items_to_yaml(value: &AssetCategory) -> serde_yaml::Value {
    let mut result = serde_yaml::Sequence::new();

    for entry in value.children.iter() {
        match entry.1 {
            AssetEntry::Asset(asset) => result.push(asset_to_yaml(asset)),
            AssetEntry::Category(_asset_category) => todo!(),
        }
    }

    return Value::Sequence(result);
}

pub fn to_yaml(value: &AssetCategory) -> serde_yaml::Value {
    info!("Current key: {}", value.name);
    let mut result = serde_yaml::Mapping::new();

    let child = value.children.iter().next();

    let has_subcategories = match child {
        Some(child) => match child.1 {
            AssetEntry::Asset(_asset) => false,
            AssetEntry::Category(_asset_category) => true,
        },
        None => false,
    };

    if !has_subcategories {
        return asset_category_items_to_yaml(value);
    } else {
        for child in value.children.iter() {
            match child.1 {
                AssetEntry::Asset(_asset) => panic!(),
                AssetEntry::Category(asset_category) => {
                    let child_value = to_yaml(asset_category);
                    result.insert(Value::String(child.0.clone()), child_value)
                }
            };
        }
    }

    return Value::Mapping(result);
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
