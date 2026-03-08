# src

## Purpose
Meta-crate source directory that conditionally re-exports metrics instrumentation modules for the Blueprint SDK. Currently exposes `blueprint-metrics-rpc-calls` when the `rpc-calls` feature is enabled (default).

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Conditional re-export bridge; when `rpc-calls` feature is enabled, re-exports `blueprint_metrics_rpc_calls` as the `rpc_calls` module.
  - **Key items**: `#[cfg(feature = "rpc-calls")]`, `pub use blueprint_metrics_rpc_calls as rpc_calls`

## Key APIs (no snippets)
- **Modules**: `blueprint_metrics::rpc_calls` - Conditional module providing `RpcCallsMetrics` for EVM RPC call instrumentation
- **Types** (via re-export): `RpcCallsMetrics` - Stateless facade to `metrics` crate; registers and records `evm_rpc_request_duration_seconds` (histogram) and `evm_rpc_request_total` (counter)

## Relationships
- **Depends on**: `blueprint-metrics-rpc-calls` (optional, gated on `rpc-calls` feature), `metrics` crate facade (v0.24.2)
- **Used by**: `blueprint-clients-evm` via `InstrumentedClient` for RPC latency/count tracking

## Files (detailed)

### `lib.rs`
- **Role**: Zero-cost conditional re-export of the rpc-calls metrics submodule.
- **Key items**: Single `#[cfg(feature = "rpc-calls")] pub use` statement
- **Interactions**: Consumers access via `blueprint_metrics::rpc_calls::RpcCallsMetrics`
- **Knobs / invariants**: `rpc-calls` feature enabled by default in parent Cargo.toml; disabling removes the module entirely

## End-to-end flow
1. Consumer declares `blueprint-metrics` dependency (rpc-calls feature on by default)
2. `RpcCallsMetrics::new()` registers metric descriptions with `metrics` crate
3. Per RPC call, `set_rpc_request_duration_seconds()` and `set_rpc_request_total()` record values
4. External metrics recorder (Prometheus, OpenTelemetry) collects time-series data

## Notes
- Minimal scope: actual metric logic resides in sibling `../rpc-calls/src/`
- Stateless design: `RpcCallsMetrics` holds no state, acts as facade
- Labels follow Prometheus naming conventions (`method`, `client_version`)
