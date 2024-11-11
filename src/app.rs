use crate::{
    fex::fextable::FileExplorerTable, handle_key_events, ui::{
        colorscheme::ColorScheme,
        render,
    }
};
use ratatui::{
    style::Color,
    Terminal,
};
use std::{fs, io, path::PathBuf};

pub enum Screen {
    SplashScreenView,
    FileExplorerView,
    CreateNewFileView,
    OptionsView,
}

pub enum PopUp {
    None,
    QuitDialog,
}

pub struct App {
    pub current_screen: Screen,
    pub current_popup: PopUp,
    pub selected_color_scheme: ColorScheme,
    pub terminal_height: u16,
    pub terminal_width: u16,
    pub file_explorer_table: FileExplorerTable,
}

impl App {
    pub fn new(color_scheme: ColorScheme, terminal_height: u16, terminal_width: u16) -> Self {
        let file_explorer_table = FileExplorerTable::new();

        Self {
            current_screen: Screen::SplashScreenView,
            current_popup: PopUp::None,
            selected_color_scheme: color_scheme,
            terminal_height,
            terminal_width,
            file_explorer_table
        }
    }

    pub fn run<B: ratatui::backend::Backend>(&mut self, terminal: &mut Terminal<B>) -> io::Result<()> {
        loop {
            render::render(terminal, self)?;
    
            if handle_key_events(self)? {
                break;
            }
        }
        Ok(())
    }

    pub fn main_pg_bg_col(&self) -> Color {
        self.selected_color_scheme.colors().main_pg_bg_col
    }

    pub fn main_pg_txt_col(&self) -> Color {
        self.selected_color_scheme.colors().main_pg_txt_col
    }

    pub fn quit_popup_bg_col(&self) -> Color {
        self.selected_color_scheme.colors().quit_popup_bg_col
    }

    pub fn quit_popup_txt_col(&self) -> Color {
        self.selected_color_scheme.colors().quit_popup_txt_col
    }

    pub fn quit_popup_txt_highlight_col(&self) -> Color {
        self.selected_color_scheme.colors().quit_popup_txt_highlight_col
    }

    pub fn file_exp_pg_bg_color(&self) -> Color {
        self.selected_color_scheme.colors().file_exp_pg_bg_color
    }

    pub fn file_exp_pg_txt_color(&self) -> Color {
        self.selected_color_scheme.colors().file_exp_pg_txt_col
    }

    pub fn file_exp_pg_folder_col(&self) -> Color {
        self.selected_color_scheme.colors().file_exp_pg_folder_col
    }

    pub fn file_exp_pg_file_color(&self) -> Color {
        self.selected_color_scheme.colors().file_exp_pg_file_color
    }

    pub fn file_exp_pg_parent_folder_col(&self) -> Color {
        self.selected_color_scheme.colors().file_exp_pg_parent_folder_col
    }

    pub fn file_exp_pg_selected_col(&self) -> Color {
        self.selected_color_scheme.colors().file_exp_pg_selected_col
    }

    pub fn table_row_normal_col(&self) -> Color {
        self.selected_color_scheme.colors().table_row_normal_col
    }

    pub fn table_row_alt_color(&self) -> Color {
        self.selected_color_scheme.colors().table_row_alt_color
    }

    pub fn info_block_bg_col(&self) -> Color {
        self.selected_color_scheme.colors().info_block_bg_col
    }

    pub fn info_block_txt_col(&self) -> Color {
        self.selected_color_scheme.colors().info_block_txt_col
    }

    pub fn info_block_txt_highlight_col(&self) -> Color {
        self.selected_color_scheme.colors().info_block_txt_highlight_col
    }

    pub fn switch_to_screen(&mut self, screen: Screen) {
        self.current_screen = screen;
    }

    pub fn switch_to_popup(&mut self, popup: PopUp) {
        self.current_popup = popup;
    }

}
