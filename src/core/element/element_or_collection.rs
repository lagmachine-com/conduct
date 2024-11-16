use log::warn;
use serde::{Deserialize, Serialize};

use super::{element::Element, element_collection::ElementCollection};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(from = "serde_yaml::Value")]
#[serde(into = "serde_yaml::Value")]
pub enum ElementOrCollection {
    Element(Element),
    Collection(ElementCollection),
}

impl Into<serde_yaml::Value> for ElementOrCollection {
    fn into(self) -> serde_yaml::Value {
        match self {
            ElementOrCollection::Element(element) => serde_yaml::to_value(element).unwrap(),
            ElementOrCollection::Collection(element_collection) => {
                serde_yaml::to_value(element_collection).unwrap()
            }
        }
    }
}

pub trait GetElements {
    fn get_elements(&self) -> Vec<Element>;
}

impl From<serde_yaml::Value> for ElementOrCollection {
    fn from(value: serde_yaml::Value) -> Self {
        warn!("Deserializing element or collection: {:?}", value);

        match value {
            serde_yaml::Value::String(str) => ElementOrCollection::Element(Element::Value(str)),
            serde_yaml::Value::Sequence(vec) => ElementOrCollection::Collection(
                serde_yaml::from_value::<ElementCollection>(serde_yaml::Value::Sequence(vec))
                    .unwrap(),
            ),
            _ => todo!(),
        }
    }
}

impl GetElements for ElementOrCollection {
    fn get_elements(&self) -> Vec<Element> {
        match self {
            ElementOrCollection::Element(element) => {
                let mut result = Vec::new();
                result.push(element.clone());
                result
            }
            ElementOrCollection::Collection(element_collection) => {
                element_collection.elements.clone()
            }
        }
    }
}
