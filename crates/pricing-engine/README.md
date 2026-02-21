# Operator RFQ Pricing Server

gRPC server that generates EIP-712 signed price quotes for Tangle services and jobs. Operators run this alongside their blueprint node. Service consumers request quotes via gRPC, then submit them on-chain.

## Two RFQ Modes

### Service Creation RFQ (`GetPrice`)

Used with `createServiceFromQuotes()` on the Tangle contract. The operator quotes a `totalCost` for running an entire service instance for a given TTL.

```
Consumer → GetPrice(blueprint_id, ttl_blocks) → Operator
Operator → signs QuoteDetails{totalCost, blueprintId, ttlBlocks, securityCommitments, resourceCommitments}
Consumer → createServiceFromQuotes(blueprintId, [signedQuotes], config, callers, ttl)
```

Price is computed from the operator's resource pricing config (`default_pricing.toml`) and node benchmarks. The engine automatically benchmarks CPU/memory/storage/GPU and multiplies by configured rates.

### Per-Job RFQ (`GetJobPrice`)

Used with `submitJobFromQuote()` on the Tangle contract. The operator quotes a specific price for a single job execution.

```
Consumer → GetJobPrice(service_id, job_index) → Operator
Operator → signs JobQuoteDetails{serviceId, jobIndex, price, timestamp, expiry}
Consumer → submitJobFromQuote(serviceId, jobIndex, inputs, [signedQuotes])
```

Price is looked up from the operator's per-job pricing config: a `(service_id, job_index) → price_in_wei` map.

## Pricing Configuration

### Resource pricing (`default_pricing.toml`)

Controls service creation quotes. Operators set per-resource rates in USD:

```toml
# Default rates for all blueprints
[default]
resources = [
  { kind = "CPU", count = 1, price_per_unit_rate = 0.001 },
  { kind = "MemoryMB", count = 1024, price_per_unit_rate = 0.00005 },
  { kind = "StorageMB", count = 1024, price_per_unit_rate = 0.00002 },
  { kind = "GPU", count = 1, price_per_unit_rate = 0.005 },
]

# Override for a specific blueprint ID
[42]
resources = [
  { kind = "CPU", count = 1, price_per_unit_rate = 0.0015 },
  { kind = "GPU", count = 2, price_per_unit_rate = 0.007 },
]
```

Supported resource kinds: `CPU`, `MemoryMB`, `StorageMB`, `NetworkEgressMB`, `NetworkIngressMB`, `GPU`, `Request`, `Invocation`, `ExecutionTimeMS`.

### Per-job pricing (`config/job_pricing.toml`)

Controls job RFQ quotes. Each section is a service ID, keys are job indices, values are prices in wei (strings for large numbers):

```toml
# Service 1
[1]
0 = "1000000000000000"       # Job 0: 0.001 ETH
1 = "5000000000000000"       # Job 1: 0.005 ETH
6 = "20000000000000000"      # Job 6: 0.02 ETH (e.g. LLM prompt)
7 = "250000000000000000"     # Job 7: 0.25 ETH (e.g. agent task)
```

Pass via CLI: `--job-pricing-config config/job_pricing.toml` or env `JOB_PRICING_CONFIG_PATH`. If not provided, `GetJobPrice` returns `NOT_FOUND` for all jobs.

For programmatic use (e.g. dynamic pricing in a custom operator binary), use `PricingEngineService::with_job_pricing()`:

```rust
let job_config = load_job_pricing_from_toml(&std::fs::read_to_string("job_pricing.toml")?)?;
let service = PricingEngineService::with_job_pricing(
    config, benchmark_cache, pricing_config,
    Arc::new(Mutex::new(job_config)),
    signer,
);
```

### Operator config (`operator.toml`)

```toml
# RocksDB path for benchmark cache
database_path = "data/pricing-engine"

# Benchmark settings
benchmark_duration = 60    # seconds per benchmark run
benchmark_interval = 5     # seconds between samples

# Operator keystore (k256 keypair for EIP-712 signing)
keystore_path = "data/keystore"

# gRPC server
rpc_bind_address = "0.0.0.0"
rpc_port = 50051
rpc_timeout = 30           # seconds
rpc_max_connections = 256

# How long signed quotes remain valid
quote_validity_duration_secs = 300
```

## Running

```bash
OPERATOR_HTTP_RPC=https://rpc.tangle.tools \
OPERATOR_WS_RPC=wss://rpc.tangle.tools \
OPERATOR_TANGLE_CONTRACT=0x... \
OPERATOR_RESTAKING_CONTRACT=0x... \
OPERATOR_STATUS_REGISTRY_CONTRACT=0x... \
cargo run -p blueprint-pricing-engine --bin pricing-engine-server
```

All CLI flags:

| Flag | Env | Description |
|------|-----|-------------|
| `--config` | `OPERATOR_CONFIG_PATH` | Path to `operator.toml` |
| `--pricing-config` | `PRICING_CONFIG_PATH` | Resource pricing table (TOML) |
| `--job-pricing-config` | `JOB_PRICING_CONFIG_PATH` | Per-job pricing table (TOML) |
| `--http-rpc-endpoint` | `OPERATOR_HTTP_RPC` | Tangle EVM HTTP RPC endpoint |
| `--ws-rpc-endpoint` | `OPERATOR_WS_RPC` | Tangle EVM WebSocket endpoint |
| `--blueprint-id` | `OPERATOR_BLUEPRINT_ID` | Blueprint ID to watch for activations |
| `--service-id` | `OPERATOR_SERVICE_ID` | Optional: fixed service ID to benchmark |
| `--tangle-contract` | `OPERATOR_TANGLE_CONTRACT` | ITangle proxy contract address |
| `--restaking-contract` | `OPERATOR_RESTAKING_CONTRACT` | MultiAssetDelegation contract |
| `--status-registry-contract` | `OPERATOR_STATUS_REGISTRY_CONTRACT` | OperatorStatusRegistry contract |

## How It Works Internally

1. **Bootstrap** — reads `operator.toml`, loads pricing table, initializes RocksDB benchmark cache, derives operator Ethereum address from k256 keystore
2. **Event ingestion** — polls `ITangle::ServiceActivated` / `ServiceTerminated` logs, enqueues benchmarks for each new activation
3. **Benchmark** — runs CPU/memory/storage/GPU benchmarks, caches profiles per blueprint
4. **RPC handling** — verifies proof-of-work, computes price (service quotes use benchmarks + resource rates; job quotes use the `JobPricingConfig` map)
5. **Signing** — hashes ABI-encoded structs with EIP-712 (`TangleQuote` domain, version `1`), signs with k256 ECDSA

## EIP-712 Signing Details

Both quote types use the same EIP-712 domain:

```
name:              "TangleQuote"
version:           "1"
chainId:           <chain_id>
verifyingContract: <tangle_proxy_address>
```

**Service quotes** use `QUOTE_TYPEHASH`:
```
QuoteDetails(uint64 blueprintId,uint64 ttlBlocks,uint256 totalCost,uint64 timestamp,uint64 expiry,AssetSecurityCommitment[] securityCommitments)
```

**Job quotes** use `JOB_QUOTE_TYPEHASH`:
```
JobQuoteDetails(uint64 serviceId,uint8 jobIndex,uint256 price,uint64 timestamp,uint64 expiry)
```

For standalone signing without the full pricing engine, use `JobQuoteSigner` from `blueprint-tangle-extra`:

```rust
use blueprint_tangle_extra::job_quote::{JobQuoteSigner, JobQuoteDetails, QuoteSigningDomain};

let signer = JobQuoteSigner::new(keypair, QuoteSigningDomain { chain_id, verifying_contract });
let signed = signer.sign(&JobQuoteDetails {
    service_id: 1,
    job_index: 7,
    price: U256::from(250_000_000_000_000_000u64),
    timestamp: now,
    expiry: now + 3600,
});
```

## For Blueprint Developers

If your blueprint uses RFQ pricing (instead of or alongside fixed `setJobEventRates`):

1. **Embed or run the pricing engine** — either integrate `PricingEngineService` into your operator binary or run it as a sidecar
2. **Define job prices** — populate `JobPricingConfig` with `(service_id, job_index) → price` entries based on your blueprint's job types and cost structure
3. **Expose operator config** — let operators tune prices via your blueprint's config file (model costs, resource multipliers, margins)
4. **Fixed rates vs RFQ** — you can use both: set `setJobEventRates()` on-chain for standard pricing, and support RFQ for jobs where operators want custom pricing (premium models, large batches, etc.)

The on-chain protocol handles verification, replay protection, payment collection, and distribution. See `tnt-core/docs/PRICING.md` for the full protocol-level pricing reference.

## Testing

```bash
# Signer roundtrip + EIP-712 compatibility
cargo test -p blueprint-pricing-engine signer_test

# RPC server unit tests (18 tests covering success, errors, signature verification)
cargo test -p blueprint-pricing-engine -- server

# All tests
cargo test -p blueprint-pricing-engine
```

## Security

- **Proof-of-work** — SHA-256 challenge prevents gRPC abuse
- **EIP-712 signatures** — quotes are signed with the operator's k256 key and verified on-chain by the Tangle contract
- **Replay protection** — each quote digest is marked as used on-chain after submission
- **Expiry** — quotes have explicit `expiry` timestamps; the `maxQuoteAge` protocol parameter (default 1 hour) rejects stale quotes
- Keystore keys never leave disk. Keep keystore path and contract addresses private.
