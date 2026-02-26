# blueprint-tangle-aggregation-svc

BLS signature aggregation service for Tangle workflows.

## What it includes

- HTTP API for task init, signature submission, status, and aggregate retrieval.
- Aggregation service/state types and persistence backends.
- Optional client module for interacting with the service.

## Typical flow

1. Initialize aggregation task.
2. Collect signatures from operators.
3. Read aggregated result once threshold is met.
4. Submit aggregate to on-chain consumer.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/tangle-aggregation-svc
