use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Margin, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, HighlightSpacing, List, ListItem, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap},
    Frame,
    Terminal
};
use std::{io, rc::Rc, vec};
use crate::{app::{App, PopUp, Screen}, column::column_info::ColumnInfo, fex::fex_data::FileExplorerData, row::row_info::RowInfo};

use super::{colorscheme::ColorScheme, utils::ToggleButton};


pub fn render<B>(terminal: &mut Terminal<B>, app: &mut App) -> io::Result<()>
where
    B: ratatui::backend::Backend,
{
    // Call terminal.draw and convert its output to io::Result<()>
    terminal.draw(|frame| {
        match app.current_screen {
            Screen::SplashScreenView => {
                match app.current_popup {
                    PopUp::None => render_splash_screen(frame, app),
                    PopUp::QuitDialog => render_quit_dialog(frame, app),
                    PopUp::SaveDialog => {},
                    PopUp::NoDBLoadedDialog => render_no_db_loaded_dialog(frame, app),
                }
            },
            Screen::CreateNewFileView => {},
            Screen::FileExplorerView => {
                match app.current_popup {
                    PopUp::None => render_file_explorer(frame, app),
                    _ => {}
                }
            },
            Screen::OpenDataBaseView => {
                match app.current_popup {
                    PopUp::None => render_database_schema_view(frame, app),
                    PopUp::QuitDialog => todo!(),
                    PopUp::SaveDialog => {},
                    PopUp::NoDBLoadedDialog => {},
                }
            },
            Screen::DataBaseTableView => {
                match app.current_popup {
                    PopUp::None => render_database_table_view(frame, app),
                    _ => {}
                }
            },
            Screen::OptionsView => {
                match app.current_popup {
                    PopUp::None => render_options_view(frame, app),
                    PopUp::SaveDialog => {},
                    PopUp::QuitDialog => {},
                    PopUp::NoDBLoadedDialog => {},
                }
            },
        }
        
    })?;
    Ok(())
}

fn render_splash_screen(frame: &mut Frame, app: &App) {
    let chunks = get_chunks(frame.area(), Direction::Vertical, vec![75, 25]);
    let main_page_style = Style::default()
        .bg(app.general_page_bg_color())
        .fg(app.general_text_color());
    let main_page_content = Paragraph::new(
        "Database terminal app v0.0.1".to_owned()
    ).style(main_page_style);

    frame.render_widget(main_page_content, chunks[0]);

    let text_bits = vec![
        "Commands:",
        "CTRL + f", " - open file explorer",
        "CTRL + d", " - open loaded database",
        "CTRL + n", " - create new database",
        "CTRL + o", " - open options",
        "ESC / CTRL + q", " - quit app",
    ];
    let info_text = format_info_text(&text_bits, app);
    let info_paragraph = Paragraph::new(info_text)
        .wrap(Wrap {trim: true})
        .style(Style::default().bg(app.info_block_bg_col()))
        .block(Block::default()
        .borders(Borders::ALL)
        .title("Info"));

    frame.render_widget(info_paragraph, chunks[1]);
}

fn render_file_explorer(frame: &mut Frame, app: &mut App) {
    let chunks = get_chunks(frame.area(), Direction::Vertical, vec![75, 25]);
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

    let text_bits = vec![
        "Commands:",
        "Arrow Keys" , " - navigate",
        "ESC / q", " - return to splash screen",
    ];
    let info_text = format_info_text(&text_bits, app);
    let info_paragraph = Paragraph::new(info_text)
        .wrap(Wrap {trim: true})
        .style(Style::default().bg(app.info_block_bg_col()))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Info")
        );

    frame.render_widget(info_paragraph, chunks[1]);
}

fn render_database_schema_view(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());
    let chunks = get_chunks(frame.area(), Direction::Vertical, vec![75, 25]);
    let db_name = app.selected_db.as_ref().expect("No DB option found").get_db_name();
    let outer_block = Block::default()
        .title(" Database View")
        .title(Line::from(format!("Currently viewing DATABASE: {}.db ", db_name)).right_aligned())
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);
    let table_column_chunks = get_chunks(inner_area, Direction::Horizontal, vec![50, 50]);

    frame.render_widget(outer_block, chunks[0]);

    render_table_list(frame, app, table_column_chunks[0]);

    render_column_list(frame, app, table_column_chunks[1]);

    let text_bits = vec![
        "Commands:", 
        "Arrow Keys", " - navigate", 
        "Enter" , " - select table", 
        "ESC / q", " - return to splash screen",
    ];
    let info_text = format_info_text(&text_bits, app);
    let info_paragraph = Paragraph::new(info_text)
        .wrap(Wrap {trim: true})
        .style(Style::default().bg(app.info_block_bg_col()))
        .block(Block::default().borders(Borders::ALL).title("Info"));

    frame.render_widget(info_paragraph, chunks[1]);
}

fn render_table_list(frame: &mut Frame, app: &mut App, area: Rect) {
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
}

fn render_column_list(frame: &mut Frame, app: &mut App, area: Rect) {
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
}

fn render_database_table_view(frame: &mut Frame, app: &mut App) {
    let db_page_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());
    let col_name_style = Style::default().fg(app.general_text_color()).add_modifier(Modifier::ITALIC | Modifier::UNDERLINED);
    let metadata_style = Style::default().fg(app.alt_text_color_2()).add_modifier(Modifier::ITALIC);
    let chunks = get_chunks(frame.area(), Direction::Vertical, vec![75, 25]);
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

    render_vertical_scrollbar(frame, inner_area, None, &mut unwrapped_row_list.scroll_state);

    let text_bits = vec![
        "Commands:", 
        "ESC / b", " - return to database view",
    ];
    let info_text = format_info_text(&text_bits, app);
    let info_paragraph = Paragraph::new(info_text)
        .wrap(Wrap {trim: true})
        .style(Style::default().bg(app.info_block_bg_col()))
        .block(Block::default().borders(Borders::ALL).title("Info"));

    frame.render_widget(info_paragraph, chunks[1]);
}

fn render_options_view(frame: &mut Frame, app: &mut App) {
    let general_text_style = Style::default().fg(app.general_text_color());
    let vertical_chunks = get_chunks(frame.area(), Direction::Vertical, vec![50, 25, 25]);
    let horizontal_chunks = get_chunks(vertical_chunks[1], Direction::Horizontal, vec![50, 50]);

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

    let toggle_button = ToggleButton {
        label: if app.options.display_col_metainfo_in_table_view {
            "Display column metadata in table view: ON"
        } else {
            "Display column metadata in table view: OFF"
        },
        active: app.options.display_col_metainfo_in_table_view,
        selected: true,
        on_style: Style::default().fg(Color::Green),
        off_style: Style::default().fg(Color::Red),
        selected_border_style: Style::default().fg(Color::Yellow),
    };
    let button_area = Rect {
        x: vertical_chunks[0].x + 1,
        y: vertical_chunks[0].y + 10,
        width: 50,
        height: 3,
    };

    frame.render_widget(toggle_button, button_area);

    let text_bits = vec![
        "Commands: ", 
        "Arrow Keys", " - navigate", 
        "ESC / q" , " - return to splash screen",
    ];
    let info_text = format_info_text(&text_bits, app);
    let info_paragraph = Paragraph::new(info_text)
        .wrap(Wrap {trim: true})
        .style(Style::default().bg(app.info_block_bg_col()))
        .block(Block::default()
            .borders(Borders::ALL)
            .title("Info")
        );

    frame.render_widget(info_paragraph, vertical_chunks[2]);
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
            area.width + (i + 1) as u16 * block_width,
            area.y, 
            block_width,
            block_width / 2
        );
        frame.render_widget(Block::default().style(Style::default().bg(*color)), color_area);
    }
}

fn render_quit_dialog(frame: &mut Frame, app: &App) {
    frame.render_widget(Clear, frame.area());

    let quit_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let popup_block = Block::default()
        .title("Are you sure you want to quit?")
        .borders(Borders::ALL)
        .style(quit_popup_style);
    let text_bits = vec![
        "", 
        "y", " - quit app", 
        "ESC / n", " - return to splash screen",
    ];
    let exit_text = format_info_text(&text_bits, app);
    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });
    let area = centered_rect(55, 30, frame.area());

    frame.render_widget(exit_paragraph, area);
}

fn render_no_db_loaded_dialog(frame: &mut Frame, app: &mut App) {
    let popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.general_text_color());
    let popup_block = Block::default()
        .borders(Borders::ALL)
        .style(popup_style);
    let text_bits = vec![
        "No database file loaded!", 
        "CTRL + f", " - open file explorer", 
        "ESC", " - return to splash screen"
    ];
    let info_text = format_info_text(&text_bits, app);
    let info_paragraph = Paragraph::new(info_text)
        .block(popup_block)
        .wrap(Wrap { trim: false } );
    let area = centered_rect(55, 30, frame.area());

    frame.render_widget(info_paragraph, area);
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

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(layout[1])[1]
}

fn get_chunks(area: Rect, direction: Direction, percentages: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = percentages.iter().map(|value| Constraint::Percentage(*value)).collect();
    let chunks = Layout::default()
        .direction(direction)
        .constraints(
            constraints
        )
        .split(area);

    chunks
}

fn format_info_text<'a>(text_bits: &'a Vec<&'a str>, app: &App) -> Text<'a> {
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
