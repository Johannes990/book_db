use ratatui::style::Color;

pub enum ColorScheme {
    Autumn,
    CoolBlue,
}

pub struct Colors {
    pub quit_popup_bg_col: Color,
    pub quit_popup_txt_col: Color,
    pub quit_popup_txt_highlight_col: Color,
    pub main_pg_bg_col: Color,
    pub main_pg_txt_col: Color,
    pub file_exp_pg_bg_color: Color,
    pub file_exp_pg_txt_col: Color,
    pub file_exp_pg_folder_col: Color,
    pub file_exp_pg_file_color: Color,
    pub file_exp_pg_parent_folder_col: Color,
    pub file_exp_pg_selected_col: Color,
    pub info_block_bg_col: Color,
    pub info_block_txt_col: Color,
    pub info_block_txt_highlight_col: Color,
}

impl ColorScheme {
    pub fn colors(&self) -> Colors {
        match self {
            ColorScheme::Autumn => Colors {
                quit_popup_bg_col: Color::Rgb(200, 140, 30),
                quit_popup_txt_col: Color::Rgb(255, 255, 200),
                quit_popup_txt_highlight_col: Color::Rgb(0, 0, 0),
                main_pg_bg_col: Color::Rgb(190, 190, 40),
                main_pg_txt_col: Color::Rgb(255, 255, 255),
                file_exp_pg_bg_color: Color::Rgb(200, 140, 30),
                file_exp_pg_txt_col: Color::Rgb(255, 255, 200),
                file_exp_pg_folder_col: Color::Rgb(255, 0, 0),
                file_exp_pg_file_color: Color::Rgb(200, 200, 200),
                file_exp_pg_parent_folder_col: Color::Rgb(0, 255, 0),
                file_exp_pg_selected_col: Color::Rgb(255, 255, 0),
                info_block_bg_col: Color::Rgb(255, 255, 255),
                info_block_txt_col: Color::Rgb(0, 0, 0),
                info_block_txt_highlight_col: Color::Rgb(255, 0, 0),
            },
            ColorScheme::CoolBlue => Colors {
                quit_popup_bg_col: Color::Rgb(0, 0, 170),
                quit_popup_txt_col: Color::Rgb(200, 200, 200),
                quit_popup_txt_highlight_col: Color::Rgb(10, 60, 200),
                main_pg_bg_col: Color::Rgb(60, 25, 220),
                main_pg_txt_col: Color::Rgb(255, 255, 255),
                file_exp_pg_bg_color: Color::Rgb(0, 140, 30),
                file_exp_pg_txt_col: Color::Rgb(0, 255, 200),
                file_exp_pg_folder_col: Color::Rgb(0, 100, 200),
                file_exp_pg_file_color: Color::Rgb(200, 200, 200),
                file_exp_pg_parent_folder_col: Color::Rgb(0, 255, 0),
                file_exp_pg_selected_col: Color::Rgb(255, 255, 0),
                info_block_bg_col: Color::Rgb(255, 255, 255),
                info_block_txt_col: Color::Rgb(0, 0, 0),
                info_block_txt_highlight_col: Color::Rgb(255, 0, 0),
            }
        }
    }
}