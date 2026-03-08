# src

## Purpose
Implementation of the `blueprint-runner` crate, the core job execution engine. Provides `BlueprintRunner` and its builder for configuring and running blueprints with protocol-specific producers/consumers, background services, optional FaaS execution, metrics, and TEE integration. Supports Tangle, EigenLayer, and Symbiotic protocols via feature-gated modules.

## Contents (one hop)
### Subdirectories
- [x] `eigenlayer/` - EigenLayer protocol support: BLS/ECDSA key registration (`bls.rs`, `ecdsa.rs`), `EigenlayerProtocolSettings` configuration, and protocol-specific error types
- [x] `symbiotic/` - Symbiotic protocol support (currently a placeholder module)
- [x] `tangle/` - Tangle protocol support: `TangleProtocolSettings` configuration and protocol-specific error types

### Files
- `lib.rs` - Crate root; defines `BlueprintConfig` trait (registration/environment hooks), `BlueprintRunner` struct with the main execution loop (producer/consumer stream processing, concurrent job dispatch, result submission), `BlueprintRunnerBuilder` for fluent configuration, `BackgroundService` trait, and `DynBlueprintConfig`
- `config.rs` - Configuration types: `BlueprintSettings` (clap-derived CLI args), `BlueprintEnvironment` (runtime config with bridge, keystore, context, supported chains), `ProtocolSettingsT` trait, `Protocol` enum (Tangle/Eigenlayer/Symbiotic), `ContextConfig`
- `error.rs` - `RunnerError` enum covering configuration, producer, consumer, job call, shutdown, and protocol-specific errors
- `faas.rs` - FaaS execution abstraction; re-exports `blueprint-faas` types when `faas` feature is enabled, provides stubs otherwise
- `metrics_server.rs` - `MetricsServerAdapter` implementing `BackgroundService` for running a Prometheus metrics server alongside the runner

## Key APIs (no snippets)
- `BlueprintRunner::builder(config, env)` - Create a runner builder
- `BlueprintRunnerBuilder` - Fluent API: `.router()`, `.producer()`, `.consumer()`, `.background_service()`, `.with_shutdown_handler()`, `.run()`
- `BlueprintConfig` trait - Protocol registration and environment setup hooks
- `BackgroundService` trait - Interface for long-running services managed by the runner
- `BlueprintEnvironment` - Runtime environment with bridge client, keystore, protocol settings
- `Protocol` enum - Supported protocol variants (Tangle, Eigenlayer, Symbiotic)

## Relationships
- Depends on `blueprint-router` for job dispatch
- Depends on `blueprint-qos` for heartbeat and metrics integration
- Depends on `blueprint-manager-bridge` for bridge client communication
- Depends on `blueprint-keystore` for key management
- Protocol modules depend on their respective client crates (`blueprint-client-tangle`, `eigensdk`)
- Consumed by all blueprint services and testing utilities
