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

use color_eyre::Result;
use crossterm::{
    ExecutableCommand,
    event::{
        self, Event, KeyCode, KeyEventKind, KeyboardEnhancementFlags, PopKeyboardEnhancementFlags,
        PushKeyboardEnhancementFlags,
    },
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::prelude::*;
use std::io;

use game::GameState;

/// Run the TUI application and return the selected menu action
pub fn run() -> Result<Option<MenuItem>> {
    // Setup terminal
    io::stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;

    // Enable advanced keyboard events (required for KeyRelease events on many terminals)
    // Ignore errors for terminals that don't support it
    let _ = io::stdout().execute(PushKeyboardEnhancementFlags(
        KeyboardEnhancementFlags::REPORT_EVENT_TYPES,
    ));

    let mut terminal = Terminal::new(CrosstermBackend::new(io::stdout()))?;
    terminal.clear()?;

    let result = run_main_loop(&mut terminal);

    // Restore terminal
    let _ = io::stdout().execute(PopKeyboardEnhancementFlags);
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

        if event::poll(std::time::Duration::from_millis(100))?
            && let Event::Key(key) = event::read()?
            && key.kind == KeyEventKind::Press
        {
            match key.code {
                KeyCode::Char('q') => app.quit(),
                KeyCode::Up | KeyCode::Char('k') => app.previous(),
                KeyCode::Down | KeyCode::Char('j') => app.next(),
                KeyCode::Enter => app.select(),
                _ => {}
            }
        }
    }
    Ok(())
}

/// Run the game loop
fn run_game<B: Backend>(terminal: &mut Terminal<B>, game: &mut GameState) -> Result<()> {
    const TICK_RATE: f32 = 1.0 / 60.0;
    let mut accumulator = 0.0;
    let mut last_time = std::time::Instant::now();

    while game.is_running() {
        let current_time = std::time::Instant::now();
        let frame_time = current_time.duration_since(last_time).as_secs_f32();
        last_time = current_time;

        let frame_time = frame_time.min(0.25);
        accumulator += frame_time;

        if event::poll(std::time::Duration::from_millis(0))? {
            loop {
                if let Event::Key(key) = event::read()? {
                    match key.kind {
                        KeyEventKind::Press => match key.code {
                            KeyCode::Left | KeyCode::Char('a') => game.moving_left = true,
                            KeyCode::Right | KeyCode::Char('d') => game.moving_right = true,
                            KeyCode::Up | KeyCode::Char('w') => game.moving_up = true,
                            KeyCode::Down | KeyCode::Char('s') => game.moving_down = true,
                            KeyCode::Char(' ') => game.firing = true,
                            KeyCode::Enter => game.toggle_pause(),
                            KeyCode::Char('q') => game.exit_to_menu(),
                            _ => {}
                        },
                        KeyEventKind::Release => match key.code {
                            KeyCode::Left | KeyCode::Char('a') => game.moving_left = false,
                            KeyCode::Right | KeyCode::Char('d') => game.moving_right = false,
                            KeyCode::Up | KeyCode::Char('w') => game.moving_up = false,
                            KeyCode::Down | KeyCode::Char('s') => game.moving_down = false,
                            KeyCode::Char(' ') => game.firing = false,
                            _ => {}
                        },
                        _ => {}
                    }
                }

                if !event::poll(std::time::Duration::from_millis(0))? {
                    break;
                }
            }
        }

        while accumulator >= TICK_RATE {
            game.update();
            accumulator -= TICK_RATE;
        }

        let alpha = accumulator / TICK_RATE;
        terminal.draw(|frame| game_ui::render(frame, game, alpha))?;
    }
    Ok(())
}
