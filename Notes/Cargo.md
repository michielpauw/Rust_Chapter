# Cargo

- Cargo: Rust package manager and build tool
- TOML (Tom's Obvious, Minimal Language): Rust's configuration format
- Cargo expects your source files to live inside the src directory. The top-level project directory is just for README files, license information, configuration
- files, and anything else not related to your code.
- `cargo build` from directory with `.toml` file: creates `Cargo.lock` (keeps track of exact dependencies)
- `cargo run` will compile if necessary, and run the binary
- `cargo check` will check whether the code compiles, but doesn't produce an executable
- `cargo build --release` will compile with optimizations
