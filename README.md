# rust-cli

A lightweight, fast command-line search tool written in **Rust**, inspired by `grep`.  
Searches for a query string in files and prints matching lines with optional case-insensitive mode.

## Features
- Search patterns in files  
- Case-insensitive matching via `CASE_INSENSITIVE` environment variable  
- Clean error messages for invalid files or inputs    
- Zero-cost abstractions thanks to Rust's ownership system

## Installation

```bash
cd rust-cli
cargo build --release
```
## Usage
./target/release/rust-cli <QUERY> <FILE>

### Example
#### Basic search (case-sensitive)
./target/release/rust-cli "rust" poem.txt

#### Case-insensitive search
CASE_INSENSITIVE=1 ./target/release/rust-cli "Rust" poem.txt

## Project Structure

rust-cli/
├── Cargo.toml      # No dependencies
├── src/
│   ├── main.rs     # Entry point
│   └── lib.rs      # Config, search logic, file I/O
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
