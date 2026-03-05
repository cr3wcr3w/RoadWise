# RoadWise

`RoadWise` is a simple Rust desktop app built with `iced`.

Train your mind for traffic signs.

<!-- https://lto.gov.ph/wp-content/uploads/2023/09/RO102_CDE_Road_and_Traffic_Rules_Signs-Signals-Markings.pdf -->

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