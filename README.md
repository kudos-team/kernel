# KudOS Kernel
## Setup
Run the following to install important things for booting:
```
rustup component add llvm-tools-preview
cargo install bootimage
```
## Building
### Regular building
Run `cargo build`, and if it says `.../Cargo.lock" does not exist ..., try: <command>` try running what it says then `cargo build` again
### Running (for testing)
Run `cargo run`

