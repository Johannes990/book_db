use crate::{
    app::{App, Mode, PopUp, Screen}, db::DB, errors::{app_error::AppError, backend::DBError, navigation::NavigationError},
    log::log, options::{OptionKind, SelectedOption}, ui::input::key_bindings::AppInputEvent,
    widgets::new_table::form::{ColumnField, TableField}, traits::writeable::Writable,
};
use crossterm::event::{KeyEvent, KeyModifiers};
use ratatui::crossterm::event::{self, Event, KeyCode, KeyEventKind};
use rusqlite::ToSql;
use std::time::Duration;

pub fn handle_key_events(app: &mut App) -> Result<bool, AppError> {
    if event::poll(Duration::from_millis(10))? {
        if let Event::Key(key_event) = event::read()? {
            // popups are always the active componenet, when they exist
            // therefore they take event priority
            match app.current_popup {
                PopUp::Quit => quit_popup_handler(app, key_event)?,
                PopUp::NoDBLoaded => no_db_loaded_popup_handler(app, key_event)?,
                PopUp::InsertRow => insert_row_popup_handler(app, key_event)?,
                PopUp::DeleteRow => delete_row_popup_handler(app, key_event)?,
                PopUp::InsertRawSql => insert_raw_sql_popup_handler(app, key_event)?,
                PopUp::InsertTable => insert_table_popup_handler(app, key_event)?,
                PopUp::DeleteTable => delete_table_popup_handler(app, key_event)?,
                PopUp::Error => error_popup_handler(app, key_event)?,
                PopUp::None => {}
            }

            match app.current_screen {
                Screen::Splash => splash_screen_handler(app, key_event)?,
                Screen::FileExplorer => file_explorer_screen_handler(app, key_event)?,
                Screen::DatabaseSchema => database_schema_screen_handler(app, key_event)?,
                Screen::DataBaseTable => database_table_screen_handler(app, key_event)?,
                Screen::Options => options_screen_handler(app, key_event)?,
                Screen::CreateNewFile => create_new_file_screen_handler(app, key_event)?,
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

fn handle_global_navigation(app: &mut App, event: &AppInputEvent) -> bool {
    match event {
        AppInputEvent::OpenSplashScreen => {
            app.switch_to_screen(Screen::Splash);
            true
        }
        AppInputEvent::OpenFileExplorerScreen => {
            app.switch_to_screen(Screen::FileExplorer);
            true
        }
        AppInputEvent::OpenDBSchemaScreen => {
            if app.selected_db.is_some() {
                app.switch_to_screen(Screen::DatabaseSchema);
            } else {
                app.switch_to_popup(PopUp::NoDBLoaded);
            }
            true
        }
        AppInputEvent::OpenDBTableScreen => {
            let Some(selected_table) = &app.selected_db_table else {
                app.current_error = Some(DBError::TableDoesNotExist("".to_string()).into());
                app.switch_to_popup(PopUp::Error);
                return true;
            };

            app.select_table_rows(selected_table.to_string());
            app.switch_to_screen(Screen::DataBaseTable);
            true
        }
        AppInputEvent::OpenCreateNewFileScreen => {
            app.create_new_db_form();
            app.switch_to_screen(Screen::CreateNewFile);
            true
        }
        AppInputEvent::OpenOptionsScreen => {
            app.switch_to_screen(Screen::Options);
            true
        }
        AppInputEvent::OpenQuitAppPopUp => {
            app.switch_to_popup(PopUp::Quit);
            true
        }
        _ => false,
    }
}

fn handle_edit_mode_input<T: Writable>(target: &mut T, key_event: &KeyEvent) -> bool {
    if key_event.kind != KeyEventKind::Press {
        return false;
    }

    // return true when Switch mode event registered
    match (key_event.code, key_event.modifiers) {
        (KeyCode::Char(c), KeyModifiers::NONE) => target.enter_char(c),
        (KeyCode::Char(c), KeyModifiers::SHIFT) => {
            for upper in c.to_uppercase() {
                target.enter_char(upper);
            }
        }
        (KeyCode::Backspace, KeyModifiers::NONE) => target.pop_char(),
        (KeyCode::BackTab, KeyModifiers::SHIFT) => return true,
        _ => {}
    }

    false
}

fn splash_screen_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if handle_global_navigation(app, &event) {
        return Ok(());
    }

    Ok(())
}

fn file_explorer_screen_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if handle_global_navigation(app, &event) {
        return Ok(());
    }

    match event {
        AppInputEvent::MoveUpPrimary => app.file_explorer_table.previous(),
        AppInputEvent::MoveDownPrimary => app.file_explorer_table.next(),
        AppInputEvent::FileExplorerSelect => {
            if app.file_explorer_table.index == 0 {
                app.file_explorer_table.parent_path();
                return Ok(());
            }

            let data_row = &app.file_explorer_table.items[app.file_explorer_table.index];
            let selected_file = data_row.path_name();
            let is_dir = data_row.is_dir();
            let new_path = app.file_explorer_table.current_path.join(selected_file);

            if *is_dir && new_path.is_dir() {
                app.file_explorer_table.open_dir(new_path);
            } else {
                app.open_file(new_path);
            }
        }
        _ => {}
    }

    Ok(())
}

fn database_schema_screen_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if handle_global_navigation(app, &event) {
        return Ok(());
    }

    match event {
        AppInputEvent::MoveUpPrimary => {
            let Some(db) = &app.selected_db.as_mut() else {
                app.current_error = Some(DBError::NoDBInMemory.into());
                app.switch_to_popup(PopUp::Error);
                return Ok(());
            };

            let tables = db.get_table_list().unwrap_or_default();

            if tables.is_empty() {
                return Ok(());
            }

            let Some(selected_table) = &app.selected_db_table else {
                return Ok(());
            };

            let current_idx = tables.iter().position(|t| t == selected_table).unwrap_or(0);
            let next_idx = if current_idx > 0 {
                current_idx - 1
            } else {
                tables.len() - 1
            };

            app.select_table(tables[next_idx].clone());

            if let Some(view) = app.table_list_view.as_mut() {
                view.previous();
            }
        }
        AppInputEvent::MoveDownPrimary => {
            let Some(db) = &app.selected_db.as_mut() else {
                return Ok(());
            };

            let tables = db.get_table_list().unwrap_or_default();

            if tables.is_empty() {
                return Ok(());
            }

            let Some(selected_table) = &app.selected_db_table else {
                return Ok(());
            };

            let current_idx = tables.iter().position(|t| t == selected_table).unwrap_or(0);
            let next_idx = if current_idx < tables.len() - 1 {
                current_idx + 1
            } else {
                0
            };

            app.select_table(tables[next_idx].clone());

            if let Some(view) = app.table_list_view.as_mut() {
                view.next();
            }
        }
        AppInputEvent::MoveUpSecondary => {
            if let Some(col_list_view) = &mut app.column_list_view {
                col_list_view.previous();
            }
        }
        AppInputEvent::MoveDownSecondary => {
            if let Some(col_list_view) = &mut app.column_list_view {
                col_list_view.next();
            }
        }
        AppInputEvent::OpenInsertRawSqlPopUp => {
            app.create_raw_sql_insert_form();
            app.switch_to_popup(PopUp::InsertRawSql);
        }
        AppInputEvent::OpenInsertTablePopUp => {
            app.create_table_insert_form();
            app.switch_to_popup(PopUp::InsertTable);
        }
        AppInputEvent::OpenDeleteTablePopUp => {
            app.create_table_delete_form();
            app.switch_to_popup(PopUp::DeleteTable);
        }
        _ => {}
    }

    Ok(())
}

fn database_table_screen_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if handle_global_navigation(app, &event) {
        return Ok(());
    }

    match event {
        AppInputEvent::MoveUpPrimary => {
            if let Some(view) = app.row_list_view.as_mut() {
                view.previous();
            }
        }
        AppInputEvent::MoveDownPrimary => {
            if let Some(view) = app.row_list_view.as_mut() {
                view.next();
            }
        }
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

    Ok(())
}

fn options_screen_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let mut changed: bool = false;  

    if app.current_mode == Mode::Edit {
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

                if changed {
                    app.options.sync_from_fields();
                    app.options
                        .save(&app.qualifier, &app.organization, &app.application)?;
                }
            }
            OptionKind::Toggle(_) => {}
        }

        if key_event.code == KeyCode::BackTab && key_event.modifiers == KeyModifiers::SHIFT {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if handle_global_navigation(app, &event) {
        return Ok(());
    }

    match event {
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

    if changed {
        app.options.sync_from_fields();
        app.options
            .save(&app.qualifier, &app.organization, &app.application)?;
    }

    Ok(())
}

fn create_new_file_screen_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_popup != PopUp::None {
        return Ok(());
    }

    if app.current_mode == Mode::Edit {
        let Some(form) = app.create_db_form.as_mut() else {
            return Ok(());
        };

        let exit = handle_edit_mode_input(form, &key_event);

        if exit {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if handle_global_navigation(app, &event) {
        return Ok(());
    }

    match event {
        AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
        AppInputEvent::ExecuteAction => {
            let Some(form) = &app.create_db_form else {
                return Ok(());
            };

            let db_name = form.fields[0].text_box.text_value.clone();
            let db_extension = ".db".to_string();

            match DB::new(db_name, db_extension) {
                Ok(db) => {
                    app.selected_db = Some(db);
                    app.fetch_table_list();
                    app.populate_table_col_map();
                    app.switch_to_screen(Screen::DatabaseSchema);
                }
                Err(e) => {
                    app.current_error =
                        Some(DBError::ConnectionCreationError(e.to_string()).into());
                    app.switch_to_popup(PopUp::Error);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn quit_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::QuitAppConfirm => {
            app.should_quit = true;
            log(format!("app.should_quit: {:?}", app.should_quit).as_str());
        }
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        _ => {}
    }

    Ok(())
}

fn no_db_loaded_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::OpenQuitAppPopUp => app.switch_to_popup(PopUp::Quit),
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        AppInputEvent::OpenFileExplorerScreen => {
            app.switch_to_screen(Screen::FileExplorer);
            app.switch_to_popup(PopUp::None);
        }
        _ => {}
    }

    Ok(())
}

fn insert_row_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if app.current_mode == Mode::Edit {
        let Some(form) = app.row_insert_form.as_mut() else {
            return Ok(());
        };

        let exit = handle_edit_mode_input(form, &key_event);

        if exit {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
        AppInputEvent::MoveUpPrimary => {
            let Some(form) = app.row_insert_form.as_mut() else {
                app.current_error = Some(AppError::Navigation(
                    NavigationError::RowInsertFormNavigation(
                        "invalid mutable handle to row insert form".to_string(),
                    ),
                ));
                app.switch_to_popup(PopUp::Error);
                return Ok(());
            };
            form.previous();
        }
        AppInputEvent::MoveDownPrimary => {
            let Some(form) = app.row_insert_form.as_mut() else {
                return Ok(());
            };
            form.next();
        }
        AppInputEvent::ExecuteAction => {
            let Some(db) = app.selected_db.as_mut() else {
                return Ok(());
            };
            let Some(table_name) = app.selected_db_table.as_ref() else {
                return Ok(());
            };
            let Some(form) = &app.row_insert_form.as_ref() else {
                return Ok(());
            };
            let mut columns: Vec<String> = Vec::new();
            let mut values: Vec<&dyn ToSql> = Vec::new();
            let mut values_str = Vec::new();

            // Pair form items with their column info
            for (item, col_info) in form.fields.iter().zip(&app.selected_table_columns) {
                columns.push(col_info.name.clone());
                values.push(&item.text_box.text_value as &dyn ToSql);
                values_str.push(&item.text_box.text_value);
            }

            match db.insert_rows_statement(table_name.clone(), columns, values) {
                Ok(_) => {
                    let Some(view) = app.table_list_view.as_mut() else {
                        return Ok(());
                    };
                    if let Some(table_info) = view.items.iter_mut().find(|t| t.name == *table_name)
                    {
                        table_info.increment_row_count();
                    }

                    app.select_table_rows(table_name.to_string());
                    app.switch_to_popup(PopUp::None);
                }
                Err(err) => {
                    app.current_error = Some(err.into());
                    app.switch_to_popup(PopUp::Error);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn delete_row_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.current_mode == Mode::Edit {
        let Some(form) = app.row_delete_form.as_mut() else {
            app.current_error = Some(
                DBError::CannotDeleteRow(
                    "cant get a mutable handle on row delete form".to_string(),
                )
                .into(),
            );
            app.switch_to_popup(PopUp::Error);
            return Ok(());
        };

        let exit = handle_edit_mode_input(form, &key_event);

        if exit {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
        AppInputEvent::MoveUpPrimary => {
            let Some(form) = app.row_delete_form.as_mut() else {
                app.current_error = Some(AppError::Navigation(
                    NavigationError::RowDeleteFormNavigation(
                        "unable to get a mutable handle on row delete form".to_string(),
                    ),
                ));
                app.switch_to_popup(PopUp::Error);
                return Ok(());
            };
            form.previous();
        }
        AppInputEvent::MoveDownPrimary => {
            let Some(form) = app.row_delete_form.as_mut() else {
                app.current_error = Some(AppError::Navigation(
                    NavigationError::RowDeleteFormNavigation(
                        "unable to get a mutable handle on row delete form".to_string(),
                    ),
                ));
                app.switch_to_popup(PopUp::Error);
                return Ok(());
            };
            form.next();
        }
        AppInputEvent::ExecuteAction => {
            let Some(db) = app.selected_db.as_mut() else {
                app.current_error = Some(DBError::NoDBInMemory.into());
                app.switch_to_popup(PopUp::Error);
                return Ok(());
            };
            let Some(table_name) = app.selected_db_table.as_ref() else {
                app.current_error = Some(DBError::NoTableInMemory.into());
                app.switch_to_popup(PopUp::Error);
                return Ok(());
            };
            let Some(form) = app.row_delete_form.as_mut() else {
                return Ok(());
            };
            let col = form.fields[0].text_box.text_value.clone();
            let row = form.fields[1].text_box.text_value.clone();

            match db.delete_row_statement(table_name, &col, &row) {
                Ok(affected) => {
                    if affected > 0 {
                        let Some(table_list) = app.table_list_view.as_mut() else {
                            return Ok(());
                        };
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
                Err(err) => {
                    app.current_error = Some(err.into());
                    app.switch_to_popup(PopUp::Error);
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn insert_raw_sql_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.current_mode == Mode::Edit {
        let Some(form) = app.raw_sql_form.as_mut() else {
            return Ok(());
        };

        let exit = handle_edit_mode_input(form, &key_event);

        if exit {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
        AppInputEvent::ExecuteAction => {
            if let Some(db) = &mut app.selected_db {
                match db.execute_raw_sql(
                    app.raw_sql_form.as_ref().unwrap().fields[0]
                        .text_box
                        .text_value
                        .clone(),
                ) {
                    Ok(_) => {
                        app.fetch_table_list();
                        app.switch_to_popup(PopUp::None);
                    }
                    Err(err) => {
                        app.current_error = Some(err.into());
                        app.switch_to_popup(PopUp::Error);
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn insert_table_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(insert_form) = app.table_insert_form.as_mut() else {
        app.current_error = Some(AppError::InvalidHandle("CreateTableForm".to_string()));
        app.switch_to_popup(PopUp::Error);
        return Ok(());
    };

    if app.current_mode == Mode::Edit {
        let Some(text_box) = insert_form.selected_textbox_mut() else {
            return Ok(());
        };

        let exit = handle_edit_mode_input(text_box, &key_event);

        if exit {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        AppInputEvent::SwitchToEdit => {
            if insert_form.selected_textbox_mut().is_some() {
                app.switch_mode(Mode::Edit)
            }
        }
        AppInputEvent::InsertColumn => insert_form.draft.add_column(),
        AppInputEvent::RemoveColumn => {
            let column_count = insert_form.draft.columns.len();

            if column_count == 0 {
                return Ok(());
            }

            if insert_form.selected_field == TableField::TableName {
                insert_form.draft.remove_column(column_count - 1);
            } else if let TableField::Column(idx, _) = insert_form.selected_field {
                if idx == 0 {
                    insert_form.selected_field = TableField::TableName;
                    insert_form.draft.remove_column(idx);
                } else {
                    insert_form.selected_field = TableField::Column(idx - 1, ColumnField::Name);
                    insert_form.draft.remove_column(idx);
                }
            }
        }
        AppInputEvent::MoveUpPrimary => insert_form.previous_form_row(),
        AppInputEvent::MoveDownPrimary => insert_form.next_form_row(),
        AppInputEvent::MoveUpSecondary => {
            insert_form.previous_form_row();
            insert_form.previous_form_row_field();
        }
        AppInputEvent::MoveDownSecondary => {
            insert_form.next_form_row();
            insert_form.next_form_row_field();
        }
        _ => {}
    }

    Ok(())
}

fn delete_table_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    if app.current_mode == Mode::Edit {
        let Some(form) = app.table_delete_form.as_mut() else {
            return Ok(());
        };

        let exit = handle_edit_mode_input(form, &key_event);

        if exit {
            app.switch_mode(Mode::Browse);
        }

        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    match event {
        AppInputEvent::ClosePopUp => app.switch_to_popup(PopUp::None),
        AppInputEvent::SwitchToEdit => app.switch_mode(Mode::Edit),
        AppInputEvent::ExecuteAction => {
            if let Some(db) = &mut app.selected_db {
                match db.drop_table(
                    app.table_delete_form.as_ref().unwrap().fields[0]
                        .text_box
                        .text_value
                        .clone(),
                ) {
                    Ok(_) => {
                        app.fetch_table_list();
                        app.switch_to_popup(PopUp::None);
                    }
                    Err(err) => {
                        app.current_error = Some(err.into());
                        app.switch_to_popup(PopUp::Error);
                    }
                }
            }
        }
        _ => {}
    }

    Ok(())
}

fn error_popup_handler(app: &mut App, key_event: KeyEvent) -> Result<(), AppError> {
    if key_event.kind != KeyEventKind::Press {
        return Ok(());
    }

    let Some(event) = app.key_bindings.resolve_event(
        app.current_screen,
        app.current_popup,
        app.current_mode,
        &key_event,
    ) else {
        return Ok(());
    };

    if event == AppInputEvent::ClosePopUp {
        app.current_error = None;
        app.switch_to_popup(PopUp::None);
    }

    Ok(())
}
