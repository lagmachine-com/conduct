use std::ops::Index;

use crate::core::project::Project;

use super::ShotEntry;

pub trait ShotResolver {
    fn get_shots(&self) -> Vec<String>;

    fn get_shot_formatted(&self, shot: &String) -> Option<String>;

    fn shot_exists(&self, shot: &String) -> bool;
}

impl ShotResolver for Project {
    fn get_shots(&self) -> Vec<String> {
        let mut result = Vec::new();

        add_shot(&mut result, &self.shots, "".to_string());

        result
    }

    fn shot_exists(&self, shot: &String) -> bool {
        let shots: Vec<String> = self.get_shots().iter().map(|s| s).collect();
        return shots.contains(&shot);
    }

    fn get_shot_formatted(&self, shot: &String) -> Option<String> {
        let shots = self.get_shots();
        let index = shots
            .iter()
            .position(|s| s == shot);

        let formatted: Option<String> = match index {
            Some(i) => Some(shots.index(i).clone()),
            None => None,
        };

        formatted
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
