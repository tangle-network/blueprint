# src

## Purpose
Low-level EVM client providing the `BackendClient` trait for block queries and an `InstrumentedClient` that wraps Alloy providers with RPC call metrics instrumentation.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root with `no_std` support, declaring `client`, `error`, and `instrumented_client` modules.
- `client.rs` - Defines the `BackendClient` trait with `block_number()` and `block_by_number()` async methods, plus an empty `Client` struct.
- `error.rs` - Error enum with Provider, InvalidAddress, Transaction, Contract, and Abi variants. Converts into `blueprint_client_core::error::Error`.
- `instrumented_client.rs` - `InstrumentedClient` struct wrapping HTTP or WS Alloy providers with `RpcCallsCollector` metrics. Provides instrumented methods for: block queries, transaction operations (send, receipt, by hash), account operations (balance, nonce, code), gas estimation, chain ID, fee history, logs/filters, subscriptions (new heads, logs, pending transactions), call/estimate, and raw JSON-RPC calls. All operations measure and record call duration.

## Key APIs
- `BackendClient` trait - abstract interface for block queries
- `InstrumentedClient::new_http()` / `new_ws()` - create instrumented provider connections
- `InstrumentedClient::block_number()` / `get_block_by_number()` / `send_transaction()` / `get_balance()` etc. - instrumented EVM operations
- `InstrumentedClientError` - connection and version errors

## Relationships
- Uses `alloy_provider` / `alloy_rpc_types_eth` for EVM interaction
- Uses `blueprint_metrics_rpc_calls::RpcCallsMetrics` for duration tracking of all RPC calls
- Error type converts into `blueprint_client_core::error::Error`
- Re-exported from `eigensdk` patterns; originally from the EigenLayer SDK
