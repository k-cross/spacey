//! TUI module for terminal-based user interface
//!
//! This module provides the retro-styled start menu and UI components
//! using ratatui for rendering.

mod app;
mod menu;
mod ui;

pub use app::App;
pub use menu::MenuItem;

use std::io;
use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::prelude::*;

/// Run the TUI application and return the selected menu action
pub fn run() -> Result<Option<MenuItem>> {
    // Setup terminal
    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let mut app = App::new();
    let result = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result.map(|_| app.selected_action())
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
    while app.is_running() {
        terminal.draw(|frame| ui::render(frame, app))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => app.quit(),
                        KeyCode::Up | KeyCode::Char('k') => app.previous(),
                        KeyCode::Down | KeyCode::Char('j') => app.next(),
                        KeyCode::Enter => app.select(),
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
