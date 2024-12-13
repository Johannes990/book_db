use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Margin, Rect},
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, 
        HighlightSpacing, List, ListItem, Paragraph, 
        Row, Scrollbar, ScrollbarOrientation, Table, 
        TableState, Wrap},
    Frame,
    Terminal
};
use std::{io, rc::Rc, vec};
use crate::{app::{App, PopUp, Screen}, fex::fexdata::FileExplorerData};


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
                    PopUp::QuitDialog => todo!()
                }
            },
            Screen::OptionsView => {},
        }
        
    })?;
    Ok(())
}

fn render_splash_screen(frame: &mut Frame, app: &App) {
    let chunks = get_chunks(frame, Direction::Vertical, vec![75, 25]);
    let main_page_style = Style::default()
        .bg(app.general_page_bg_color())
        .fg(app.general_text_color());
    let main_page_content = Paragraph::new(
        "Database terminal app v0.0.1".to_owned()
    ).style(main_page_style);

    frame.render_widget(main_page_content, chunks[0]);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("f", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - open file explorer to load existing database file, ", Style::default().fg(app.general_text_color())),
        Span::styled("c", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - create new database file, ", Style::default().fg(app.general_text_color())),
        Span::styled("o", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - open options page, ", Style::default().fg(app.general_text_color())),
        Span::styled("ESC / q", Style::default().fg(app.alt_text_color_1())),
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
    let chunks = get_chunks(frame, Direction::Vertical, vec![75, 25]);
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

    render_table(frame, &mut app.file_explorer_table.state, header, rows, col_constraints.to_vec(), chunks[0], highlight_col);
    render_vertical_scrollbar(frame, chunks[0], app, None);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", Style::default().fg(app.general_text_color())),
        Span::styled("Up / Down", Style::default().fg(app.alt_text_color_1())),
        Span::styled(" - Navigate, ", Style::default().fg(app.general_text_color())),
        Span::styled("Esc", Style::default().fg(app.alt_text_color_1())),
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
    let alt_text_style_2 = Style::default().fg(app.alt_text_color_2());
    let chunks = get_chunks(frame, Direction::Vertical, vec![75, 25]);
    let db_name = Option::expect(app.selected_db.as_ref(), "No DB Option found").get_db_name();
    let db_page_style = Style::default()
        .bg(app.general_page_bg_color())
        .fg(app.general_text_color());
    let outer_block = Block::bordered().title("DB view").style(db_page_style);
    let inner_block = Block::bordered().title("TABLES").style(db_page_style);
    let inner_area = outer_block.inner(chunks[0]);
    let table_names: Vec<String> = app.get_db().get_table_list().unwrap();
    let table_names_content: Vec<ListItem> = table_names
        .into_iter()
        .map(|table_name| ListItem::from(format!("{}\n", table_name)))
        .collect();
    let table_names_paragraph = List::new(table_names_content).style(db_page_style).block(inner_block);
    let text = Paragraph::new(Line::from(vec![
        Span::styled("Selected Database file: ", general_text_style),
        Span::styled(format!("{}.db", db_name), alt_text_style_2),
    ]))
    .style(db_page_style)
    .block(outer_block);

    frame.render_widget(text, chunks[0]);
    frame.render_widget(table_names_paragraph, inner_area);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", general_text_style),
        Span::styled("Esc / q", alt_text_style_1),
        Span::styled(" - Back to splash screen", general_text_style),
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
        .borders(Borders::ALL)
        .title("Info")
    );

    frame.render_widget(info_text, chunks[1]);
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

fn render_vertical_scrollbar(frame: &mut Frame, area: Rect, app: &mut App, endpoints: Option<&str>) {
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
        &mut app.file_explorer_table.scroll_state,
    );

}

fn render_table(frame: &mut Frame, state: &mut TableState, header: Row, rows: Vec<Row>, col_widths: Vec<Constraint>, area: Rect, highlight_col: Color) {
    let selected_style = Style::default()
        .bg(highlight_col);
    let table = Table::new(
        rows,
        col_widths,
    )
    .header(header)
    .row_highlight_style(selected_style)
    .highlight_spacing(HighlightSpacing::Always);
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

fn get_chunks(frame: &Frame, direction: Direction, percentages: Vec<u16>) -> Rc<[Rect]> {
    let constraints: Vec<Constraint> = percentages.iter().map(|value| Constraint::Percentage(*value)).collect();
    let chunks = Layout::default()
        .direction(direction)
        .constraints(
            constraints
        )
        .split(frame.area());

    chunks
}
