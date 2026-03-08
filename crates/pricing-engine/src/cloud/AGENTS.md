# cloud

## Purpose
Fetches real-time pricing data from cloud provider APIs for both FaaS (serverless) and VM instance types. Supports AWS, GCP, Azure, DigitalOcean, and Vultr. All pricing is fetched from live APIs with caching -- no hardcoded values.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module root re-exporting `FaasPricing`, `FaasPricingFetcher` from `faas` and `InstanceInfo`, `PricingFetcher` from `vm`.
- `faas.rs` - FaaS pricing fetcher with 1-hour cache. `FaasPricingFetcher` fetches from: AWS Price List API (no auth), GCP Cloud Billing Catalog API (requires `GCP_API_KEY` env var, falls back to documented rates), Azure Retail Prices API (no auth). `FaasPricing` struct holds per-GB-second memory cost, per-request cost, and per-vCPU-second compute cost. Includes `estimate_execution_cost` for calculating total execution cost.
- `vm.rs` - VM instance pricing fetcher. `PricingFetcher` with HTTP client and in-memory cache. `InstanceInfo` holds name, vCPUs, memory_gb, hourly_price. `find_best_instance` selects the cheapest instance meeting CPU/memory/price requirements. `get_instances` fetches instance lists by provider and region.

## Key APIs (no snippets)
- `FaasPricingFetcher::fetch_aws_lambda_pricing` / `fetch_gcp_functions_pricing` / `fetch_azure_functions_pricing` - Fetch serverless pricing per region.
- `FaasPricingFetcher::estimate_execution_cost` - Calculates cost from pricing, memory, duration, and request count.
- `PricingFetcher::find_best_instance` - Finds cheapest VM meeting resource requirements.
- `PricingFetcher::get_instances` - Fetches instance list for a provider/region.

## Relationships
- Uses `crate::error::{PricingError, Result}` for error handling.
- Uses `crate::types::CloudProvider` enum for provider identification.
- Depends on `reqwest` for HTTP requests, `tokio::sync::RwLock` for cache concurrency.
