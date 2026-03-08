# rpc-calls

## Purpose
Lightweight metrics instrumentation crate (`blueprint-metrics-rpc-calls`) for tracking EVM JSON-RPC call performance. Records request duration histograms and request count counters, labeled by RPC method and client version.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Single-file implementation containing `RpcCallsMetrics` struct

### Files
- `Cargo.toml` - Crate manifest; sole dependency is the `metrics` crate; features: `std` (default)
- `CHANGELOG.md` - Release history
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `RpcCallsMetrics::new` - Registers `evm_rpc_request_duration_seconds` histogram and `evm_rpc_request_total` counter metric descriptions
- `RpcCallsMetrics::set_rpc_request_duration_seconds` - Records duration for a given RPC method and client version
- `RpcCallsMetrics::set_rpc_request_total` - Sets the absolute count for a given RPC method and client version

## Relationships
- Uses the `metrics` facade crate; requires a metrics recorder to be installed at runtime to capture data
- Designed for use by EVM client crates to instrument RPC calls
