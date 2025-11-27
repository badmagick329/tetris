# Tetris CLI

A classic Tetris game for the terminal, written in Rust.

## Features

- 📊 Score tracking with bonus points for clearing multiple lines
- 🎵 Optional background music support
- 🖥️ Cross-platform (Windows, Linux, macOS)

## Controls

| Key     | Action        |
| ------- | ------------- |
| `←`     | Move left     |
| `→`     | Move right    |
| `↓`     | Move down     |
| `↑`     | Rotate        |
| `Space` | Instant drop  |
| `p`     | Pause/Resume  |
| `d`     | Disable sound |
| `q`     | Quit          |

## Scoring

| Lines Cleared | Points |
| ------------- | ------ |
| 1             | 800    |
| 2             | 1,200  |
| 3             | 1,800  |
| 4 (Tetris!)   | 2,000  |
| 5+            | 3,200  |

## Installation

### Pre-built Binaries

Download the latest release for your platform from the [Releases](https://github.com/badmagick329/tetris/releases) page.

### Building from Source

#### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

#### Build

```bash
# Clone the repository
git clone https://github.com/badmagick329/tetris
cd tetris

# Build in release mode
cargo build --release

# The binary will be at target/release/tetris (or tetris.exe on Windows)
```

#### Run

```bash
cargo run --release
```

## Music

The game supports background music. To enable it, place an MP3 file named `soundtrack.mp3` in the same directory as the executable. You can use any MP3 file of your choice.

If no `soundtrack.mp3` file is found, the game will run silently.
