use crate::ui::colorscheme::ColorScheme;
use strum::{EnumIter, IntoEnumIterator};

#[derive(EnumIter, Clone, Copy, PartialEq)]
pub enum SelectedOption {
    TableMetainfoToggle,
    InsertMetainfoToggle,
}

pub struct Options {
    pub available_color_schemes: Vec<ColorScheme>,
    pub selected_color_scheme: ColorScheme,
    pub available_options: Vec<SelectedOption>,
    pub selected_option: SelectedOption,
    pub display_col_metainfo_in_table_view: bool,
    pub display_col_metainfo_in_insert_view: bool,
}

impl Options {
    pub fn new(default_color_scheme: ColorScheme) -> Self {
        let available_color_schemes = ColorScheme::iter().collect();
        let available_options = SelectedOption::iter().collect();
        Self {
            available_color_schemes,
            selected_color_scheme: default_color_scheme,
            available_options,
            selected_option: SelectedOption::TableMetainfoToggle,
            display_col_metainfo_in_table_view: true,
            display_col_metainfo_in_insert_view: true,
        }
    }

    fn select_color_scheme(&mut self, color_scheme: ColorScheme) {
        if self.available_color_schemes.contains(&color_scheme) {
            self.selected_color_scheme = color_scheme;
        }
    }

    pub fn list_color_schemes(&self) -> &Vec<ColorScheme> {
        &self.available_color_schemes
    }

    pub fn previous_color_scheme(&mut self) {
        if let Some(index) = self.available_color_schemes
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

    pub fn set_display_col_metainfo_in_table_view(&mut self, value: bool) {
        self.display_col_metainfo_in_table_view = value;        
    }

    pub fn set_display_col_metainfo_in_insert_view(&mut self, value: bool) {
        self.display_col_metainfo_in_insert_view = value;
    }

    pub fn previous_option(&mut self) {
        if let Some(index) = self.available_options
            .iter()
            .position(|&so| so == self.selected_option) {
                let prev_index = if index == 0 {
                    self.available_options.len() - 1
                } else {
                    index - 1
                };
                self.select_option(self.available_options[prev_index]);
            }
    }

    pub fn next_option(&mut self) {
        if let Some(index) = self.available_options
            .iter()
            .position(|&so| so == self.selected_option) {
                let next_index = (index + 1) % self.available_options.len();
                self.select_option(self.available_options[next_index]);
            }
    }

    fn select_option(&mut self, option: SelectedOption) {
        if self.available_options.contains(&option) {
            self.selected_option = option;
        }
    }
}