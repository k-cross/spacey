//! TUI module for terminal-based user interface
//!
//! This module provides the retro-styled start menu and UI components
//! using ratatui for rendering.

mod app;
mod game;
mod game_ui;
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

use game::GameState;

/// Run the TUI application and return the selected menu action
pub fn run() -> Result<Option<MenuItem>> {
    // Setup terminal
    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let result = run_main_loop(&mut terminal);

    // Restore terminal
    disable_raw_mode()?;
    io::stdout().execute(LeaveAlternateScreen)?;

    result
}

/// Main application loop handling menu and game states
fn run_main_loop<B: Backend>(terminal: &mut Terminal<B>) -> Result<Option<MenuItem>> {
    loop {
        // Run menu and get selection
        let mut app = App::new();
        run_menu(terminal, &mut app)?;

        match app.selected_action() {
            Some(MenuItem::StartGame) => {
                // Run the game
                let mut game = GameState::new();
                run_game(terminal, &mut game)?;
                // Game exited - loop back to menu
            }
            Some(MenuItem::Exit) => {
                return Ok(Some(MenuItem::Exit));
            }
            Some(action) => {
                // Other menu items - for now just return
                return Ok(Some(action));
            }
            None => {
                // User quit with 'q'
                return Ok(None);
            }
        }
    }
}

/// Run the menu loop
fn run_menu<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) -> Result<()> {
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

/// Run the game loop
fn run_game<B: Backend>(terminal: &mut Terminal<B>, game: &mut GameState) -> Result<()> {
    while game.is_running() {
        // Update game state
        game.update();

        // Render
        terminal.draw(|frame| game_ui::render(frame, game))?;

        // Handle input
        if event::poll(std::time::Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Left | KeyCode::Char('a') => game.move_left(),
                        KeyCode::Right | KeyCode::Char('d') => game.move_right(),
                        KeyCode::Up | KeyCode::Char('w') => game.move_up(),
                        KeyCode::Down | KeyCode::Char('s') => game.move_down(),
                        KeyCode::Enter => game.toggle_pause(),
                        KeyCode::Char('q') => game.exit_to_menu(),
                        _ => {}
                    }
                }
            }
        }
    }
    Ok(())
}
