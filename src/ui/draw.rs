use ratatui::{
    layout::{Layout, Constraint, Direction},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    style::{Color, Style, Stylize},
    text::Text,
    Frame,
    Terminal,
};
use std::io;

use crate::ui::app::{App, PopUp};

pub fn draw<B>(terminal: &mut Terminal<B>, app: &App) -> io::Result<()>
where
    B: ratatui::backend::Backend,
{
    // Call terminal.draw and convert its output to io::Result<()>
    terminal.draw(|frame| {
        match app.current_popup {
            PopUp::None => draw_main_view(frame, app),
            PopUp::QuitDialog => draw_quit_dialog(frame),
        }
    })?;
    Ok(())
}

fn draw_main_view(frame: &mut Frame, app: &App) {
    let greeting = Paragraph::new("Hello and welcome to this initial page of my terminal db app! Press 'q' to quit.")
        .white()
        .on_cyan();
    frame.render_widget(greeting, frame.area());
}

fn draw_quit_dialog(frame: &mut Frame) {
    frame.render_widget(Clear, frame.area());
    let popup_block = Block::default()
        .title("Are you sure you want to quit?")
        .borders(Borders::ALL)
        .style(Style::new().red())
        .bg(Color::Black);

    let exit_text = Text::styled(
        "Press 'y' or 'Y' to quit, 'n' or 'N' to return to main window.",
        Style::new().fg(Color::Red),
    );

    let exit_paragraph = Paragraph::new(exit_text)
        .block(popup_block)
        .wrap(Wrap { trim: false });

    let area = centered_rect(55, 30, frame.area());

    frame.render_widget(exit_paragraph, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
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
        .split(popup_layout[1])[1]
}
