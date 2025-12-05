<!--
  Guidance for AI coding agents working on the RustMaterial repo.
  Keep this file concise and focused on patterns and commands discoverable
  from the repository layout (do not add generic, aspirational rules).
-->

# Copilot Instructions — RustMaterial

Purpose: help an AI quickly become productive in this learning repo.

- **Project layout**: root contains `README.md`, learning folder `00_Demo/`, and small miscellaneous notes. The primary runnable crate is `00_Demo/` (see `00_Demo/Cargo.toml`).

- **Where to run code**: change into `00_Demo/` and use Cargo. Examples are implemented as separate binaries under `00_Demo/src/bin/`.
  - Run a specific demo: `cd 00_Demo && cargo run --bin hello_world`
  - Build all examples: `cd 00_Demo && cargo build`

- **Edition & tooling**: examples use the Rust 2021 edition. Respect `cargo fmt` and `cargo clippy` style when editing examples. If you modify code, run:
  - `cd 00_Demo && cargo fmt -- --check`
  - `cd 00_Demo && cargo clippy -- -D warnings`

- **Naming & patterns visible in this repo**:
  - Example binaries live in `00_Demo/src/bin/<name>.rs` (each file contains a `fn main()`).
  - Keep examples short and focused: small functions and inline comments, similar to `00_Demo/src/bin/hello_world.rs`.
  - Prefer explicit `Result` returns for fallible examples (show error handling patterns). Use `anyhow` only if added to `Cargo.toml`.

- **Developer workflows & quick commands** (discoverable in repo):
  - Install Rust: use `rustup` (README contains exact commands).
  - Run a single example: `cd 00_Demo && cargo run --bin <name>`
  - Format: `cd 00_Demo && cargo fmt`
  - Lint: `cd 00_Demo && cargo clippy`

- **Devcontainer / Codespaces**: this repo includes a `.devcontainer/` folder with a `devcontainer.json` and `Dockerfile`. When opening in Codespaces or a local devcontainer, the container will set up Rust toolchain and developer components (rustfmt, clippy). Use the container's terminal to run Cargo commands.

- **Files to inspect for context**:
  - `README.md` — project intent and install/run steps.
  - `00_Demo/Cargo.toml` — the crate metadata and edition.
  - `00_Demo/src/bin/*.rs` — example patterns to mimic for new lessons.

- **What to avoid**:
  - Don't introduce large external dependencies without mentioning why; this repo is educational and favors standard library-first examples.
  - Don't reorganize the repo into an unrelated workspace layout; new lessons should be added under `00_Demo/` as additional binaries.

If anything in the repository appears incomplete or unintentionally missing (for example: missing `Cargo.toml` before these changes), ask the maintainer which structure they prefer before large refactors.
