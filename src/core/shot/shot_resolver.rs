use crate::core::project::Project;

use super::ShotEntry;

pub trait ShotResolver {
    fn get_shots(&self) -> Vec<String>;
}

impl ShotResolver for Project {
    fn get_shots(&self) -> Vec<String> {
        let mut result = Vec::new();

        add_shot(&mut result, &self.shots, "".to_string());

        result
    }
}
fn add_shot(list: &mut Vec<String>, entry: &ShotEntry, current_path: String) {
    match entry {
        ShotEntry::Subcategory(category) => {
            for (key, value) in category.iter() {
                let path = if current_path.is_empty() {
                    key.to_string()
                } else {
                    current_path.clone() + "/" + key.as_str()
                };

                add_shot(list, value, path);
            }
        }
        ShotEntry::ShotList(vec) => {
            for entry in vec.iter() {
                let path = if current_path.is_empty() {
                    entry.to_string()
                } else {
                    current_path.clone() + "/" + entry.as_str()
                };

                list.push(path);
            }
        }
    }
}
