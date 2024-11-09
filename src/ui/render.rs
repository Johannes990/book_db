use ratatui::{
    prelude::Rect,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph, Wrap},
    text::{Line, Span},
    Frame,
    Terminal
};
use std::{io, rc::Rc};
use crate::app::{App, PopUp, Screen};


pub fn render<B>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()>
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
        Span::styled("f", Style::default().fg(Color::Cyan)),
        Span::raw(" - open file explorer to load existing database file, "),
        Span::styled("c", Style::default().fg(Color::Cyan)),
        Span::raw(" - create new database file, "),
        Span::styled("o", Style::default().fg(Color::Cyan)),
        Span::raw(" - open options page, "),
        Span::styled("ESC / q", Style::default().fg(Color::Cyan)),
        Span::raw(" - quit app.")
    ]))
    .block(Block::default().borders(Borders::ALL). title("Info"));

frame.render_widget(info_text, chunks[1]);
}

fn render_file_explorer(frame: &mut Frame, app: &App) {
    let chunks = get_vertical_chunks(frame, 75);

    let mut items: Vec<ListItem> = Vec::new();
    let parent_style = if app.selected_index == 0 {
        Style::default().bg(Color::LightYellow).fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Yellow)
    };
    items.push(ListItem::new("..").style(parent_style));

    let visible_files = app.file_list.iter().skip(app.scroll_offset).enumerate();
    
    for (i, (file, is_dir)) in visible_files {
        let actual_idx = i + app.scroll_offset + 1;

        let style = if *is_dir && actual_idx == app.selected_index {
            Style::default().bg(Color::LightYellow).fg(Color::Red)
        } else if *is_dir {
            Style::default().fg(Color::Red)
        } else if actual_idx == app.selected_index {
            Style::default().bg(Color::LightYellow).fg(Color::Gray)
        } else {
            Style::default().fg(Color::Gray)
        };

        items.push(ListItem::new(file.clone()).style(style));
    }

    let list_widget = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("File Explorer"));
    
    frame.render_widget(list_widget, chunks[0]);

    let info_text = Paragraph::new(Line::from(vec![
        Span::raw("Commands: "),
        Span::styled("Up / Down", Style::default().fg(Color::Cyan)),
        Span::raw(" - Navigate, "),
        Span::styled("Esc", Style::default().fg(Color::Cyan)),
        Span::raw(" - Back to splash screen"),
    ]))
    .block(Block::default().borders(Borders::ALL). title("Info"));

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
        Span::styled("y", Style::default().fg(Color::Cyan)),
        Span::raw(" to quit, "),
        Span::styled("Esc / n", Style::default().fg(Color::Cyan)),
        Span::raw(" to return to main window"),
    ]);

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(55, 30, frame.area());

    frame.render_widget(exit_paragraph, area);
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
