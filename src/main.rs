use crate::ui::manager::{CryptoChat, Page};

mod net;
mod ui;

fn main() -> iced::Result {
    iced::run("Test App", CryptoChat::update, CryptoChat::view)
}
