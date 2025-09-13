use std::{collections::BTreeMap, sync::RwLock};

use clap::{command, Args};
use log::info;
use ts_rs::TS;

use crate::core::{
    asset::{AssetCategory, AssetEntry},
    project::Project,
};
use serde::{Deserialize, Serialize};

use super::{args::CommonArgs, error::CommandError, Command, CommandContext};

#[derive(Debug, Args, Serialize, Deserialize)]
pub struct GetAssetTreeArgs {
    #[command(flatten)]
    #[serde(flatten)]
    common: CommonArgs,
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
#[serde(tag = "type")]
pub enum AssetTreeEntry {
    Asset,
    Category(AssetTreeCategory),
}

#[derive(Debug, Serialize, Deserialize, Clone, TS)]
#[ts(export, export_to = "../ui/src/bindings/bindings_gen.ts")]
pub struct AssetTreeCategory {
    children: BTreeMap<String, AssetTreeEntry>,
}

fn category_to_tree_category(category: &AssetCategory) -> AssetTreeCategory {
    let mut result = AssetTreeCategory {
        children: BTreeMap::new(),
    };

    for child in category.children.iter() {
        match child.1 {
            AssetEntry::Asset(_asset) => {
                result
                    .children
                    .insert(child.0.clone(), AssetTreeEntry::Asset);
            }
            AssetEntry::Category(asset_category) => {
                let cat = category_to_tree_category(&asset_category);
                result
                    .children
                    .insert(child.0.clone(), AssetTreeEntry::Category(cat));
            }
        }
    }

    result
}

impl Command for GetAssetTreeArgs {
    fn execute(
        self,
        project: &RwLock<Project>,
        _context: CommandContext,
    ) -> Result<std::option::Option<serde_json::Value>, CommandError> {
        let project = project.read().unwrap();

        let result = category_to_tree_category(&project.assets);

        info!("Returning asset tree: {:#?}", result);

        Ok(Some(serde_json::to_value(result).unwrap()))
    }
}
