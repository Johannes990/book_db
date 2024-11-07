use std::{io, fs};

use crossterm::event::{KeyModifiers, KeyboardEnhancementFlags, PushKeyboardEnhancementFlags};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::{App, PopUp, Screen};

pub fn setup_keyboard_enchancements() {
    PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES |
        KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
    );
}

pub fn handle_key_events(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key_event) = event::read()? {
        match app.current_screen {
            Screen::SplashScreenView => {
                match app.current_popup {
                    PopUp::None => {
                        if key_event.kind == KeyEventKind::Press {
                            match key_event.code {
                                KeyCode::Esc | KeyCode::Char('q') => app.switch_to_popup(PopUp::QuitDialog),
                                KeyCode::Char('f') => app.switch_to_screen(Screen::FileExplorerView),
                                KeyCode::Char('c') => app.switch_to_screen(Screen::CreateNewFileView),
                                KeyCode::Char('o') => app.switch_to_screen(Screen::OptionsView),
                                _ => {}
                            }
                        }
                    }
                    PopUp::QuitDialog => {
                        if key_event.kind == KeyEventKind::Press {
                            match (key_event.code, key_event.modifiers) {
                                (KeyCode::Esc, KeyModifiers::NONE) |
                                (KeyCode::Char('n'), KeyModifiers::NONE) => {
                                    app.switch_to_popup(PopUp::None);
                                }
                                (KeyCode::Char('y'), KeyModifiers::NONE) => return Ok(true),
                                _ => {}
                            }
                        }
                    }
                    _ => {}
                }
            },
            Screen::FileExplorerView => {
                if key_event.kind == KeyEventKind::Press {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::SplashScreenView),
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if app.selected_index > 0 {
                                app.selected_index -= 1;
                            }
                        },
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if app.selected_index < app.file_list.len() {
                                app.selected_index += 1;
                            }
                        }
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            // app.selected_index = 0 when we have '..' selected (go back to parent dir)
                            if app.selected_index == 0 {
                                if let Some(parent) = app.current_path.parent() {
                                    app.current_path = parent.to_path_buf();
                                    app.update_file_list()?;
                                }
                            } else {
                                let selected_file = &app.file_list[app.selected_index - 1];
                                let new_path = app.current_path.join(selected_file);

                                if new_path.is_dir() {
                                    app.current_path = new_path;
                                    app.update_file_list()?;
                                } else {
                                    // handle file opening...
                                }
                            }
                        }
                        _ => {}
                    }
                }
            },
            Screen::OptionsView => {

            },
            Screen::CreateNewFileView => {

            },
            _ => {}
        }
    }
    Ok(false)
}