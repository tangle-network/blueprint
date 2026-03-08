# src (rpc-calls)

## Purpose
Provides Prometheus-compatible metrics for EVM JSON-RPC calls, tracking request duration (histogram) and total request count (counter) with method and client version labels. Based on the EigenLayer SDK logging pattern.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Defines `RpcCallsMetrics` struct. On construction, registers `evm_rpc_request_duration_seconds` histogram and `evm_rpc_request_total` counter descriptions. Provides `set_rpc_request_duration_seconds` to record request duration and `set_rpc_request_total` to set absolute request counts, both labeled by method and client_version.

## Key APIs (no snippets)
- `RpcCallsMetrics::new` / `Default` - Registers metric descriptions with the `metrics` crate.
- `RpcCallsMetrics::set_rpc_request_duration_seconds` - Records RPC request duration histogram sample.
- `RpcCallsMetrics::set_rpc_request_total` - Sets absolute RPC request counter value.

## Relationships
- Depends on the `metrics` crate for metric registration and recording (histogram/counter).
- Standalone crate; can be used by any EVM client implementation to instrument RPC calls.
