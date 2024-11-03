use std::io;

use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::ui::app::{App, PopUp};

pub fn handle_key_events(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key) = event::read()? {
        match app.current_popup {
            PopUp::None => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Char('Q') => {
                            app.current_popup = PopUp::QuitDialog;
                        }
                        _ => {}
                    }
                }
            }
            PopUp::QuitDialog => {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('n') | KeyCode::Char('N') => {
                            app.current_popup = PopUp::None;
                        }
                        KeyCode::Char('y') | KeyCode::Char('Y') => return Ok(true),
                        _ => {}
                    }
                }
            }
            _ => {}

        }
    }
    Ok(false)
}