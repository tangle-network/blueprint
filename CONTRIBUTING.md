# Contributing to Blueprint

This document defines how we ship high-confidence changes in this repository.

## Development Setup

1. Install system dependencies.
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install build-essential cmake libssl-dev pkg-config

   # macOS
   brew install openssl cmake
   ```
2. Install Rust toolchain `1.88` (see `rust-toolchain.toml`).
   ```bash
   rustup toolchain install 1.88
   rustup default 1.88
   rustup component add rustfmt clippy rust-src
   ```
3. Clone and enter the repository.
   ```bash
   git clone https://github.com/tangle-network/blueprint.git
   cd blueprint
   ```

## Repository Layout

- `cli/`: `cargo-tangle` CLI.
- `crates/`: SDK crates (`core`, `manager`, `networking`, `stores`, `testing-utils`, etc.).
- `examples/`: reference blueprints that must stay buildable.
- `docs/`: long-form specifications, RFCs, and operator guides.
- `.github/`: CI workflows and PR automation.

## Branches and Commits

- Branch prefixes:
  - `feature/` for features
  - `fix/` for bug fixes
  - `docs/` for docs-only changes
  - `refactor/` for internal code movement
  - `test/` for test-only updates
- Commit format: Conventional Commits (`feat(scope): ...`, `fix(scope): ...`).

## Local Verification

Run these before opening a PR:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -D warnings
cargo test --workspace --all-features
cargo build --workspace --all-features
```

For focused iteration, use `cargo test -p <crate>`.

## Harness Engineering Standard

Blueprint follows a harness-first workflow for risky changes:

1. Reproduce the problem with the smallest failing harness/test.
2. State invariants and fail-closed behavior before implementing.
3. Patch the code.
4. Prove the fix with targeted tests and negative-path coverage.
5. Document operational impact (operator/customer/developer) when behavior changes.

Use the playbook for full guidance:

- [`docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md`](docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md)
- [`docs/engineering/HARNESS_ENGINEERING_SPEC.md`](docs/engineering/HARNESS_ENGINEERING_SPEC.md)
- [`docs/engineering/HARNESS_REVIEW_CHECKLIST.md`](docs/engineering/HARNESS_REVIEW_CHECKLIST.md)

## Pull Request Requirements

Every PR must:

1. Fill out the PR template completely.
2. Include verification commands with pass/fail outcomes.
3. Call out risks, migration notes, and behavior changes.
4. Add tests for new logic and regressions.
5. Update docs/examples when interfaces or flows change.

Large changes may require an RFC in `docs/rfcs/`.

## Review Expectations

Reviewers should prioritize:

1. Correctness and security regressions.
2. Backward-compatibility and migration clarity.
3. Test quality (including negative/fail-closed paths).
4. Operational observability and rollback safety.

For questions, open an issue or use Discord: <https://discord.com/invite/cv8EfJu3Tn>.
