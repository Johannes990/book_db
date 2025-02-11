use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Margin, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, HighlightSpacing, List, ListItem, Paragraph, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState, Wrap},
    Frame,
    Terminal
};
use std::{io, rc::Rc, vec};
use crate::{app::{App, PopUp, Screen}, fex::fex_data::FileExplorerData, options};

use super::colorscheme::ColorScheme;


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
                    PopUp::None => render_database_view(frame, app),
                    PopUp::QuitDialog => todo!(),
                    PopUp::SaveDialog => {},
                    PopUp::NoDBLoadedDialog => {},
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

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("CTRL + f", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - open file explorer to load existing database file, ", Style::default().fg(app.general_text_color())),
        Span::styled("CTRL + d", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - open loaded database file, ", Style::default().fg(app.general_text_color())),
        Span::styled("CTRL + n", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - create new database file, ", Style::default().fg(app.general_text_color())),
        Span::styled("CTRL + o", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - open options page, ", Style::default().fg(app.general_text_color())),
        Span::styled("ESC / CTRL + q", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - quit app.", Style::default().fg(app.general_text_color()))
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
    .borders(Borders::ALL)
    .title("Info"));

frame.render_widget(info_text, chunks[1]);
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

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", Style::default().fg(app.general_text_color())),
        Span::styled("Up / Down", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - Navigate, ", Style::default().fg(app.general_text_color())),
        Span::styled("Esc / q", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - Back to splash screen", Style::default().fg(app.general_text_color())),
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
        .borders(Borders::ALL)
        .title("Info")
    );

    frame.render_widget(info_text, chunks[1]);
}

fn render_database_view(frame: &mut Frame, app: &mut App) {
    let general_text_style = Style::default().fg(app.general_text_color());
    let alt_text_style_1 = Style::default().fg(app.alt_text_color_1());
    let db_page_style = Style::default().bg(app.general_page_bg_color()).fg(app.general_text_color());

    let chunks = get_chunks(frame.area(), Direction::Vertical, vec![75, 25]);

    let db_name = app.selected_db.as_ref().expect("No DB option found").get_db_name();
    let outer_block = Block::default()
        .title("Database View")
        .title(Line::from(format!("Currently viewing: {}.db", db_name)).right_aligned())
        .style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);
    let table_column_chunks = get_chunks(inner_area, Direction::Horizontal, vec![35, 65]);

    frame.render_widget(outer_block, chunks[0]);

    render_table_list(frame, app, table_column_chunks[0]);

    render_column_list(frame, app, table_column_chunks[1]);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", general_text_style),
        Span::styled("Arrow keys", alt_text_style_1),
        Span::styled(" - Navigate tables, ", general_text_style),
        Span::styled("Enter", alt_text_style_1),
        Span::styled(" - Select table, ", general_text_style),
        Span::styled("Esc / q", alt_text_style_1),
        Span::styled(" - Back to splash screen", general_text_style),
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default().borders(Borders::ALL).title("Info"));

    frame.render_widget(info_text, chunks[1]);
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
        Constraint::Percentage(50), // table name
        Constraint::Percentage(25), // row count
        Constraint::Percentage(25), // type (table, view)
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
            Cell::from(Text::from(col.name.clone())),
            Cell::from(Text::from(col.col_type.clone())),
            Cell::from(Text::from(col_constraint_text)),
        ])
    }).collect();

    let col_constraints = [
        Constraint::Percentage(35),
        Constraint::Percentage(15),
        Constraint::Percentage(50),
    ];
    let highlight_color = app.file_exp_pg_selected_col();

    let unwrapped_column_list = app.column_list_view.as_mut().unwrap();

    render_table(frame, &mut unwrapped_column_list.state,
                 Some(header), rows,
                 col_constraints.to_vec(), area,
                 highlight_color, Borders::ALL, Some("Columns".to_string()));

    render_vertical_scrollbar(frame, area, None, &mut unwrapped_column_list.scroll_state);
}

fn render_options_view(frame: &mut Frame, app: &mut App) {
    let general_text_style = Style::default().fg(app.general_text_color());
    let alt_text_style_1 = Style::default().fg(app.alt_text_color_1());
    let chunks = get_chunks(frame.area(), Direction::Horizontal, vec![33, 33, 34]);
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
    frame.render_widget(color_scheme_list, chunks[0]);
    options::Options::render_color_scheme_preview(frame, chunks[1], &app.options.selected_color_scheme);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", general_text_style),
        Span::styled("Up / Down", alt_text_style_1),
        Span::styled(" - Navigate, ", general_text_style),
        Span::styled("Esc / q", alt_text_style_1),
        Span::styled(" - Back to splash screen", general_text_style),
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
        .borders(Borders::ALL)
        .title("Info")
    );

    frame.render_widget(info_text, chunks[2]);
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
    let exit_text = Line::from(vec![
        Span::styled("Press ", Style::default().fg(app.general_text_color())),
        Span::styled("y", Style::default().fg(app.alt_text_color_2())),
        Span::styled(" to quit, ", Style::default().fg(app.general_text_color())),
        Span::styled("Esc / n", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" to return to main window", Style::default().fg(app.general_text_color())),
    ]);
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
        .title("No database file loaded!")
        .borders(Borders::ALL)
        .style(popup_style);
    let info_text = Line::from(vec![
        Span::styled("No database file has been loaded. ", Style::default().fg(app.general_text_color())),
        Span::styled("CTRL + f", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - open file explorer to load a .db file, ", Style::default().fg(app.general_text_color())),
        Span::styled("Esc", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - return to splash screen.", Style::default().fg(app.general_text_color())),
    ]);
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
