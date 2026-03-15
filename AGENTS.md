# AGENTS.md

Guidance for autonomous coding agents operating in this repository.

## Project Snapshot

- Package: `hive-engine`
- Language: Rust (`edition = "2021"`, `rust-version = "1.94"`)
- Shape: library crate (`hive`) + CLI binary (`hive-engine`)
- Domain: Hive board game engine (state, movement, rules)

## Repository Map

- `Cargo.toml`: crate metadata, targets, dependencies
- `src/lib.rs`: library entry (`pub mod engine;`)
- `src/engine.rs`: top-level engine module exports
- `src/engine/`: board, pieces, rules, game logic
- `src/main.rs`: interactive CLI prototype
- `src/misc/`: CLI helper utilities
- `src/tests/grid_tests.rs`: grid-level tests
- `src/tests/moves_tests.rs`: movement generation tests
- `src/tests/rules_tests.rs`: game rule tests
- `.github/workflows/rust.yml`: CI build + test pipeline
- `rustfmt.toml`: formatting source of truth

## Build, Lint, and Test Commands

### Core commands

- Build all targets: `cargo build --verbose`
- Run full test suite: `cargo test --verbose`
- Run CLI prototype: `cargo run`

### Formatting and linting

- Format workspace: `cargo fmt --all`
- Check formatting only: `cargo fmt --all --check`
- Strict linting: `cargo clippy --all-targets --all-features -- -D warnings`

### Single-test execution (important)

- List available tests first: `cargo test -- --list`
- Run one exact test name:
  - `cargo test given_grid_when_available_moves_queenbee_should_return_correct_moves -- --exact`
- Run tests by substring filter:
  - `cargo test queen_surrounded_rule`
- Run one test and keep output:
  - `cargo test given_filled_grid_when_one_hive_rule_with_correct_hex_should_return_true -- --exact --nocapture`

### Run tests by area

- Grid tests: `cargo test grid_tests`
- Move-generation tests: `cargo test moves_tests`
- Rule tests: `cargo test rules_tests`

### Useful focused commands

- Compile tests without running: `cargo test --no-run`
- Run library tests only: `cargo test --lib`
- Run binary tests only: `cargo test --bin hive-engine`

## CI Expectations

- CI runs on push and PR to `master`
- `.github/workflows/rust.yml` currently runs:
  - `cargo build --verbose`
  - `cargo test --verbose`
- Keep local changes compatible with these commands
- Do not alter release workflow files unless explicitly requested

## Code Style Guidelines

### Formatting

- Always follow `rustfmt.toml`:
  - `tab_spaces = 2`
  - `imports_granularity = "Crate"`
  - `reorder_imports = true`
- Run `cargo fmt --all` after edits
- Avoid unrelated formatting churn

### Imports

- Keep imports minimal and used
- Prefer conventional grouping order:
  - standard library (`std::...`)
  - external crates
  - local crate modules (`crate::...`)
- Let rustfmt control final ordering/granularity
- Remove dead imports instead of suppressing warnings

### Types and API design

- Prefer domain types (`Game`, `Grid`, `Hex`, `Piece`) over primitive-heavy signatures
- Use enums for closed state spaces (piece kind, color, winner state)
- Keep signatures explicit; avoid implicit or surprising conversions
- For fallible library operations, prefer `Result`/`Option` over sentinel values
- Preserve public API shape unless change is explicitly requested

### Naming conventions

- Follow standard Rust naming:
  - types/traits/enums: `PascalCase`
  - functions/modules/variables: `snake_case`
  - constants/statics: `SCREAMING_SNAKE_CASE`
- Maintain Hive vocabulary consistency (`queen`, `hex`, `grid`, `piece`)
- Match existing descriptive test naming style (`given_when_then`)

### Control flow and readability

- Prefer readable rule logic over compact cleverness
- Use early returns for invalid branches and guard clauses
- Extract helpers when logic repeats or nesting grows
- Avoid unnecessary abstraction in movement/rule hot paths

### Error handling

- Avoid introducing panics in engine/library paths
- Reserve `unwrap`/`expect` mainly for tests and CLI prototype code
- If `expect` is used, provide a specific actionable message
- On invalid game actions, return safely according to current API behavior

### Ownership and performance

- Prefer borrowing over cloning unless cloning improves clarity materially
- Be cautious with allocations in frequently called move/rule logic
- Keep lightweight value types `Copy` where established
- Prioritize correctness before optimization; optimize with clear motivation

### Testing standards

- Any movement/rule change should include relevant test updates
- Reuse deterministic board setup helpers for scenario tests
- Keep assertions specific and meaningful
- Add regression tests for bug fixes (fails before, passes after)

### Documentation and comments

- Add comments only for non-obvious logic
- Keep comments accurate and close to the code they explain
- Update `README.md` when user-facing behavior/usage changes

## Agent Workflow Expectations

- Keep diffs focused; avoid unrelated refactors
- Respect existing module boundaries and file layout
- Before handoff, run at least:
  - `cargo fmt --all --check`
  - `cargo test --verbose`
- For non-trivial logic changes, also run:
  - `cargo clippy --all-targets --all-features -- -D warnings`

## Cursor and Copilot Rules

Checked paths in this repository:

- `.cursorrules`: not present
- `.cursor/rules/`: not present
- `.github/copilot-instructions.md`: not present

If any of these files are added later, treat them as higher-priority agent guidance and update this document.
