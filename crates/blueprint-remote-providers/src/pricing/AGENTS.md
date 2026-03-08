# pricing

## Purpose
Pricing and cost estimation for multi-cloud deployments. Bridges the `blueprint-remote-providers` resource model with the `blueprint-pricing-engine` crate, providing cost calculations via real provider APIs or TOML configuration files. All hardcoded pricing has been deprecated.

## Contents (one hop)
### Subdirectories
- [x] `public/` - Public cloud pricing aggregators (no auth required): `VantageAggregator` for AWS/Azure pricing from instances.vantage.sh, `VultrPublicPricing` with hardcoded plan fallbacks

### Files
- `mod.rs` - Module orchestration; re-exports `FaasPricing`, `FaasPricingFetcher`, `InstanceInfo`, `PricingFetcher` from `blueprint-pricing-engine-lib` and local modules.
  - **Key items**: `pub use blueprint_pricing_engine_lib::*`, `pub mod public`, `pub mod cost`, `pub mod service`, `pub mod integration`
- `cost.rs` - `CostEstimator` and `CostReport` types. Deprecated: all methods return errors directing to real pricing APIs.
  - **Key items**: `CostEstimator`, `CostReport`, `estimate()`, `track_usage()`, `alert_if_exceeds()`
- `service.rs` - `PricingService` unified service. Deprecated: methods return errors.
  - **Key items**: `PricingService`, `calculate_cost()`, `CostReport`
- `integration.rs` - `PricingCalculator` integrating with pricing engine via TOML config files; provides provider comparison.
  - **Key items**: `PricingCalculator::from_config_file()`, `calculate_cost()`, `compare_providers()`, `DetailedCostReport`, `ResourceUsageMetrics`
  - **Interactions**: Loads TOML pricing configs, converts `ResourceSpec` to pricing units, supports spot instance discounts (0.7x)

## Key APIs (no snippets)
- **Types**: `PricingCalculator` (config-driven, only via `from_config_file()`), `DetailedCostReport` (with `summary()` and `exceeds_threshold()`), `CostEstimator` (deprecated), `PricingService` (deprecated)
- **Re-exports**: `PricingFetcher`, `FaasPricingFetcher`, `InstanceInfo`, `FaasPricing` from `blueprint-pricing-engine-lib`
- **Functions**: `pricing_engine_compat::to_resource_units()`, `create_benchmark_profile()`

## Relationships
- **Depends on**: `blueprint-pricing-engine` (source of truth for pricing), `crate::core` (CloudProvider, ResourceSpec), `reqwest`, `serde`, `toml`, `chrono`
- **Used by**: `crate::providers::aws::instance_mapper` (find best instance), `crate::infra::auto` (auto-deployment cheapest provider selection), `crate::lib` (public API re-exports)

## Notes
- `CostEstimator` and `PricingService` are deprecated stubs returning errors; use `PricingFetcher`/`FaasPricingFetcher` instead
- `PricingCalculator::new()` returns error; must use `from_config_file()` with TOML config
- Vantage API provides AWS/Azure pricing; GCP not available on Vantage
- Vultr has 6 hardcoded fallback plans; prefer `PricingFetcher` for live data
