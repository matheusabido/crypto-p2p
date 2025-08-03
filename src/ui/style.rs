use iced::{Background, Color, Shadow, Theme, border::Radius, widget::button};

pub fn button_outline_style(_theme: &Theme, status: button::Status) -> button::Style {
    match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::WHITE,
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::WHITE,
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
            border: iced::Border {
                color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.1))),
            text_color: Color::WHITE,
            border: iced::Border {
                color: Color::WHITE,
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
    }
}
