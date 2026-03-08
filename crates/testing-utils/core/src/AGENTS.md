# src

## Purpose
Core test utilities shared across all protocol-specific testing crates. Provides a generic `TestRunner` for wiring jobs and background services into a `BlueprintRunner`, a `TestEnv` trait for protocol-specific test environments, manifest reading helpers, and log initialization.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - `read_cargo_toml_file(path)` for manifest parsing; `setup_log()` tracing-subscriber initializer; re-exports `TestRunnerError` as `Error` and `TestRunner`
- `runner.rs` - `TestRunner<Ctx>` wrapping a `Router<Ctx>` and `BlueprintRunnerBuilder` with `add_job`, `add_background_service`, `qos_service`, `with_faas_executor`, and `run(context)` methods; `TestEnv` trait defining the protocol-agnostic test harness interface (`new`, `add_job`, `add_background_service`, `run_runner`)
- `error.rs` - `TestRunnerError` enum aggregating errors from clients, setup, execution, I/O, keystore, URL parsing, runner, auth, and bridge

## Key APIs
- `TestRunner::new(config, env)` - creates a runner with a `BlueprintConfig` and `BlueprintEnvironment`; auto-wires a `pending()` shutdown handler
- `TestRunner::add_job(job)` - registers a job at the next sequential index
- `TestRunner::add_background_service(service)` - registers a background service
- `TestRunner::qos_service(qos)` - integrates the unified QoS service as a background service with completion-channel bridging
- `TestRunner::run(context)` - consumes the runner, attaches context to the router, and executes
- `TestEnv` trait - protocol-agnostic interface: `type Config`, `type Context`, `new()`, `add_job()`, `add_background_service()`, `run_runner()`

## Relationships
- Depends on `blueprint_runner` for `BlueprintRunner`, `BlueprintConfig`, `BackgroundService`
- Depends on `blueprint_router` for `Router`
- Depends on `blueprint_qos` for QoS service integration
- Extended by `crates/testing-utils/eigenlayer/` (`EigenlayerBLSTestEnv`) and used by `crates/testing-utils/anvil/` (`BlueprintHarness`)
- Re-exported by protocol-specific testing crates as their base
