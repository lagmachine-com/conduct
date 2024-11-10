use super::{department::DepartmentFinder, project::Project};

pub trait ElementFinder {
    fn get_elements_for_asset(
        &self,
        asset_name: String,
        filter_department: Option<String>,
    ) -> Vec<String>;
}

impl ElementFinder for Project {
    fn get_elements_for_asset(
        &self,
        asset_name: String,
        filter_department: Option<String>,
    ) -> Vec<String> {
        let mut result = Vec::new();

        let asset = match self.get_asset_by_name(asset_name.clone()) {
            Some(asset) => asset,
            None => {
                return result;
            }
        };

        // Get elements from Category Template
        if let Some(category) = self.get_category_by_path(asset.1) {
            if let Some(template) = &category.template {
                for pair in &template.departments {
                    match filter_department {
                        Some(ref filter) => {
                            if pair.0 != filter {
                                continue;
                            }
                        }
                        None => (),
                    }

                    for element in pair.1.iter() {
                        result.push(element.clone());
                    }
                }
            }
        }

        for dept in self.get_departments_for_asset(asset_name) {
            match filter_department {
                Some(ref filter_dept) => {
                    if &dept != filter_dept {
                        continue;
                    }
                }
                None => (),
            }

            // Get elements from deparment default elements
            match self.departments.get(&dept) {
                Some(dept) => {
                    for entry in dept.default_elements.iter() {
                        result.push(entry.clone());
                    }
                }
                None => (),
            }

            // Get elements from per-asset department configuration
            match asset.0.departments.get(&dept) {
                Some(dept) => {
                    for entry in dept {
                        result.push(entry.clone());
                    }
                }
                None => (),
            }
        }

        return result;
    }
}
