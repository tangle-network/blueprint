# pricing-engine

## Purpose
Pricing engine for Tangle Blueprint services (`blueprint-pricing-engine`). gRPC server that generates EIP-712 signed price quotes for service deployments and per-job execution. Supports two RFQ modes: service creation quotes (resource-based pricing with hardware benchmarks) and per-job quotes (configurable price maps). Operators run this alongside their blueprint node; consumers request quotes via gRPC and submit them on-chain.

## Contents (one hop)
### Subdirectories
- [x] `config/` - Default TOML pricing configurations: `default_pricing.toml` (resource-based rates per blueprint) and `job_pricing.toml` (per-job pricing by service/job index)
- [x] `proto/` - Protobuf service definition (`pricing.proto`) for the gRPC API
- [x] `src/` - Crate source: `app.rs` (bootstrap/lifecycle), `benchmark/` (CPU/memory/storage/GPU benchmarking with caching), `benchmark_cache.rs` (RocksDB-backed cache), `cloud/` (VM and FaaS pricing fetchers), `config.rs` (operator config), `handlers.rs` (blueprint event handlers), `pricing.rs` (price calculation models), `pow.rs` (SHA-256 proof-of-work for anti-abuse), `signer.rs` (EIP-712 signing), `service/` (blockchain event ingestion and `rpc/` gRPC server), `types.rs`, `utils.rs`, `tests.rs`
- [x] `tests/` - Integration tests: EVM listener, pricing config validation, signer roundtrip

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-pricing-engine`); lib + binary (`pricing-engine-server`); depends on `blueprint-core`, `blueprint-crypto`, `blueprint-keystore`, `blueprint-networking`, `blueprint-client-tangle`, `blueprint-tangle-extra`, Alloy EVM stack, `tonic`/`prost` for gRPC, `sysinfo`/`num_cpus` for benchmarking, `reqwest` for cloud pricing APIs, `sha2` for PoW
- `README.md` - Comprehensive documentation: RFQ modes, pricing configuration, EIP-712 signing details, CLI flags, security model
- `build.rs` - tonic-build protobuf compilation
- `operator.toml` - Sample operator configuration (database path, benchmark settings, keystore path, gRPC server settings, quote validity)

## Key APIs (no snippets)
- `PricingEngineService` / `run_rpc_server()` -- gRPC service handling `GetPrice` and `GetJobPrice` RPCs
- `calculate_price()` / `PriceModel` / `ResourcePricing` / `SubscriptionPricing` -- price computation from resource rates and benchmarks
- `OperatorSigner` / `SignedQuote` / `SignedJobQuote` -- EIP-712 signing of quote structs
- `BenchmarkCache` / `run_benchmark` / `run_benchmark_suite` -- hardware benchmarking with persistent caching
- `OperatorConfig` / `load_config_from_path()` -- operator configuration loading
- `generate_challenge()` / `generate_proof()` / `verify_proof()` -- proof-of-work anti-abuse system
- `init_benchmark_cache()` / `init_pricing_config()` / `init_job_pricing_config()` / `init_subscription_pricing_config()` -- initialization helpers

## Relationships
- Depends on `blueprint-core`, `blueprint-crypto`, `blueprint-keystore`, `blueprint-networking`, `blueprint-client-tangle`, `blueprint-tangle-extra`
- Optionally consumed by `blueprint-manager` (via `remote-providers` feature)
- Uses `blueprint-tangle-extra` keepers for on-chain event listening
- EIP-712 signing compatible with `blueprint-tangle-extra::job_quote::JobQuoteSigner`
- Feature `pricing-engine-e2e-tests` gates end-to-end test suite
