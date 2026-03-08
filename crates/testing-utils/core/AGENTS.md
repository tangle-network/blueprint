# core

## Purpose
Core testing primitives crate (`blueprint-core-testing-utils`) providing the foundational `TestRunner` abstraction and shared utilities used by all protocol-specific testing utility crates. Handles runner construction, job registration, response waiting, and logging setup.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Implementation: `TestRunner` struct for building and running blueprint test scenarios, `TestRunnerError` type, `read_cargo_toml_file` helper, and `setup_log` for test logging initialization

### Files
- `Cargo.toml` - Crate manifest; depends on `blueprint-runner`, `blueprint-router`, `blueprint-qos`, `blueprint-auth`, `blueprint-clients`, `blueprint-keystore`, `blueprint-manager-bridge`, `cargo_toml`, `tracing-subscriber`
- `CHANGELOG.md` - Release history
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `TestRunner::new(config, env)` - Create a test runner with a blueprint config and environment
- `TestRunner::add_job(job)` - Register a job handler with auto-incrementing job IDs
- `TestRunnerError` - Error type for test runner failures
- `read_cargo_toml_file(path)` - Parse a Cargo.toml manifest for test configuration
- `setup_log()` - Initialize tracing subscriber with env-filter for test output

## Relationships
- Foundation for `blueprint-anvil-testing-utils` and `blueprint-eigenlayer-testing-utils`
- Wraps `BlueprintRunner` and `Router` from the runner and router crates
- Provides the `Error` type re-exported by protocol-specific testing crates
