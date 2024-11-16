use log::{info, warn};

use crate::core::{
    asset::Asset,
    context::{Context, ContextMode},
    department::DepartmentFinder,
    project::Project,
};

use super::{element::Element, operators::ElementOperation};

pub trait ElementResolver {
    fn get_elements(&self, asset_name: String, context: &Context) -> Vec<String>;
}

impl ElementResolver for Project {
    // Resolve the list of elements for a given asset
    fn get_elements(&self, asset_name: String, context: &Context) -> Vec<String> {
        let asset = self.get_asset_by_name(asset_name.clone());

        info!("Getting assets with context: {:#?}", context);

        let (asset, category_path) = match asset {
            Some(asset) => asset,
            None => {
                warn!("Asset {} does not exist", asset_name);
                panic!()
            }
        };

        let mut result = Vec::new();

        add_elements_from_asset(&mut result, asset, context);
        add_elements_from_category_template(&mut result, &category_path, self, context);
        add_elements_from_department_default(&mut result, self, asset, context);

        result
    }
}

fn add_elements_from_department_default(
    list: &mut Vec<String>,
    project: &Project,
    asset: &Asset,
    context: &Context,
) {
    info!("Adding elements from default departments");
    match context.mode {
        ContextMode::Load => {
            for (dept, _elements) in asset.departments.iter() {
                match project.get_department(dept) {
                    Some(dept) => {
                        add_elements(list, &dept.default_elements, context);
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
                    Some(dept) => add_elements(list, &dept.default_elements, context),
                    None => (),
                }
            }
        }
    }
}

fn add_elements_from_category_template(
    list: &mut Vec<String>,
    category_path: &String,
    project: &Project,
    context: &Context,
) {
    info!("Adding elements from category template");
    let category = project.get_category_by_path(category_path.clone());
    match category {
        Some(category) => match &category.template {
            Some(asset) => add_elements_from_asset(list, &asset, context),
            None => (),
        },
        None => (),
    }
}

fn add_elements_from_asset(list: &mut Vec<String>, asset: &Asset, context: &Context) {
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

        add_elements(list, &elements.elements, context);
    }
}

fn add_elements(list: &mut Vec<String>, elements: &Vec<Element>, context: &Context) {
    for element in elements.iter() {
        match element {
            Element::Value(name) => {
                info!(" - got element: {}", name);
                list.push(name.clone());
            }
            Element::Operator(element_operator) => {
                let elements = ElementOperation::get_elements(element_operator, context);
                add_elements(list, &elements, context);
            }
        }
    }
}
