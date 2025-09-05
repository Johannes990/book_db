use crate::{
    app::{App, PopUp, Screen},
    column::column_info::ColumnInfo,
    options::{SelectedOption, SelectedScheme},
    row::row_info::RowInfo,
    ui::colors::app_colors::ColorScheme,
    widgets::selectable_line::SelectableLine,
};
use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Alignment, Margin, Rect},
    style::{Color, Modifier, Style},
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
    let loaded_db_name = app
        .selected_db
        .as_ref()
        .map(|db| format!("{}.{}", db.db_name, db.db_extension))
        .unwrap_or("None".to_string());
    let selected_table_name = app
        .selected_db_table
        .as_ref()
        .map_or("None", |name| name);
    let main_page_content = vec![
        Line::from(" Database terminal app v0.1.0"),
        Line::from(""),
        Line::from(format!(
            " Loaded Database: {}",
            loaded_db_name
        )),
        Line::from(format!(
            " Loaded Table: {}",
            selected_table_name,
        ))
    ];
    let main_page_paragraph =
        Paragraph::new(main_page_content).style(main_page_style);

    frame.render_widget(main_page_paragraph, chunks[0]);

    let info_bits = vec![
        "Commands:",
        "f",
        " - open file explorer",
        "d",
        " - open loaded database",
        "n",
        " - create new database file",
        "o",
        " - open options",
        "ESC / q",
        " - quit app",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_file_explorer_screen(frame: &mut Frame, app: &mut App) {
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let scrollbar_style = Style::default().fg(app.border_color());
    let fexp_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let file_explorer_block = Block::default()
        .title(" Explorer")
        .title(
            Line::from(format!(
                "Currently in: {} ",
                app.file_explorer_table.current_path.display()
            ))
            .centered(),
        )
        .title(
            Line::from(format!(
                "Mode: {} ",
                app.current_mode
            ))
            .right_aligned()
        )
        .borders(Borders::NONE)
        .style(fexp_page_style);

    frame.render_widget(file_explorer_block, chunks[0]);

    let header = ["File/Folder", "Size", "Date created"]
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

    let highlight_col = app.background_highlight_color();
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
        highlight_col,
        border_block,
    );

    render_vertical_scrollbar(
        frame,
        scrollbar_style,
        scrollbar_area,
        None,
        &mut app.file_explorer_table.scroll_state,
    );

    let info_bits = vec![
        "Commands:",
        "↑ / ↓",
        " - navigate",
        "ESC / q",
        " - return to splash screen",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_database_schema_screen(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default()
        .bg(app.background_color())
        .fg(app.text_color());
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let db_name = app
        .selected_db
        .as_ref()
        .expect("No DataBase found")
        .get_db_name();
    let outer_block = Block::default()
        .title(Line::from(format!("Current Database: {} ", db_name)).left_aligned())
        .title(Line::from(format!("Mode: {} ", app.current_mode)).right_aligned())
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);
    let table_column_chunks =
        get_chunks_from_percentages(inner_area, Direction::Horizontal, vec![50, 50]);

    frame.render_widget(outer_block, chunks[0]);

    render_table_list(frame, app, table_column_chunks[0]);

    render_column_list(frame, app, table_column_chunks[1]);

    let info_bits = vec![
        "Commands:",
        "↑ / ↓",
        " - navigate",
        "Enter",
        " - select table",
        "n",
        " - create new table",
        "d",
        " - delete table",
        "ESC / q",
        " - return to splash screen",
    ];
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

    let info_bits = vec![
        "Commands:",
        "CTRL + s",
        " - create new database",
        "ESC / q",
        " - return to splash screen",
    ];
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
    let table_name = app.selected_db_table.as_ref().expect("unknown");
    let outer_block = Block::default()
        .title(Line::from(format!(" Current Table: {} ", table_name)).left_aligned())
        .title(Line::from(format!("Mode: {} ", app.current_mode)).right_aligned())
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
    let highlight_col = app.background_highlight_color();
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
        highlight_col,
        border_block,
    );

    render_vertical_scrollbar(
        frame,
        scrollbar_style,
        inner_area,
        None,
        &mut unwrapped_row_list.scroll_bar_state,
    );

    let info_bits = vec![
        "Commands:",
        "i",
        " - create new entry",
        "d",
        " - delete entry",
        "ESC / b",
        " - return to database view",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_options_screen(frame: &mut Frame, app: &mut App) {
    let general_page_style = Style::default()
        .fg(app.text_color())
        .bg(app.background_color());
    let selected_style = Style::default().fg(app.error_color());
    let options_block = Block::default()
        .title("Options View")
        .style(general_page_style);

    frame.render_widget(options_block, frame.area());

    let vertical_chunks =
        get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![50, 25, 25]);
    let horizontal_chunks =
        get_chunks_from_percentages(vertical_chunks[1], Direction::Horizontal, vec![50, 50]);

    let header = Row::new(vec![Cell::from("Color Schemes")]);
    let constraints = vec![Constraint::Min(5)];
    let border_block_style = Style::default()
        .bg(app.background_color())
        .fg(app.border_color());
    let border_block = Block::new()
        .borders(Borders::ALL)
        .style(border_block_style);

    let selected_scheme = app.options.selected_color_scheme;
    let text_color = app.text_color();
    let text_highlight_color = app.text_highlight_color();
    let background_highlight_color = app.background_highlight_color();
    let border_color = app.border_color();
    let color_table = &mut app.options.available_color_schemes;

    let rows: Vec<_> = color_table
        .items
        .iter()
        .map(|scheme| {
            let scheme_name = format!("{:?}", scheme);
            let style = if *scheme == selected_scheme {
                Style::default()
                    .fg(text_highlight_color)
                    .bg(background_highlight_color)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(text_color)
            };
            Row::new(vec![Cell::from(scheme_name)]).style(style)
        })
        .collect();

        render_table(
            frame,
            &mut color_table.state,
            Some(header),
            rows,
            constraints,
            horizontal_chunks[0],
            background_highlight_color,
            border_block
        );

        render_vertical_scrollbar(
            frame,
            Style::default().fg(border_color),
            horizontal_chunks[0],
            Some("•"),
            &mut color_table.scroll_bar_state,
        );

    render_color_scheme_preview(
        frame,
        horizontal_chunks[1],
        &app.options.selected_color_scheme,
        border_block_style,
    );

    let table_metainfo_toggle_button = SelectableLine::default(
        "Display column metadata in table view: ",
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
    let insert_metainfo_toggle_button = SelectableLine::default(
        "Display column metadata in insert view: ",
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

    let info_bits = vec![
        "Commands: ",
        "← / →",
        " - switch between color schemes",
        "↑ / ↓",
        " - switch between options",
        "ESC / q",
        " - return to splash screen",
    ];
    render_info_paragraph(&info_bits, frame, app, vertical_chunks[2]);
}

fn render_quit_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(55, 30, frame.area());
    let quit_popup_style = Style::default()
        .bg(app.warning_color())
        .fg(app.text_color());
    let info_bits = vec![
        "",
        "y",
        " - quit app",
        "ESC / n",
        " - return to splash screen",
    ];
    render_titled_paragraph(
        frame,
        app,
        &info_bits,
        "Are you sure you want to quit",
        quit_popup_style,
        area,
    );
}

fn render_no_db_loaded_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let popup_style = Style::default()
        .bg(app.background_alt_color())
        .fg(app.text_color());
    let info_bits = vec![
        "",
        "CTRL + f",
        " - open file explorer",
        "ESC",
        " - return to splash screen",
    ];
    render_titled_paragraph(
        frame,
        app,
        &info_bits,
        "No database file loaded",
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

    let info_bits = vec![
        "Commands:",
        "CTRL + s",
        " - save entry",
        "ESC / ALT + q",
        " - return to database table view",
    ];

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

    let info_bits = vec![
        "Commands:",
        "Enter",
        " - execute sql",
        "ESC",
        " - return to database schema view",
    ];
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

    let info_bits = vec![
        "Commands:",
        "Enter",
        " - drop table with given name",
        "ESC",
        " - return to database schema view",
    ];

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

    let info_bits = vec![
        "Commands:",
        "Enter",
        " - delete entry with given ID",
        "ESC",
        " - return to database table view",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_error_popup(frame: &mut Frame, app: &mut App) {
    if let Some(error) = &app.current_error {
        let area = centered_rect(40, 30, frame.area());
        let style = Style::default().bg(app.error_color()).fg(app.text_color());
        let error_block = Block::default()
            .borders(Borders::ALL)
            .title("Error")
            .border_style(Style::default().fg(Color::Red))
            .style(style);
        let error_message = format!("Error: {}", error);
        let mut error_text = Text::from(error_message);
        error_text.push_line(Span::raw("Press ESC to close this popup"));
        let paragraph = Paragraph::new(error_text)
            .block(error_block)
            .alignment(Alignment::Center)
            .wrap(Wrap { trim: true });

        frame.render_widget(paragraph, area);
    }
}

fn render_table_list(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.table_list_view.is_some() {
        let scrollbar_style = Style::default().fg(app.border_color());
        let row_style = Style::default()
            .bg(app.background_color())
            .fg(app.text_color());
        let header = Row::new(vec![
            Cell::new("Name"),
            Cell::new("Rows"),
            Cell::new("Type"),
        ])
        .style(row_style);
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
                    Cell::from(Text::from(if table.is_view { "View" } else { "Table" })),
                ])
                .style(row_style)
            })
            .collect();

        let col_constraints = [
            Constraint::Min(15),   // table name
            Constraint::Min(7),    // row count
            Constraint::Length(7), // type (table, view)
        ];

        let table_title = "Tables";
        let border_block_style = Style::default()
            .bg(app.background_color())
            .fg(app.border_color());
        let border_block = Block::new()
            .borders(Borders::ALL)
            .style(border_block_style)
            .title(table_title);
        let highlight_color = app.background_highlight_color();
        let unwrapped_table_list = app.table_list_view.as_mut().unwrap();

        render_table(
            frame,
            &mut unwrapped_table_list.state,
            Some(header),
            rows,
            col_constraints.to_vec(),
            area,
            highlight_color,
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
        let empty_block = Block::default().title("Tables").borders(Borders::ALL);
        let paragraph = Paragraph::new("Empty Schema")
            .block(empty_block)
            .style(style);

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
    }
}

fn render_column_list(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.column_list_view.is_some() {
        let scrollbar_style = Style::default().fg(app.border_color());

        let header = ["Name", "Type", "Constraints"]
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
                    col_constraint_text.push_str("[PK]");
                }
                if col.is_unique {
                    col_constraint_text.push_str("[UNIQUE]");
                }
                if col.is_not_null {
                    col_constraint_text.push_str("[NOT NULL]");
                }
                if col.is_fk {
                    let ref_table = col.references_table.as_deref().unwrap_or("Unknown");
                    col_constraint_text.push_str(&format!("[FK -> {}]", ref_table).to_string());
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

        let highlight_color = app.background_highlight_color();
        let table_title = "Columns";
        let border_block_style = Style::default()
            .bg(app.background_color())
            .fg(app.border_color());
        let border_block = Block::new()
            .borders(Borders::ALL)
            .style(border_block_style)
            .title(table_title);
        let unwrapped_column_list: &mut crate::column::column_list::ColumnListView =
            app.column_list_view.as_mut().unwrap();

        render_table(
            frame,
            &mut unwrapped_column_list.state,
            Some(header),
            rows,
            col_constraints.to_vec(),
            area,
            highlight_color,
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
        let empty_block = Block::default().title("Columns").borders(Borders::ALL);
        let paragraph = Paragraph::new("No columns").block(empty_block).style(style);

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
    highlight_col: Color,
    block: Block,
) {
    let selected_style = Style::default().bg(highlight_col);
    let mut table = Table::new(rows, col_widths)
        .block(block)
        .row_highlight_style(selected_style)
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

fn format_info_text<'a>(text_bits: &'a [&str], app: &App) -> Text<'a> {
    let general_text_style = Style::default().fg(app.text_color());
    let alt_text_style_1 = Style::default().fg(app.text_alt_color());

    let mut info_text = Text::default();

    for (i, bit) in text_bits.iter().enumerate() {
        if i == 0 {
            info_text = Text::from(Span::styled::<&str, Style>(bit, general_text_style));
        } else if i % 2 != 0 {
            info_text.push_line(Span::styled::<&str, Style>(bit, alt_text_style_1));
        } else {
            info_text.push_span(Span::styled::<&str, Style>(bit, general_text_style));
        }
    }

    info_text
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

fn render_titled_paragraph(
    frame: &mut Frame,
    app: &App,
    info_bits: &[&str],
    title: &str,
    style: Style,
    area: Rect,
) {
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

fn render_info_paragraph(info_bits: &[&str], frame: &mut Frame, app: &App, area: Rect) {
    let info_style = Style::default()
        .fg(app.border_color())
        .bg(app.background_alt_color());

    render_titled_paragraph(frame, app, info_bits, "Info", info_style, area);
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
