# Mini Games Hub (Rust)

Welcome to the Rust version of Mini Games Hub! A collection of classic arcade games implemented in Rust using the macroquad game framework.

## Features

- 🐍 **Snake** - Classic snake game where you eat food and grow longer
- 🧩 **Tetris** - Block-dropping puzzle game with line clearing
- 🏓 **Pong** - Two-player paddle game
- 🤖 **Pong AI** - Single-player pong against AI opponent

## Installation Guide

### Prerequisites
- Rust toolchain (install from [rustup.rs](https://rustup.rs/))

### Running the Game

1. Clone or navigate to this directory
2. Run the game:
```bash
cargo run --release
```

That's it! The game will compile and launch automatically.

## Controls

### Menu Navigation
- **UP/DOWN** arrows to navigate
- **ENTER** to select game
- **ESC** to return to menu (from any game)

### Snake
- **Arrow keys** to change direction

### Tetris
- **Left/Right arrows** to move piece
- **Down arrow** to drop faster
- **Up arrow** to rotate piece

### Pong (2-Player)
- **W/S** for left paddle
- **UP/DOWN arrows** for right paddle

### Pong AI (vs Computer)
- **W/S** to control your paddle
- AI controls the right paddle automatically

## Building

To build an optimized executable:
```bash
cargo build --release
```

The executable will be in `target/release/minigameshub.exe` (Windows) or `target/release/minigameshub` (Linux/macOS).

## Project Structure

- `src/main.rs` - Main game manager and menu system
- `src/snake.rs` - Snake game implementation
- `src/tetris.rs` - Tetris game implementation
- `src/pong.rs` - Two-player Pong game
- `src/pong_ai.rs` - Single-player Pong vs AI

## Differences from Python Version

This Rust implementation provides:
- Better performance due to Rust's compiled nature
- Memory safety guarantees
- Cross-platform compatibility
- Single executable deployment (no Python/pygame installation required)

## Dependencies

- [macroquad](https://github.com/not-fl3/macroquad) - Simple and easy to use game library for Rust
