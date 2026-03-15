# Repository Guidelines

## Project Structure & Module Organization
This repository is a Rust workspace for the `mogura` molecular visualizer. Workspace members live in sibling crates: `mogura-cli` for the desktop entry point, `mogura-wasm` for the web build, `bevy-mogura` for Bevy integration, and `mogura-io`, `mogura-ss`, and `mogura-asl` for parsing and domain logic. Shared documentation sits in `README.md`. Static images and demo media live in `assets/`, and `examples/` holds sample material rather than Rust integration tests.

## Build, Test, and Development Commands
Use Cargo from the workspace root unless a crate-specific README says otherwise.

- `cargo check` validates the full workspace quickly.
- `cargo test` runs unit tests across all crates.
- `cargo clippy -- -D warnings` treats lint warnings as errors.
- `cargo fmt -- --check` verifies Rust formatting.
- `cargo make dev` runs tests, then starts `mogura` locally.
- `cargo make wasm` serves the WASM app from `mogura-wasm/` with Trunk.
- `cargo make wasm-release` builds an optimized WASM bundle and runs `wasm-opt`.
- `cargo make wasm-prepare` copies the optimized WASM bundle to `../mogura-wasm-gh/` for GitHub Pages deployment.

## Gotchas & Prerequisites
- **WASM builds require `trunk`** (`cargo install trunk`) — `cargo make wasm` and `cargo make wasm-release` call `trunk` directly.
- **`RUSTFLAGS` is set automatically** by `cargo make` tasks. Do NOT manually run `trunk serve` or `trunk build` in `mogura-wasm/` without first exporting `RUSTFLAGS="--cfg web_sys_unstable_apis"`, or the build will fail with missing unstable API errors.
- **MSRV is Rust 1.85.0** (`rust-version` in `Cargo.toml`). The workspace uses Rust 2024 edition.

## Coding Style & Naming Conventions
The workspace uses Rust 2024 edition with 4-space indentation (`rustfmt.toml`). Run `cargo fmt` before pushing. Keep crate and module names lowercase with hyphens for crates (`mogura-cli`) and snake_case for Rust items. Favor small modules with clear domain boundaries; parser and selection logic already live in dedicated crates. TOML files are formatted with `taplo fmt`, and spelling is checked with `typos`.

## Testing Guidelines
Tests are primarily inline unit tests using `#[cfg(test)]`, for example in `mogura-asl/src/lib.rs`. Add focused unit tests next to the code they verify, and use descriptive snake_case test names such as `parse_nested_selection_expr`. Run `cargo test` locally before opening a PR. If you touch WASM-specific behavior, also smoke-test with `cargo make wasm`.

## Commit & Pull Request Guidelines
Recent history favors short, imperative subjects such as `add citation`, `update readme`, and `Chore (#1)`. Keep commit messages concise, lowercase when practical, and scoped to one change. PRs should explain user-visible behavior, list affected crates, and link related issues. Include screenshots or short recordings for UI or rendering changes, especially for `mogura-cli` and `mogura-wasm`.

## Quality & Tooling
Install `pre-commit` if you want the same checks enforced locally. The configured hooks run `taplo fmt`, `typos`, `cargo fmt -- --check`, `cargo check`, `cargo test`, `cargo clippy -- -D warnings`, and `cargo deny check`.
