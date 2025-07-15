use std::io;
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{
    self,
    Event,
    KeyCode,
    KeyEventKind
};
use rusqlite::ToSql;
use crate::{app::{App, PopUp, Screen, AppMode}, options::SelectedOption};

pub fn handle_key_events(app: &mut App) -> io::Result<bool> {
    if let Event::Key(key_event) = event::read()? {
        match app.current_screen {
            Screen::Splash => splash_screen_handler(app, key_event),
            Screen::FileExplorer => file_explorer_screen_handler(app, key_event),
            Screen::DatabaseSchema => database_schema_screen_handler(app, key_event),
            Screen::DataBaseTable => database_table_screen_handler(app, key_event),
            Screen::Options => options_screen_handler(app, key_event),
            Screen::CreateNewFile => create_new_file_screen_handler(app, key_event),
        }
    }

    Ok(app.should_quit)
}

fn splash_screen_handler(app: &mut App, key_event: KeyEvent) {
    match app.current_popup {
        PopUp::None => {
            if key_event.kind == KeyEventKind::Press {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) | 
                    (KeyCode::Char('q'), KeyModifiers::CONTROL) => app.switch_to_popup(PopUp::Quit),
                    (KeyCode::Char('f'), KeyModifiers::CONTROL) => app.switch_to_screen(Screen::FileExplorer),
                    (KeyCode::Char('n'), KeyModifiers::CONTROL) => app.switch_to_screen(Screen::CreateNewFile),
                    (KeyCode::Char('o'), KeyModifiers::CONTROL) => app.switch_to_screen(Screen::Options),
                    (KeyCode::Char('d'), KeyModifiers::CONTROL) => {
                        if app.selected_db.is_some() {
                            app.switch_to_screen(Screen::DatabaseSchema);
                        } else {
                            app.switch_to_popup(PopUp::NoDBLoaded);
                        }
                        
                    }
                    _ => {}
                }
            }
        },
        PopUp::Quit => {
            if key_event.kind == KeyEventKind::Press {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) |
                    (KeyCode::Char('n'), KeyModifiers::NONE) => app.switch_to_popup(PopUp::None),
                    (KeyCode::Char('y'), KeyModifiers::NONE) => app.should_quit = true,
                    _ => {}
                }
            }
        },
        PopUp::NoDBLoaded => {
            if key_event.kind == KeyEventKind::Press {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_popup(PopUp::None),
                    (KeyCode::Char('f'), KeyModifiers::CONTROL) => {
                        app.switch_to_popup(PopUp::None);
                        app.switch_to_screen(Screen::FileExplorer);
                    }
                    _ => {}
                }
            }
        },
        _ => {}
    }
}

fn file_explorer_screen_handler(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Esc, KeyModifiers::NONE) | 
            (KeyCode::Char('q'), KeyModifiers::NONE) => app.switch_to_screen(Screen::Splash),
            (KeyCode::Up, KeyModifiers::NONE) => app.file_explorer_table.previous(),
            (KeyCode::Down, KeyModifiers::NONE) => app.file_explorer_table.next(),
            (KeyCode::Enter, KeyModifiers::NONE) => {
                // app.selected_index = 0 when we have '..' selected (go back to parent dir)
                if app.file_explorer_table.index == 0 {
                    if let Some(parent) = app.file_explorer_table.current_path.parent() {
                        app.file_explorer_table.current_path = parent.to_path_buf();
                        app.file_explorer_table.update_file_list();
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
                        app.file_explorer_table.update_file_list();
                        app.file_explorer_table.update_scrollbar_state();
                    } else {
                        // handle file opening...
                        if let Err(_e) = app.open_db_file(new_path) {
                            app.switch_to_screen(Screen::Splash);
                        } else {
                            app.switch_to_screen(Screen::DatabaseSchema);
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn database_schema_screen_handler(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::NONE) |
            (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::Splash),
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
                    app.select_table_rows(selected_table.to_string());
                    app.switch_to_screen(Screen::DataBaseTable);
                }
            },
            _ => {}
        }
    }
}

fn database_table_screen_handler(app: &mut App, key_event: KeyEvent) {
    match app.current_popup {
        PopUp::None => {
            if key_event.kind == KeyEventKind::Press {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Char('b'), KeyModifiers::NONE) |
                    (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::DatabaseSchema),
                    (KeyCode::Up, KeyModifiers::NONE) => {
                        if let Some(_) = &app.selected_db {
                            if let Some(_) = &app.selected_db_table {
                                if let Some(_) = &app.row_list_view {
                                    let _ = &app.row_list_view.as_mut().unwrap().previous();
                                }
                            }
                        }
                    },
                    (KeyCode::Down, KeyModifiers::NONE) => {
                        if let Some(_) = &app.selected_db {
                            if let Some(_) = &app.selected_db_table {
                                if let Some(_) = &app.row_list_view {
                                    let _ = &app.row_list_view.as_mut().unwrap().next();
                                }
                            }
                        }
                    },
                    (KeyCode::Char('i'), KeyModifiers::NONE) => {
                        let table_cols: Vec<String> = app.selected_table_columns
                            .iter()
                            .map(|col_info| col_info.name.clone())
                            .collect();
                        app.create_table_insert_form(table_cols);   // create table insert form before rendering
                        app.switch_app_mode(AppMode::Editing);
                        app.switch_to_popup(PopUp::InsertRow);  // rendering is done after switch to InsertRow pop-up
                    },
                    (KeyCode::Char('d'), KeyModifiers::NONE) => app.switch_to_popup(PopUp::DeleteRow),
                    _ => {}
                }
            }
        },
        PopUp::InsertRow => {
            match app.current_mode {
                AppMode::Browsing => {
                },
                AppMode::Editing => {
                    if key_event.kind == KeyEventKind::Press {
                        match (key_event.code, key_event.modifiers) {
                            (KeyCode::Esc, KeyModifiers::NONE) |
                            (KeyCode::Char('q'), KeyModifiers::ALT) => {
                                app.switch_app_mode(AppMode::Browsing);
                                app.switch_to_popup(PopUp::None);
                            } ,
                            (KeyCode::Up, KeyModifiers::NONE) => app.table_insert_form.as_mut().unwrap().previous(),
                            (KeyCode::Down, KeyModifiers::NONE) => app.table_insert_form.as_mut().unwrap().next(),
                            (KeyCode::Char(c), KeyModifiers::NONE) => app.table_insert_form.as_mut().unwrap().enter_char(c),
                            (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                                for c_uppercase in c.to_uppercase() {
                                    app.table_insert_form.as_mut().unwrap().enter_char(c_uppercase);
                                }
                            },
                            (KeyCode::Backspace, KeyModifiers::NONE) => app.table_insert_form.as_mut().unwrap().pop_char(),
                            (KeyCode::Char('s'), KeyModifiers::CONTROL) => {
                                if let Some(db) = app.selected_db.as_mut() {
                                    if let Some(table_name) = &app.selected_db_table {
                                        if let Some(form) = &app.table_insert_form.as_ref() {
                                            let mut columns: Vec<String> = Vec::new();
                                            let mut values: Vec<&dyn ToSql> = Vec::new();
                                            let mut values_str = Vec::new();
                                            
                                            // Pair form items with their column info
                                            for (item, col_info) in form.items
                                                    .iter()
                                                    .zip(&app.selected_table_columns) {
                                                columns.push(col_info.name.clone());
                                                values.push(&item.text_value as &dyn ToSql);
                                                values_str.push(&item.text_value);
                                            }

                                            match db.insert_statement(table_name.clone(), columns, values) {
                                                Ok(_) => {
                                                    app.switch_app_mode(AppMode::Browsing);
                                                    app.switch_to_popup(PopUp::None);
                                                    if let Some(selected_table) = &app.selected_db_table {
                                                        app.select_table_rows(selected_table.to_string());
                                                    }
                                                },
                                                Err(e) => eprintln!("Insert failed: {}", e)
                                            }
                                        }
                                    }
                                }
                            },
                            _ => {}
                        }
                    }
                }
            }
            
        },
        PopUp::DeleteRow => {
            if key_event.kind == KeyEventKind::Press {
                match (key_event.code, key_event.modifiers) {
                    (KeyCode::Enter, KeyModifiers::NONE) => {},
                    (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_popup(PopUp::None),
                    _ =>{}
                }
            }
        }
        _ => {}
    }
}

fn options_screen_handler(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::NONE) |
            (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::Splash),
            (KeyCode::Up, KeyModifiers::NONE) => app.options.previous_option(),
            (KeyCode::Down, KeyModifiers::NONE) => app.options.next_option(),
            (KeyCode::Left, KeyModifiers::NONE) => app.options.previous_color_scheme(),
            (KeyCode::Right, KeyModifiers::NONE) => app.options.next_color_scheme(),
            (KeyCode::Enter, KeyModifiers::NONE) => {
                match app.options.selected_option {
                    SelectedOption::InsertMetainfoToggle => {
                        app.options.set_display_col_metainfo_in_insert_view(
                            !app.options.display_col_metainfo_in_insert_view
                        );
                    },
                    SelectedOption::TableMetainfoToggle => {
                        app.options.set_display_col_metainfo_in_table_view(
                            !app.options.display_col_metainfo_in_table_view
                        );
                    },
                }
            }
            _ => {}
        }
    }
}

fn create_new_file_screen_handler(app: &mut App, key_event: KeyEvent) {
    if key_event.kind == KeyEventKind::Press {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Esc, KeyModifiers::NONE) => app.switch_to_screen(Screen::Splash),
            _ => {}
        }
    }
}
