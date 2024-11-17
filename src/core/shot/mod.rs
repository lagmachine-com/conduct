pub mod shot_resolver;

use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use serde_yaml::{Sequence, Value};

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(untagged)]
#[serde(from = "serde_yaml::Value")]
#[serde(into = "serde_yaml::Value")]
pub enum ShotEntry {
    #[serde(with = "indexmap::map::serde_seq")]
    Subcategory(IndexMap<String, ShotEntry>),
    ShotList(Vec<String>),
}

impl Into<serde_yaml::Value> for ShotEntry {
    fn into(self) -> serde_yaml::Value {
        match self {
            ShotEntry::Subcategory(index_map) => {
                let mut result = serde_yaml::Mapping::new();

                for (key, value) in index_map.iter() {
                    let data = serde_yaml::to_value(value).unwrap();
                    result.insert(serde_yaml::Value::String(key.to_string()), data);
                }

                return Value::Mapping(result);
            }
            ShotEntry::ShotList(vec) => {
                let mut result = Sequence::new();

                for shot in vec.iter() {
                    result.push(Value::String(shot.to_string()));
                }

                return Value::Sequence(result);
            }
        }
    }
}

impl From<serde_yaml::Value> for ShotEntry {
    fn from(value: serde_yaml::Value) -> Self {
        match value {
            serde_yaml::Value::Mapping(mapping) => {
                let mut data = IndexMap::new();

                for (key, value) in mapping.iter() {
                    let key = key.as_str().unwrap();
                    let value: ShotEntry = serde_yaml::from_value(value.clone()).unwrap();
                    data.insert(key.to_string(), value);
                }

                return ShotEntry::Subcategory(data);
            }
            serde_yaml::Value::Sequence(seq) => {
                let mut data = Vec::new();

                for entry in seq.iter() {
                    data.push(entry.as_str().unwrap().to_string());
                }

                return ShotEntry::ShotList(data);
            }
            _ => todo!(),
        }
    }
}
