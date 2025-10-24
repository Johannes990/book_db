use crate::{
    app::{App, PopUp, Screen},
    column::column_info::ColumnInfo,
    options::{OptionKind, SelectedScheme},
    row::row_info::RowInfo,
    ui::{colors::app_colors::ColorScheme, input::key_bindings::AppInputEvent},
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
    let mut frame_area = frame.area();

    if app.options.render_footer {
        let chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, vec![1]);
        frame_area = chunks[0];
        handle_footer_data_and_rendering(frame, app, chunks[1]);
    }

    let (main_chunk, info_chunk) = split_with_optional_info_chunk(frame_area, app);
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

    frame.render_widget(main_page_paragraph, main_chunk);

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_file_explorer_screen(frame: &mut Frame, app: &mut App) {
    let mut frame_area = frame.area();

    if app.options.render_footer {
        let chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, vec![1]);
        frame_area = chunks[0];
        handle_footer_data_and_rendering(frame, app, chunks[1]);
    }

    let (main_chunk, info_chunk) = split_with_optional_info_chunk(frame_area, app);
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

    frame.render_widget(file_explorer_block, main_chunk);

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
        x: main_chunk.x,
        y: main_chunk.y + 1,
        height: main_chunk.height - 1,
        width: main_chunk.width,
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

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_database_schema_screen(frame: &mut Frame, app: &mut App) {
    let mut frame_area = frame.area();

    if app.options.render_footer {
        let chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, vec![1]);
        frame_area = chunks[0];
        handle_footer_data_and_rendering(frame, app, chunks[1]);
    }

    let db_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let (main_chunk, info_chunk) = split_with_optional_info_chunk(frame_area, app);
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
    let inner_area = outer_block.inner(main_chunk);
    let table_column_chunks =
        get_chunks_from_percentages(inner_area, Direction::Horizontal, vec![50, 50]);

    frame.render_widget(outer_block, main_chunk);
    render_table_list(frame, app, table_column_chunks[0]);
    render_column_list(frame, app, table_column_chunks[1]);

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_new_database_screen(frame: &mut Frame, app: &mut App) {
    let mut frame_area = frame.area();

    if app.options.render_footer {
        let chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, vec![1]);
        frame_area = chunks[0];
        handle_footer_data_and_rendering(frame, app, chunks[1]);
    }

    let page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let (main_chunk, info_chunk) = split_with_optional_info_chunk(frame_area, app);
    let insert_text_area_on_style = Style::default()
        .bg(app.background_highlight_color())
        .fg(app.text_highlight_color());

    if let Some(form) = &mut app.create_db_form {
        form.set_styles(insert_text_area_on_style, Style::default(), page_style);
    }

    if let Some(form) = &app.create_db_form {
        form.render_widget_and_cursor(frame, main_chunk);
    }

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_database_table_screen(frame: &mut Frame, app: &mut App) {
    let mut frame_area = frame.area();

    if app.options.render_footer {
        let chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, vec![1]);
        frame_area = chunks[0];
        handle_footer_data_and_rendering(frame, app, chunks[1]);
    }

    let (main_chunk, info_chunk) = split_with_optional_info_chunk(frame_area, app);
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
    let inner_area = outer_block.inner(main_chunk);

    frame.render_widget(outer_block, main_chunk);

    let language_strings = App::get_strings_for_col_info(&app.language);
    let header_cells: Vec<Cell> = app
        .selected_table_columns
        .iter()
        .map(|col| {
            let display_metainfo = &app.options.display_col_metainfo_in_table_view;
            let line = col.get_line_from_col_info(
                language_strings,
                *display_metainfo,
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

    let table_name = table_name.to_string();
    let border_block_style = Style::default()
        .bg(app.background_color())
        .fg(app.border_color());
    let border_block = Block::new()
        .borders(Borders::ALL)
        .style(border_block_style)
        .title(table_name);
    let unwrapped_row_list = app.row_list_view.as_mut().unwrap();
    let min = 5;
    let max = 40;
    let col_constraints = compute_col_widths(
        &app.selected_table_columns,
        &unwrapped_row_list.items,
        min,
        max,
        language_strings,
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

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_options_screen(frame: &mut Frame, app: &mut App) {
    let mut frame_area = frame.area();

    if app.options.render_footer {
        let chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, vec![1]);
        frame_area = chunks[0];
        handle_footer_data_and_rendering(frame, app, chunks[1]);
    }

    let general_page_style = Style::default()
        .fg(app.text_color())
        .bg(app.background_color());
    let selected_style = Style::default().fg(app.error_color());
    let options_title = &app.language.screen_options_title;
    let options_block = Block::default()
        .title(options_title.to_string())
        .style(general_page_style);

    frame.render_widget(options_block, frame_area);

    let color_scheme_preview_rows = 8;
    let limits = if app.options.render_info_section {
        vec![color_scheme_preview_rows, app.options.info_section_height]
    } else {
        vec![color_scheme_preview_rows]
    };
    let vertical_chunks = get_chunks_from_fixed_limits(frame_area, Direction::Vertical, limits);
    let horizontal_chunks =
        get_chunks_from_percentages(vertical_chunks[1], Direction::Horizontal, vec![50, 50]);
    let color_schemes_string = &app.language.screen_options_color_schemes;
    let header = Row::new(vec![Cell::from(color_schemes_string.to_string())]);
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

    let selectable_options_strings = [
        &app.language.screen_options_metadata_in_table,
        &app.language.screen_options_metadata_in_insert,
        &app.language.screen_options_render_footer,
        &app.language.screen_options_render_info,
        &app.language.screen_options_info_block_height,
        &app.language.screen_options_log_performance_info,
    ];
    let highlight_marker_symbol = &app.language.widget_selectable_field_highlight_marker;
    let option_on_string = &app.language.widget_selectable_field_on_value;
    let option_off_string = &app.language.widget_selectable_field_off_value;

    for (i, field) in app.options.fields.iter().enumerate() {
        let content = match &field.kind {
            OptionKind::Toggle(value) => {
                let prefix = if field.selected {
                    format!("{} ", highlight_marker_symbol)
                } else {
                    "  ".to_string()
                };
                let postfix = if *value {
                    option_on_string
                } else {
                    option_off_string
                };
                format!("{}{}: {}", prefix, selectable_options_strings[i], postfix)
            }
            OptionKind::TextInput(ref text) => {
                let prefix = if field.selected {
                    format!("{} ", highlight_marker_symbol)
                } else {
                    "  ".to_string()
                };
                format!("{}{}: {}", prefix, selectable_options_strings[i], text)
            }
        };
        let block = Block::default().borders(Borders::NONE);
        let width = content.len() as u16;
        let label = Line::from(content);
        let paragraph = Paragraph::new(label)
            .style(if field.selected {
                selected_style
            } else {
                general_page_style
            })
            .block(block);
        let option_widget_area = Rect {
            x: vertical_chunks[0].x + 1,
            y: vertical_chunks[0].y + 1 + i as u16,
            width,
            height: 1,
        };
        frame.render_widget(paragraph, option_widget_area);
    }

    if app.options.render_info_section {
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
            AppInputEvent::SwitchToEdit,
        ];

        let info_bits = app
            .key_bindings
            .get_info_bits_from_events(&events, &app.language);

        render_info_paragraph(&info_bits, frame, app, vertical_chunks[2]);
    }
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
        if app.options.render_info_section {
            &info_bits
        } else {
            &[]
        },
        quit_confirmation_string,
        quit_popup_style,
        area,
    );
}

fn render_no_db_loaded_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let popup_style = Style::default()
        .bg(app.warning_color())
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
        if app.options.render_info_section {
            &info_bits
        } else {
            &[]
        },
        no_db_loaded_string,
        popup_style,
        area,
    );
}

fn render_insert_row_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 55, frame.area());
    let (main_chunk, info_chunk) = split_with_optional_info_chunk(area, app);
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
    let language_strings = App::get_strings_for_col_info(&app.language);

    if let Some(form) = app.row_insert_form.as_mut() {
        form.set_styles(
            insert_text_area_on_style,
            insert_text_area_off_style,
            insert_row_popup_style,
        );

        let popup_block = Block::default()
            .borders(Borders::ALL)
            .style(insert_row_popup_style);

        frame.render_widget(Clear, main_chunk);
        frame.render_widget(popup_block, main_chunk);

        let text_area = main_chunk.inner(Margin {
            horizontal: 1,
            vertical: 1,
        });

        for (i, col_info) in app.selected_table_columns.iter().enumerate() {
            let x = text_area.x;
            let y = text_area.y + i as u16;
            let display_metainfo = &app.options.display_col_metainfo_in_insert_view;

            let mut label_line = col_info.get_line_from_col_info(
                language_strings,
                *display_metainfo,
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

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_insert_table_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 55, frame.area());
    let (main_chunk, info_chunk) = split_with_optional_info_chunk(area, app);
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
        form.render_widget_and_cursor(frame, main_chunk);
    }

    if let Some(info_chunk) = info_chunk {
        let events = [
            AppInputEvent::ClosePopUp,
            AppInputEvent::SwitchToEdit,
            AppInputEvent::ExecuteAction,
        ];

        let info_bits = app
            .key_bindings
            .get_info_bits_from_events(&events, &app.language);

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_drop_table_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 55, frame.area());
    let (main_chunk, info_chunk) = split_with_optional_info_chunk(area, app);
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
        form.render_widget_and_cursor(frame, main_chunk);
    }

    if let Some(info_chunk) = info_chunk {
        let events = [
            AppInputEvent::ClosePopUp,
            AppInputEvent::SwitchToEdit,
            AppInputEvent::ExecuteAction,
        ];

        let info_bits = app
            .key_bindings
            .get_info_bits_from_events(&events, &app.language);

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_delete_row_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 55, frame.area());
    let (main_chunk, info_chunk) = split_with_optional_info_chunk(area, app);
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
        form.render_widget_and_cursor(frame, main_chunk);
    }

    if let Some(info_chunk) = info_chunk {
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

        render_info_paragraph(&info_bits, frame, app, info_chunk);
    }
}

fn render_error_popup(frame: &mut Frame, app: &mut App) {
    if let Some(error) = &app.current_error {
        let area = centered_rect(40, 30, frame.area());
        let style = Style::default().bg(app.error_color()).fg(app.text_color());
        let error_title = &app.language.popup_error_title;
        let error_block = Block::default()
            .borders(Borders::ALL)
            .title(error_title.to_string())
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
            Cell::new(header_name_string.to_string()),
            Cell::new(header_rows_string.to_string()),
            Cell::new(header_type_string.to_string()),
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
                        view_element_string.to_string()
                    } else {
                        table_element_string.to_string()
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
            .title(table_title.to_string());
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
            .title(table_title.to_string())
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
            header_name_string.to_string(),
            header_type_string.to_string(),
            header_constraints_string.to_string(),
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
            .title(column_list_title.to_string());
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
            .title(column_list_title.to_string())
            .borders(Borders::ALL);
        let column_list_emtpy_string = &app.language.column_list_emtpy_placeholder;
        let paragraph = Paragraph::new(column_list_emtpy_string.to_string())
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

fn get_chunks_from_fixed_limits(area: Rect, direction: Direction, limits: Vec<u16>) -> Rc<[Rect]> {
    let mut constraints: Vec<Constraint> = limits
        .iter()
        .map(|value| Constraint::Length(*value))
        .collect();
    constraints.insert(0, Constraint::Fill(1));

    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area)
}

fn split_with_optional_info_chunk(area: Rect, app: &App) -> (Rect, Option<Rect>) {
    let show_info = app.options.render_info_section;
    let limits = if show_info {
        vec![app.options.info_section_height]
    } else {
        vec![]
    };
    let chunks = get_chunks_from_fixed_limits(area, Direction::Vertical, limits);
    (chunks[0], if show_info { Some(chunks[1]) } else { None })
}

fn compute_col_widths(
    cols: &[ColumnInfo],
    rows: &[RowInfo],
    min: usize,
    max: usize,
    language_strings: (&str, &str, &str, &str, &str),
    display_metainfo: &bool,
) -> Vec<Constraint> {
    cols.iter()
        .enumerate()
        .map(|(i, col)| {
            let header_len = col.col_name_length(language_strings, *display_metainfo);
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

fn handle_footer_data_and_rendering(frame: &mut Frame, app: &App, area: Rect) {
    let datetime_format = "%b %d %H:%M ";
    let mut perf_info = String::from(" Perf profile unavailable");

    if let Some(stats) = app.statistics.get_statistics_data() {
        let prec = 3;
        let megs = 1024.0 * 1024.0;
        perf_info = format!(
            " {}: {:.prec$}%  {}: {:.prec$}MB",
            app.language.screen_splash_avg_proc_cpu_usage,
            stats.avg_process_cpu_usage / 24.0,
            app.language.screen_splash_avg_proc_memory_usage,
            stats.avg_process_memory_usage / megs,
        );
    }

    let date_and_time = chrono::Local::now().format(datetime_format).to_string();

    render_footer_row(frame, app, area, perf_info, date_and_time);
}

fn render_footer_row(
    frame: &mut Frame,
    app: &App,
    area: Rect,
    left_info: String,
    right_info: String,
) {
    let footer_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    let footer_block = Block::default()
        .style(footer_style)
        .title(Line::from(left_info).left_aligned())
        .title(Line::from(right_info).right_aligned());

    frame.render_widget(footer_block, area);
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

#[cfg(test)]
mod tests {
    use ratatui::layout::{Direction, Rect};

    use crate::ui::render::{get_chunks_from_fixed_limits, get_chunks_from_percentages};

    use super::centered_rect;

    #[test]
    fn test_centered_rect_returns_ok_values() {
        let base_area = Rect {
            x: 10,
            y: 10,
            width: 20,
            height: 20,
        };

        let test_rect = centered_rect(50, 50, base_area);

        assert_eq!(test_rect.height, 10);
        assert_eq!(test_rect.width, 10);
        assert_eq!(test_rect.x, 15);
        assert_eq!(test_rect.y, 15);
    }

    #[test]
    fn test_centered_rect_returns_0_for_0_size_rect() {
        let base_area = Rect {
            x: 0,
            y: 0,
            height: 0,
            width: 0,
        };

        let test_rect = centered_rect(50, 50, base_area);

        assert_eq!(test_rect.height, 0);
        assert_eq!(test_rect.width, 0);
    }

    #[test]
    fn test_centered_rect_returns_correct_x_values() {
        let base_area = Rect {
            x: 17,
            y: 0,
            height: 100,
            width: 100,
        };

        let percentages = [0, 25, 50, 75, 87, 93, 96];

        for percentage in percentages {
            let test_rect = centered_rect(percentage, 50, base_area);
            let percent_mult = (100.0 - percentage as f32) / 100.0;
            let half_base_width = base_area.width as f32 / 2.0;
            let correct_x = base_area.x as f32 + (half_base_width * percent_mult).ceil();

            assert_eq!(test_rect.x, correct_x as u16);
        }
    }

    #[test]
    fn test_centered_rect_returns_correct_y_values() {
        let base_area = Rect {
            x: 0,
            y: 13,
            width: 100,
            height: 100,
        };

        let percentages = [0, 25, 50, 75, 87, 93, 96];

        for percentage in percentages {
            let test_rect = centered_rect(50, percentage, base_area);
            let percent_mult = (100.0 - percentage as f32) / 100.0;
            let half_base_height = base_area.height as f32 / 2.0;
            let correct_y = base_area.y as f32 + (half_base_height * percent_mult).ceil();

            assert_eq!(test_rect.y, correct_y as u16);
        }
    }

    #[test]
    fn test_centered_rect_returns_correct_height_values() {
        let base_area = Rect {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        let percentages = [0, 25, 50, 75, 87, 93, 96];

        for percentage in percentages {
            let test_rect = centered_rect(100, percentage, base_area);
            let correct_height = ((percentage as f32 * base_area.height as f32) / 100.0).ceil();

            assert_eq!(test_rect.height, correct_height as u16);
        }
    }

    #[test]
    fn test_centered_rect_returns_correct_width_values() {
        let base_area = Rect {
            x: 0,
            y: 0,
            height: 200,
            width: 200,
        };

        let percentages = [0, 33, 50, 75, 67, 85, 90, 97];

        for percentage in percentages {
            let test_area = centered_rect(percentage, 100, base_area);
            let correct_width = ((percentage as f32 * base_area.width as f32) / 100.0).ceil();

            assert_eq!(test_area.width, correct_width as u16);
        }
    }

    #[test]
    fn test_get_chunks_from_percentages_vertical_2_chunks() {
        let base_area = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        let percentages = vec![50, 50];
        // chopping in the vertical so height gets cut in half
        let chunks = get_chunks_from_percentages(base_area, Direction::Vertical, percentages);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 50,
            width: 100,
        };
        let correct_chunk_2 = Rect {
            x: 0,
            y: 50,
            height: 50,
            width: 100,
        };

        assert_eq!(chunks[0], correct_chunk_1);
        assert_eq!(chunks[1], correct_chunk_2);
    }

    #[test]
    fn test_get_chunks_from_percentages_vertical_3_chunks() {
        let base_area = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        let percentages = vec![33, 34, 33];
        let chunks = get_chunks_from_percentages(base_area, Direction::Vertical, percentages);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 33,
            width: 100,
        };
        let correct_chunk_2 = Rect {
            x: 0,
            y: 33,
            height: 34,
            width: 100,
        };
        let correct_chunk_3 = Rect {
            x: 0,
            y: 67,
            height: 33,
            width: 100,
        };

        assert_eq!(chunks[0], correct_chunk_1);
        assert_eq!(chunks[1], correct_chunk_2);
        assert_eq!(chunks[2], correct_chunk_3);
    }

    #[test]
    fn test_get_chunks_from_percentages_horizontal_2_chunks() {
        let base_area = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        let percentages = vec![50, 50];
        // chopping in the horizontal so width gets cut in half
        let chunks = get_chunks_from_percentages(base_area, Direction::Horizontal, percentages);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 50,
        };
        let correct_chunk_2 = Rect {
            x: 50,
            y: 0,
            height: 100,
            width: 50,
        };

        assert_eq!(chunks[0], correct_chunk_1);
        assert_eq!(chunks[1], correct_chunk_2);
    }

    #[test]
    fn test_get_chunks_from_percentages_horizontal_3_chunks() {
        let base_area = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 100,
        };

        let percentages = vec![33, 34, 33];
        let chunks = get_chunks_from_percentages(base_area, Direction::Horizontal, percentages);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 100,
            width: 33,
        };
        let correct_chunk_2 = Rect {
            x: 33,
            y: 0,
            height: 100,
            width: 34,
        };
        let correct_chunk_3 = Rect {
            x: 67,
            y: 0,
            height: 100,
            width: 33,
        };

        assert_eq!(chunks[0], correct_chunk_1);
        assert_eq!(chunks[1], correct_chunk_2);
        assert_eq!(chunks[2], correct_chunk_3);
    }

    #[test]
    fn test_get_chunks_from_fixed_limits_vertical_0_limits() {
        let base_rect = Rect {
            x: 10,
            y: 10,
            height: 10,
            width: 33,
        };

        let limits = vec![];
        let chunk = get_chunks_from_fixed_limits(base_rect, Direction::Vertical, limits);

        assert_eq!(chunk.first().unwrap().clone(), base_rect);
    }

    #[test]
    fn test_get_chunks_from_fixed_limits_vertical_1_limit() {
        let base_rect = Rect {
            x: 0,
            y: 0,
            height: 25,
            width: 33,
        };

        let limits = vec![10];
        let chunks = get_chunks_from_fixed_limits(base_rect, Direction::Vertical, limits);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 15,
            width: 33,
        };
        let correct_chunk_2 = Rect {
            x: 0,
            y: 15,
            height: 10,
            width: 33,
        };

        assert_eq!(chunks.first().unwrap().clone(), correct_chunk_1);
        assert_eq!(chunks.get(1).unwrap().clone(), correct_chunk_2);
    }

    #[test]
    fn test_get_chunks_from_fixed_limits_vertical_several_limits() {
        let base_rect = Rect {
            x: 0,
            y: 0,
            height: 50,
            width: 50,
        };

        let limits = vec![5, 5, 10];
        let chunks = get_chunks_from_fixed_limits(base_rect, Direction::Vertical, limits);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 30,
            width: 50,
        };
        let correct_chunk_2 = Rect {
            x: 0,
            y: 30,
            height: 5,
            width: 50,
        };
        let correct_chunk_3 = Rect {
            x: 0,
            y: 35,
            height: 5,
            width: 50,
        };
        let correct_chunk_4 = Rect {
            x: 0,
            y: 40,
            height: 10,
            width: 50,
        };

        assert_eq!(chunks.first().unwrap().clone(), correct_chunk_1);
        assert_eq!(chunks.get(1).unwrap().clone(), correct_chunk_2);
        assert_eq!(chunks.get(2).unwrap().clone(), correct_chunk_3);
        assert_eq!(chunks.get(3).unwrap().clone(), correct_chunk_4);
    }

    #[test]
    fn test_get_chunks_from_fixed_limits_horizontal_0_limits() {
        let base_rect = Rect {
            x: 10,
            y: 10,
            height: 10,
            width: 33,
        };

        let limits = vec![];
        let chunk = get_chunks_from_fixed_limits(base_rect, Direction::Horizontal, limits);

        assert_eq!(chunk.first().unwrap().clone(), base_rect);
    }

    #[test]
    fn test_get_chunks_from_fixed_limits_horizontal_1_limit() {
        let base_rect = Rect {
            x: 0,
            y: 0,
            height: 25,
            width: 33,
        };

        let limits = vec![10];
        let chunks = get_chunks_from_fixed_limits(base_rect, Direction::Horizontal, limits);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 25,
            width: 23,
        };
        let correct_chunk_2 = Rect {
            x: 23,
            y: 0,
            height: 25,
            width: 10,
        };

        assert_eq!(chunks.first().unwrap().clone(), correct_chunk_1);
        assert_eq!(chunks.get(1).unwrap().clone(), correct_chunk_2);
    }

    #[test]
    fn test_get_chunks_from_fixed_limits_horizontal_several_limits() {
        let base_rect = Rect {
            x: 0,
            y: 0,
            height: 50,
            width: 50,
        };

        let limits = vec![15, 5, 10];
        let chunks = get_chunks_from_fixed_limits(base_rect, Direction::Horizontal, limits);
        let correct_chunk_1 = Rect {
            x: 0,
            y: 0,
            height: 50,
            width: 20,
        };
        let correct_chunk_2 = Rect {
            x: 20,
            y: 0,
            height: 50,
            width: 15,
        };
        let correct_chunk_3 = Rect {
            x: 35,
            y: 0,
            height: 50,
            width: 5,
        };
        let correct_chunk_4 = Rect {
            x: 40,
            y: 0,
            height: 50,
            width: 10,
        };

        assert_eq!(chunks.first().unwrap().clone(), correct_chunk_1);
        assert_eq!(chunks.get(1).unwrap().clone(), correct_chunk_2);
        assert_eq!(chunks.get(2).unwrap().clone(), correct_chunk_3);
        assert_eq!(chunks.get(3).unwrap().clone(), correct_chunk_4);
    }
}
