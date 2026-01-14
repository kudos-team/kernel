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
cargo build --release
```
### Running
```bash
cargo run
```
### Testing
```bash
cargo test
```
## What the files do
### `tests/`
This contains tests that get individually compiled and ran during `cargo test`

### `src/`
#### For the main program
- `main.rs` - contains stuff that's ran directly - e.g. panic code, the main function, test running code, etc.
- `boot.rs` - contains the function ran on boot (after loading other stuff)

#### For the core library
- `lib.rs` - contains imports for the rest of the code and provides some things for testing (e.g. exiting qemu)
- `sigslt.rs` - contains stuff for signals and slots
- `serial.rs` - contains stuff for printing to the console (the one you run `cargo test` in, not the actual kernel's console) for testing
- `vga_buffer.rs` - handles printing to the screen (the real screen)
- `interrupts.rs` - handles interrupts (e.g. clocks, devices, breakpoints, double fault errors)
- `gdt.rs` - handles stuff for a 'global descriptor table' (helpful for `interrupts.rs`)
- `memory.rs` - handles memory stuff like pageing and whatnot
- `allocator.rs` - handles allocation of memory (also defines which allocator to use at the bottom of the file)
    - `allocator/*.rs` - different implementations for the allocator
- `task/mod.rs` - contains the basic structure for tasks (async and stuff)
    - `task/executor.rs` - contains the thing that runs tasks - an executor (and waker and stuff)
    - `task/keyboard.rs` - contains some stuff for waiting for keyboard events

- `utils/` - contains various utilities for doing awesome things!
    - `utils/fancy.rs` - contains stuff to make the output fancy!
    - `utils/keys.rs` - contains abstractions for interacting with the keyboard!

