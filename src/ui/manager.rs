use iced::{Element, Theme};

use crate::{
    net::common::Behavior,
    ui::page::{
        chat_page::{ChatMessage, ChatPage},
        home_page::{HomeMessage, HomePage},
        waiting_page::WaitingPage,
    },
};

pub trait Page {
    fn view(&self) -> Element<Message>;
    fn update(&mut self, message: Message);
}

pub struct CryptoChat {
    page: Box<dyn Page>,
}

impl Default for CryptoChat {
    fn default() -> Self {
        Self {
            page: Box::new(HomePage::default()),
        }
    }
}

impl Page for CryptoChat {
    fn view(&self) -> Element<Message> {
        self.page.view()
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::Move(page) => {
                match page {
                    Pages::ChatPage => self.page = Box::new(ChatPage::default()),
                    Pages::HomePage => self.page = Box::new(HomePage::default()),
                    Pages::WaitingPage(behavior) => {
                        self.page = Box::new(WaitingPage::new(behavior))
                    }
                };
            }
            _ => self.page.update(message),
        }
    }
}

impl CryptoChat {
    pub fn theme(&self) -> Theme {
        Theme::Dark
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    HomePage(HomeMessage),
    ChatPage(ChatMessage),
    Move(Pages),
}

#[derive(Debug, Clone)]
pub enum Pages {
    HomePage,
    ChatPage,
    WaitingPage(Behavior),
}
