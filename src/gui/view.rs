use super::{message::Message, UIState};

use iced::widget::{button, column, row, text, Column};

impl UIState {
    pub fn view(&self) -> Column<Message> {
        // We use a column: a simple vertical layout
        column![
            row![
                text(self.project.get_identifier()),
                text(self.project.get_display_name())
            ],
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),
            // We show the value of the counter here
            text(self.value).size(50),
            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }
}
