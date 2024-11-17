use std::path::PathBuf;

use log::{info, warn};

use crate::core::{
    context::Context, element::element_resolver::ElementResolver, project,
    shot::shot_resolver::ShotResolver,
};

use super::ExportError;

pub fn resolve_element_path(
    project: &project::Project,
    department: String,
    asset_name: String,
    element_name: String,
    shot: Option<String>,
) -> Result<(PathBuf, String), ExportError> {
    let asset = project.get_asset_by_name(asset_name.clone());

    let (_asset, path) = match &asset {
        Some(asset) => asset,
        None => {
            warn!("Could not find asset entry!");
            return Err(ExportError::NotImplemented);
        }
    };

    info!("Found asset at path: {}", path);

    let element = project.get_element(
        asset_name.clone(),
        element_name.clone(),
        &Context {
            department: Some(department.clone()),
            mode: crate::core::context::ContextMode::Export,
            shot: shot,
        },
    );

    let element_data = match element {
        Some(data) => data,
        None => {
            warn!("Could not find resolve element '{}'!", element_name);
            return Err(ExportError::NotImplemented);
        }
    };

    let mut result = PathBuf::new();

    if element_data.is_shot_local() {
        let shot = match element_data.get_shot() {
            Some(shot) => shot,
            None => {
                warn!("Element is shot local but no shot was resolved");
                return Err(ExportError::NotImplemented);
            }
        };

        warn!("TODO: Implement scene local file path handling");

        if project.shot_exists(&shot) == false {
            warn!("Resolved shot {} does not exist", shot);
            return Err(ExportError::NotImplemented);
        }

        result.push("shot");

        for part in shot.split("/").into_iter() {
            result.push(part.to_lowercase());
        }

        info!("Result path: {}", result.to_str().unwrap());
    } else {
        result.push("asset");
    }

    for part in path.split("/").into_iter() {
        result.push(part);
    }

    result.push(
        element_data
            .get_asset_name()
            .expect("Element did not have a valid asset name"),
    );
    result.push(&department);
    result.push(&element_name);

    let file_name = if element_data.is_shot_local() {
        let shot_name = element_data.get_shot().unwrap().replace("/", "-");
        format!(
            "{}_{}_{}_{}",
            asset_name, shot_name, department, element_name
        )
    } else {
        format!("{}_{}_{}", asset_name, department, element_name)
    };

    Ok((result, file_name))
}
