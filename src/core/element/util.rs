use crate::core::element::{
    element::Element, element_resolver::ResolvedElement, resolved_element_data::ResolvedElementData,
};

pub trait ResolveListWithContext {
    fn with_context(&self, context: ResolvedElementData) -> Vec<ResolvedElement>;
}

impl ResolveListWithContext for Vec<Element> {
    fn with_context(&self, context: ResolvedElementData) -> Vec<ResolvedElement> {
        self.iter()
            .map(|e| ResolvedElement {
                element: e.clone(),
                context: context.clone(),
            })
            .collect()
    }
}
