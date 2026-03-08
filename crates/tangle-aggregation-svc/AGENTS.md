# tangle-aggregation-svc

## Purpose
HTTP-based BLS signature aggregation service for Tangle v2. Collects BLS signatures from operators, aggregates them once a threshold is met, and provides the aggregated result for on-chain submission to Tangle contracts.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Service implementation: HTTP API routes (`api.rs`), aggregation logic (`service.rs`), task state management (`state.rs`), persistence backends (`persistence.rs`), optional HTTP client (`client.rs`), shared types (`types.rs`)
- [x] `tests/` - Integration tests (`integration.rs`)

### Files
- `Cargo.toml` - Crate manifest (`blueprint-tangle-aggregation-svc`); optional `client` feature (adds `reqwest`-based HTTP client)
- `README.md` - Flow overview and API endpoint documentation

## Key APIs (no snippets)
- `AggregationService` - Core service that manages task lifecycle, signature collection, and aggregation
- `ServiceConfig` - Configuration for the aggregation service (thresholds, timeouts, etc.)
- `run(addr, config)` - Convenience function to start the HTTP service on a given address
- `api::router(service)` - Builds the axum router with all endpoints
- HTTP endpoints: `POST /v1/tasks/init`, `POST /v1/tasks/submit`, `POST /v1/tasks/status`, `POST /v1/tasks/aggregate`
- `TaskState`, `TaskStatus`, `AggregationState`, `OperatorInfo`, `ThresholdType` - State management types
- `FilePersistence`, `NoPersistence`, `PersistenceBackend` - Pluggable persistence for task state
- `AggregationServiceClient` (feature `client`) - HTTP client for interacting with the service
- `CleanupWorkerHandle` - Background task for expired task cleanup

## Relationships
- Depends on `blueprint-crypto-bn254` and `blueprint-crypto-core` for BLS cryptography (ark-bn254)
- Depends on `blueprint-client-tangle` for Tangle contract interactions
- Uses `alloy-*` crates for EVM types and signing
- Consumed by `blueprint-tangle-extra` (feature `aggregation`) as the HTTP aggregation backend
- The `client` module is used by operators to submit signatures and query aggregation results

## Notes
- Operator flow: (1) initialize task, (2) each operator signs and submits, (3) fetch aggregated result once threshold met, (4) submit to on-chain contract
- Supports both count-based and stake-weighted threshold types
- Not part of the workspace lint/edition inheritance -- uses its own edition = "2021"
