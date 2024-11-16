use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::{
    context::Context,
    element::{
        element::Element,
        element_or_collection::{ElementOrCollection, GetElements},
    },
};

use super::ElementOperation;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementOpDepends {
    #[serde(flatten)]
    args: BTreeMap<String, serde_yaml::Value>,

    #[serde(rename = "value")]
    elements: Box<ElementOrCollection>,
}

impl ElementOperation for ElementOpDepends {
    fn get_elements(&self, _context: &Context) -> Vec<Element> {
        return self.elements.get_elements();
    }
}
