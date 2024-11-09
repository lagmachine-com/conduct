use super::project::Project;

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

        for dept in asset.0.departments.iter() {
            match filter_department {
                Some(ref filter_dept) => {
                    if dept.0 != filter_dept {
                        continue;
                    }
                }
                None => (),
            }

            match self.departments.get(dept.0) {
                Some(dept) => {
                    for entry in dept.default_elements.iter() {
                        result.push(entry.clone());
                    }
                }
                None => (),
            }

            for entry in dept.1.iter() {
                result.push(entry.clone());
            }
        }

        return result;
    }
}
