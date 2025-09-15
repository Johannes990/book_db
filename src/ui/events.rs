use crate::{
    app::{App, Mode, PopUp, Screen},
    db::{DBError, DB},
    log::log,
    options::{OptionKind, SelectedOption},
    ui::input::key_bindings::AppInputEvent,
};
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusqlite::ToSql;
use std::{io, time::Duration};

pub fn handle_key_events(app: &mut App) -> io::Result<bool> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            match app.current_screen {
                Screen::Splash => splash_screen_handler(app, key_event)?,
                Screen::FileExplorer => file_explorer_screen_handler(app, key_event)?,
                Screen::DatabaseSchema => database_schema_screen_handler(app, key_event)?,
                Screen::DataBaseTable => database_table_screen_handler(app, key_event)?,
                Screen::Options => options_screen_handler(app, key_event)?,
                Screen::CreateNewFile => create_new_file_screen_handler(app, key_event)?,
            }
            match app.current_popup {
                PopUp::Quit => quit_popup_handler(app, key_event)?,
                PopUp::NoDBLoaded => no_db_loaded_popup_handler(app, key_event)?,
                PopUp::InsertRow => insert_row_popup_handler(app, key_event)?,
                PopUp::DeleteRow => delete_row_popup_handler(app, key_event)?,
                PopUp::InsertTable => insert_table_popup_handler(app, key_event)?,
                PopUp::DeleteTable => delete_table_popup_handler(app, key_event)?,
                PopUp::Error => error_popup_handler(app, key_event)?,
                PopUp::None => {}
            }
        }
    }

    if app.should_quit {
        let _ = app
            .key_bindings
            .save(&app.qualifier, &app.organization, &app.application);
    }

    Ok(app.should_quit)
}

fn splash_screen_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        match event {
            AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
            AppInputEvent::OpenFileExplorerScreen => app.switch_to_screen(Screen::FileExplorer),
            AppInputEvent::OpenDBSchemaScreen => {
                if app.selected_db.is_some() {
                    app.switch_to_screen(Screen::DatabaseSchema);
                } else {
                    app.switch_to_popup(PopUp::NoDBLoaded);
                }
            }
            AppInputEvent::OpenDBTableScreen => app.switch_to_screen(Screen::DataBaseTable),
            AppInputEvent::OpenCreateNewFileScreen => {
                app.create_new_db_form();
                app.switch_to_screen(Screen::CreateNewFile);
            }
            AppInputEvent::OpenOptionsScreen => app.switch_to_screen(Screen::Options),
            _ => {}
        }
    }

    Ok(())
}

fn file_explorer_screen_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        match event {
            AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
            AppInputEvent::OpenSplashScreen => app.switch_to_screen(Screen::Splash),
            AppInputEvent::OpenDBSchemaScreen => {
                if app.selected_db.is_some() {
                    app.switch_to_screen(Screen::DatabaseSchema);
                } else {
                    app.switch_to_popup(PopUp::NoDBLoaded);
                }
            }
            AppInputEvent::OpenCreateNewFileScreen => {
                app.create_new_db_form();
                app.switch_to_screen(Screen::CreateNewFile);
            }
            AppInputEvent::OpenOptionsScreen => app.switch_to_screen(Screen::Options),
            AppInputEvent::MoveUpPrimary => app.file_explorer_table.previous(),
            AppInputEvent::MoveDownPrimary => app.file_explorer_table.next(),
            AppInputEvent::FileExplorerSelect => {
                if app.file_explorer_table.index == 0 {
                    if let Some(parent) = app.file_explorer_table.current_path.parent() {
                        app.file_explorer_table.current_path = parent.to_path_buf();
                        app.file_explorer_table.update_file_list();
                        app.file_explorer_table.update_scrollbar_state();
                    }
                } else {
                    let data_row = &app.file_explorer_table.items[app.file_explorer_table.index];
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

    Ok(())
}

fn database_schema_screen_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.selected_db.is_none() {
        app.current_error = Some(DBError::ConnectionCreationError(
            "Cannot open selected database schema!".to_string(),
        ));
        app.switch_to_popup(PopUp::Error);
    }

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        match event {
            AppInputEvent::OpenSplashScreen => app.switch_to_screen(Screen::Splash),
            AppInputEvent::OpenFileExplorerScreen => app.switch_to_screen(Screen::FileExplorer),
            AppInputEvent::OpenCreateNewFileScreen => {
                app.create_new_db_form();
                app.switch_to_screen(Screen::CreateNewFile);
            }
            AppInputEvent::OpenOptionsScreen => app.switch_to_screen(Screen::Options),
            AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
            AppInputEvent::MoveUpPrimary => {
                let db = &app.selected_db.as_mut().unwrap();
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
            AppInputEvent::MoveDownPrimary => {
                let db = &app.selected_db.as_mut().unwrap();
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
            AppInputEvent::MoveUpSecondary => {
                if app.column_list_view.is_some() {
                    let _ = &app.column_list_view.as_mut().unwrap().previous();
                }
            }
            AppInputEvent::MoveDownSecondary => {
                if app.column_list_view.is_some() {
                    let _ = &app.column_list_view.as_mut().unwrap().next();
                }
            }
            AppInputEvent::OpenInsertTablePopUp => {
                app.create_table_insert_form();
                app.switch_to_popup(PopUp::InsertTable);
            }
            AppInputEvent::OpenDeleteTablePopUp => {
                app.create_table_delete_form();
                app.switch_to_popup(PopUp::DeleteTable);
            }
            AppInputEvent::OpenDBTableScreen => {
                if let Some(selected_table) = &app.selected_db_table {
                    app.select_table_rows(selected_table.to_string());
                    app.switch_to_screen(Screen::DataBaseTable);
                }
            }
            _ => {}
        }
    }

    Ok(())
}

fn database_table_screen_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.selected_db_table.is_none() {
        app.current_error = Some(DBError::TableDoesNotExist(
            "Table does not exist!".to_string(),
        ));
        app.switch_to_popup(PopUp::Error);
    }

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        match event {
            AppInputEvent::OpenSplashScreen => app.switch_to_screen(Screen::Splash),
            AppInputEvent::OpenFileExplorerScreen => app.switch_to_screen(Screen::FileExplorer),
            AppInputEvent::OpenDBSchemaScreen => app.switch_to_screen(Screen::DatabaseSchema),
            AppInputEvent::OpenCreateNewFileScreen => {
                app.create_new_db_form();
                app.switch_to_screen(Screen::CreateNewFile);
            }
            AppInputEvent::OpenOptionsScreen => app.switch_to_screen(Screen::Options),
            AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
            AppInputEvent::MoveUpPrimary => app.row_list_view.as_mut().unwrap().previous(),
            AppInputEvent::MoveDownPrimary => app.row_list_view.as_mut().unwrap().next(),
            AppInputEvent::OpenInsertRowPopUp => {
                let table_cols: Vec<String> = app
                    .selected_table_columns
                    .iter()
                    .map(|col_info| col_info.name.clone())
                    .collect();
                app.create_row_insert_form(table_cols);
                app.switch_to_popup(PopUp::InsertRow);
            }
            AppInputEvent::OpenDeleteRowPopUp => {
                app.create_row_delete_form();
                app.switch_to_popup(PopUp::DeleteRow);
            }
            _ => {}
        }
    }

    Ok(())
}

fn options_screen_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }
    let mut changed: bool = false;

    match app.current_mode {
        Mode::Browse => {
            if let Some(event) = app.key_bindings.resolve_event(
                app.current_screen,
                app.current_popup,
                app.current_mode,
                key_event,
            ) {
                match event {
                    AppInputEvent::OpenSplashScreen => app.switch_to_screen(Screen::Splash),
                    AppInputEvent::OpenFileExplorerScreen => {
                        app.switch_to_screen(Screen::FileExplorer)
                    }
                    AppInputEvent::OpenDBSchemaScreen => {
                        app.switch_to_screen(Screen::DatabaseSchema)
                    }
                    AppInputEvent::OpenDBTableScreen => app.switch_to_screen(Screen::DataBaseTable),
                    AppInputEvent::OpenCreateNewFileScreen => {
                        app.switch_to_screen(Screen::CreateNewFile)
                    }
                    AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
                    AppInputEvent::MoveUpPrimary => {
                        app.options.previous_option();
                        changed = true;
                    }
                    AppInputEvent::MoveDownPrimary => {
                        app.options.next_option();
                        changed = true;
                    }
                    AppInputEvent::MoveUpSecondary => {
                        app.options.previous_color_scheme();
                        changed = true;
                    }
                    AppInputEvent::MoveDownSecondary => {
                        app.options.next_color_scheme();
                        changed = true;
                    }
                    AppInputEvent::ToggleOption => {
                        let index = app.options.index;
                        app.options.fields[index].toggle();
                        changed = true;
                    }
                    AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
                    _ => {}
                }
            }
        }
        Mode::Edit => {
            let active_field = &mut app.options.fields[app.options.index];
            match &mut active_field.kind {
                OptionKind::TextInput(_) => {
                    match (key_event.code, key_event.modifiers) {
                        (KeyCode::Char(c), KeyModifiers::NONE) => {
                            active_field.enter_char(c);
                            changed = true;
                        }
                        (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                            for upper in c.to_uppercase() {
                                active_field.enter_char(upper);
                            }
                            changed = true;
                        }
                        (KeyCode::Backspace, KeyModifiers::NONE) => {
                            active_field.pop_char();
                            changed = true;
                        }
                        _ => {}
                    }
                    if let Some(value) = active_field.parse_value::<u16>() {
                        if app.options.selected_option == SelectedOption::InfoSectionHeight {
                            app.options.info_section_height = value;
                        }
                    }
                }
                OptionKind::Toggle(_) => {}
            }
            if key_event.code == KeyCode::BackTab && key_event.modifiers == KeyModifiers::SHIFT {
                app.switch_mode(Mode::Browse);
            }
        }
    }

    if changed {
        app.options.sync_from_fields();
        app.options
            .save(&app.qualifier, &app.organization, &app.application)?;
    }

    Ok(())
}

fn create_new_file_screen_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.current_mode {
        Mode::Browse => {
            if let Some(event) = app.key_bindings.resolve_event(
                app.current_screen,
                app.current_popup,
                app.current_mode,
                key_event,
            ) {
                match event {
                    AppInputEvent::OpenSplashScreen => app.switch_to_screen(Screen::Splash),
                    AppInputEvent::OpenFileExplorerScreen => {
                        app.switch_to_screen(Screen::FileExplorer)
                    }
                    AppInputEvent::OpenDBSchemaScreen => {
                        app.switch_to_screen(Screen::DatabaseSchema)
                    }
                    AppInputEvent::OpenDBTableScreen => app.switch_to_screen(Screen::DataBaseTable),
                    AppInputEvent::OpenOptionsScreen => app.switch_to_screen(Screen::Options),
                    AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
                    AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
                    AppInputEvent::ExecuteAction => {
                        if app.selected_db.is_none() {
                            if let Some(form) = &app.create_db_form {
                                let db_name = form.fields[0].text_value.clone();
                                let db_extension = ".db".to_string();

                                match DB::new(db_name, db_extension) {
                                    Ok(db) => {
                                        app.selected_db = Some(db);
                                        app.fetch_table_list();
                                        app.populate_table_col_map();
                                        app.switch_to_screen(Screen::DatabaseSchema);
                                    }
                                    Err(e) => {
                                        let err = DBError::ConnectionCreationError(e.to_string());
                                        app.show_error(err);
                                    }
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Mode::Edit => {
            let form = app.create_db_form.as_mut().unwrap();
            match (key_event.code, key_event.modifiers) {
                (KeyCode::Char(c), KeyModifiers::NONE) => form.enter_char(c),
                (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    for upper in c.to_uppercase() {
                        form.enter_char(upper);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => form.pop_char(),
                (KeyCode::BackTab, KeyModifiers::SHIFT) => app.switch_mode(Mode::Browse),
                _ => {}
            }
        }
    }

    Ok(())
}

fn quit_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    log("popup quit");

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        match event {
            AppInputEvent::QuitAppConfirm => {
                app.should_quit = true;
                log(format!("app.should_quit: {:?}", app.should_quit).as_str());
            }
            AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
            _ => {}
        }
    }

    Ok(())
}

fn no_db_loaded_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        match event {
            AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
            AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
            AppInputEvent::OpenFileExplorerScreen => app.switch_to_screen(Screen::FileExplorer),
            _ => {}
        }
    }

    Ok(())
}

fn insert_row_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.current_mode {
        Mode::Browse => {
            if let Some(event) = app.key_bindings.resolve_event(
                app.current_screen,
                app.current_popup,
                app.current_mode,
                key_event,
            ) {
                match event {
                    AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
                    AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
                    AppInputEvent::MoveUpPrimary => {
                        app.row_insert_form.as_mut().unwrap().previous()
                    }
                    AppInputEvent::MoveDownPrimary => app.row_insert_form.as_mut().unwrap().next(),
                    AppInputEvent::ExecuteAction => {
                        let db = app.selected_db.as_mut().unwrap();
                        let table_name = app.selected_db_table.as_ref().unwrap();
                        let form = &app.row_insert_form.as_ref().unwrap();
                        let mut columns: Vec<String> = Vec::new();
                        let mut values: Vec<&dyn ToSql> = Vec::new();
                        let mut values_str = Vec::new();

                        // Pair form items with their column info
                        for (item, col_info) in form.fields.iter().zip(&app.selected_table_columns)
                        {
                            columns.push(col_info.name.clone());
                            values.push(&item.text_value as &dyn ToSql);
                            values_str.push(&item.text_value);
                        }

                        match db.insert_rows_statement(table_name.clone(), columns, values) {
                            Ok(_) => {
                                let table_list = app.table_list_view.as_mut().unwrap();
                                if let Some(table_info) =
                                    table_list.items.iter_mut().find(|t| t.name == *table_name)
                                {
                                    table_info.increment_row_count();
                                }

                                app.select_table_rows(table_name.to_string());
                                app.switch_to_popup(PopUp::None);
                            }
                            Err(e) => {
                                app.show_error(e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Mode::Edit => {
            let form = app.row_insert_form.as_mut().unwrap();
            match (key_event.code, key_event.modifiers) {
                (KeyCode::Char(c), KeyModifiers::NONE) => form.enter_char(c),
                (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    for upper in c.to_uppercase() {
                        form.enter_char(upper);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => form.pop_char(),
                (KeyCode::BackTab, KeyModifiers::SHIFT) => app.switch_mode(Mode::Browse),
                _ => {}
            }
        }
    }

    Ok(())
}

#[allow(unused_variables)]
fn delete_row_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.current_mode {
        Mode::Browse => {
            if let Some(event) = app.key_bindings.resolve_event(
                app.current_screen,
                app.current_popup,
                app.current_mode,
                key_event,
            ) {
                match event {
                    AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
                    AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
                    AppInputEvent::MoveUpPrimary => {
                        app.row_delete_form.as_mut().unwrap().previous()
                    }
                    AppInputEvent::MoveDownPrimary => app.row_delete_form.as_mut().unwrap().next(),
                    AppInputEvent::ExecuteAction => {
                        let db = app.selected_db.as_mut().unwrap();
                        let table_name = app.selected_db_table.as_ref().unwrap();
                        let form = app.row_delete_form.as_mut().unwrap();
                        let col = form.fields[0].text_value.clone();
                        let row = form.fields[1].text_value.clone();

                        match db.delete_row_statement(table_name, &col, &row) {
                            Ok(affected) => {
                                if affected > 0 {
                                    let table_list = app.table_list_view.as_mut().unwrap();
                                    if let Some(table_info) =
                                        table_list.items.iter_mut().find(|t| t.name == *table_name)
                                    {
                                        table_info.decrement_row_count();
                                    }

                                    log(format!("Deleted {} rows", affected).as_str());
                                } else {
                                    log("No matching rows found");
                                }
                                app.select_table_rows(table_name.to_string());
                                app.switch_to_popup(PopUp::None);
                            }
                            Err(e) => {
                                app.show_error(e);
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Mode::Edit => {
            let form = app.row_delete_form.as_mut().unwrap();
            match (key_event.code, key_event.modifiers) {
                (KeyCode::Char(c), KeyModifiers::NONE) => form.enter_char(c),
                (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    for upper in c.to_uppercase() {
                        form.enter_char(upper);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => form.pop_char(),
                (KeyCode::BackTab, KeyModifiers::SHIFT) => app.switch_mode(Mode::Browse),
                _ => {}
            }
        }
    }

    Ok(())
}

fn insert_table_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.current_mode {
        Mode::Browse => {
            if let Some(event) = app.key_bindings.resolve_event(
                app.current_screen,
                app.current_popup,
                app.current_mode,
                key_event,
            ) {
                match event {
                    AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
                    AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
                    AppInputEvent::ExecuteAction => {
                        if let Some(db) = &mut app.selected_db {
                            match db.execute_raw_sql(
                                app.table_insert_form.as_ref().unwrap().fields[0]
                                    .text_value
                                    .clone(),
                            ) {
                                Ok(_) => {
                                    app.fetch_table_list();
                                    app.switch_to_popup(PopUp::None);
                                }
                                Err(e) => {
                                    app.show_error(e);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Mode::Edit => {
            let form = app.table_insert_form.as_mut().unwrap();
            match (key_event.code, key_event.modifiers) {
                (KeyCode::Char(c), KeyModifiers::NONE) => form.enter_char(c),
                (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    for upper in c.to_uppercase() {
                        form.enter_char(upper);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => form.pop_char(),
                (KeyCode::BackTab, KeyModifiers::SHIFT) => app.switch_mode(Mode::Browse),
                _ => {}
            }
        }
    }

    Ok(())
}

fn delete_table_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    match app.current_mode {
        Mode::Browse => {
            if let Some(event) = app.key_bindings.resolve_event(
                app.current_screen,
                app.current_popup,
                app.current_mode,
                key_event,
            ) {
                match event {
                    AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
                    AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
                    AppInputEvent::ExecuteAction => {
                        if let Some(db) = &mut app.selected_db {
                            match db.drop_table(
                                app.table_delete_form.as_ref().unwrap().fields[0]
                                    .text_value
                                    .clone(),
                            ) {
                                Ok(_) => {
                                    app.fetch_table_list();
                                    app.switch_to_popup(PopUp::None);
                                }
                                Err(e) => {
                                    app.show_error(e);
                                }
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
        Mode::Edit => {
            let form = app.table_delete_form.as_mut().unwrap();
            match (key_event.code, key_event.modifiers) {
                (KeyCode::Char(c), KeyModifiers::NONE) => form.enter_char(c),
                (KeyCode::Char(c), KeyModifiers::SHIFT) => {
                    for upper in c.to_uppercase() {
                        form.enter_char(upper);
                    }
                }
                (KeyCode::Backspace, KeyModifiers::NONE) => form.pop_char(),
                (KeyCode::BackTab, KeyModifiers::SHIFT) => app.switch_mode(Mode::Browse),
                _ => {}
            }
        }
    }

    Ok(())
}

fn error_popup_handler(app: &mut App, key_event: KeyEvent) -> io::Result<()> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        key_event,
    ) {
        if event == AppInputEvent::ClosePopUp {
            app.current_error = None;
            app.switch_to_popup(PopUp::None);
        }
    }

    Ok(())
}
