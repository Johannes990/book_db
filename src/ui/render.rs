use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::{Margin, Rect},
    style::Style,
    text::{Line, Span, Text},
    widgets::{Block, Borders, Cell, Clear, HighlightSpacing, 
        Paragraph, Row, Scrollbar, ScrollbarOrientation, 
        Table, Wrap},
    Frame,
    Terminal
};
use std::{io, rc::Rc};
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
                    PopUp::None => {
                        render_file_explorer(frame, app);
                        render_scrollbar(frame, app);
                    }
                    _ => {}
                }
            },
            Screen::OptionsView => {},
        }
        
    })?;
    Ok(())
}

fn render_splash_screen(frame: &mut Frame, app: &App) {
    let chunks = get_vertical_chunks(frame, 75);

    let main_page_style = Style::default()
        .bg(app.main_pg_bg_col())
        .fg(app.main_pg_txt_col());

    let main_page_content = Paragraph::new(
        "Database terminal app v0.0.1".to_owned()
    ).style(main_page_style);

    frame.render_widget(main_page_content, chunks[0]);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("f", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - open file explorer to load existing database file, ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("c", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - create new database file, ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("o", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - open options page, ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("ESC / q", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - quit app.", Style::default().fg(app.info_block_txt_col()))
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
    .borders(Borders::ALL)
    .title("Info"));

frame.render_widget(info_text, chunks[1]);
}

/*
fn render_file_explorer(frame: &mut Frame, app: &App) {
    let chunks = get_vertical_chunks(frame, 75);

    let file_explorer_page_style = Style::default()
        .bg(app.file_exp_pg_bg_color())
        .fg(app.file_exp_pg_txt_color());

    let mut fex_items: Vec<ListItem> = Vec::new();
    let parent_folder_style = if app.selected_index == 0 {
        Style::default()
            .bg(app.file_exp_pg_selected_col())
            .fg(app.file_exp_pg_parent_folder_col())
    } else {
        Style::default()
            .fg(app.file_exp_pg_parent_folder_col())
    };
    fex_items
        .push(ListItem::new("..")
        .style(parent_folder_style));

    let visible_files = app.file_list.iter().skip(app.scroll_offset).enumerate();
    
    for (i, (file, is_dir)) in visible_files {
        let actual_idx = i + app.scroll_offset + 1;

        let style = if *is_dir && actual_idx == app.selected_index {
            Style::default()
                .bg(app.file_exp_pg_selected_col())
                .fg(app.file_exp_pg_folder_col())
        } else if *is_dir {
            Style::default()
                .fg(app.file_exp_pg_folder_col())
        } else if actual_idx == app.selected_index {
            Style::default()
                .bg(app.file_exp_pg_selected_col())
                .fg(app.file_exp_pg_file_color())
        } else {
            Style::default()
                .fg(app.file_exp_pg_file_color())
        };

        fex_items.push(ListItem::new(file.clone()).style(style));
    }

    let list_widget = List::new(fex_items)
        .style(file_explorer_page_style)
        .block(Block::default()
        .borders(Borders::ALL)
        .title("File Explorer"));
    
    frame.render_widget(list_widget, chunks[0]);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("Up / Down", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - Navigate, ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("Esc", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - Back to splash screen", Style::default().fg(app.info_block_txt_col())),
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
    .borders(Borders::ALL)
    .title("Info"));

    frame.render_widget(info_text, chunks[1]);
}
    */

fn render_file_explorer(frame: &mut Frame, app: &mut App) {
    let chunks = get_vertical_chunks(frame, 75);
    let fexp_page_style = Style::default()
        .bg(app.file_exp_pg_bg_color())
        .fg(app.file_exp_pg_txt_color());
    let selected_style = Style::default()
        .bg(app.file_exp_pg_selected_col());
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
    let visible_files: Vec<Row> = app.file_explorer_table.items.iter().enumerate().map(|(i, data)| {
        let color = match i % 2 {
            0 => app.table_row_normal_col(),
            _ => app.table_row_alt_color(),
        };
        let item = data.ref_array();
        item.into_iter()
            .map(|content| Cell::from(Text::from(format!("{content}"))))
            .collect::<Row>()
            .style(Style::new().bg(color).fg(app.file_exp_pg_file_color()))
    }).collect();
    let table = Table::new(
        visible_files,
        [
            Constraint::Min(app.file_explorer_table.longest_item_lens.0 + 1),
            Constraint::Length(app.file_explorer_table.longest_item_lens.1 + 1),
            Constraint::Length(app.file_explorer_table.longest_item_lens.2 + 4),
        ],
    )
    .header(header)
    .row_highlight_style(selected_style)
    .highlight_spacing(HighlightSpacing::Always);
    frame.render_stateful_widget(table, chunks[0], &mut app.file_explorer_table.state);

    let info_text = Paragraph::new(Line::from(vec![
        Span::styled("Commands: ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("Up / Down", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - Navigate, ", Style::default().fg(app.info_block_txt_col())),
        Span::styled("Esc", Style::default().fg(app.info_block_txt_highlight_col())),
        Span::styled(" - Back to splash screen", Style::default().fg(app.info_block_txt_col())),
    ]))
    .wrap(Wrap {trim: true})
    .style(Style::default().bg(app.info_block_bg_col()))
    .block(Block::default()
    .borders(Borders::ALL)
    .title("Info"));

    frame.render_widget(info_text, chunks[1]);
}

fn render_quit_dialog(frame: &mut Frame, app: &App) {
    frame.render_widget(Clear, frame.area());

    let quit_popup_style = Style::default()
        .bg(app.quit_popup_bg_col())
        .fg(app.quit_popup_txt_col());

    let popup_block = Block::default()
        .title("Are you sure you want to quit?")
        .borders(Borders::ALL)
        .style(quit_popup_style);

    
    let exit_text = Line::from(vec![
        Span::raw("Press "),
        Span::styled("y", Style::default().fg(app.quit_popup_txt_highlight_col())),
        Span::raw(" to quit, "),
        Span::styled("Esc / n", Style::default().fg(app.quit_popup_txt_highlight_col())),
        Span::raw(" to return to main window"),
    ]);

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(55, 30, frame.area());

    frame.render_widget(exit_paragraph, area);
}

fn render_scrollbar(frame: &mut Frame, app: &mut App) {
    frame.render_stateful_widget(
        Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .begin_symbol(None)
            .end_symbol(None),
        frame.area().inner(Margin {
            vertical: 1,
            horizontal: 1,
        }),
        &mut app.file_explorer_table.scroll_state,
    );

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

fn get_vertical_chunks(frame: &Frame, percent_y: u16) -> Rc<[Rect]> {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(percent_y),
            Constraint::Percentage(100 - percent_y),
        ])
        .split(frame.area());

    chunks
}
