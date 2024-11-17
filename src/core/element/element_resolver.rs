use std::collections::BTreeMap;

use log::{info, warn};

use crate::core::{
    asset::Asset,
    context::{Context, ContextMode},
    project::Project,
};

use super::{
    element::Element, operators::ElementOperation, resolved_element_data::ResolvedElementData,
    util::ResolveListWithContext,
};

pub trait ElementResolver {
    fn get_elements(
        &self,
        asset_name: String,
        context: &Context,
    ) -> BTreeMap<String, ResolvedElementData>;

    fn get_element(
        &self,
        asset_name: String,
        element_name: String,
        context: &Context,
    ) -> Option<ResolvedElementData>;
}

pub struct ResolvedElement {
    pub element: Element,
    pub context: ResolvedElementData,
}

impl ElementResolver for Project {
    fn get_element(
        &self,
        asset_name: String,
        element_name: String,
        context: &Context,
    ) -> Option<ResolvedElementData> {
        let result = self.get_elements(asset_name, context);

        return result.get(&element_name).cloned();
    }

    // Resolve the list of elements for a given asset
    fn get_elements(
        &self,
        asset_name: String,
        context: &Context,
    ) -> BTreeMap<String, ResolvedElementData> {
        let asset = self.get_asset_by_name(asset_name.clone());

        info!("Getting assets with context: {:#?}", context);

        let (asset, category_path) = match asset {
            Some(asset) => asset,
            None => {
                warn!("Asset {} does not exist", asset_name);
                panic!()
            }
        };

        let mut result = BTreeMap::<String, ResolvedElementData>::new();

        let mut element_data = ResolvedElementData::new();
        element_data.set_asset(&asset_name);
        if let Some(shot) = &context.shot {
            element_data.set_shot(shot);
        }

        add_elements_from_asset(&mut result, asset, context, element_data.clone());
        add_elements_from_category_template(
            &mut result,
            &category_path,
            self,
            context,
            element_data.clone(),
        );
        add_elements_from_department_default(&mut result, self, asset, context, element_data);

        result
    }
}

fn add_elements_from_department_default(
    result: &mut BTreeMap<String, ResolvedElementData>,
    project: &Project,
    asset: &Asset,
    context: &Context,
    element_data: ResolvedElementData,
) {
    info!("Adding elements from default departments");
    match context.mode {
        ContextMode::Load => {
            for (dept, _elements) in asset.departments.iter() {
                match project.get_department(dept) {
                    Some(dept) => {
                        add_elements_with_context(
                            result,
                            &dept.default_elements,
                            context,
                            element_data.clone(),
                        );
                    }
                    None => (),
                }
            }
        }
        ContextMode::Export => {
            let depts = match &context.department {
                Some(dept) => {
                    if asset.departments.contains_key(dept) {
                        vec![dept.to_string()]
                    } else {
                        vec![]
                    }
                }
                None => asset.departments.keys().map(|v| v.to_string()).collect(),
            };

            for department_name in depts {
                let dept = project.get_department(&department_name);

                match dept {
                    Some(dept) => add_elements_with_context(
                        result,
                        &dept.default_elements,
                        context,
                        element_data.clone(),
                    ),
                    None => (),
                }
            }
        }
    }
}

fn add_elements_from_category_template(
    result: &mut BTreeMap<String, ResolvedElementData>,
    category_path: &String,
    project: &Project,
    context: &Context,
    element_data: ResolvedElementData,
) {
    info!("Adding elements from category template");
    let category = project.get_category_by_path(category_path.clone());
    match category {
        Some(category) => match &category.template {
            Some(asset) => add_elements_from_asset(result, &asset, context, element_data),
            None => (),
        },
        None => (),
    }
}

fn add_elements_from_asset(
    result: &mut BTreeMap<String, ResolvedElementData>,
    asset: &Asset,
    context: &Context,
    element_data: ResolvedElementData,
) {
    info!("Adding elements from asset");
    for (name, elements) in asset.departments.iter() {
        match context.mode {
            // If we are in export context, we only want to see the elements this department owns
            ContextMode::Export => {
                if context.department.is_some() && context.department.as_ref().unwrap() != name {
                    continue;
                }
            }
            _ => (),
        };

        add_elements_with_context(result, &elements.elements, context, element_data.clone());
    }
}

fn add_elements_with_context(
    result: &mut BTreeMap<String, ResolvedElementData>,
    elements: &Vec<Element>,
    context: &Context,
    element_data: ResolvedElementData,
) {
    let data = elements.with_context(element_data);
    add_elements(result, &data, context);
}

fn add_elements(
    result: &mut BTreeMap<String, ResolvedElementData>,
    elements: &Vec<ResolvedElement>,
    context: &Context,
) {
    for element in elements.iter() {
        match &element.element {
            Element::Value(name) => {
                info!(" - got element: {}", name);
                result.insert(name.clone(), element.context.clone());
            }
            Element::Operator(element_operator) => {
                let elements = ElementOperation::get_elements(
                    element_operator,
                    context,
                    element.context.clone(),
                );
                add_elements(result, &elements, context);
            }
        }
    }
}
