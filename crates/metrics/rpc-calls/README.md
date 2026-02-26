# blueprint-metrics-rpc-calls

RPC metrics helpers for EVM/JSON-RPC interactions.

## What it tracks

- Request duration histogram: `evm_rpc_request_duration_seconds`
- Request count counter: `evm_rpc_request_total`

## Primary type

- `RpcCallsMetrics` for declaring and recording the above metrics with method/client labels.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/metrics/rpc-calls
