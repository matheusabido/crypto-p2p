use iced::{
    Alignment, Font, Length, Padding,
    font::Weight,
    widget::{Column, TextInput, button, column, row, text, text_input},
};

use crate::{
    net::common::Behavior,
    ui::{
        manager::{Message, Page},
        style::{button_outline_style, button_primary_style},
    },
};

#[derive(Default)]
pub struct HomePage {
    behavior: Behavior,
    connection_addr: String,
    listen_port: String,
}

impl Page for HomePage {
    fn view(&self) -> iced::Element<Message> {
        let title_text = text("Crypto P2P").size(24).font(Font {
            weight: Weight::Bold,
            ..Default::default()
        });

        let profile_buttons = self.create_profile_buttons();
        let connection_input = self.create_connection_input();

        let start_button = button("Start")
            .style(|theme, status| button_primary_style(theme, status))
            .padding(Padding {
                top: 8.0,
                right: 32.0,
                bottom: 8.0,
                left: 32.0,
            });

        let connection_menu = column![profile_buttons, connection_input, start_button]
            .spacing(8)
            .align_x(Alignment::Center);

        let main_menu = column![title_text, connection_menu]
            .align_x(Alignment::Center)
            .width(Length::Fill)
            .spacing(16)
            .padding(8);

        let interface = row![main_menu]
            .align_y(Alignment::Center)
            .height(Length::Fill);
        interface.into()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HomePage(home_message) => match home_message {
                HomeMessage::SelectListen => self.behavior = Behavior::Listen,
                HomeMessage::SelectConnect => self.behavior = Behavior::Connect,
                HomeMessage::InputChanged(val) => match self.behavior {
                    Behavior::Connect => self.connection_addr = val,
                    Behavior::Listen => self.listen_port = val,
                },
            },
            _ => {}
        }
    }
}

impl HomePage {
    fn create_profile_buttons(&self) -> Column<Message> {
        let listen_button = button("Listen")
            .on_press(HomeMessage::SelectListen.into())
            .style(|theme, status| match self.behavior {
                Behavior::Listen => button_primary_style(theme, status),
                Behavior::Connect => button_outline_style(theme, status),
            });

        let connect_button = button("Connect")
            .on_press(HomeMessage::SelectConnect.into())
            .style(|theme, status| match self.behavior {
                Behavior::Listen => button_outline_style(theme, status),
                Behavior::Connect => button_primary_style(theme, status),
            });

        let select_profile_text = text("Please, select your connection profile.");
        let buttons_row = row![listen_button, connect_button].spacing(8);
        let buttons_col = column![select_profile_text, buttons_row]
            .spacing(8)
            .align_x(Alignment::Center);

        buttons_col
    }

    fn create_connection_input(&self) -> TextInput<'_, Message> {
        let input_placeholder = match self.behavior {
            Behavior::Connect => "Connection IP",
            Behavior::Listen => "Port to listen",
        };

        let connection_value = match self.behavior {
            Behavior::Connect => &self.connection_addr,
            Behavior::Listen => &self.listen_port,
        };

        let connection_input = text_input(input_placeholder, connection_value)
            .width(300)
            .on_input(|s| HomeMessage::InputChanged(s).into())
            .line_height(text::LineHeight::Relative(1.5));

        connection_input
    }
}

#[derive(Debug, Clone)]
pub enum HomeMessage {
    SelectListen,
    SelectConnect,
    InputChanged(String),
}

impl Into<Message> for HomeMessage {
    fn into(self) -> Message {
        Message::HomePage(self)
    }
}
