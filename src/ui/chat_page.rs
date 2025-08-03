use iced::widget::{button, column, text};

use crate::ui::{
    manager::{Message, Page, Pages},
    style::button_outline_style,
};

#[derive(Default)]
pub struct ChatPage;
impl Page for ChatPage {
    fn view(&self) -> iced::Element<Message> {
        let home_text = text("You're currently on the Chat Page");
        let move_button = button("Move to Home Page").on_press(Message::Move(Pages::HomePage));
        let bye_button = button("Bye from Chat")
            .on_press(ChatMessage::Bye.into())
            .style(button_outline_style);

        let interface = column![home_text, bye_button, move_button]
            .padding(8)
            .spacing(8);
        interface.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::ChatPage(chat_message) => match chat_message {
                ChatMessage::Bye => println!("Bye!"),
            },
            _ => {}
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ChatMessage {
    Bye,
}

impl Into<Message> for ChatMessage {
    fn into(self) -> Message {
        Message::ChatPage(self)
    }
}
