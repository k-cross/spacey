# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

`spacey` is a retro space-shooter game rendered as a terminal UI (Rust, `ratatui` + `crossterm`, edition 2024).

For the full architecture and module map, read: @AGENTS.md

## Validating changes

After editing, run these in order (this is the contributor loop; there is no CI):

```
cargo fmt
cargo clippy -- -D warnings   # warnings are errors
cargo test
```

Run the game with `cargo run` (add `--release` for smooth timing).

## Version control

This repo is managed with **jujutsu (`jj`), not plain `git`** — the working copy is often in detached HEAD by design. Use `jj` commands for commits, history, and branches; do not run `git commit` / `git branch` directly.

## Environment

Development runs in a `devenv` (Nix) shell auto-loaded by `direnv` (`.envrc`). It sets `RUST_LOG=trace` and `RUST_BACKTRACE=1`. `jj` is provided by the devenv shell.

## Gotchas

- **No workspace.** Single binary crate; all game code lives under `src/tui/`. Core logic and its tests are in `src/tui/game.rs`.
- The game loop uses a **fixed-timestep accumulator** with alpha interpolation for rendering (`src/tui/mod.rs`, `src/tui/game_ui.rs`) — update logic and render logic are separate; don't fold movement into the render path.
- Entities use **data-oriented parallel arrays (SoA)**, not per-entity structs. Add a component by extending the parallel `Vec`s, keeping indices aligned.
- Retro aesthetic is intentional: phosphor green `Color::Rgb(0, 200, 0)`.
- **WASM/web is aspirational only** — README mentions it, but the native TUI is the only working target. There is no `wasm-pack`, `index.html`, or web code.
- `main.rs`'s post-TUI menu dispatch prints "not yet implemented"; actual gameplay runs inside the TUI loop.
