# Rust Book
https://rust-book.cs.brown.edu/

## Creating a new Project with Cargo
- `cargo new hello_cargo`

## Compiling and Running
- Compiling: `rustc main.rs`
- Running: `./main`

## Building with Cargo
- Debug Build: `cargo build`
    - or `cargo run` for development
- Checking if it compiles: `cargo check`
    - (faster than building)
- Building for Release: `cargo build --release`

## Crates
- Adding crates: Modify `Cargo.toml` to include a crate like below
```
[dependencies]
rand = "0.8.5"
```

- Updating crates: use `cargo update`