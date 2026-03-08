# metrics

## Purpose
Metrics meta-crate (`blueprint-metrics`). Re-exports metrics instrumentation sub-crates under a single dependency. Currently contains only the `rpc-calls` sub-crate for EVM JSON-RPC call tracking.

## Contents (one hop)
### Subdirectories
- [x] `rpc-calls/` - Sub-crate (`blueprint-metrics-rpc-calls`) providing `RpcCallsMetrics` struct with histogram/counter instrumentation for EVM JSON-RPC request duration and total counts; depends on the `metrics` crate
- [x] `src/` - Single `lib.rs` that conditionally re-exports `blueprint_metrics_rpc_calls` as `rpc_calls`

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-metrics`); optional dep on `blueprint-metrics-rpc-calls`; feature `rpc-calls` (default)
- `README.md` - Brief description of the meta-crate and its re-exports

## Key APIs (no snippets)
- `rpc_calls::RpcCallsMetrics` -- registers and records `evm_rpc_request_duration_seconds` histogram and `evm_rpc_request_total` counter, labeled by method and client version

## Relationships
- `rpc-calls` sub-crate depends on the `metrics` facade crate
- Used by EVM-interacting crates that want standardized RPC metrics
- Follows the meta-crate pattern used elsewhere in the workspace (e.g., `blueprint-crypto`, `blueprint-stores`)
