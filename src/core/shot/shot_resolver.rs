use crate::core::project::Project;

use super::ShotEntry;

pub trait ShotResolver {
    fn get_shots(&self) -> Vec<String>;
}

impl ShotResolver for Project {
    fn get_shots(&self) -> Vec<String> {
        let mut result = Vec::new();

        add_shot(&mut result, &self.scenes, "".to_string());

        result
    }
}
fn add_shot(list: &mut Vec<String>, entry: &ShotEntry, current_path: String) {
    match entry {
        ShotEntry::Subcategory(btree_map) => {
            for (key, value) in btree_map.iter() {
                let path = if current_path.is_empty() {
                    key.to_string()
                } else {
                    current_path.clone() + "/" + key.as_str()
                };

                add_shot(list, value, path);
            }
        }
        ShotEntry::Scenes(vec) => {
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
