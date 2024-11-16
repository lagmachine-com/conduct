use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::element::element_or_collection::ElementOrCollection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementOpDepends {
    #[serde(flatten)]
    args: BTreeMap<String, serde_yaml::Value>,

    #[serde(rename = "value")]
    elements: Box<ElementOrCollection>,
}
