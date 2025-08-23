use ratatui::style::Color;
use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Debug, PartialEq, Serialize, Deserialize)]
pub enum ColorScheme {
    Autumn,
    CoolLilac,
    CoolBlue,
    SeaFoam,
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
                background_highlight: Color::Rgb(191, 112, 75),
                warning: Color::Rgb(255, 255, 0),
                error: Color::Rgb(255, 0, 0),
                border: Color::Rgb(64, 35, 33),
                accent: Color::Rgb(150, 167, 35),
            },
            ColorScheme::CoolLilac => Colors {
                text: Color::Rgb(11, 57, 97),
                text_alt: Color::Rgb(87, 54, 145),
                text_highlight: Color::Rgb(8, 18, 105),
                background: Color::Rgb(181, 197, 255),
                background_alt: Color::Rgb(114, 135, 212),
                background_highlight: Color::Rgb(174, 99, 255),
                warning: Color::Rgb(168, 109, 155),
                error: Color::Rgb(255, 0, 0),
                border: Color::Rgb(53, 6, 62),
                accent: Color::Rgb(0, 0, 0),
            },
            ColorScheme::CoolBlue => Colors {
                text: Color::Rgb(28, 47, 87),
                text_alt: Color::Rgb(26, 48, 145),
                text_highlight: Color::Rgb(4, 13, 51),
                background: Color::Rgb(242, 247, 255),
                background_alt: Color::Rgb(227, 237, 255),
                background_highlight: Color::Rgb(140, 207, 255),
                warning: Color::Rgb(150, 188, 255),
                error: Color::Rgb(220, 60, 80),
                border: Color::Rgb(28, 47, 87),
                accent: Color::Rgb(0, 190, 230),
            },
            ColorScheme::SeaFoam => Colors {
                text: Color::Rgb(15, 30, 60),
                text_alt: Color::Rgb(30, 70, 100),
                text_highlight: Color::Rgb(0, 90, 140),
                background: Color::Rgb(225, 245, 255),
                background_alt: Color::Rgb(200, 230, 245),
                background_highlight: Color::Rgb(180, 220, 240),
                warning: Color::Rgb(255, 200, 120),
                error: Color::Rgb(220, 60, 80),
                border: Color::Rgb(40, 80, 120),
                accent: Color::Rgb(0, 170, 200),
            },
        }
    }
}
