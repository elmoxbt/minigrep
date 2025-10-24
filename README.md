# Minigrep

A lightweight, fast command-line search tool written in **Rust**, inspired by `grep`.  
Searches for a query string in files and prints matching lines with optional case-insensitive mode.

## Features
- Search patterns in one or more files  
- Case-insensitive matching via `--ignore-case`  
- Clean error messages for invalid files or inputs  
- Built with `clap` for intuitive CLI argument parsing  
- Zero-cost abstractions thanks to Rust’s ownership system  

## Installation

```bash
git clone https://github.com/elmoxbt/minigrep.git
cd minigrep
cargo build --release
```
## Usage
./target/release/minigrep <QUERY> <FILE> [--ignore-case]

### Example
#### Basic search
./target/release/minigrep "rust" poem.txt

#### Case-insensitive search
./target/release/minigrep "Rust" poem.txt --ignore-case

## Project Structure

minigrep/
├── Cargo.toml      # Dependencies: clap, env_logger
├── src/
│   └── main.rs     # CLI parsing, file I/O, search logic
└── README.md

## Build and Test

cargo build          # Compile
cargo test           # Run tests (expandable in lib.rs)
cargo clippy         # Lint code
cargo run --release -- "pattern" file.txt

## Contribution

Fork the repo
Create a branch: git checkout -b feature/xyz
Commit: git commit -m "Add xyz"
Push & open a PR

Ideas: regex support, colored output, recursive directory search.

## License

MIT License – free to use and modify.
