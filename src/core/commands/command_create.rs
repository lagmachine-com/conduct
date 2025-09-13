use core::fmt;
use std::{collections::BTreeMap, sync::RwLock};

use clap::{command, Args};
use log::info;

use crate::core::{
    asset::{Asset, AssetEntry},
    project::Project,
};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct CreateArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,

    #[arg(short, long)]
    pub category: Option<String>,
}

impl Command for CreateArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        info!("Returning result from command create!");

        let mut p = project.write().unwrap();

        match self.common.asset {
            Some(asset) => match create_asset(&mut p, asset) {
                Err(err) => return Err(err),
                Ok(_) => (),
            },
            None => (),
        };

        match self.category {
            Some(category) => match p.create_category_tree_from_path(&category) {
                Some(err) => return Err(CommandError::Message(format!("{}", err).to_string())),
                None => (),
            },
            None => (),
        }

        if context.is_cli {
            p.save();
        }

        Ok(None)
    }
}

fn create_asset(
    p: &mut std::sync::RwLockWriteGuard<'_, Project>,
    asset: String,
) -> Result<(), CommandError> {
    let parts: Vec<String> = asset.split("/").map(|x| x.to_string()).collect();
    let asset_name = parts.last().unwrap();
    match p.get_asset_by_name(asset_name.clone()) {
        Some(existing) => {
            return Err(CommandError::Message(format!(
                "Asset '{}' already exists at '{}'",
                asset_name, existing.1
            )))
        }
        None => (),
    }
    let path = &parts[..(parts.len() - 1)];
    let category_path = path.join("/");
    match p.create_category_tree_from_path(&category_path) {
        Some(err) => return Err(CommandError::Message(format!("{}", err).to_string())),
        None => (),
    }

    let category = p.get_mut_category_by_path(category_path);

    match category {
        Some(category) => {
            for child in category.children.iter() {
                match child.1 {
                    AssetEntry::Asset(asset) => (),
                    AssetEntry::Category(asset_category) => {
                        return Err(CommandError::Message(
                            "Trying to add an asset to a category which contains other categories, this is not currently supported!"
                                .to_string(),
                        ))
                    }
                }
            }

            info!("Adding asset '{}' to category: '{}'", asset, category.name);
            category.children.insert(
                asset_name.clone(),
                AssetEntry::Asset(Asset {
                    departments: BTreeMap::new(),
                }),
            );
        }
        None => (),
    }

    Ok(())
}
