use std::{collections::HashMap, path::PathBuf};

use log::{trace, warn};
use serde::{Deserialize, Serialize};

use crate::core::{
    context::Context, element::element_resolver::ElementResolver, project,
    shot::shot_resolver::ShotResolver,
};

use super::ExportError;

fn seperate_shots_and_assets_default() -> bool {
    true
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommonVersionControlConfig {
    #[serde(skip_serializing_if = "Vec::is_empty", default)]
    pub path_order: Vec<String>,

    #[serde(default = "seperate_shots_and_assets_default")]
    pub seperate_shots_and_assets: bool,
}

pub fn get_default_path_order() -> Vec<String> {
    return vec![
        "shot".to_string(),
        "category".to_string(),
        "asset".to_string(),
        "department".to_string(),
        "element".to_string(),
    ];
}

pub fn resolve_element_path(
    project: &project::Project,
    department: String,
    asset_name: String,
    element_name: String,
    shot: Option<String>,
    config: &CommonVersionControlConfig,
) -> Result<(PathBuf, String), ExportError> {
    let mut map = HashMap::<&str, String>::new();

    map.insert("department", department.clone());
    map.insert("asset", asset_name.clone());

    match project.get_asset_by_name(asset_name.clone()) {
        Some(asset) => {
            trace!("Found asset at path: {}", asset.1);
            map.insert("category", asset.1);
        }
        None => {
            warn!("Could not find asset entry!");
            return Err(ExportError::NotImplemented);
        }
    };

    let element_data = match project.get_element(
        asset_name.clone(),
        element_name.clone(),
        &Context {
            department: Some(department.clone()),
            mode: crate::core::context::ContextMode::Export,
            shot: shot,
        },
    ) {
        Some(data) => {
            map.insert("element", element_name.clone());

            if data.is_shot_local() {
                match data.get_shot() {
                    Some(shot) => {
                        map.insert("shot", shot.clone());
                        trace!("Resolved shot: {}", shot)
                    },
                    None => {
                        return Err(ExportError::Message("Tried to resolve the path of a shot_local element, but we are not in a shot context".into()))
                    },
                }
            }

            data
        }
        None => {
            warn!("Could not find resolve element '{}'!", element_name);
            return Err(ExportError::NotImplemented);
        }
    };

    let mut result = PathBuf::new();

    if config.seperate_shots_and_assets {
        if element_data.is_shot_local() {
            let shot = match element_data.get_shot() {
                Some(shot) => shot,
                None => {
                    warn!("Element is shot local but no shot was resolved");
                    return Err(ExportError::NotImplemented);
                }
            };

            if project.shot_exists(&shot) == false {
                warn!("Resolved shot {} does not exist", shot);
                return Err(ExportError::NotImplemented);
            }

            result.push("shot");
        } else {
            result.push("asset");
        }
    }

    let mut path_order = &config.path_order;
    let default_path_order = get_default_path_order();
    if path_order.is_empty() {
        path_order = &default_path_order;
    }

    for entry in path_order {
        trace!("Adding {} to path", entry);
        match map.get(entry.as_str()) {
            Some(entry) => {
                for part in entry.split("/").into_iter() {
                    trace!("Pushing: {}", part);
                    result.push(part);
                }
            }
            None => {
                trace!("Part '{}' was not included in the path order", entry)
            }
        }
    }

    let file_name = if element_data.is_shot_local() {
        let shot_name = element_data
            .get_shot()
            .expect("Element data is shot local, but the element data didn't resolve to a shot")
            .replace("/", "-");
        format!(
            "{}_{}_{}_{}",
            asset_name, shot_name, department, element_name
        )
    } else {
        format!("{}_{}_{}", asset_name, department, element_name)
    };

    Ok((result, file_name))
}
