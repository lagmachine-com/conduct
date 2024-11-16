use serde::{Deserialize, Serialize};

use crate::core::element::element_or_collection::ElementOrCollection;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementOpIsDepartment {
    #[serde(rename = "0")]
    department: String,
    #[serde(rename = "value")]
    elements: Box<ElementOrCollection>,
}
