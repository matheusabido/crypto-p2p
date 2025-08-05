use crate::ui::manager::{CryptoChat, Page};

mod net;
mod ui;

fn main() -> iced::Result {
    iced::application("Test App", CryptoChat::update, CryptoChat::view)
        .theme(CryptoChat::theme)
        .run()
}
