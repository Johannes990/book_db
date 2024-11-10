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
    pub current_path: PathBuf,
    pub file_list: Vec<(String, bool)>,
    pub selected_index: usize,
    pub scroll_offset: usize,
    pub terminal_height: u16,
    pub terminal_width: u16,
    pub file_explorer_table: FileExplorerTable,
}

impl App {
    pub fn new(color_scheme: ColorScheme, terminal_height: u16, terminal_width: u16) -> Self {
        let initial_path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("/"));
        let file_list = get_files_in_dir(&initial_path);
        let file_explorer_table = FileExplorerTable::new();

        Self {
            current_screen: Screen::SplashScreenView,
            current_popup: PopUp::None,
            selected_color_scheme: color_scheme,
            current_path: initial_path,
            file_list,
            selected_index: 0,
            scroll_offset: 0,
            terminal_height,
            terminal_width,
            file_explorer_table
        }
    }

    pub fn update_file_list(&mut self) -> io::Result<()> {
        self.file_list = get_files_in_dir(&self.current_path);
        self.selected_index = 0;
        self.scroll_offset = 0;
        Ok(())
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

fn get_files_in_dir(path: &PathBuf) -> Vec<(String, bool)> {
    match fs::read_dir(path) {
        Ok(entries) => entries
            .filter_map(|entry| entry.ok())
            .map(|entry| {
                let is_dir = entry.file_type().map_or(false, |t| t.is_dir());
                (entry.file_name().into_string().unwrap_or_else(|_| "Invalid UTF-8".into()), is_dir)
            })
            .collect(),
        Err(_) => vec![("<Error reading directory>".into(), false)],
    }
}