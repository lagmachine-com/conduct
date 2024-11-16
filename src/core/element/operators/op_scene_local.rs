use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::core::{
    context::Context,
    element::{
        element_or_collection::{ElementOrCollection, GetElements},
        element_resolver::ResolvedElement,
        resolved_element_data::ResolvedElementData,
        util::ResolveListWithContext,
    },
};

use super::ElementOperation;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ElementOpSceneLocal {
    #[serde(flatten)]
    args: BTreeMap<String, serde_yaml::Value>,

    #[serde(rename = "value")]
    elements: Box<ElementOrCollection>,
}

impl ElementOperation for ElementOpSceneLocal {
    fn get_elements(
        &self,
        _context: &Context,
        element_data: ResolvedElementData,
    ) -> Vec<ResolvedElement> {
        let mut element_data = element_data.clone();
        element_data.set_scene_local(true);

        return self
            .elements
            .get_elements()
            .with_context(element_data.clone());
    }
}
