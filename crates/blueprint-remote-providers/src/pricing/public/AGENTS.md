# public

## Purpose
Public cloud pricing aggregators that fetch live pricing data from external sources for AWS, Azure, and Vultr. Used to estimate infrastructure costs without requiring cloud provider credentials.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `VantageAggregator` and `VultrPublicPricing`
- `vantage.rs` - `VantageAggregator` fetching AWS and Azure instance pricing from instances.vantage.sh; parses `VantageInstance` records with hourly cost, CPU, memory; constants `AWS_URL` and `AZURE_URL`
- `vultr.rs` - `VultrPublicPricing` with 6 hardcoded `VultrPlan` entries (vc2 plans); provides static pricing without API calls

## Key APIs (no snippets)
- `VantageAggregator::fetch_aws()` / `fetch_azure()` - async methods returning `Vec<VantageInstance>` with live pricing
- `VantageInstance` - instance_type, vcpus, memory_gb, hourly_cost
- `VultrPublicPricing::plans()` - returns hardcoded `Vec<VultrPlan>` for Vultr vc2 instances
- `VultrPlan` - plan_id, vcpus, ram_mb, monthly_cost

## Relationships
- **Imports from**: `reqwest`, `serde`, `crate::core::error`
- **Used by**: `pricing/mod.rs` re-exports these aggregators for use by instance mappers and cost estimation logic
