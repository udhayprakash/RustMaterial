# RustMaterial

Rust Language Learning material

## Rust

- Rust is blazingly fast systems programming language that prevents segfaults and guarantees thread safety.
- Originally Rust was designed by Graydon Hoare, Now it managed by Rust project developers.
- The first version of Rust was released in the year 2010.
- Rust is syntactically similar to C++.

## Rust Features

- zero-cost abstractions
- move semantics
- guaranteed memory safety
- threads without data races
- trait-based generics
- pattern matching
- type inference
- minimal runtime
- efficient C bindings

### Rust's Garbage Collector

- Rust uses a static garbage collector.
- It works on the principle of automatic memory management which means it automatically recycles the memory that will not be used again.

## Companies Using Rust 
- 360dialog
- OneSignal
- Coursera
- Atlassian
- Braintree
- npm, Inc
- Mozilla
- Academia.edu
- Xero

## Cargo

- It's a build system and package manager built for Rust users to manager projects in it.
- The Cargo system manages three things for users, building code, downloading the libraries, and rebuilding those libraries.

### Cargo.lock 
- When a user runs cargo build command it automatically creates a file named as Cargo.lock to keep track of dependencies in the user application.


### Cargo Commands

    cargo new
    cargo build
        cross compilation to linux is achieved by running
                cargo build --target x86_64-unknown-linux-musl
        To cross compile to a static Linux binary, install
                rust-std-linux

    cargo test
    cargo run
    cargo fmt
    cargo clippy
    cargo vendor


### Rust Modules 
- Tokio : Asynchronous run-time for Rust
- Hyper : fast, safe HTTP implementation written in and for Rust
- Tower : Library for modular and reusable components for building robust networking clients & servers.



## websites
        rustup.rs
        

## Installation 

### 1. Linux
This repository contains learning material for Rust. The examples and runnable demos are located in the `00_Demo/` folder (a small Cargo project).

Install Rust using the recommended `rustup` tool. These steps work on most Linux distributions.

1. Install rustup (official installer):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Follow the interactive prompt (choose the default 'stable' toolchain). After installation, reload your shell or run:

```bash
source "$HOME/.cargo/env"
```

2. Verify Rust and Cargo are installed:

```bash
rustc --version
cargo --version
```

3. Install recommended developer tools used in this repo (optional but useful):

```bash
rustup component add rustfmt clippy rust-src
cargo install cargo-edit cargo-watch
```

4. Run the demo examples (from repository root):

```bash
cd 00_Demo
cargo run --bin hello_world
```

Notes:
- The `00_Demo/` directory is a standalone Cargo crate containing multiple example binaries in `00_Demo/src/bin/`.
- Use `cargo run --bin <name>` to execute a specific example, or `cargo build --release` to produce optimized artifacts.