use serde::{Deserialize, Serialize};
use strum_macros::EnumIter;

use crate::ui::colors::app_colors::{rgb, AppColors, ColorScheme};

#[derive(Clone, Copy, EnumIter, Debug, PartialEq, Serialize, Deserialize)]
pub enum StaticColors {
    Autumn,
    CoolLilac,
    CoolBlue,
    SeaFoam,
    BlackAndWhite,
    WhiteAndBlack,
}

impl ColorScheme for StaticColors {
    fn colors(&self) -> AppColors {
        match self {
            StaticColors::Autumn => AppColors {
                text: rgb(64, 35, 33),
                text_alt: rgb(189, 62, 34),
                text_highlight: rgb(64, 35, 33),
                background: rgb(247, 201, 35),
                background_alt: rgb(247, 173, 35),
                background_highlight: rgb(191, 112, 75),
                warning: rgb(255, 255, 0),
                error: rgb(255, 0, 0),
                border: rgb(64, 35, 33),
                accent: rgb(150, 167, 35),
            },
            StaticColors::CoolLilac => AppColors {
                text: rgb(11, 57, 97),
                text_alt: rgb(87, 54, 145),
                text_highlight: rgb(8, 18, 105),
                background: rgb(181, 197, 255),
                background_alt: rgb(114, 135, 212),
                background_highlight: rgb(174, 99, 255),
                warning: rgb(168, 109, 155),
                error: rgb(255, 0, 0),
                border: rgb(53, 6, 62),
                accent: rgb(0, 0, 0),
            },
            StaticColors::CoolBlue => AppColors {
                text: rgb(28, 47, 87),
                text_alt: rgb(26, 48, 145),
                text_highlight: rgb(4, 13, 51),
                background: rgb(242, 247, 255),
                background_alt: rgb(227, 237, 255),
                background_highlight: rgb(140, 207, 255),
                warning: rgb(150, 188, 255),
                error: rgb(220, 60, 80),
                border: rgb(28, 47, 87),
                accent: rgb(0, 190, 230),
            },
            StaticColors::SeaFoam => AppColors {
                text: rgb(15, 30, 60),
                text_alt: rgb(30, 70, 100),
                text_highlight: rgb(0, 90, 140),
                background: rgb(225, 245, 255),
                background_alt: rgb(200, 230, 245),
                background_highlight: rgb(180, 220, 240),
                warning: rgb(255, 200, 120),
                error: rgb(220, 60, 80),
                border: rgb(40, 80, 120),
                accent: rgb(0, 170, 200),
            },
            StaticColors::BlackAndWhite => AppColors {
                text: rgb(0, 0, 0),
                text_alt: rgb(0, 0, 0),
                text_highlight: rgb(255, 255, 255),
                background: rgb(255, 255, 255),
                background_alt: rgb(255, 255, 255),
                background_highlight: rgb(0, 0, 0),
                warning: rgb(0, 0, 0),
                error: rgb(0, 0, 0),
                border: rgb(0, 0, 0),
                accent: rgb(0, 0, 0),
            },
            StaticColors::WhiteAndBlack => AppColors{
                text: rgb(255, 255, 255),
                text_alt: rgb(255, 255, 255),
                text_highlight: rgb(0, 0, 0),
                background: rgb(0, 0, 0),
                background_alt: rgb(0, 0, 0),
                background_highlight: rgb(255, 255, 255),
                warning: rgb(255, 255, 255),
                error: rgb(255, 255, 255),
                border: rgb(255, 255, 255),
                accent: rgb(255, 255, 255),
            },
        }
    }
}
