use std::collections::BTreeMap;

use log::{debug, info, warn};
use serde::{Deserialize, Serialize};
use serde_yaml::{Mapping, Value};

use super::element::element_collection::ElementCollection;

#[derive(Clone, Debug)]
pub enum AssetEntry {
    Asset(Asset),
    Category(AssetCategory),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Asset {
    #[serde(skip_serializing_if = "BTreeMap::is_empty", default)]
    pub departments: BTreeMap<String, ElementCollection>,
}

#[derive(Clone, Debug)]
pub struct AssetCategory {
    pub name: String,
    pub template: Option<Asset>,
    pub children: BTreeMap<String, AssetEntry>,
}

pub fn parse_category_assets(value: &Vec<serde_yaml::Value>, key: String) -> AssetEntry {
    let mut result = AssetCategory {
        name: key.clone(),
        template: None,
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
            warn!("Reading asset from yaml");

            warn!("Entry: {:?}", entry);
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
        template: None,
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

        let mut entry = parse_entry(data, key.clone());
        match entry {
            AssetEntry::Category(asset_category) => {
                let mut category = asset_category.clone();
                if let Some(template) = category.children.remove("$template") {
                    match template {
                        AssetEntry::Asset(asset) => {
                            category.template = Some(asset);
                        }
                        _ => (),
                    }
                }
                entry = AssetEntry::Category(category)
            }
            _ => (),
        }

        result.children.insert(key.clone(), entry);
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

fn asset_to_yaml(value: &Asset, name: String) -> serde_yaml::Value {
    let mut result = Mapping::new();

    result.insert(Value::String(name), serde_yaml::to_value(value).unwrap());

    return Value::Mapping(result);
}

fn asset_category_items_to_yaml(value: &AssetCategory) -> serde_yaml::Value {
    let mut result = serde_yaml::Sequence::new();

    if let Some(template) = &value.template {
        result.push(asset_to_yaml(&template, "$template".to_string()));
    }

    for entry in value.children.iter() {
        match entry.1 {
            AssetEntry::Asset(asset) => result.push(asset_to_yaml(asset, entry.0.clone())),
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
