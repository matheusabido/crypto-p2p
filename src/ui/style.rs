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

pub fn button_primary_style(_theme: &Theme, status: button::Status) -> button::Style {
    let base_color = Color::from_rgb(0.1, 0.4, 0.75);
    let hover_color = Color::from_rgb(0.15, 0.45, 0.8);
    let pressed_color = Color::from_rgb(0.08, 0.35, 0.65);
    let disabled_color = Color::from_rgba(0.1, 0.4, 0.75, 0.5);

    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(base_color)),
            text_color: Color::WHITE,
            border: iced::Border {
                color: base_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(hover_color)),
            text_color: Color::WHITE,
            border: iced::Border {
                color: hover_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(pressed_color)),
            text_color: Color::WHITE,
            border: iced::Border {
                color: pressed_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(disabled_color)),
            text_color: Color::from_rgba(1.0, 1.0, 1.0, 0.7),
            border: iced::Border {
                color: disabled_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
    }
}
