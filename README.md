# Spacey

A first-person ASCII-based space shooter game built with Rust, targeting WebAssembly for browser deployment while maintaining native TUI support.

## Overview

Navigate your spaceship through space in this retro-styled ASCII game. Engage hostile entities in first-person combat, all rendered in beautiful terminal graphics. Built with Rust for performance and compiled to WebAssembly for universal accessibility.

## Features

- **First-Person Perspective**: Experience space combat from the cockpit
- **ASCII Graphics**: Classic terminal-style visuals that work anywhere
- **Cross-Platform**: Native terminal support and browser deployment via WASM
- **Fast-Paced Action**: Dodge and shoot your way through enemies
- **Rust Performance**: Leveraging Rust's speed and safety guarantees

## Prerequisites

- [devenv](https://devenv.sh/) - Development environment management
- Rust toolchain (managed through devenv)
- wasm-pack (managed through devenv)

## Getting Started

### Setting Up the Development Environment

This project uses `devenv` to manage all development dependencies and tools.

1. Install devenv if you haven't already:
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

- **Arrow Keys / WASD**: Navigate your spaceship
- **Space**: Fire weapons
- **ESC**: Pause / Menu
- **Q**: Quit (TUI version)

## Development

### Project Structure

```
.
├── src/
│   ├── main.rs          # Entry point
│   ├── game/            # Game logic
│   ├── renderer/        # ASCII rendering engine
│   ├── entities/        # Player, enemies, projectiles
│   └── wasm/            # WebAssembly bindings
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

- [ ] Basic movement and shooting mechanics
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
