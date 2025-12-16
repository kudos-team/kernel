# KudOS Kernel
## Setup
Run the following to install important things for booting:
```
rustup component add llvm-tools-preview
cargo install bootimage
```
Also for running and testing ensure you have Qemu installed.
## Doing stuff with the code
### Building
```bash
cargo build
```
### Running
```bash
cargo run
```
### Testing
```bash
cargo test
```

