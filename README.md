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

        curl -sf