mod message;
mod update;
mod view;

use iced::{Task, Theme};

struct UIState {
    project: crate::core::project::Project,
    value: i32,
}

impl UIState {
    pub fn theme(&self) -> Theme {
        Theme::Ferra
    }
}

pub fn gui(project: crate::core::project::Project) {
    let state = UIState {
        project: project,
        value: 0,
    };

    _ = iced::application("Conduct", UIState::update, UIState::view)
        .theme(UIState::theme)
        .run_with(|| (state, Task::none()));
}
