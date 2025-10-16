# Repository Guidelines

## Project Structure & Module Organization
The workspace is anchored by the root `Cargo.toml`, `dist-workspace.toml`, and `workspace-hack/` helper crate. Core runtime crates live in `crates/`, grouped by capability (`core/`, `networking/`, `manager/`, `stores/`, `testing-utils/`, etc.), and should keep public APIs surfaced through `crates/sdk/`. The `cli/` directory hosts the `cargo-tangle` binary with integration coverage in `cli/tests/`. Starter blueprints live in `examples/` and must remain buildable against the latest SDK. Long-form specifications and operator guides are under `docs/`. Use `docker-compose.yml` only when you need optional external services; it is not exercised in CI.

## Build, Test, and Development Commands
- `cargo fmt --all`: Format Rust sources using the shared `rustfmt.toml` rules.
- `cargo clippy --workspace --all-targets --all-features -D warnings`: Enforce lint cleanliness across every crate.
- `cargo test --workspace --all-features`: Run unit, doc, and integration tests; prefer `cargo test -p <crate>` for focused runs.
- `cargo build --workspace --all-features`: Compile everything locally before publishing a crate or tagging a release.
- `cargo run -p blueprint-manager -- --help`: Smoke-check runtime binaries after changes to orchestration crates.

## Coding Style & Naming Conventions
Rust files use the pinned stable toolchain 1.88 (managed via `rust-toolchain.toml`) with 4-space indentation. Follow idiomatic naming: modules and functions in `snake_case`, types and traits in `PascalCase`, constants in `SCREAMING_SNAKE_CASE`. Keep public exports centralized in each crate’s `lib.rs`, re-exporting from submodules instead of deep relative paths. Format documentation comments thoughtfully—`rustfmt` is configured to wrap doc examples—and order imports with standard/prelude/external grouping. For manifest updates, run `taplo fmt` to honor `taplo.toml`.

## Testing Guidelines
Write async tests with `tokio::test` where network or time-based behavior is exercised, and mirror fixtures in `crates/testing-utils/` when possible. Place integration suites in each crate’s `tests/` directory or under `cli/tests/` for CLI flows, naming files with the feature under test (`manager_failover.rs`, `router_happy_path.rs`). Maintain edge-case coverage for multi-node scenarios and include negative-path assertions. Before opening a PR, run `cargo test --workspace --all-features` and document any intentionally skipped targets.

## Commit & Pull Request Guidelines
Adopt Conventional Commits (`feat(router):`, `fix(cli):`, etc.) and align branches with the `feature/`, `fix/`, or `docs/` prefixes already in use. Each PR description should summarize behavior changes, list the verification commands you ran, and link any tracked issues. Attach logs or terminal snippets when altering CLI UX, and update `docs/` or examples when interfaces shift. Ensure CI is green, request a maintainer review, and keep PRs scoped to a single logical change set.
