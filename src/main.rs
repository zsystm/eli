// src/main.rs

mod rpc;
mod app;
mod events;
mod ui;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyEvent},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use app::{App, AppMode};
use events::{handle_main_mode, handle_param_input_mode, handle_history_mode};
use ui::draw_ui;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize terminal
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // create app state
    let mut app = App::new();

    // main event loop
    loop {
        terminal.draw(|f| draw_ui(f, &mut app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match app.mode {
                    AppMode::Main       => handle_main_mode(&mut app, key).await,
                    AppMode::ParamInput => handle_param_input_mode(&mut app, key).await,
                    AppMode::History    => handle_history_mode(&mut app, key).await,
                }
            }
        }
        if app.should_quit {
            break;
        }
    }

    // restore terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;
    Ok(())
}
