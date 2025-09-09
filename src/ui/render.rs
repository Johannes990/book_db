use crate::{
    app::{App, PopUp, Screen},
    column::column_info::ColumnInfo,
    options::{SelectedOption, SelectedScheme},
    row::row_info::RowInfo,
    ui::{colors::app_colors::ColorScheme, input::key_bindings::AppInputEvent},
    widgets::selectable_line::SelectableLine,
};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Alignment, Margin, Rect},
    style::{Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Clear, HighlightSpacing, Paragraph, Row, Scrollbar,
        ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap,
    },
    Frame, Terminal,
};
use std::{io, rc::Rc, vec};

pub fn render<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B: ratatui::backend::Backend,
{
    // Call terminal.draw and convert its output to io::Result<()>
    terminal.draw(|frame| {
        match app.current_screen {
            Screen::Splash => render_splash_screen(frame, app),
            Screen::FileExplorer => render_file_explorer_screen(frame, app),
            Screen::DatabaseSchema => render_database_schema_screen(frame, app),
            Screen::DataBaseTable => render_database_table_screen(frame, app),
            Screen::CreateNewFile => render_new_database_screen(frame, app),
            Screen::Options => render_options_screen(frame, app),
        }
        match app.current_popup {
            PopUp::None => {}
            PopUp::Quit => render_quit_popup(frame, app),
            PopUp::NoDBLoaded => render_no_db_loaded_popup(frame, app),
            PopUp::InsertRow => render_insert_row_popup(frame, app),
            PopUp::DeleteRow => render_delete_row_popup(frame, app),
            PopUp::InsertTable => render_insert_table_popup(frame, app),
            PopUp::DeleteTable => render_drop_table_popup(frame, app),
            PopUp::Error => render_error_popup(frame, app),
        }
    })?;
    Ok(())
}

fn render_splash_screen(frame: &mut Frame, app: &App) {
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let main_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let loaded_db_name_default = &app.language.screen_splash_db_placeholder;
    let loaded_db_name = app
        .selected_db
        .as_ref()
        .map(|db| format!("{}.{}", db.db_name, db.db_extension))
        .unwrap_or(loaded_db_name_default.to_string());
    let selected_table_name_default = &app.language.screen_splash_table_placeholder;
    let selected_table_name = app
        .selected_db_table
        .as_ref()
        .map_or(selected_table_name_default, |name| name);
    let app_name = &app.language.screen_splash_application_name;
    let loaded_db_string = &app.language.screen_splash_loaded_db;
    let loaded_table_string = &app.language.screen_splash_loaded_table;
    let main_page_content = vec![
        Line::from(format!(" {}", app_name)),
        Line::from(""),
        Line::from(format!(" {}: {}", loaded_db_string, loaded_db_name)),
        Line::from(format!(" {}: {}", loaded_table_string, selected_table_name)),
    ];
    let main_page_paragraph = Paragraph::new(main_page_content).style(main_page_style);

    frame.render_widget(main_page_paragraph, chunks[0]);

    let events = [
        AppInputEvent::OpenFileExplorerScreen,
        AppInputEvent::OpenDBSchemaScreen,
        AppInputEvent::OpenDBTableScreen,
        AppInputEvent::OpenCreateNewFileScreen,
        AppInputEvent::OpenOptionsScreen,
        AppInputEvent::OpenQuitAppPopUp,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_file_explorer_screen(frame: &mut Frame, app: &mut App) {
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let scrollbar_style = Style::default().fg(app.border_color());
    let fexp_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let file_explorer_title = &app.language.screen_file_explorer_title;
    let file_explorer_currently_in_string = &app.language.screen_file_explorer_current_location;
    let current_app_mode_string = &app.language.mode_current_mode;
    let file_explorer_block = Block::default()
        .title(format!(" {}", file_explorer_title))
        .title(
            Line::from(format!(
                "{}: {} ",
                file_explorer_currently_in_string,
                app.file_explorer_table.current_path.display()
            ))
            .centered(),
        )
        .title(
            Line::from(format!(
                "{}: {} ",
                current_app_mode_string, app.current_mode
            ))
            .right_aligned(),
        )
        .borders(Borders::NONE)
        .style(fexp_page_style);

    frame.render_widget(file_explorer_block, chunks[0]);

    let header_file_folder_string = &app.language.screen_file_explorer_file_folder_header;
    let header_size_string = &app.language.screen_file_explorer_size_header;
    let header_date_created_string = &app.language.screen_file_explorer_date_created_header;
    let header = [
        header_file_folder_string.as_str(),
        header_size_string.as_str(),
        header_date_created_string.as_str(),
    ]
    .into_iter()
    .map(Cell::from)
    .collect::<Row>()
    .style(fexp_page_style)
    .height(1);
    let rows: Vec<Row> = app
        .file_explorer_table
        .items
        .iter()
        .enumerate()
        .map(|(i, data)| {
            let color = match i % 2 {
                0 => app.background_color(),
                _ => app.background_alt_color(),
            };
            let item = data.ref_array();
            item.into_iter()
                .map(|content| Cell::from(Text::from(content.to_string())))
                .collect::<Row>()
                .style(Style::new().bg(color).fg(app.text_color()))
        })
        .collect();
    let col_constraints = [
        Constraint::Min(app.file_explorer_table.longest_item_lens.0 + 1),
        Constraint::Length(app.file_explorer_table.longest_item_lens.1 + 1),
        Constraint::Length(app.file_explorer_table.longest_item_lens.2 + 4),
    ];

    let highlight_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());
    let border_block_style = Style::default()
        .bg(app.background_color())
        .fg(app.border_color());
    let border_block = Block::new().borders(Borders::ALL).style(border_block_style);
    let table_chunk_area_without_top_row = Rect {
        x: chunks[0].x,
        y: chunks[0].y + 1,
        height: chunks[0].height - 1,
        width: chunks[0].width,
    };

    let (table_area, scrollbar_area) =
        get_table_and_scrollbar_areas(table_chunk_area_without_top_row);

    render_table(
        frame,
        &mut app.file_explorer_table.state,
        Some(header),
        rows,
        col_constraints.to_vec(),
        table_area,
        highlight_style,
        border_block,
    );

    render_vertical_scrollbar(
        frame,
        scrollbar_style,
        scrollbar_area,
        None,
        &mut app.file_explorer_table.scroll_state,
    );

    let events = [
        AppInputEvent::OpenQuitAppPopUp,
        AppInputEvent::OpenSplashScreen,
        AppInputEvent::OpenDBSchemaScreen,
        AppInputEvent::OpenCreateNewFileScreen,
        AppInputEvent::OpenOptionsScreen,
        AppInputEvent::MoveUpPrimary,
        AppInputEvent::MoveDownPrimary,
        AppInputEvent::FileExplorerSelect,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_database_schema_screen(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let no_db_found_string = &app.language.screen_db_schema_no_db_found;
    let db_name = app
        .selected_db
        .as_ref()
        .expect(no_db_found_string)
        .get_db_name();
    let current_db_string = &app.language.screen_db_schema_current_db;
    let current_app_mode_string = &app.language.mode_current_mode;
    let outer_block = Block::default()
        .title(Line::from(format!("{}: {} ", current_db_string, db_name)).left_aligned())
        .title(
            Line::from(format!(
                "{}: {} ",
                current_app_mode_string, app.current_mode
            ))
            .right_aligned(),
        )
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);
    let table_column_chunks =
        get_chunks_from_percentages(inner_area, Direction::Horizontal, vec![50, 50]);

    frame.render_widget(outer_block, chunks[0]);
    render_table_list(frame, app, table_column_chunks[0]);
    render_column_list(frame, app, table_column_chunks[1]);

    let events = [
        AppInputEvent::OpenSplashScreen,
        AppInputEvent::OpenFileExplorerScreen,
        AppInputEvent::OpenCreateNewFileScreen,
        AppInputEvent::OpenOptionsScreen,
        AppInputEvent::OpenQuitAppPopUp,
        AppInputEvent::MoveUpPrimary,
        AppInputEvent::MoveDownPrimary,
        AppInputEvent::MoveUpSecondary,
        AppInputEvent::MoveDownSecondary,
        AppInputEvent::OpenInsertTablePopUp,
        AppInputEvent::OpenDeleteTablePopUp,
        AppInputEvent::OpenDBTableScreen,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_new_database_screen(frame: &mut Frame, app: &mut App) {
    let page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let insert_text_area_on_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());

    if let Some(form) = &mut app.create_db_form {
        form.set_styles(insert_text_area_on_style, Style::default(), page_style);
    }

    if let Some(form) = &app.create_db_form {
        form.render_widget_and_cursor(frame, chunks[0]);
    }

    let events = [
        AppInputEvent::OpenSplashScreen,
        AppInputEvent::OpenFileExplorerScreen,
        AppInputEvent::OpenDBSchemaScreen,
        AppInputEvent::OpenDBTableScreen,
        AppInputEvent::OpenOptionsScreen,
        AppInputEvent::OpenQuitAppPopUp,
        AppInputEvent::SwitchToEdit,
        AppInputEvent::ExecuteAction,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_database_table_screen(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let col_name_style = Style::default()
        .fg(app.text_color())
        .add_modifier(Modifier::ITALIC | Modifier::UNDERLINED);
    let metadata_style = Style::default()
        .fg(app.text_alt_color())
        .add_modifier(Modifier::ITALIC);
    let scrollbar_style = Style::default().fg(app.border_color());
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let empty_table_string = &app.language.screen_db_table_table_placeholder;
    let table_name = app.selected_db_table.as_ref().expect(empty_table_string);
    let current_table_string = &app.language.screen_db_table_current_table;
    let current_app_mode_string = &app.language.mode_current_mode;
    let outer_block = Block::default()
        .title(Line::from(format!(" {}: {} ", current_table_string, table_name)).left_aligned())
        .title(
            Line::from(format!(
                "{}: {} ",
                current_app_mode_string, app.current_mode
            ))
            .right_aligned(),
        )
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);

    frame.render_widget(outer_block, chunks[0]);

    let header_cells: Vec<Cell> = app
        .selected_table_columns
        .iter()
        .map(|col| {
            let line = col.get_line_from_col_info(
                &app.options.display_col_metainfo_in_table_view,
                col_name_style,
                metadata_style,
            );
            Cell::from(line)
        })
        .collect();
    let header = Row::new(header_cells).style(db_page_style);
    let highlight_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());
    let rows: Vec<_> = app
        .row_list_view
        .as_ref()
        .unwrap()
        .items
        .iter()
        .map(|row| {
            let row_cells = row
                .values
                .iter()
                .map(|val| Cell::from(val.clone()))
                .collect::<Vec<_>>();
            Row::new(row_cells).style(db_page_style)
        })
        .collect();

    let table_title = table_name.to_string();
    let border_block_style = Style::default()
        .bg(app.background_color())
        .fg(app.border_color());
    let border_block = Block::new()
        .borders(Borders::ALL)
        .style(border_block_style)
        .title(table_title);
    let unwrapped_row_list = app.row_list_view.as_mut().unwrap();
    let min = 5;
    let max = 40;
    let col_constraints = compute_col_widths(
        &app.selected_table_columns,
        &unwrapped_row_list.items,
        min,
        max,
        &app.options.display_col_metainfo_in_table_view,
    );

    render_table(
        frame,
        &mut unwrapped_row_list.state,
        Some(header),
        rows,
        col_constraints,
        inner_area,
        highlight_style,
        border_block,
    );

    render_vertical_scrollbar(
        frame,
        scrollbar_style,
        inner_area,
        None,
        &mut unwrapped_row_list.scroll_bar_state,
    );

    let events = [
        AppInputEvent::OpenSplashScreen,
        AppInputEvent::OpenFileExplorerScreen,
        AppInputEvent::OpenDBSchemaScreen,
        AppInputEvent::OpenCreateNewFileScreen,
        AppInputEvent::OpenOptionsScreen,
        AppInputEvent::OpenQuitAppPopUp,
        AppInputEvent::MoveUpPrimary,
        AppInputEvent::MoveDownPrimary,
        AppInputEvent::OpenInsertRowPopUp,
        AppInputEvent::OpenDeleteRowPopUp,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_options_screen(frame: &mut Frame, app: &mut App) {
    let general_page_style = Style::default()
        .fg(app.text_color())
        .bg(app.background_color());
    let selected_style = Style::default().fg(app.error_color());
    let options_title = &app.language.screen_options_title;
    let options_block = Block::default()
        .title(format!("{}", options_title))
        .style(general_page_style);

    frame.render_widget(options_block, frame.area());

    let vertical_chunks =
        get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![50, 25, 25]);
    let horizontal_chunks =
        get_chunks_from_percentages(vertical_chunks[1], Direction::Horizontal, vec![50, 50]);
    let color_schemes_string = &app.language.screen_options_color_schemes;
    let header = Row::new(vec![Cell::from(format!("{}", color_schemes_string))]);
    let constraints = vec![Constraint::Min(5)];
    let border_block_style = Style::default()
        .bg(app.background_color())
        .fg(app.border_color());
    let border_block = Block::new().borders(Borders::ALL).style(border_block_style);
    let selected_scheme = app.options.selected_color_scheme;
    let text_color = app.text_color();
    let highlight_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());
    let border_color = app.border_color();
    let color_table = &mut app.options.available_color_schemes;
    let rows: Vec<_> = color_table
        .items
        .iter()
        .map(|scheme| {
            let scheme_name = format!("{:?}", scheme);
            let style = if *scheme == selected_scheme {
                Style::default().fg(text_color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(text_color)
            };
            Row::new(vec![Cell::from(scheme_name)]).style(style)
        })
        .collect();

    let color_scheme_table_inner_area = Rect {
        x: horizontal_chunks[0].x,
        y: horizontal_chunks[0].y,
        width: horizontal_chunks[0].width.saturating_sub(3),
        height: horizontal_chunks[0].height,
    };

    render_table(
        frame,
        &mut color_table.state,
        Some(header),
        rows,
        constraints,
        color_scheme_table_inner_area,
        highlight_style,
        border_block,
    );

    render_vertical_scrollbar(
        frame,
        Style::default().fg(border_color),
        horizontal_chunks[0],
        None,
        &mut color_table.scroll_bar_state,
    );

    render_color_scheme_preview(
        frame,
        horizontal_chunks[1],
        &app.options.selected_color_scheme,
        border_block_style,
    );

    let table_metainfo_string = &app.language.screen_options_metadata_in_table;
    let table_metainfo_toggle_button = SelectableLine::default(
        format!("{}: ", table_metainfo_string).as_str(),
        app.options.display_col_metainfo_in_table_view,
        matches!(
            app.options.selected_option,
            SelectedOption::TableMetainfoToggle
        ),
        general_page_style,
        selected_style,
    );
    let table_metainfo_toggle_area = Rect {
        x: vertical_chunks[0].x + 1,
        y: vertical_chunks[0].y + 1,
        width: 50,
        height: 1,
    };

    let insert_metainfo_string = &app.language.screen_options_metadata_in_insert;
    let insert_metainfo_toggle_button = SelectableLine::default(
        format!("{}: ", insert_metainfo_string).as_str(),
        app.options.display_col_metainfo_in_insert_view,
        matches!(
            app.options.selected_option,
            SelectedOption::InsertMetainfoToggle
        ),
        general_page_style,
        selected_style,
    );
    let insert_metainfo_toggle_area = Rect {
        x: vertical_chunks[0].x + 1,
        y: vertical_chunks[0].y + 2,
        width: 50,
        height: 1,
    };

    frame.render_widget(table_metainfo_toggle_button, table_metainfo_toggle_area);
    frame.render_widget(insert_metainfo_toggle_button, insert_metainfo_toggle_area);

    let events = [
        AppInputEvent::OpenSplashScreen,
        AppInputEvent::OpenFileExplorerScreen,
        AppInputEvent::OpenDBSchemaScreen,
        AppInputEvent::OpenDBTableScreen,
        AppInputEvent::OpenCreateNewFileScreen,
        AppInputEvent::OpenQuitAppPopUp,
        AppInputEvent::MoveUpPrimary,
        AppInputEvent::MoveDownPrimary,
        AppInputEvent::MoveUpSecondary,
        AppInputEvent::MoveDownSecondary,
        AppInputEvent::ToggleOption,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, vertical_chunks[2]);
}

fn render_quit_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(55, 30, frame.area());
    let quit_popup_style = Style::default()
        .bg(app.warning_color())
        .fg(app.text_color());

    let events = [AppInputEvent::QuitAppConfirm, AppInputEvent::ClosePopUp];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);
    let quit_confirmation_string = &app.language.popup_quit_confirmation;

    render_titled_paragraph(
        frame,
        app,
        &info_bits,
        quit_confirmation_string,
        quit_popup_style,
        area,
    );
}

fn render_no_db_loaded_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let popup_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());

    let events = [
        AppInputEvent::OpenFileExplorerScreen,
        AppInputEvent::ClosePopUp,
        AppInputEvent::OpenQuitAppPopUp,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);
    let no_db_loaded_string = &app.language.popup_no_db_loaded;

    render_titled_paragraph(
        frame,
        app,
        &info_bits,
        no_db_loaded_string,
        popup_style,
        area,
    );
}

fn render_insert_row_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 55, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![70, 30]);
    let insert_row_popup_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    let metadata_style = Style::default()
        .fg(app.text_alt_color())
        .add_modifier(Modifier::ITALIC);
    let insert_text_area_on_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());
    let insert_text_area_off_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());

    if let Some(form) = &mut app.row_insert_form {
        form.set_styles(
            insert_text_area_on_style,
            insert_text_area_off_style,
            insert_row_popup_style,
        );

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .style(insert_row_popup_style);

        frame.render_widget(Clear, chunks[0]);
        frame.render_widget(popup_block, chunks[0]);

        let text_area = chunks[0].inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        for (i, col_info) in app.selected_table_columns.iter().enumerate() {
            let x = text_area.x;
            let y = text_area.y + i as u16;

            let mut label_line = col_info.get_line_from_col_info(
                &app.options.display_col_metainfo_in_insert_view,
                insert_row_popup_style,
                metadata_style,
            );

            let label_width = line_width(&label_line) as u16;

            let field = &form.fields[i];
            if field.selected {
                label_line.spans.push(Span::styled(
                    field.text_value.clone(),
                    insert_text_area_on_style,
                ))
            } else {
                label_line.spans.push(Span::styled(
                    field.text_value.clone(),
                    insert_text_area_off_style,
                ));
            }

            frame
                .buffer_mut()
                .set_line(x, y, &label_line, text_area.width);

            if field.selected {
                if let Some(cursor_pos) = field.cursor_position(Rect {
                    x: x + label_width,
                    y,
                    width: text_area.width,
                    height: 1,
                }) {
                    frame.set_cursor_position(cursor_pos);
                }
            }
        }
    }

    let events = [
        AppInputEvent::ClosePopUp,
        AppInputEvent::SwitchToEdit,
        AppInputEvent::MoveUpPrimary,
        AppInputEvent::MoveDownPrimary,
        AppInputEvent::ExecuteAction,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_insert_table_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 40, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![70, 30]);
    let insert_table_popup_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    let insert_text_area_on_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());
    let insert_text_area_off_style = Style::default()
        .bg(app.text_color())
        .fg(app.background_alt_color());

    if let Some(form) = &mut app.table_insert_form {
        form.set_styles(
            insert_text_area_on_style,
            insert_text_area_off_style,
            insert_table_popup_style,
        );
    }

    if let Some(form) = &app.table_insert_form {
        form.render_widget_and_cursor(frame, chunks[0]);
    }

    let events = [
        AppInputEvent::ClosePopUp,
        AppInputEvent::SwitchToEdit,
        AppInputEvent::ExecuteAction,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_drop_table_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![40, 60]);
    let drop_table_popup_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    let text_area_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());

    if let Some(form) = &mut app.table_delete_form {
        form.set_styles(text_area_style, Style::default(), drop_table_popup_style);
    }

    if let Some(form) = &app.table_delete_form {
        form.render_widget_and_cursor(frame, chunks[0]);
    }

    let events = [
        AppInputEvent::ClosePopUp,
        AppInputEvent::SwitchToEdit,
        AppInputEvent::ExecuteAction,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_delete_row_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![70, 30]);

    let delete_row_popup_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    let delete_text_area_on_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_color());
    let delete_text_area_off_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    if let Some(form) = &mut app.row_delete_form {
        form.set_styles(
            delete_text_area_on_style,
            delete_text_area_off_style,
            delete_row_popup_style,
        );
    }

    if let Some(form) = &app.row_delete_form {
        form.render_widget_and_cursor(frame, chunks[0]);
    }

    let events = [
        AppInputEvent::ClosePopUp,
        AppInputEvent::SwitchToEdit,
        AppInputEvent::MoveUpPrimary,
        AppInputEvent::MoveDownPrimary,
        AppInputEvent::ExecuteAction,
    ];

    let info_bits = app
        .key_bindings
        .get_info_bits_from_events(&events, &app.language);

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_error_popup(frame: &mut Frame, app: &mut App) {
    if let Some(error) = &app.current_error {
        let area = centered_rect(40, 30, frame.area());
        let style = Style::default().bg(app.error_color()).fg(app.text_color());
        let error_title = &app.language.popup_error_title;
        let error_block = Block::default()
            .borders(Borders::ALL)
            .title(format!("{}", error_title))
            .border_style(Style::default().fg(app.border_color()))
            .style(style);
        let error_message = format!("{}", error);
        let mut error_text = Text::from(error_message);

        error_text.push_line(Span::raw(""));

        let events = [AppInputEvent::ClosePopUp];

        let info_bits = app
            .key_bindings
            .get_info_bits_from_events(&events, &app.language);

        let info_text = format_info_text(&info_bits, app);

        for bit in info_text {
            error_text.lines.push(bit);
        }

        let paragraph = Paragraph::new(error_text)
            .block(error_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
    }
}

fn render_table_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let table_title = &app.language.table_list_title;

    if app.table_list_view.is_some() {
        let scrollbar_style = Style::default().fg(app.border_color());
        let row_style = Style::default()
            .bg(app.background_color())
            .fg(app.text_color());
        let header_name_string = &app.language.table_list_name_header;
        let header_rows_string = &app.language.table_list_rows_header;
        let header_type_string = &app.language.table_list_type_header;
        let header = Row::new(vec![
            Cell::new(format!("{}", header_name_string)),
            Cell::new(format!("{}", header_rows_string)),
            Cell::new(format!("{}", header_type_string)),
        ])
        .style(row_style);
        let view_element_string = &app.language.table_list_view_element;
        let table_element_string = &app.language.table_list_table_element;
        let rows: Vec<Row> = app
            .table_list_view
            .as_ref()
            .unwrap()
            .items
            .iter()
            .map(|table| {
                Row::new(vec![
                    Cell::from(Text::from(table.name.clone())),
                    Cell::from(Text::from(table.row_count.to_string())),
                    Cell::from(Text::from(if table.is_view {
                        format!("{}", view_element_string)
                    } else {
                        format!("{}", table_element_string)
                    })),
                ])
                .style(row_style)
            })
            .collect();

        let col_constraints = [
            Constraint::Min(15),   // table name
            Constraint::Min(7),    // row count
            Constraint::Length(7), // type (table, view)
        ];

        let border_block_style = Style::default()
            .bg(app.background_color())
            .fg(app.border_color());
        let border_block = Block::new()
            .borders(Borders::ALL)
            .style(border_block_style)
            .title(format!("{}", table_title));
        let highlight_style = Style::default()
            .bg(app.background_highlight_color())
            .fg(app.text_highlight_color());
        let unwrapped_table_list = app.table_list_view.as_mut().unwrap();

        render_table(
            frame,
            &mut unwrapped_table_list.state,
            Some(header),
            rows,
            col_constraints.to_vec(),
            area,
            highlight_style,
            border_block,
        );

        render_vertical_scrollbar(
            frame,
            scrollbar_style,
            area,
            None,
            &mut unwrapped_table_list.scroll_state,
        );
    } else {
        let style = Style::default()
            .bg(app.background_color())
            .fg(app.text_color());
        let empty_block = Block::default()
            .title(format!("{}", table_title))
            .borders(Borders::ALL);
        let table_list_empty_string = &app.language.table_list_emtpy_placeholder;
        let paragraph = Paragraph::new(table_list_empty_string.to_string())
            .block(empty_block)
            .style(style);

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
    }
}

fn render_column_list(frame: &mut Frame, app: &mut App, area: Rect) {
    let column_list_title = &app.language.column_list_title;

    if app.column_list_view.is_some() {
        let scrollbar_style = Style::default().fg(app.border_color());
        let header_name_string = &app.language.column_list_name_header;
        let header_type_string = &app.language.column_list_type_header;
        let header_constraints_string = &app.language.column_list_constraints_header;
        let header = [
            format!("{}", header_name_string),
            format!("{}", header_type_string),
            format!("{}", header_constraints_string),
        ]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(Style::default().fg(app.text_color()));

        let rows: Vec<Row> = app
            .column_list_view
            .as_ref()
            .unwrap()
            .items
            .iter()
            .map(|col| {
                let mut col_constraint_text = "".to_string();
                if col.is_pk {
                    let pk_string = &app.language.sql_pk_constraint;
                    col_constraint_text.push_str(format!("[{}]", pk_string).as_str());
                }
                if col.is_unique {
                    let unique_string = &app.language.sql_unique_constraint;
                    col_constraint_text.push_str(format!("[{}]", unique_string).as_str());
                }
                if col.is_not_null {
                    let not_null_string = &app.language.sql_not_null_constraint;
                    col_constraint_text.push_str(format!("[{}]", not_null_string).as_str());
                }
                if col.is_fk {
                    let unknown_ref_table_string = &app.language.column_list_unknown_fk_ref;
                    let fk_string = &app.language.sql_fk_constraint;
                    let ref_table = col
                        .references_table
                        .as_deref()
                        .unwrap_or(unknown_ref_table_string);
                    col_constraint_text
                        .push_str(&format!("[{} -> {}]", fk_string, ref_table).to_string());
                }

                Row::new(vec![
                    Cell::from(Text::from(col.name.to_string())),
                    Cell::from(Text::from(col.col_type.to_string())),
                    Cell::from(Text::from(col_constraint_text)),
                ])
            })
            .collect();

        let col_constraints = [
            Constraint::Min(15),
            Constraint::Length(8),
            Constraint::Min(10),
        ];

        let highlight_style = Style::default()
            .bg(app.background_highlight_color())
            .fg(app.text_highlight_color());
        let border_block_style = Style::default()
            .bg(app.background_color())
            .fg(app.border_color());
        let border_block = Block::new()
            .borders(Borders::ALL)
            .style(border_block_style)
            .title(format!("{}", column_list_title));
        let unwrapped_column_list: &mut crate::column::column_list::ColumnListView =
            app.column_list_view.as_mut().unwrap();

        render_table(
            frame,
            &mut unwrapped_column_list.state,
            Some(header),
            rows,
            col_constraints.to_vec(),
            area,
            highlight_style,
            border_block,
        );

        render_vertical_scrollbar(
            frame,
            scrollbar_style,
            area,
            None,
            &mut unwrapped_column_list.scroll_state,
        );
    } else {
        let style = Style::default()
            .bg(app.background_color())
            .fg(app.text_color());
        let empty_block = Block::default()
            .title(format!("{}", column_list_title))
            .borders(Borders::ALL);
        let column_list_emtpy_string = &app.language.column_list_emtpy_placeholder;
        let paragraph = Paragraph::new(format!("{}", column_list_emtpy_string))
            .block(empty_block)
            .style(style);

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
    }
}

fn render_color_scheme_preview(
    frame: &mut Frame,
    area: Rect,
    color_scheme: &SelectedScheme,
    border_style: Style,
) {
    let border_block = Block::default().style(border_style).borders(Borders::ALL);
    let colors = color_scheme.colors();
    let color_vec = [
        colors.text,
        colors.text_alt,
        colors.text_highlight,
        colors.background,
        colors.background_alt,
        colors.background_highlight,
        colors.warning,
        colors.error,
        colors.border,
        colors.accent,
    ];

    frame.render_widget(border_block.clone(), area);

    let inner_area = border_block.inner(area);
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(vec![
            Constraint::Ratio(1, color_vec.len() as u32);
            color_vec.len()
        ])
        .split(inner_area);

    for (color, chunk) in color_vec.iter().zip(chunks.iter()) {
        frame.render_widget(Block::default().style(Style::default().bg(*color)), *chunk);
    }
}

fn render_vertical_scrollbar(
    frame: &mut Frame,
    style: Style,
    area: Rect,
    endpoints: Option<&str>,
    scroll_bar_state: &mut ScrollbarState,
) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(endpoints)
            .end_symbol(endpoints)
            .style(style),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        scroll_bar_state,
    );
}

#[allow(clippy::too_many_arguments)]
fn render_table(
    frame: &mut Frame,
    state: &mut TableState,
    header: Option<Row>,
    rows: Vec<Row>,
    col_widths: Vec<Constraint>,
    area: Rect,
    highlight_style: Style,
    block: Block,
) {
    let mut table = Table::new(rows, col_widths)
        .block(block)
        .row_highlight_style(highlight_style)
        .highlight_spacing(HighlightSpacing::Always);

    if let Some(table_header) = header {
        table = table.header(table_header);
    }

    frame.render_stateful_widget(table, area, state);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: Rect) -> Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    area
}

fn get_chunks_from_percentages(
    area: Rect,
    direction: Direction,
    percentages: Vec<u16>,
) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = percentages
        .iter()
        .map(|value| Constraint::Percentage(*value))
        .collect();

    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}

fn compute_col_widths(
    cols: &[ColumnInfo],
    rows: &[RowInfo],
    min: usize,
    max: usize,
    display_metainfo: &bool,
) -> Vec<Constraint> {
    cols.iter()
        .enumerate()
        .map(|(i, col)| {
            let header_len = col.col_name_length(display_metainfo);
            let max_data_len = rows
                .iter()
                .map(|row| row.values.get(i).map_or(0, |val| val.len()))
                .max()
                .unwrap_or(0);
            let width = header_len.max(max_data_len).clamp(min, max);

            Constraint::Length(width as u16)
        })
        .collect()
}

fn render_titled_paragraph<S>(
    frame: &mut Frame,
    app: &App,
    info_bits: &[S],
    title: &str,
    style: Style,
    area: Rect,
) where
    S: AsRef<str>,
{
    let titled_paragraph_block = Block::default()
        .style(style)
        .title(title)
        .borders(Borders::ALL);
    let info_text = format_info_text(info_bits, app);
    let titled_paragraph = Paragraph::new(info_text)
        .wrap(Wrap { trim: true })
        .block(titled_paragraph_block);

    frame.render_widget(Clear, area);
    frame.render_widget(titled_paragraph, area);
}

fn render_info_paragraph<S>(info_bits: &[S], frame: &mut Frame, app: &App, area: Rect)
where
    S: AsRef<str>,
{
    let info_style = Style::default()
        .fg(app.border_color())
        .bg(app.background_alt_color());
    let info_title = &app.language.info_block_title;

    render_titled_paragraph(frame, app, info_bits, info_title, info_style, area);
}

fn format_info_text<'a, S>(text_bits: &'a [S], app: &App) -> Text<'a>
where
    S: AsRef<str>,
{
    let general_text_style = Style::default().fg(app.text_color());
    let alt_text_style_1 = Style::default().fg(app.text_alt_color());

    let mut info_text = Text::default();

    for (i, bit) in text_bits.iter().enumerate() {
        let s = bit.as_ref();

        if i % 2 == 0 {
            info_text.push_span(Span::styled::<&str, Style>(s, alt_text_style_1));
            info_text.push_span(Span::styled(" - ", general_text_style));
        } else {
            info_text.push_span(Span::styled::<&str, Style>(s, general_text_style));
            if i < text_bits.len() - 1 {
                info_text.push_span(Span::styled(", ", general_text_style));
            }
        }
    }

    info_text
}

fn line_width(line: &Line) -> usize {
    line.spans.iter().map(|span| span.width()).sum()
}

fn get_table_and_scrollbar_areas(area: Rect) -> (Rect, Rect) {
    let scrollbar_width = 3;
    let table_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width.saturating_sub(scrollbar_width),
        height: area.height,
    };
    let scrollbar_area = Rect {
        x: area.x + table_area.width,
        y: area.y,
        width: scrollbar_width,
        height: area.height,
    };

    (table_area, scrollbar_area)
}
