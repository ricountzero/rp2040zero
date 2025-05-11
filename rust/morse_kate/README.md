# Morse Kate - RP2040 Zero Project

This project is a Rust-based firmware for the RP2040 Zero board, featuring WS2812 LED control. It is built using the [Embassy](https://github.com/embassy-rs/embassy) framework, a modern embedded framework for Rust that provides async/await support and hardware abstraction layers.

## Example

Here's an example of the morse code animation in action:

![Morse Code Example](/images/morse_KATE_example.gif)

## Prerequisites for Fedora Linux

Install the following packages:

```bash
# Install Rust and cargo
sudo dnf install rust cargo

# Install build dependencies
sudo dnf install gcc-arm-none-eabi
sudo dnf install pkg-config
sudo dnf install libudev-devel

# Install probe-rs for flashing
cargo install probe-rs --features cli

# Install elf2uf2-rs for UF2 conversion
cargo install elf2uf2-rs
```

## Building and Running

To build the project in release mode:

```bash
cargo build --release
```

To run the project in release mode:

```bash
cargo run --bin morse_kate --release
```

## Flashing to RP2040 Zero

1. Connect your RP2040 Zero board via USB
2. Hold the BOOTSEL button while connecting the board to enter bootloader mode
3. The board should appear as a USB mass storage device
4. Copy the UF2 file to the mounted drive

The UF2 file will be located at:
```
target/thumbv6m-none-eabi/release/morse_kate.uf2
```

## Project Structure

- `src/main.rs` - Main firmware code
- `Cargo.toml` - Project dependencies and configuration 