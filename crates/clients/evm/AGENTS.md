# evm

## Purpose
Crate (`blueprint-client-evm`) providing a general-purpose EVM client for Tangle Blueprints. Wraps alloy provider functionality with instrumentation, metrics collection, and Blueprint-specific error handling for interacting with EVM-compatible chains.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Source code with `client.rs` (EVM client implementation), `error.rs` (error types), `instrumented_client.rs` (metrics-instrumented client wrapper with RPC call tracking), and `lib.rs` (module declarations).

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-client-core`, alloy crates (primitives, provider, transport, network, RPC types, pubsub), `blueprint-metrics-rpc-calls` for metrics, and `tokio`
- `README.md` - Crate documentation

## Key APIs
- EVM client wrapping alloy provider with Blueprint error handling
- `instrumented_client` -- metrics-instrumented wrapper tracking RPC call counts, latencies, and errors

## Relationships
- Depends on `blueprint-client-core` for shared client primitives
- Uses `blueprint-metrics-rpc-calls` for RPC call instrumentation
- Dev-dependencies include `blueprint-chain-setup-anvil` and `blueprint-anvil-testing-utils` for local Anvil-based integration testing
- Used by higher-level clients (Tangle, EigenLayer) and blueprint services that need direct EVM interaction
