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
pub struct ElementOpDepartmentIsNot {
    #[serde(rename = "0")]
    department: String,
    #[serde(rename = "value")]
    elements: Box<ElementOrCollection>,
}

impl ElementOperation for ElementOpDepartmentIsNot {
    fn get_elements(&self, context: &Context) -> Vec<Element> {
        match &context.department {
            Some(dept) => {
                if &self.department == dept {
                    self.elements.get_elements()
                } else {
                    vec![]
                }
            }
            None => self.elements.get_elements(),
        }
    }
}
