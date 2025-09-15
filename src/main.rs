mod app;
mod column;
mod db;
mod file_explorer;
mod lang;
mod log;
mod options;
mod perf;
mod row;
mod table;
mod threading;
mod ui;
mod widgets;

use app::App;
use crossterm::{
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    prelude::{Backend, CrosstermBackend},
    Terminal,
};
use rusqlite::Result;
use std::{io, sync::mpsc::Receiver};
use ui::events::handle_key_events;

use crate::{
    perf::resources::Resources, threading::spawn_profiler_thread,
    ui::colors::static_colors::StaticColors,
};

fn main() -> io::Result<()> {
    let qualifier = "".to_string();
    let organization = "JohannesCorp".to_string();
    let application = "Libry".to_string();

    let mut stdout = std::io::stdout();

    let _ = execute!(
        stdout,
        PushKeyboardEnhancementFlags(
            KeyboardEnhancementFlags::DISAMBIGUATE_ESCAPE_CODES
                | KeyboardEnhancementFlags::REPORT_ALL_KEYS_AS_ESCAPE_CODES
        )
    );

    let profiler_rx = spawn_profiler_thread();

    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = setup_terminal(backend)?;
    let default_color_scheme = StaticColors::SaturatedSummer;
    let mut app = setup_app(
        &terminal,
        qualifier,
        organization,
        application,
        default_color_scheme,
        profiler_rx,
    )?;
    let res = app.run(&mut terminal);
    handle_errors(res);
    teardown_terminal(&mut terminal)?;

    let _ = execute!(stdout, PopKeyboardEnhancementFlags);

    Ok(())
}

fn setup_terminal<B>(mut backend: B) -> Result<Terminal<B>, io::Error>
where
    B: Backend + std::io::Write,
{
    enable_raw_mode()?;
    execute!(backend, EnterAlternateScreen)?;
    let terminal = Terminal::new(backend)?;

    Ok(terminal)
}

fn setup_app<B>(
    terminal: &Terminal<B>,
    qual_str: String,
    org_str: String,
    app_str: String,
    color_scheme: StaticColors,
    profiler_rx: Receiver<Resources>,
) -> Result<App, io::Error>
where
    B: Backend,
{
    let _terminal_height = terminal.size()?.height;
    let _terminal_width = terminal.size()?.width;
    let mut app = App::new(qual_str, org_str, app_str, color_scheme)?;

    app.set_profiler_rx(profiler_rx);

    Ok(app)
}

fn teardown_terminal<B>(terminal: &mut Terminal<B>) -> Result<(), io::Error>
where
    B: Backend + std::io::Write,
{
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn handle_errors(res: io::Result<()>) {
    if let Err(err) = res {
        eprintln!("Error: {:?}", err)
    }
}
