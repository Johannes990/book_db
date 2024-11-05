use crate::{
    ui::{
        colorscheme::ColorScheme,
        render,
    },
    handle_key_events,
};
use ratatui::{
    style::Color,
    Terminal,
};
use std::io;

pub enum Screen {
    DataBaseEntryView,
}

pub enum PopUp {
    None,
    QuitDialog,
}

pub struct App {
    pub current_screen: Screen,
    pub current_popup: PopUp,
    pub selected_color_scheme: ColorScheme,
}

impl App {
    pub fn new(color_scheme: ColorScheme) -> Self { 
        Self {
            current_screen: Screen::DataBaseEntryView,
            current_popup: PopUp::None,
            selected_color_scheme: color_scheme
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
}