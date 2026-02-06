# Spacey

A first-person ASCII-based space shooter game built with Rust, targeting WebAssembly for browser deployment while maintaining native TUI support.

## Overview

Navigate your spaceship through space in this retro-styled ASCII game. Engage hostile entities in first-person combat, all rendered in beautiful terminal graphics. Built with Rust for performance and compiled to WebAssembly for universal accessibility.

## Features

- **First-Person Perspective**: Space combat from the cockpit
- **ASCII Graphics**: Terminal-style visuals that work anywhere
- **Cross-Platform**: Terminal support and browser deployment via WASM
- **Fast-Paced Action**: Dodge and shoot your way through enemies

## Prerequisites

- [devenv](https://devenv.sh/) - Development environment management
- Rust toolchain (managed through devenv)
- wasm-pack (managed through devenv)

## Getting Started

As of right now, if you have a rust toolchain installed, you can run:
1. `cargo build --release`
2. `cargo run --release`

This is enough to get you an environment but may not always be the case.

### Setting Up the Development Environment

This project uses `devenv` to manage all development dependencies and tools.

Note: personally I use `direnv` which automatically loads the `devenv` environment when I enter the project directory while also letting me use my normal shell instead of it's default `bash`.

1. Install devenv
   ```sh
   # See https://devenv.sh/getting-started/ for installation instructions
   ```

2. Enter the development environment:
   ```sh
   devenv shell
   ```

   This will automatically set up:
   - Rust toolchain
   - wasm-pack
   - Other necessary development tools

### Building

#### For Native TUI
```sh
cargo build --release
```

### Running

#### Native Terminal Version
```sh
cargo run
```

## Controls

- **Arrow Keys / WASD**: Navigate options / Move ship view
- **Space**: Fire lasers
- **Enter**: Select option / Pause game
- **Q**: Quit (TUI version)

## Development

### Project Structure

```
.
├── src/
│   ├── main.rs          # Entry point
│   └── tui/             # TUI implementation
│       ├── mod.rs       # Module root & event loop
│       ├── app.rs       # App state
│       ├── menu.rs      # Menu logic
│       ├── ui.rs        # Menu rendering
│       ├── game.rs      # Game state
│       └── game_ui.rs   # Game rendering
├── assets/              # ASCII art and resources
├── devenv.nix           # Development environment configuration
├── devenv.lock          # Locked dependencies
└── Cargo.toml           # Rust dependencies
```

### Building for Different Targets

The game is designed to work in both environments:
- **TUI Mode**: Uses terminal capabilities for native performance (ratatui)
- **Web Mode**: Renders to HTML canvas with WASM for browser compatibility

### Running Tests

```sh
cargo test
```

## Technical Details

- **Language**: Rust
- **Rendering**: ASCII/ANSI escape codes (TUI), Canvas API (Web)
- **Build Tool**: Cargo, wasm-pack
- **Environment Management**: devenv

## Roadmap

- [x] Basic movement
- [ ] Shooting mechanics
- [ ] World and Rotational Geometry/Positioning
- [ ] Enemy AI patterns
- [ ] Multiple enemy types
- [ ] Score tracking and leaderboards
- [ ] Sound effects (WASM)
- [ ] Power-ups and weapons
- [ ] Level progression
- [ ] Boss battles
- [ ] WASM

## Acknowledgments

Inspired by classic ASCII games and space shooters of the past.
