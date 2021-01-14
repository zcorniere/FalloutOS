# FalloutOS

## Purpose
The goal of the project is to reproduce the interface offered by various terminal in the Fallout games.

And for me to pratice the related technologies, like low-level memory management, cpu scheduling, asm, etc...

## Dependencies
This is written in Rust, so you obviously need to have [rust](https://rustup.rs/).

In order to run this, you need to have installed `qemu`  and `bootimage`.

## Ubuntu
```bash
# apt-get install qemu -y
```

## Fedora
```bash
# dnf install qemu -y
```

### Rust component
```bash
rustup toolchain install nightly
rustup component add rust-src
rustup component add llvm-tools-preview
cargo install bootimage
```

Now you can hit `cargo run`
