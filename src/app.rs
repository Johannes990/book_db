use std::io;

use ratatui::{
    crossterm::event::{self, KeyEvent, KeyEventKind, KeyCode},
    style::Stylize,
    widgets::Paragraph,
    DefaultTerminal
};

pub fn run(mut terminal: DefaultTerminal) -> io::Result<()> {
    loop {
        terminal.draw(|frame| {
            let greeting = Paragraph::new("Hello my new db terminal app! (press 'q' to quit)")
            .white()
            .on_cyan();
        frame.render_widget(greeting, frame.area());
        })?;

        if let event::Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press && (key.code == KeyCode::Char('q') || key.code == KeyCode::Char('Q')) {
                return Ok(())
            }
        }
    }
}