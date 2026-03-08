# rpc

## Purpose
gRPC pricing service implementation. Exposes a tonic-based `PricingEngine` RPC server that computes resource-based quotes, per-job prices, and subscription pricing with PoW (proof-of-work) anti-spam protection and operator-signed quotes. Optionally includes x402 cross-chain payment settlement options.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declaration for `server`.
- `server.rs` - Full gRPC service implementation.
  - `PricingEngineService` holds `OperatorConfig`, `BenchmarkCache`, per-blueprint resource pricing configs, per-job pricing map (`(service_id, job_index) -> U256`), subscription pricing config, `OperatorSigner`, PoW difficulty, and optional `X402SettlementConfig`.
  - Implements `PricingEngine` tonic trait with `get_price()` (resource-based quotes with benchmark-adjusted pricing, PoW challenge/verify, operator signature) and `get_job_price()` (per-job pricing lookup with signed quotes and optional x402 settlement options).
  - `X402SettlementConfig` and `X402AcceptedToken` mirror `blueprint-x402` types to avoid cyclic dependencies; includes CAIP-2 network identifiers, exchange rates, and markup.
  - `SubscriptionPricingConfig` maps optional blueprint IDs to `SubscriptionPricing`.
  - `run_pricing_server()` starts the tonic server with CORS support on a configurable address.

## Key APIs (no snippets)
- **Types**: `PricingEngineService`, `X402SettlementConfig`, `X402AcceptedToken`, `JobPricingConfig`, `SubscriptionPricingConfig`
- **RPC methods**: `get_price(GetPriceRequest) -> GetPriceResponse`, `get_job_price(GetJobPriceRequest) -> GetJobPriceResponse`
- **Functions**: `run_pricing_server()`, `PricingEngineService::new()`, `::new_with_configs()`, `::with_x402_config()`, `::with_pow_difficulty()`

## Relationships
- **Depends on**: `crate::pricing` (price calculation functions), `crate::pow` (PoW challenge/verify), `crate::signer` (`OperatorSigner`, `SignableQuote`), `crate::benchmark_cache`, `crate::config::OperatorConfig`, `tonic` (gRPC server), `tower-http` (CORS)
- **Used by**: Blueprint operators running pricing services; clients query quotes before submitting jobs
