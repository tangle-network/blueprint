# src

## Purpose
Implementation of the `blueprint-tangle-extra` crate, providing the producer/consumer pattern for processing jobs on Tangle v2 EVM contracts. Includes job event polling, result submission, metadata extraction, signature aggregation, caching, and lifecycle automation services (keepers).

## Contents (one hop)
### Subdirectories
- [x] `extract/` - Job call metadata extractors: `TangleArg`/`TangleResult` for ABI-encoded arguments, plus extractors for `call_id`, `service_id`, `caller` address, `block_number`, `block_hash`, and `timestamp` from job call metadata
- [x] `services/` - Lifecycle automation keepers (feature-gated `keepers`): `EpochKeeper` for epoch transitions, `RoundKeeper` for round management, `StreamKeeper` for data streaming, `BillingService` for billing operations; all implement `BackgroundKeeper` trait

### Files
- `lib.rs` - Crate root; declares all modules, conditionally enables `job_quote` and `services` with `keepers` feature, re-exports key types
- `producer.rs` - `TangleProducer` that polls `JobSubmitted` EVM events and converts them into a `JobCall` stream
- `consumer.rs` - `TangleConsumer` that submits job results via the `submitResult` contract function
- `aggregating_consumer.rs` - `AggregatingConsumer` that collects results from multiple operators and submits aggregated signatures; `AggregationServiceConfig` for HTTP/P2P strategies
- `aggregation.rs` - Signature aggregation types: `AggregatedResult`, `G1Point`, `G2Point`, `SignerBitmap`, `AggregationError`
- `strategy.rs` - `AggregationStrategy` enum (HTTP or P2P gossip), `ThresholdType`, `StrategyError`
- `cache.rs` - `ServiceConfigCache` for caching operator weights and service configuration; `CacheSyncService` for background cache invalidation; `SharedServiceConfigCache` alias
- `layers.rs` - `TangleLayer` Tower middleware that injects Tangle-specific metadata into job calls
- `job_quote.rs` - Per-job RFQ quote signing and verification (feature-gated `keepers`)

## Key APIs (no snippets)
- `TangleProducer` - Stream-based producer polling for on-chain job events
- `TangleConsumer` - Sink-based consumer submitting results to contracts
- `AggregatingConsumer` - Multi-operator result aggregation and submission
- `TangleLayer` - Tower layer injecting Tangle metadata into the job pipeline
- `ServiceConfigCache` / `CacheSyncService` - Cached operator and service configuration
- `TangleArg` / `TangleResult` extractors (in `extract/`)
- `EpochKeeper`, `RoundKeeper`, `StreamKeeper` (in `services/`)

## Relationships
- Depends on `blueprint-client-tangle` for EVM contract interactions
- Depends on `blueprint-crypto` for aggregation (BLS/BN254 when `aggregation` feature is enabled)
- Used by `blueprint-runner` with Tangle protocol as the default producer/consumer
- Used by `blueprint-anvil-testing-utils` for test harness setup
- `TangleLayer` and extractors are the primary interface for blueprint job authors
