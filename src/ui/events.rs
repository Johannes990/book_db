use std::io;
use crossterm::event::KeyModifiers;
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use crate::app::{App, PopUp, Screen};


pub fn handle_key_events(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key_event) = event::read()? {
        match app.current_screen {
            Screen::SplashScreenView => {
                match app.current_popup {
                    PopUp::None => {
                        if key_event.kind == KeyEventKind::Press {
                            match (key_event.code, key_event.modifiers) {
                                (KeyCode::Esc, KeyModifiers::NONE) | 
                                (KeyCode::Char('q'), KeyModifiers::CONTROL) => app.switch_to_popup(PopUp::QuitDialog),
                                (KeyCode::Char('f'), KeyModifiers::CONTROL) => app.switch_to_screen(Screen::FileExplorerView),
                                (KeyCode::Char('n'), KeyModifiers::CONTROL) => app.switch_to_screen(Screen::CreateNewFileView),
                                (KeyCode::Char('o'), KeyModifiers::CONTROL) => app.switch_to_screen(Screen::OptionsView),
                                (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                                    if app.selected_db.is_some() {
                                        app.switch_to_screen(Screen::OpenDataBaseView);
                                    } else {
                                        app.switch_to_popup(PopUp::NoDBLoadedDialog);
                                    }
                                    
                                }
                                _ => {}
                            }
                        }
                    }
                    PopUp::QuitDialog => {
                        if key_event.kind == KeyEventKind::Press {
                            match (key_event.code, key_event.modifiers) {
                                (KeyCode::Esc, KeyModifiers::NONE) |
                                (KeyCode::Char('n'), KeyModifiers::NONE) => app.switch_to_popup(PopUp::None),
                                (KeyCode::Char('y'), KeyModifiers::NONE) => return Ok(true),
                                _ => {}
                            }
                        }
                    }
                    PopUp::SaveDialog => {},
                    PopUp::NoDBLoadedDialog => {
                        if key_event.kind == KeyEventKind::Press {
                            match (key_event.code, key_event.modifiers) {
                                (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_popup(PopUp::None),
                                (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                                    app.switch_to_popup(PopUp::None);
                                    app.switch_to_screen(Screen::FileExplorerView);
                                }
                                _ => {}
                            }
                        }
                    }
                }
            },
            Screen::FileExplorerView => {
                if key_event.kind == KeyEventKind::Press {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Esc, KeyModifiers::NONE) | 
                        (KeyCode::Char('q'), KeyModifiers::NONE) => app.switch_to_screen(Screen::SplashScreenView),
                        (KeyCode::Up, KeyModifiers::NONE) => app.file_explorer_table.previous(),
                        (KeyCode::Down, KeyModifiers::NONE) => app.file_explorer_table.next(),
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            // app.selected_index = 0 when we have '..' selected (go back to parent dir)
                            if app.file_explorer_table.index == 0 {
                                if let Some(parent) = app.file_explorer_table.current_path.parent() {
                                    app.file_explorer_table.current_path = parent.to_path_buf();
                                    app.file_explorer_table.update_file_list()?;
                                    app.file_explorer_table.update_scrollbar_state();
                                }
                            } else {
                                let data_row = &app
                                    .file_explorer_table.items[app.file_explorer_table.index];
                                let selected_file = data_row.path_name();
                                let is_dir = data_row.is_dir();
                                let new_path = app.file_explorer_table.current_path.join(selected_file);

                                if *is_dir && new_path.is_dir() {
                                    app.file_explorer_table.current_path = new_path;
                                    app.file_explorer_table.index = 0;
                                    app.file_explorer_table.update_file_list()?;
                                    app.file_explorer_table.update_scrollbar_state();
                                } else {
                                    // handle file opening...
                                    if let Err(_e) = app.open_db_file(new_path) {
                                        app.switch_to_screen(Screen::SplashScreenView);
                                    } else {
                                        app.switch_to_screen(Screen::OpenDataBaseView);
                                    }
                                }
                            }
                        }
                        _ => {}
                    }
                }
            },
            Screen::OpenDataBaseView => {
                if key_event.kind == KeyEventKind::Press {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::NONE) |
                        (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::SplashScreenView),
                        (KeyCode::Up, KeyModifiers::NONE) => {
                            if let Some(db) = &app.selected_db {
                                let tables = db.get_table_list().unwrap_or_default();
                                if let Some(selected_table) = &app.selected_db_table {
                                    let current_idx = tables.iter().position(|t| t == selected_table).unwrap_or(0);
                                    if current_idx > 0 {
                                        app.select_table(tables[current_idx - 1].clone());
                                    } else if current_idx == 0 {
                                        app.select_table(tables[tables.len() - 1].clone());
                                    } else if !tables.is_empty() {
                                        app.select_table(tables[0].clone());
                                    }
                                    let _ = &app.table_list_view.as_mut().unwrap().previous();
                                }
                            }
                        },
                        (KeyCode::Down, KeyModifiers::NONE) => {
                            if let Some(db) = &app.selected_db {
                                let tables = db.get_table_list().unwrap_or_default();
                                if let Some(selected_table) = &app.selected_db_table {
                                    let current_idx = tables.iter().position(|t| t == selected_table).unwrap_or(0);
                                    if current_idx < tables.len() - 1 {
                                        app.select_table(tables[current_idx + 1].clone());
                                    } else if !tables.is_empty() {
                                        app.select_table(tables[0].clone());
                                    }
                                    let _ = &app.table_list_view.as_mut().unwrap().next();
                                }
                            }
                        },
                        (KeyCode::Char('s'), KeyModifiers::NONE) => {
                            if let Some(_) = &app.selected_db {
                                if let Some(_) = &app.column_list_view {
                                    let _ = &app.column_list_view.as_mut().unwrap().previous();
                                }
                            }
                        },
                        (KeyCode::Char('x'), KeyModifiers::NONE) => {
                            if let Some(_) = &app.selected_db {
                                if let Some(_) = &app.column_list_view {
                                    let _ = &app.column_list_view.as_mut().unwrap().next();
                                }
                            }
                        },
                        (KeyCode::Enter, KeyModifiers::NONE) => {
                            if let Some(selected_table) = &app.selected_db_table {
                                app.select_table(selected_table.clone());
                            }
                        },
                        _ => {}
                    }
                }
            }
            Screen::OptionsView => {
                if key_event.kind == KeyEventKind::Press {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char('q'), KeyModifiers::NONE) |
                        (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::SplashScreenView),
                        (KeyCode::Up, KeyModifiers::NONE) => app.options.previous_color_scheme(),
                        (KeyCode::Down, KeyModifiers::NONE) => app.options.next_color_scheme(),
                        _ => {}
                    }
                }
            },
            Screen::CreateNewFileView => {
                if key_event.kind == KeyEventKind::Press {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::SplashScreenView),
                        _ => {}
                    }
                }
            },
            
        }
    }
    Ok(false)
}
