# Kecho

`kecho` is a simple Rust desktop app built with `iced`.

It listens for keyboard input and shows your configured keyword when you type it.

## Tech Stack

- [`iced` 0.14](https://crates.io/crates/iced)

## Getting Started

### 1. Prerequisites

- Install Rust: https://rustup.rs

### 2. Run the app

```bash
cargo run
```

## Development

### Format code

```bash
cargo fmt --all
```

### Lint code

```bash
cargo clippy --all-targets --all-features -- -D warnings
```

## Project Structure

- `src/main.rs` - App entry point and UI logic
- `Cargo.toml` - Project manifest and dependencies