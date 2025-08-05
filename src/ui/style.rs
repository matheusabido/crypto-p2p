use iced::{Background, Color, Shadow, Theme, border::Radius, widget::button};

use crate::ui::utils::{darken, lighten};

pub fn button_outline_style(theme: &Theme, status: button::Status) -> button::Style {
    let base_color = theme.palette().text;
    match status {
        button::Status::Active => button::Style {
            background: None,
            text_color: base_color,
            border: iced::Border {
                color: base_color,
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(Color::from_rgba(1.0, 1.0, 1.0, 0.05))),
            text_color: base_color,
            border: iced::Border {
                color: base_color,
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Disabled => button::Style {
            background: None,
            text_color: lighten(base_color, 0.1),
            border: iced::Border {
                color: lighten(base_color, 0.1),
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(Color::from_rgba(0.0, 0.0, 0.0, 0.1))),
            text_color: base_color,
            border: iced::Border {
                color: base_color,
                width: 1.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
    }
}

pub fn button_primary_style(theme: &Theme, status: button::Status) -> button::Style {
    let base_color = theme.palette().primary;
    let hover_color = lighten(theme.palette().primary, 0.05);
    let pressed_color = darken(theme.palette().primary, 0.05);
    let disabled_color = lighten(theme.palette().primary, 0.15);
    let text_color = theme.palette().text;

    match status {
        button::Status::Active => button::Style {
            background: Some(Background::Color(base_color)),
            text_color: text_color,
            border: iced::Border {
                color: base_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Hovered => button::Style {
            background: Some(Background::Color(hover_color)),
            text_color: text_color,
            border: iced::Border {
                color: hover_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Pressed => button::Style {
            background: Some(Background::Color(pressed_color)),
            text_color: text_color,
            border: iced::Border {
                color: pressed_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
        button::Status::Disabled => button::Style {
            background: Some(Background::Color(disabled_color)),
            text_color: lighten(text_color, 0.1),
            border: iced::Border {
                color: disabled_color,
                width: 0.0,
                radius: Radius::new(5),
            },
            shadow: Shadow::default(),
        },
    }
}
