use std::borrow::Cow;

use iced::{
    Alignment, Length,
    widget::{button, column, row, text},
};

use crate::{
    net::common::Behavior,
    ui::{
        manager::{Message, Page, Pages},
        style::button_outline_style,
    },
};

pub struct WaitingPage {
    behavior: Behavior,
    tries: u8,
}

impl Page for WaitingPage {
    fn view(&self) -> iced::Element<Message> {
        let waiting_text = text(match self.behavior {
            Behavior::Connect => Cow::Owned(format!("Connecting... {}/3", self.tries)),
            Behavior::Listen => Cow::Borrowed("Waiting for a connection..."),
        });

        let cancel_button = button("Cancel")
            .style(button_outline_style)
            .on_press(Message::Move(Pages::HomePage));

        let col = column![waiting_text, cancel_button]
            .spacing(8)
            .align_x(Alignment::Center)
            .width(Length::Fill);
        let row = row![col].align_y(Alignment::Center).height(Length::Fill);

        row.into()
    }

    fn update(&mut self, _message: Message) {
        println!("Hello!");
    }
}

impl WaitingPage {
    pub fn new(behavior: Behavior) -> Self {
        WaitingPage {
            behavior: behavior,
            tries: 1,
        }
    }
}
