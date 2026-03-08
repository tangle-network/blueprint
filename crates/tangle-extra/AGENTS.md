# tangle-extra

## Purpose
Producer/Consumer extras for Tangle v2 EVM-based blueprints. Provides the `TangleProducer` (polls `JobSubmitted` events), `TangleConsumer` (submits individual results), and `AggregatingConsumer` (BLS signature aggregation via HTTP service or P2P gossip). Also includes lifecycle automation keepers, metadata extractors, caching, and Tower layers.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Core modules: `producer.rs`, `consumer.rs`, `aggregating_consumer.rs`, `aggregation.rs`, `strategy.rs`, `cache.rs`, `layers.rs`, `extract/` (metadata extractors), `services/` (keeper background services), `job_quote.rs` (RFQ quote signing)
- [x] `tests/` - Integration and e2e tests: `aggregation_e2e.rs`, `anvil_integration.rs`, `producer.rs`

### Files
- `Cargo.toml` - Crate manifest (`blueprint-tangle-extra`); features: `std` (default), `aggregation` (HTTP-based), `p2p-aggregation` (gossip-based), `keepers` (lifecycle automation)
- `README.md` - Detailed documentation with architecture diagram, threshold types, API reference, and on-chain integration guide

## Key APIs (no snippets)
- `TangleProducer` - Polls for `JobSubmitted` events from Tangle v2 contracts and streams `JobCall`s
- `TangleConsumer` - Submits individual job results via `submitResult` contract call
- `AggregatingConsumer` - Collects BLS signatures from operators and submits aggregated results; configurable via `AggregationStrategy`
- `AggregationStrategy` - Enum: `HttpService(HttpServiceConfig)` or `P2PGossip(P2PGossipConfig)` for choosing aggregation transport
- `ThresholdType` - `CountBased` or `StakeWeighted` (matches on-chain `Types.ThresholdType`)
- `TangleLayer` - Tower layer for injecting Tangle-specific metadata into job calls
- `ServiceConfigCache` / `SharedServiceConfigCache` - Cached operator weights and service configuration with TTL
- `CacheSyncService` - Background service that keeps the cache in sync with on-chain state
- Extractors (in `extract/`): extract `call_id`, `service_id`, operator info, etc. from job call metadata
- Keepers (feature `keepers`): `EpochKeeper`, `RoundKeeper`, `StreamKeeper` for lifecycle automation
- `job_quote` (feature `keepers`): RFQ quote signing and verification for per-job pricing

## Relationships
- Depends on `blueprint-core`, `blueprint-std`, `blueprint-client-tangle` (Tangle contract client)
- Optional deps: `blueprint-tangle-aggregation-svc` (feature `aggregation`), `blueprint-networking` + `blueprint-networking-agg-sig-gossip-extension` (feature `p2p-aggregation`), `blueprint-keystore` + `alloy` (feature `keepers`)
- Re-exported by `blueprint-sdk` as `tangle` module (behind `tangle` feature)
- Works with `blueprint-runner` as the producer/consumer pair for Tangle v2 blueprints

## Notes
- Three aggregation modes: no aggregation (TangleConsumer), HTTP-based (via tangle-aggregation-svc), P2P gossip (via networking extension)
- Threshold configuration (count-based vs stake-weighted, basis points) matches on-chain Solidity contract interface
- Tests require an Anvil instance; the `anvil_integration.rs` tests are serial
