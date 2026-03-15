# AGENTS.md

Instructions for coding agents operating in this repository.

## Project overview

- Crate name: `hive-engine`
- Language: Rust (`edition = "2021"`)
- Rust version: `1.82`
- Shape: library + CLI binary
- Domain: Hive board game engine and move/rule validation

## Repository map

- `Cargo.toml`: crate metadata and dependencies
- `src/lib.rs`: library entrypoint (`pub mod engine;`)
- `src/main.rs`: interactive CLI prototype
- `src/engine/`: core game implementation
- `src/misc/`: CLI helper utilities
- `src/tests/`: unit tests linked into modules via `#[cfg(test)]`
- `tests/web.rs`: wasm-only integration test (`target_arch = "wasm32"`)
- `.github/workflows/rust.yml`: CI build/test
- `.github/workflows/release-plz.yml`: release automation
- `rustfmt.toml`: formatting rules

## Build / lint / test commands

### Core commands

- Build:
  - `cargo build --verbose`
- Run tests:
  - `cargo test --verbose`
- Run CLI:
  - `cargo run`

### Recommended local quality gate

- Format check:
  - `cargo fmt --all --check`
- Lint (strict):
  - `cargo clippy --all-targets --all-features -- -D warnings`
- Test suite:
  - `cargo test --verbose`

### Run a single test (important)

- Exact test name:
  - `cargo test given_grid_when_placing_piece_to_hex_then_hex_contains_piece -- --exact`
- Name substring:
  - `cargo test queen_surrended_rule`
- With output:
  - `cargo test given_filled_grid_when_one_hive_rule_with_correct_hex_should_return_true -- --exact --nocapture`

### Run tests by area

- Move logic tests: `cargo test moves_tests`
- Rule logic tests: `cargo test rules_tests`
- Grid tests: `cargo test grid_tests`

### wasm tests

- `tests/web.rs` is gated by `#![cfg(target_arch = "wasm32")]`
- Native `cargo test` does not execute wasm-only tests
- Run wasm-specific tests only when wasm/web behavior is in scope

## CI behavior

- CI runs on push/PR to `master`
- CI commands (from `.github/workflows/rust.yml`):
  - `cargo build --verbose`
  - `cargo test --verbose`
- Release tasks are handled by release-plz workflow
- Do not change CI/release workflow files unless explicitly asked

## Code style and conventions

### Formatting

- Follow `rustfmt.toml`:
  - `tab_spaces = 2`
  - `imports_granularity = "Crate"`
  - `reorder_imports = true`
- Run `cargo fmt` after edits

### Imports

- Keep imports ordered and minimal
- Preferred grouping:
  - `std::...`
  - external crates (`serde`, `uuid`, etc.)
  - crate-local (`crate::...`)
- Use grouped crate imports where readable (`use crate::engine::{...};`)
- Remove unused imports; keep warnings clean

### Types and typing discipline

- Prefer explicit domain types (`Game`, `Grid`, `Hex`, `Piece`, enums)
- Keep derives consistent with use (`Debug`, `Clone`, `Copy`, `PartialEq`, serde derives)
- Preserve serialization support where present (`Serialize`, `Deserialize`)
- Use `Option`/`Result` for absence/failure rather than sentinel patterns

### Naming

- Follow Rust norms and existing code style:
  - types/enums/traits: `PascalCase`
  - functions/modules/files/variables: `snake_case`
  - constants/statics: `SCREAMING_SNAKE_CASE`
- Test function names follow behavior style (`given_when_then`)
- Keep game-domain vocabulary consistent (`queen`, `hex`, `grid`, `piece`)

### Error handling

- Avoid introducing panic paths in core engine logic
- Prefer `Result` for fallible operations in library-facing code
- `unwrap`/`expect` are acceptable in tests and temporary prototypes
- If using `expect`, provide specific actionable context

### Control flow and readability

- Keep rule logic explicit and easy to audit
- Prefer small helpers for repeated logic
- Avoid unnecessary abstraction and stylistic churn
- Match local style in touched files

### Ownership/performance basics

- Prefer borrowing over cloning unless clone improves clarity materially
- Avoid unnecessary allocations in move/rule hot paths
- Prioritize correctness first, then optimize with clear rationale

## Testing guidelines

- Any rule/move behavior change should include test updates
- Reuse deterministic board setup patterns (`initialize_grid` style)
- Keep assertions specific and behavior-focused
- For bug fixes, add a regression test that fails pre-fix

## API and docs

- Treat public engine API as stable unless change is requested
- If changing public signatures, update tests/docs in same change
- Update `README.md` when usage or behavior changes

## Agent workflow expectations

- Make focused diffs; avoid unrelated refactors
- Before final handoff, run at least:
  - `cargo fmt --all --check`
  - `cargo test --verbose`
- Run clippy for logic-heavy edits

## Cursor/Copilot rules check

- `.cursorrules`: not found
- `.cursor/rules/`: not found
- `.github/copilot-instructions.md`: not found
- If these appear later, treat them as higher-priority instructions and fold relevant guidance into this file
