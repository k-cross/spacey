# Spacey - AI Agent Documentation

This document provides context and guidelines for AI agents working on the Spacey codebase.

## Project Architecture

Spacey is a Rust-based TUI space shooter using `ratatui` for rendering and `crossterm` for terminal handling.

### Module Structure (`src/tui/`)

- **`mod.rs`**: Entry point. Manages the main event loop and transitions between `App` (Menu) and `Game` states.
- **`app.rs`**: Handles the Start Menu state, including selection logic.
- **`menu.rs`**: Defines `MenuItem` enums and labels.
- **`ui.rs`**: Renders the Start Menu (ASCII title, options).
- **`game.rs`**: Core game logic.
  - `GameState`: Struct holding ship position (`ship_x`, `ship_y`), score, and flags (`paused`, `should_exit`).
  - `update()`: called every frame to advance animation time.
- **`game_ui.rs`**: Renders the Game Screen.
  - Uses a calculated perspective grid to simulate forward motion.
  - `render_pause_overlay()`: Draws over the game when `paused` is true.

## Design Patterns

- **State Separation**: Menu and Game are distinct states managed by separate loops in `mod.rs`.
- **Immediate Mode Rendering**: The UI is redrawn every frame based on the current state.
- **Phosphor Aesthetics**: Use greens (`Color::Rgb(0, 200, 0)`) for that retro CRT look.

## Testing

- Run `cargo test` to verify logic.
- Visual verification via `cargo run` is often necessary for TUI changes.

## Future Plans

- **Entities**: Enemy logic will need to be added to `GameState`.
- **WASM**: Future web target will likely require abstracting the rendering backend further.
