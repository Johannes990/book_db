use crate::ui::colorscheme::ColorScheme;
use ratatui::{
    Frame, 
    prelude::Rect,
    style::Style,
    widgets::Block
};
use strum::IntoEnumIterator;

pub struct Options {
    pub available_color_schemes: Vec<ColorScheme>,
    pub selected_color_scheme: ColorScheme,
}

impl Options {
    pub fn new(default_color_scheme: ColorScheme) -> Self {
        let available_color_schemes = ColorScheme::iter().collect();
        Self {
            available_color_schemes,
            selected_color_scheme: default_color_scheme,
        }
    }

    pub fn select_color_scheme(&mut self, color_scheme: ColorScheme) {
        if self.available_color_schemes.contains(&color_scheme) {
            self.selected_color_scheme = color_scheme;
        }
    }

    pub fn list_color_schemes(&self) -> &Vec<ColorScheme> {
        &self.available_color_schemes
    }

    pub fn previous_color_scheme(&mut self) {
        if let Some(index) = self
            .available_color_schemes
            .iter()
            .position(|&cs| cs == self.selected_color_scheme) {
                let prev_index = if index == 0 {
                    self.available_color_schemes.len() - 1
                } else {
                    index - 1
                };
                self.select_color_scheme(self.available_color_schemes[prev_index]);
            }
    }

    pub fn next_color_scheme(&mut self) {
        if let Some(index) = self.available_color_schemes
            .iter()
            .position(|&cs| cs == self.selected_color_scheme) {
                let next_index = (index + 1) % self.available_color_schemes.len();
                self.select_color_scheme(self.available_color_schemes[next_index]);
            }
    }

    pub fn render_color_scheme_preview(frame: &mut Frame, area: Rect, color_scheme: &ColorScheme) {
        let colors = color_scheme.colors();
        let color_vec = vec![
            colors.general_text_color,
            colors.alt_text_color_1,
            colors.alt_text_color_2,
            colors.quit_popup_bg_col,
            colors.general_page_bg_color,
            colors.file_exp_pg_selected_col,
            colors.table_row_normal_col,
            colors.table_row_alt_color,
            colors.info_block_bg_col,
        ];
        let block_width = area.width / color_vec.len() as u16;
        for (i, color) in color_vec.iter().enumerate() {
            let color_area = Rect::new(
                area.width + i as u16 * block_width,
                area.y, 
                block_width,
                area.height
            );
            frame.render_widget(Block::default().style(Style::default().bg(*color)), color_area);
        }
    }
}