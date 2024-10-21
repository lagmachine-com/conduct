use log::{debug, info};
use std::{collections::HashMap, hash::Hash};

pub enum AssetEntry {
    Asset(Asset),
    Category(AssetCategory),
}

pub struct Asset {
    pub identifier: String,
}

pub struct AssetCategory {
    pub name: String,
    pub children: HashMap<String, AssetEntry>,
}

fn is_mapping_asset(map: &serde_yaml::Mapping) -> bool {
    return map.contains_key("departments");
}

pub fn parse_entry_as_asset(map: &serde_yaml::Mapping, key: String) -> Asset {
    let asset = Asset {
        identifier: key.to_owned(),
    };

    info!("Found asset: {}", asset.identifier);

    return asset;
}

pub fn parse_entry(map: &serde_yaml::Mapping, key: String) -> AssetEntry {
    if is_mapping_asset(map) {
        return AssetEntry::Asset(parse_entry_as_asset(map, key));
    }

    info!("Found asset category: {}", key);

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
