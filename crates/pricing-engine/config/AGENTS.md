# config

## Purpose
Static TOML configuration files for the Tangle Pricing Engine gRPC server. Defines resource-based pricing rates (USD) for service creation quotes and per-job fixed prices (wei) for job execution quotes.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `default_pricing.toml` - Resource pricing rates per unit per second. Contains `[default]` fallback section and blueprint-specific override sections (`[1]`, `[2]`).
  - **Key items**: 9 resource kinds (CPU, MemoryMB, StorageMB, NetworkEgressMB, NetworkIngressMB, GPU, Request, Invocation, ExecutionTimeMS), `price_per_unit_rate` in USD
- `job_pricing.toml` - Per-job fixed prices in wei, indexed by `[service_id]` then `job_index`. No fallback; missing entries return NOT_FOUND.
  - **Key items**: service ID sections, job index keys, wei string values (U256-compatible)

## Key APIs (no snippets)
- **Loading**: `init_pricing_config(path)`, `init_job_pricing_config(path)` (async, returns `Arc<Mutex<HashMap>>`)
- **Parsing**: `load_pricing_from_toml()`, `load_job_pricing_from_toml()`
- **Calculation**: `calculate_price(profile, rates, blueprint_id, ttl_blocks, security_reqs)` -> `PriceModel`; formula: `count * rate * ttl_blocks * block_time(6s) * security_factor`

## Relationships
- **Depends on**: TOML format specification
- **Used by**: `src/lib.rs` (async loaders), `src/pricing.rs` (parsing/calculation), `src/service/rpc/server.rs` (gRPC handlers), `tests/pricing_config_test.rs`
- **Data/control flow**:
  - CLI specifies `--pricing-config` and `--job-pricing-config` paths
  - Server loads at startup, wraps in `Arc<Mutex>` for thread-safe access
  - `GetPrice` RPC looks up resource rates, runs benchmark, calculates cost
  - `GetJobPrice` RPC looks up fixed price from map, returns wei amount

## Files (detailed)

### `default_pricing.toml`
- **Role**: Resource pricing table with defaults and per-blueprint overrides.
- **Key items**: `[default]` section with 9 resources, `[1]` higher CPU + GPU, `[2]` premium tier
- **Interactions**: Parsed by `load_pricing_from_toml()`, fed to `calculate_price()`
- **Knobs / invariants**: Unconfigured resources get zero charge; blueprint-specific overrides fully replace defaults for that blueprint

### `job_pricing.toml`
- **Role**: Fixed per-job pricing in wei (native token), not USD.
- **Key items**: `[1]` service with jobs 0-7, values as strings for U256 support
- **Interactions**: Parsed by `load_job_pricing_from_toml()`, keyed by `(service_id, job_index)` tuple
- **Knobs / invariants**: No default fallback; missing entries return gRPC NOT_FOUND; job pricing config is optional (omitting disables job quotes)

## End-to-end flow
1. CLI args specify config file paths
2. Server loads TOML at startup into `Arc<Mutex<HashMap>>`
3. GetPrice: lookup rates -> benchmark profile -> calculate -> EIP-712 sign -> respond
4. GetJobPrice: lookup fixed price -> build JobQuoteDetails -> EIP-712 sign -> respond
5. All quotes include PoW anti-abuse and explicit expiry timestamps

## Notes
- Two pricing models: service creation (resource-based USD) vs job execution (fixed wei)
- Configs are immutable after startup; no hot-reload
- Resource pricing falls back to `[default]`; job pricing has no fallback
- All quotes are EIP-712 signed for on-chain verification
