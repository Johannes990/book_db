use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColorScheme {
    Autumn,
    CoolBlue,
}

pub struct Colors {
    pub text: Color,
    pub text_alt: Color,
    pub text_highlight: Color,
    pub background: Color,
    pub background_alt: Color,
    pub background_highlight: Color,
    pub warning: Color,
    pub error: Color,
    pub border: Color,
    pub accent: Color,
}

impl ColorScheme {
    pub fn colors(&self) -> Colors {
        match self {
            ColorScheme::Autumn => Colors {
                text: Color::Rgb(64, 35, 33),
                text_alt: Color::Rgb(189, 62, 34),
                text_highlight: Color::Rgb(64, 35, 33),
                background: Color::Rgb(247, 201, 35),
                background_alt: Color::Rgb(247, 173, 35),
                background_highlight : Color::Rgb(191, 112, 75),
                warning: Color::Rgb(255, 255, 0),
                error: Color::Rgb(255, 0, 0),
                border: Color::Rgb(64, 35, 33),
                accent: Color::Rgb(150, 167, 35),
            },
            ColorScheme::CoolBlue => Colors {
                text: Color::Rgb(255, 255, 255),
                text_alt: Color::Rgb(255, 255, 0),
                text_highlight: Color::Rgb(255, 0, 0),
                background: Color::Rgb(60, 25, 220),
                background_alt: Color::Rgb(0, 0, 0),
                background_highlight: Color::Rgb(0, 0, 0),
                warning: Color::Rgb(255, 255, 0),
                error: Color::Rgb(255, 0, 0),
                border: Color::Rgb(0, 0, 0),
                accent: Color::Rgb(0, 0, 0),
            },
        }
    }
}
