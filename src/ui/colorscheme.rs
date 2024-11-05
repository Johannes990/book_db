use ratatui::style::Color;

pub enum ColorScheme {
    Autumn,
    CoolBlue,
}

pub struct Colors {
    pub quit_popup_bg_col: Color,
    pub quit_popup_txt_col: Color,
    pub main_pg_bg_col: Color,
    pub main_pg_txt_col: Color,
}

impl ColorScheme {
    pub fn colors(&self) -> Colors {
        match self {
            ColorScheme::Autumn => Colors {
                quit_popup_bg_col: Color::Rgb(200, 140, 30),
                quit_popup_txt_col: Color::Rgb(255, 255, 200),
                main_pg_bg_col: Color::Rgb(190, 190, 40),
                main_pg_txt_col: Color::Rgb(255, 255, 255),
            },
            ColorScheme::CoolBlue => Colors {
                quit_popup_bg_col: Color::Rgb(0, 0, 170),
                quit_popup_txt_col: Color::Rgb(200, 200, 200),
                main_pg_bg_col: Color::Rgb(60, 25, 220),
                main_pg_txt_col: Color::Rgb(255, 255, 255)
            }
        }
    }
}