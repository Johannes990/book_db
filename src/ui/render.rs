use ratatui::{
    layout::{Constraint, Direction, Flex, Layout},
    prelude::{Alignment, Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{
        Block, Borders, Cell, Clear, HighlightSpacing, List, ListItem, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap
    },
    Frame,
    Terminal
};
use std::{io, rc::Rc, vec};
use crate::{
    app::{App, PopUp, Screen},
    column::column_info::ColumnInfo,
    file_explorer::file_explorer_data::FileExplorerData,
    options::SelectedOption,
    row::row_info::RowInfo,
    widgets::selectable_line::SelectableLine,
};

use super::colorscheme::ColorScheme;


pub fn render<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B: ratatui::backend::Backend,
{
    // Call terminal.draw and convert its output to io::Result<()>
    terminal.draw(|frame| {
        match app.current_screen {
            Screen::Splash => {
                render_splash_screen(frame, app);
                match app.current_popup {
                    PopUp::NoDBLoaded => render_no_db_loaded_popup(frame, app),
                    PopUp::Quit => render_quit_popup(frame, app),
                    _ => {},
                }
            },
            Screen::CreateNewFile => {
                render_new_database_screen(frame, app);
                match app.current_popup {
                    PopUp::Error(ref err) => {
                        let msg = format!("{}", err);
                        render_error_popup(frame, app, &msg);
                    },
                    _ => {}
                    
                }
            },
            Screen::FileExplorer => {
                render_file_explorer_screen(frame, app);
                match app.current_popup {
                    _ => {},
                }
            },
            Screen::DatabaseSchema => {
                render_database_schema_screen(frame, app);
                match app.current_popup {
                    PopUp::InsertTable => render_insert_table_popup(frame, app),
                    PopUp::DeleteTable => render_drop_table_popup(frame, app),
                    _ => {},
                }
            },
            Screen::DataBaseTable => {
                render_database_table_screen(frame, app);
                match app.current_popup {
                    PopUp::InsertRow => render_insert_row_popup(frame, app),
                    PopUp::DeleteRow => render_delete_row_popup(frame, app),
                    PopUp::Error(ref err) => {
                        let msg = format!("{}", err);
                        render_error_popup(frame, app, &msg);
                    }
                    _ => {},
                }
            },
            Screen::Options => {
                render_options_screen(frame, app);
                match app.current_popup {
                    _ => {},
                }
            },
        }
        
    })?;
    Ok(())
}

fn render_splash_screen(frame: &mut Frame, app: &App) {
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let main_page_style = Style::default()
        .bg(app.general_page_bg_color())
        .fg(app.general_text_color());
    let main_page_content = Paragraph::new(
        "Database terminal app v0.0.1".to_owned()
    ).style(main_page_style);

    frame.render_widget(main_page_content, chunks[0]);

    let info_bits = vec![
        "Commands:",
        "f", " - open file explorer",
        "d", " - open loaded database",
        "n", " - create new database file",
        "o", " - open options",
        "ESC / q", " - quit app",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_file_explorer_screen(frame: &mut Frame, app: &mut App) {
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let fexp_page_style = Style::default()
        .bg(app.general_page_bg_color())
        .fg(app.general_text_color());
    let mut fex_items: Vec<FileExplorerData> = Vec::new();
    fex_items.push(
        FileExplorerData::new("..".to_string(),
        "".to_string(), 
        "".to_string(), 
        true)
    );
    let header = ["File/Folder", "Size", "Date created"]
        .into_iter()
        .map(Cell::from)
        .collect::<Row>()
        .style(fexp_page_style)
        .height(1);
    let rows: Vec<Row> = app.file_explorer_table.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.table_row_normal_col(),
            _ => app.table_row_alt_color(),
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().bg(color).fg(app.general_text_color()))
    }).collect();
    let col_constraints = [
        Constraint::Min(app.file_explorer_table.longest_item_lens.0 + 1),
        Constraint::Length(app.file_explorer_table.longest_item_lens.1 + 1),
        Constraint::Length(app.file_explorer_table.longest_item_lens.2 + 4),
    ];
    let highlight_col = app.file_exp_pg_selected_col();

    render_table(frame, &mut app.file_explorer_table.state, 
                 Some(header), rows, 
                 col_constraints.to_vec(), chunks[0], 
                 highlight_col, Borders::NONE, None);

    render_vertical_scrollbar(frame, chunks[0], None, &mut app.file_explorer_table.scroll_state);

    let info_bits = vec![
        "Commands:",
        "↑ / ↓" , " - navigate",
        "ESC / q", " - return to splash screen",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_database_schema_screen(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let db_name = app.selected_db.as_ref().expect("No DB option found").get_db_name();
    let outer_block = Block::default()
        .title(" Database View")
        .title(Line::from(format!("Currently viewing DATABASE: {}.db ", db_name)).right_aligned())
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);
    let table_column_chunks = get_chunks_from_percentages(inner_area, Direction::Horizontal, vec![50, 50]);

    frame.render_widget(outer_block, chunks[0]);

    render_table_list(frame, app, table_column_chunks[0]);

    render_column_list(frame, app, table_column_chunks[1]);

    let info_bits = vec![
        "Commands:", 
        "↑ / ↓", " - navigate", 
        "Enter" , " - select table",
        "n", " - create new table",
        "d", " - delete table",
        "ESC / q", " - return to splash screen",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_new_database_screen(frame: &mut Frame, app: &mut App) {
    let page_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let insert_text_area_on_style = Style::default().bg(app.text_entry_box_bg_col()).fg(app.general_text_color());
    let form = app.create_db_form.as_mut().unwrap();
    form.set_on_style(insert_text_area_on_style);
    let block = Block::default()
        .title("Creating new database")
        .borders(Borders::ALL)
        .style(page_style);
    let text_area = chunks[0].inner(Margin { horizontal: 1, vertical: 1 });

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(block, chunks[0]);
    frame.render_widget(&form.text_field, text_area);

    if let Some(cursor_pos) = form.text_field.cursor_position(text_area) {
        frame.set_cursor_position(cursor_pos);
    }

    let info_bits = vec![
        "Commands:",
        "CTRL + s", " - create new database",
        "ESC / q", " - return to splash screen"
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_database_table_screen(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());
    let col_name_style = Style::default().fg(app.general_text_color()).add_modifier(Modifier::ITALIC | Modifier::UNDERLINED);
    let metadata_style = Style::default().fg(app.alt_text_color_2()).add_modifier(Modifier::ITALIC);
    let chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![75, 25]);
    let table_name = app.selected_db_table.as_ref().expect("unknown");
    let outer_block = Block::default()
        .title(" Table View")
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);

    frame.render_widget(outer_block, chunks[0]);

    let header_cells: Vec<Cell> = app
        .selected_table_columns
        .iter()
        .map(|col| {
            let line = col.column_name_spans(
                &app.options.display_col_metainfo_in_table_view,
                col_name_style,
                metadata_style,
            );
            Cell::from(line)
        })
        .collect();
    let header = Row::new(header_cells).style(db_page_style);
    let highlight_col = app.file_exp_pg_selected_col();
    let rows: Vec<_> = app.row_list_view.as_ref().unwrap().items.iter().map(|row| {
        let row_cells = row.values.iter().map(|val| Cell::from(val.clone())).collect::<Vec<_>>();
        Row::new(row_cells).style(db_page_style)
    }).collect();
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

    render_table(frame,
        &mut unwrapped_row_list.state,
        Some(header),
        rows,
        col_constraints,
        inner_area,
        highlight_col,
        Borders::ALL,
        Some(table_name.to_string())
    );

    render_vertical_scrollbar(frame, inner_area, None, &mut unwrapped_row_list.scroll_bar_state);

    let info_bits = vec![
        "Commands:", 
        "i", " - create new entry",
        "d", " - delete entry",
        "ESC / b", " - return to database view",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_options_screen(frame: &mut Frame, app: &mut App) {
    let general_text_style = Style::default().fg(app.general_text_color());
    let vertical_chunks = get_chunks_from_percentages(frame.area(), Direction::Vertical, vec![50, 25, 25]);
    let horizontal_chunks = get_chunks_from_percentages(vertical_chunks[1], Direction::Horizontal, vec![50, 50]);

    let color_schemes: &Vec<ColorScheme> = app.list_available_color_schemes();
    let color_scheme_items: Vec<ListItem> = color_schemes.into_iter()
        .map(|scheme| {
            let scheme_name = format!("{:?}", scheme);
            let style = if *scheme == app.options.selected_color_scheme {
                Style::default()
                .fg(app.file_exp_pg_selected_col())
                .add_modifier(Modifier::BOLD)
            } else {
                general_text_style
            };
            ListItem::new(scheme_name).style(style)
        })
        .collect();
    let color_scheme_list = List::new(color_scheme_items)
        .block(Block::default().borders(Borders::ALL).title("Color Schemes"))
        .highlight_style(Style::default().bg(app.general_page_bg_color()));

    frame.render_widget(color_scheme_list, horizontal_chunks[0]);

    render_color_scheme_preview(frame, horizontal_chunks[1], &app.options.selected_color_scheme);

    let table_metainfo_toggle_button = SelectableLine::default(
        "Display column metadata in table view: ",
        app.options.display_col_metainfo_in_table_view,
        matches!(app.options.selected_option, SelectedOption::TableMetainfoToggle),
    );
    let table_metainfo_toggle_area = Rect {
        x: vertical_chunks[0].x + 1,
        y: vertical_chunks[0].y + 10,
        width: 50,
        height: 3,
    };
    let insert_metainfo_toggle_button = SelectableLine::default(
        "Display column metadata in insert view: ",
        app.options.display_col_metainfo_in_insert_view,
        matches!(app.options.selected_option, SelectedOption::InsertMetainfoToggle)
    );
    let insert_metainfo_toggle_area = Rect {
        x: vertical_chunks[0].x + 1,
        y: vertical_chunks[0].y + 12,
        width: 50, 
        height: 3,
    };

    frame.render_widget(table_metainfo_toggle_button, table_metainfo_toggle_area);
    frame.render_widget(insert_metainfo_toggle_button, insert_metainfo_toggle_area);

    let info_bits = vec![
        "Commands: ", 
        "← / →", " - switch between color schemes", 
        "↑ / ↓", " - switch between options",
        "ESC / q" , " - return to splash screen",
    ];
    render_info_paragraph(&info_bits, frame, app, vertical_chunks[2]);
}

fn render_quit_popup(frame: &mut Frame, app: &App) {
    let area = centered_rect(55, 30, frame.area());
    let quit_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let info_bits = vec![
        "", 
        "y", " - quit app", 
        "ESC / n", " - return to splash screen",
    ];
    render_titled_paragraph(frame, app, &info_bits, "Are you sure you want to quit", quit_popup_style, area);
}

fn render_no_db_loaded_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let info_bits = vec![
        "", 
        "CTRL + f", " - open file explorer", 
        "ESC", " - return to splash screen"
    ];
    render_titled_paragraph(frame, app, &info_bits, "No database file loaded", popup_style, area);
}

fn render_insert_row_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 55, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![70, 30]);
    let insert_row_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let metadata_style = Style::default().fg(app.alt_text_color_2()).add_modifier(Modifier::ITALIC);
    let insert_text_area_on_style = Style::default().bg(app.text_entry_box_bg_col()).fg(app.general_text_color());
    let insert_text_area_off_style = Style::default().bg(app.text_entry_box_bg_col()).fg(app.file_exp_pg_selected_col());
    let form = app.table_insert_form.as_mut().unwrap();
    form.set_off_style(insert_text_area_off_style);
    form.set_on_style(insert_text_area_on_style);
    let title_text = format!("Enter new entry into table {}", app.selected_db_table.as_deref().unwrap());
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .title(title_text)
        .style(insert_row_popup_style);

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(popup_block, chunks[0]);

    let text_area = chunks[0].inner(Margin { horizontal: 1, vertical: 1 });

    for (i, col_info) in app.selected_table_columns.iter().enumerate() {
        let x = text_area.x;
        let y = text_area.y + i as u16;

        let mut line = Line::from(vec![Span::raw(col_info.name.clone())]);
        if app.options.display_col_metainfo_in_insert_view {
            line.spans.push(Span::styled(
                format!(" [{}]", col_info.col_type.clone()),
                metadata_style
            ));
        }

        line.spans.push(Span::raw(" "));
        let field = &form.items[i];
        if field.selected {
            line.spans.push(Span::styled(field.text_value.clone(), insert_text_area_on_style))
        } else {
            line.spans.push(Span::styled(field.text_value.clone(), insert_text_area_off_style));
        }

        frame.buffer_mut().set_line(x, y, &line, text_area.width);

        if field.selected {
            if let Some(cursor_pos) = field.cursor_position(Rect {
                x: x + (col_info.col_name_length(&app.options.display_col_metainfo_in_insert_view) + 1) as u16,
                y,
                width: text_area.width,
                height: 1
            }) {
                frame.set_cursor_position(cursor_pos);
            }
        }
    }

    let info_bits = vec![
        "Commands:",
        "CTRL + s", " - save entry",
        "ESC / ALT + q", " - return to database table view",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_insert_table_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 40, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![70, 30]);
    let insert_table_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let insert_text_area_on_style = Style::default().bg(app.text_entry_box_bg_col()).fg(app.general_text_color());
    let insert_text_area_off_style = Style::default().bg(app.text_entry_box_bg_col()).fg(app.file_exp_pg_selected_col());
    let form = app.create_table_form.as_mut().unwrap();
    form.set_on_style(insert_text_area_on_style);
    form.set_off_style(insert_text_area_off_style);
    let title_text = format!("Create new table into database {}", app.selected_db.as_ref().unwrap().get_db_name());
    let create_table_block = Block::default()
        .borders(Borders::ALL)
        .style(insert_table_popup_style)
        .title(title_text);
    let text_area = chunks[0].inner(Margin { horizontal: 1, vertical: 1 });

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(create_table_block, chunks[0]);
    frame.render_widget(&form.text_field, text_area);

    if let Some(cursor_pos) = form.text_field.cursor_position(text_area) {
        frame.set_cursor_position(cursor_pos);
    }

    let info_bits = vec![
        "Commands:",
        "Enter", " - execute sql",
        "ESC", " - return to database schema view",
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_drop_table_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![40, 60]);
    let drop_table_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let text_area_style = Style::default().bg(app.text_entry_box_bg_col()).fg(app.general_text_color());
    let form = app.drop_table_form.as_mut().unwrap();
    form.set_on_style(text_area_style);
    let title_text = format!("Drop table from database {}", app.selected_db.as_ref().unwrap().get_db_name());
    let drop_table_block = Block::default()
        .borders(Borders::ALL)
        .style(drop_table_popup_style)
        .title(title_text);
    let text_area = chunks[0].inner(Margin { horizontal: 1, vertical: 1 });

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(drop_table_block, chunks[0]);
    frame.render_widget(&form.text_field, text_area);

    if let Some(cursor_pos) = form.text_field.cursor_position(text_area) {
        frame.set_cursor_position(cursor_pos);
    }

    let info_bits = vec![
        "Commands:",
        "Enter", " - drop table with given name",
        "ESC", " - return to database schema view",
    ];

    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_delete_row_popup(frame: &mut Frame, app: &mut App) {
    let area = centered_rect(55, 30, frame.area());
    let chunks = get_chunks_from_percentages(area, Direction::Vertical, vec![70, 30]);
    let delete_row_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let delete_text_area_on_style = Style::default()
        .bg(app.text_entry_box_bg_col())
        .fg(app.general_text_color());
    let delete_text_area_off_style = Style::default()
        .bg(app.text_entry_box_bg_col())
        .fg(app.file_exp_pg_selected_col());
    if let Some(form) = &mut app.table_delete_form {
        form.set_on_style(delete_text_area_on_style);
        form.set_off_style(delete_text_area_off_style);
    }
    let title_text = format!("Delete entry from table {}", app.selected_db_table.as_deref().unwrap());
    let delete_form_block = Block::default()
        .borders(Borders::ALL)
        .title(title_text)
        .style(delete_row_popup_style);

    let form_inner = delete_form_block.inner(chunks[0]);
    let form_chunks = get_chunks_from_constraints(
        form_inner,
        Direction::Vertical,
        vec![
            Constraint::Length(3),
            Constraint::Length(3),
        ],
    );

    frame.render_widget(Clear, chunks[0]);
    frame.render_widget(delete_form_block.clone(), chunks[0]);

    if let Some(form) = &app.table_delete_form {
        let col_name_block = Block::default()
            .borders(Borders::ALL)
            .title("Column name");
        frame.render_widget(&form.col_name_entry, col_name_block.inner(form_chunks[0]));
        frame.render_widget(col_name_block, form_chunks[0]);
        
        let value_block = Block::default()
            .borders(Borders::ALL)
            .title("Row value");
        frame.render_widget(&form.row_value_entry, value_block.inner(form_chunks[1]));
        frame.render_widget(value_block, form_chunks[1]);
    }

    let info_bits = vec![
        "Commands:",
        "Enter", " - delete entry with given ID",
        "ESC", " - return to database table view"
    ];
    render_info_paragraph(&info_bits, frame, app, chunks[1]);
}

fn render_error_popup(frame: &mut Frame, app: &mut App, msg: &str) {
    let area = centered_rect(40 , 30, frame.area());
    let error_block = Block::default()
        .borders(Borders::ALL)
        .title("Error")
        .border_style(Style::default().fg(Color::Red))
        .style(Style::default().bg(app.quit_popup_bg_col()).fg(Color::White));
    let mut error_text = Text::default();
    error_text.push_line(Span::raw(msg));
    error_text.push_line(Span::raw("Press ESC to close this popup"));
    let paragraph = Paragraph::new(error_text)
        .block(error_block)
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });

    frame.render_widget(paragraph, area);
}

fn render_table_list(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.table_list_view.is_some(){
        let row_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());
        let header = Row::new(vec![
            Cell::new("Name"),
            Cell::new("Rows"),
            Cell::new("Type"),
        ]).style(row_style);
        let rows: Vec<Row> = app.table_list_view.as_ref().unwrap().items.iter().map(|table| {
            Row::new(vec![
                Cell::from(Text::from(table.name.clone())),
                Cell::from(Text::from(table.row_count.to_string())),
                Cell::from(Text::from(if table.is_view { "View" } else { "Table" })),
            ])
            .style(row_style)
        }).collect();
        let col_constraints = [
            Constraint::Min(15), // table name
            Constraint::Min(7), // row count
            Constraint::Length(7), // type (table, view)
        ];
        let highlight_color = app.file_exp_pg_selected_col();
        let unwrapped_table_list = app.table_list_view.as_mut().unwrap();

        render_table(frame, &mut unwrapped_table_list.state,
            Some(header), rows,
            col_constraints.to_vec(), area,
            highlight_color, Borders::ALL, Some("Tables".to_string()));

        render_vertical_scrollbar(frame, area, None, &mut unwrapped_table_list.scroll_state);
    } else {
        let style = Style::default()
            .bg(app.general_page_bg_color())
            .fg(app.general_text_color());
        let empty_block = Block::default()
            .title("Tables")
            .borders(Borders::ALL);
        let paragraph = Paragraph::new("Empty Schema")
            .block(empty_block)
            .style(style);

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
    }
    
}

fn render_column_list(frame: &mut Frame, app: &mut App, area: Rect) {
    if app.column_list_view.is_some() {
        let header = ["Name", "Type", "Constraints"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default().fg(app.general_text_color()));
        let rows: Vec<Row> = app.column_list_view.as_ref().unwrap().items.iter().map(|col| {
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
        }).collect();
        let col_constraints = [
            Constraint::Min(15),
            Constraint::Length(8),
            Constraint::Min(10),
        ];
        let highlight_color = app.file_exp_pg_selected_col();
        let unwrapped_column_list = app.column_list_view.as_mut().unwrap();

        render_table(frame, &mut unwrapped_column_list.state,
                    Some(header), rows,
                    col_constraints.to_vec(), area,
                    highlight_color, Borders::ALL, Some("Columns".to_string()));

        render_vertical_scrollbar(frame, area, None, &mut unwrapped_column_list.scroll_state);
    } else {
        let style = Style::default()
            .bg(app.general_page_bg_color())
            .fg(app.general_text_color());
        let empty_block = Block::default()
            .title("Columns")
            .borders(Borders::ALL);
        let paragraph = Paragraph::new("No columns")
            .block(empty_block)
            .style(style);

        frame.render_widget(Clear, area);
        frame.render_widget(paragraph, area);
    }
    
}

fn render_color_scheme_preview(frame: &mut Frame, area: Rect, color_scheme: &ColorScheme) {
    let colors = color_scheme.colors();
    let color_vec = vec![
        colors.general_text_color,
        colors.alt_text_color_1,
        colors.alt_text_color_2,
        colors.quit_popup_bg_col,
        colors.general_page_bg_color,
        colors.file_exp_pg_selected_col,
        colors.table_row_normal_col,
        colors.table_row_alt_color,
        colors.info_block_bg_col,
    ];
    let block_width = area.width / color_vec.len() as u16;
    for (i, color) in color_vec.iter().enumerate() {
        let color_area = Rect::new(
            area.width + (i as u16) * block_width,
            area.y, 
            block_width,
            block_width / 2
        );
        frame.render_widget(Block::default().style(Style::default().bg(*color)), color_area);
    }
}

fn render_vertical_scrollbar(frame: &mut Frame, area: Rect, endpoints: Option<&str>, scroll_bar_state: &mut ScrollbarState) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(endpoints)
            .end_symbol(endpoints)
            .style(Style::default()),
        area.inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        scroll_bar_state,
    );
}

fn render_table(frame: &mut Frame, state: &mut TableState, 
                header: Option<Row>, rows: Vec<Row>,
                col_widths: Vec<Constraint>, area: Rect, 
                highlight_col: Color, borders: Borders,
                title: Option<String>) {
    let table_title = title.unwrap_or_default();
    let block_style = Block::new().borders(borders).title(table_title);
    let selected_style = Style::default()
        .bg(highlight_col);
    let mut table = Table::new(
        rows,
        col_widths,
    )
    .block(block_style)
    .row_highlight_style(selected_style)
    .highlight_spacing(HighlightSpacing::Always);

    if let Some(table_header) = header {
        table = table.header(table_header);
    }

    frame.render_stateful_widget(table, area, state);
}

fn centered_rect(percent_x: u16, percent_y: u16, area: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let vertical = Layout::vertical([Constraint::Percentage(percent_y)]).flex(Flex::Center);
    let horizontal = Layout::horizontal([Constraint::Percentage(percent_x)]).flex(Flex::Center);
    let [area] = vertical.areas(area);
    let [area] = horizontal.areas(area);

    area
}

fn get_chunks_from_percentages(area: Rect, direction: Direction, percentages: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = percentages.iter().map(|value| Constraint::Percentage(*value)).collect();
    let chunks = Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area);

    chunks
}

fn get_chunks_from_constraints(area: Rect, direction: Direction, constraints: Vec<Constraint>) -> Rc<[Rect]> {
    let chunks = Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(area);

    chunks
}

fn format_info_text<'a>(text_bits: &'a [&str], app: &App) -> Text<'a> {
    let general_text_style = Style::default().fg(app.general_text_color());
    let alt_text_style_1 = Style::default().fg(app.alt_text_color_1());

    let mut info_text = Text::default();

    for (i, bit) in text_bits.iter().enumerate() {
        if i == 0 {
            info_text = Text::from(Span::styled::<&str, Style>(bit.as_ref(), general_text_style));
        }
        else {
            if i % 2 != 0 {
                info_text.push_line(Span::styled::<&str, Style>(bit.as_ref(), alt_text_style_1));
            }
            else {
                info_text.push_span(Span::styled::<&str, Style>(bit.as_ref(), general_text_style));
            }
        }
    }

    info_text
}

fn compute_col_widths(cols: &[ColumnInfo], rows: &[RowInfo], min: usize, max: usize, display_metainfo: &bool) -> Vec<Constraint> {
    cols.iter().enumerate().map(|(i, col)| {
        let header_len = col.col_name_length(display_metainfo);
        let max_data_len = rows.iter()
            .map(|row| row.values.get(i).map_or(0, |val| val.len()))
            .max()
            .unwrap_or(0);
        let width = header_len.max(max_data_len).clamp(min, max);

        Constraint::Length(width as u16)
    }).collect()
}

fn render_titled_paragraph(frame: &mut Frame, app: &App, info_bits: &[&str], title: &str, style: Style, area: Rect) {
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

fn render_info_paragraph(info_bits : &[&str], frame: &mut Frame, app: & App, area: Rect) {
    let info_style = Style::default()
        .bg(app.info_block_bg_col());

    render_titled_paragraph(frame, app, info_bits, "Info", info_style, area);
}
