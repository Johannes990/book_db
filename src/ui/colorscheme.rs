use ratatui::style::Color;
use strum_macros::EnumIter;

#[derive(Clone, Copy, EnumIter, Debug, PartialEq)]
pub enum ColorScheme {
    Autumn,
    CoolBlue,
}

pub struct Colors {
    pub general_text_color: Color,
    pub alt_text_color_1: Color,
    pub alt_text_color_2: Color,
    pub quit_popup_bg_col: Color,
    pub general_page_bg_color: Color,
    pub file_exp_pg_selected_col: Color,
    pub table_row_normal_col: Color,
    pub table_row_alt_color: Color,
    pub info_block_bg_col: Color,
    pub text_entry_box_bg_col: Color,
}

impl ColorScheme {
    pub fn colors(&self) -> Colors {
        match self {
            ColorScheme::Autumn => Colors {
                general_text_color: Color::Rgb(0, 0, 0),
                alt_text_color_1: Color::Rgb(200, 255, 200),
                alt_text_color_2: Color::Rgb(255, 0, 0),
                quit_popup_bg_col: Color::Rgb(200, 140, 30),
                general_page_bg_color: Color::Rgb(190, 190, 40),
                file_exp_pg_selected_col: Color::Rgb(255, 255, 0),
                table_row_normal_col: Color::Rgb(200, 100, 0),
                table_row_alt_color: Color::Rgb(180, 110, 0),
                info_block_bg_col: Color::Rgb(255, 255, 255),
                text_entry_box_bg_col: Color::Rgb(0, 255, 0),
            },
            ColorScheme::CoolBlue => Colors {
                general_text_color: Color::Rgb(255, 255, 255),
                alt_text_color_1: Color::Rgb(255, 255, 0),
                alt_text_color_2: Color::Rgb(255, 0, 0),
                quit_popup_bg_col: Color::Rgb(0, 0, 170),
                general_page_bg_color: Color::Rgb(60, 25, 220),
                file_exp_pg_selected_col: Color::Rgb(255, 255, 0),
                table_row_normal_col: Color::Rgb(10, 10, 20),
                table_row_alt_color: Color::Rgb(10, 10, 25),
                info_block_bg_col: Color::Rgb(30, 30, 255),
                text_entry_box_bg_col: Color::Rgb(0, 30, 200),
            }
        }
    }
}