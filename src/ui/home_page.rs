use iced::widget::{button, column, text};

use crate::ui::{
    manager::{Message, Page, Pages},
    style::button_outline_style,
};

#[derive(Default)]
pub struct HomePage;
impl Page for HomePage {
    fn view(&self) -> iced::Element<Message> {
        let home_text = text("You're currently on the Home Page");
        let move_button = button("Move to Chat Page").on_press(Message::Move(Pages::ChatPage));
        let hello_button = button("Hello from Home")
            .on_press(HomeMessage::Hello.into())
            .style(button_outline_style);

        let interface = column![home_text, hello_button, move_button]
            .padding(8)
            .spacing(8);
        interface.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HomePage(home_message) => match home_message {
                HomeMessage::Hello => println!("Hello!"),
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum HomeMessage {
    Hello,
}

impl Into<Message> for HomeMessage {
    fn into(self) -> Message {
        Message::HomePage(self)
    }
}
