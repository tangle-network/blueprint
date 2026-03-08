# runner

## Purpose
Execution runtime for Blueprint jobs. Owns the long-running event loop: consumes `JobCall`s from producers, routes them through the `Router`, and forwards `JobResult`s to consumers. Also hosts background services (webhooks, x402 gateways, metrics server, etc.) and handles protocol-specific registration.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Core runner implementation: builder, config, protocol-specific modules (eigenlayer, tangle, symbiotic), FaaS support, metrics server, error types
- [x] `tests/` - Integration tests (`runner_tests.rs`)

### Files
- `Cargo.toml` - Crate manifest (`blueprint-runner`); feature-gated protocol support (eigenlayer, tangle, symbiotic), networking, TEE, TLS, FaaS
- `CHANGELOG.md` - Release history
- `README.md` - Usage overview and minimal setup example

## Key APIs (no snippets)
- `BlueprintRunner` - Builder-pattern runner that wires producers, consumers, router, background services, and optional protocol config; `.run()` drives the main loop
- `BlueprintConfig` trait (+ `DynBlueprintConfig`) - Protocol registration hook; implementations handle on-chain operator registration
- `BackgroundService` trait (+ `DynBackgroundService`) - Long-lived tasks (webhooks, x402 gateway, metrics) managed alongside the runner
- `BlueprintEnvironment` (in `config` module) - CLI-parsed environment: RPC URLs, keystore path, protocol settings, service ID
- `error::RunnerError` / `JobCallError` / `ProducerError` - Error hierarchy for the runner lifecycle

## Relationships
- Depends on `blueprint-core` (JobCall/JobResult primitives), `blueprint-router` (request dispatch), `blueprint-std`, `blueprint-keystore`, `blueprint-crypto`, `blueprint-qos` (heartbeat)
- Optional deps: `blueprint-networking` (P2P), `blueprint-tee` (TEE mode), `blueprint-auth` (TLS registration), `blueprint-faas` (FaaS execution), `blueprint-client-tangle` (Tangle v2), `blueprint-evm-extra` + `eigensdk` (EigenLayer)
- Re-exported by `blueprint-sdk` as `runner` module
- Consumed by all blueprint binaries as the top-level execution driver

## Notes
- Default features: `std` + `networking`
- Protocol features (`eigenlayer`, `tangle`) each pull in their own signing and client dependencies
- `symbiotic` feature is defined but currently unused
