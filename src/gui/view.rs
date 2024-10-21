use crate::core::asset::AssetCategory;

use super::{message::Message, UIState};

use iced::{
    widget::{button, column, container, row, text, Column},
    Color, Element,
};
use serde_yaml::mapping::Entry;

impl UIState {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            row![
                text(self.project.get_display_name()),
                text(self.project.get_identifier()).size(12),
            ]
            .spacing(5)
            .padding(5),
            self.assets_view(&self.project.assets)
        ]
    }

    fn assets_view(&self, category: &AssetCategory) -> iced::Element<Message> {
        let column = (&category.children)
            .iter()
            .fold(Column::new(), |column, pair| match pair.1 {
                crate::core::asset::AssetEntry::Asset(asset) => {
                    column.push(text(asset.name.clone()).style(text::primary))
                }
                crate::core::asset::AssetEntry::Category(asset_category) => column.push(column![
                    text(asset_category.name.clone()).style(text::secondary),
                    container(self.assets_view(asset_category)).padding(15),
                ]),
            });

        return column.into();
    }
}
