# src

## Purpose
Implementation of the Tangle pricing engine, a service that calculates costs for blueprint service deployments based on resource requirements, hardware benchmarks, and cloud provider pricing. Supports competitive bidding in a decentralized marketplace with proof-of-work challenges, operator signing, and both per-job and subscription pricing models.

## Contents (one hop)
### Subdirectories
- [x] `benchmark/` - Hardware benchmarking suite covering CPU, GPU, memory, I/O, and network performance; produces `BenchmarkProfile` scores used for pricing calculations
- [x] `cloud/` - Cloud pricing data fetchers: `vm.rs` for VM instance pricing (`PricingFetcher`, `InstanceInfo`) and `faas.rs` for function-as-a-service pricing (`FaasPricing`, `FaasPricingFetcher`)
- [x] `service/` - Service layer with `blockchain/` (blockchain event listening for blueprint updates) and `rpc/` (gRPC server exposing `PricingEngineService` for job, subscription, and resource pricing queries)

### Files
- `lib.rs` - Crate root; declares all modules, re-exports key types, provides initialization functions for benchmark cache, pricing config, job pricing, and subscription pricing
- `main.rs` - Binary entry point for the standalone pricing engine service
- `app.rs` - Application lifecycle: operator signer init, config loading, blockchain listener spawning, event processor, shutdown coordination
- `config.rs` - `OperatorConfig` with TOML deserialization for RPC URLs, keystore, database path, and pricing config paths
- `pricing.rs` - Core pricing logic: `ResourcePricing`, `PriceModel`, `SubscriptionPricing`, `calculate_price`, and TOML config loaders for job/subscription/resource pricing
- `benchmark_cache.rs` - `BenchmarkCache` backed by sled DB for persisting benchmark results per blueprint
- `handlers.rs` - Event handler for `BlueprintCreated` events; runs benchmarks and caches results
- `pow.rs` - Proof-of-work challenge generation and verification for rate-limiting pricing queries
- `signer.rs` - `OperatorSigner` for signing price quotes; `SignedQuote` and `SignedJobQuote` types
- `types.rs` - `ResourceUnit` enum (CPU, Memory, Storage, GPU, Network, Custom) and related types
- `error.rs` - `PricingError` enum and `Result` alias
- `utils.rs` - Shared utility functions
- `tests.rs` - Unit tests for pricing logic

## Key APIs (no snippets)
- `calculate_price` - Computes total cost from resource requirements and pricing config
- `run_benchmark` / `run_benchmark_suite` - Executes hardware benchmarks
- `BenchmarkCache` - Sled-backed cache for benchmark results
- `PricingEngineService` / `run_rpc_server` - gRPC service for pricing queries
- `OperatorSigner` / `SignedQuote` - Cryptographic signing of price quotes
- `generate_proof` / `verify_proof` - Proof-of-work for rate limiting
- `init_benchmark_cache`, `init_pricing_config`, `init_job_pricing_config`, `init_subscription_pricing_config` - Initialization helpers

## Relationships
- Uses protobuf-generated types from `pricing_engine` module (built via `build.rs`)
- Depends on `blueprint-core` for logging
- Standalone service that operators run alongside the blueprint manager
