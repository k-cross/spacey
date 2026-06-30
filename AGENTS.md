# Spacey - AI Agent Documentation

This document provides context and guidelines for AI agents working on the Spacey codebase.

## Project Architecture

Spacey is a Rust-based TUI space shooter using `ratatui` for rendering and `crossterm` for terminal handling.

### Module Structure (`src/tui/`)

- **`mod.rs`**: Entry point. Manages the main event loop (using an Accumulator pattern for fixed physics timestep) and transitions between `App` (Menu), `Game`, and `Leaderboard` states.
- **`app.rs`**: Handles the Start Menu state, including selection logic.
- **`menu.rs`**: Defines `MenuItem` enums and labels.
- **`ui.rs`**: Renders the Start Menu (ASCII title, options).
- **`game.rs`**: Core game logic. Implements a Data-Oriented Component Architecture (ECS) with a `GameState` struct holding parallel arrays for `Position`, `Velocity`, `Collider`, etc. The `update()` method runs fixed-timestep subsystems with optimized active-entity pre-collection passes for fast collision detection.
- **`game_ui.rs`**: Renders the Game Screen. It consumes an `alpha` value to interpolate positions between the current and previous frames for visually smooth rendering regardless of the physics tick rate.
- **`leaderboard.rs`**: High scores display screen.
- **`name_entry.rs`**: Input screen for securing a high score.

## Design Patterns

- **State Separation**: Menu and Game are distinct states managed by separate loops in `mod.rs`.
- **Accumulator Loop**: Physics steps run at a fixed rate, decoupled from rendering.
- **Data-Oriented ECS**: Entities are not OOP objects; they are indices into parallel component arrays (SoA) for better cache performance.
- **Immediate Mode Rendering**: The UI is redrawn every frame based on the current state.
- **Phosphor Aesthetics**: Use greens (`Color::Rgb(0, 200, 0)`) for that retro CRT look.

## Testing

- Run `cargo test` to verify logic.
- The `src/tui/game.rs` tests include explicit logic (`test_collision_logic`) and benchmark-style guards (`test_update_performance`) to prevent $O(N^2)$ regressions in the ECS logic.
- Visual verification via `cargo run` is often necessary for TUI changes.

## Future Plans

- **Entities**: Basic enemies with linear downward movement and player/laser collision logic exist. Future enhancements may include more complex Enemy AI patterns or formations.
- **WASM**: Future web target will likely require abstracting the rendering backend further.
