# Operator RFQ pricing

The pricing engine returns operator-signed quotes for service creation and job
execution. Its gRPC contract is [pricing.proto](proto/pricing.proto); on-chain
verification lives in [tnt-core](https://github.com/tangle-network/tnt-core/tree/main/src/core).

`GetPrice` prices a service from resource configuration and node benchmarks,
or from subscription/event configuration. `GetJobPrice` prices a specific job,
requester and exact `inputs_hash`. The requester must be a nonzero address;
`inputs_hash` must be the hash of the inputs submitted on-chain.

## GPU job quotes

Set all three allocation fields on `GetJobPriceRequest`:

- `gpu_class`: a canonical id from the [SDK GPU catalog](../core/gpu-catalog.json).
- `gpu_count`: the number of GPUs, bounded by that catalog class's `max_per_node`.
- `duration_secs`: a nonzero rental duration.

Set `[gpu_pricing]` in the existing operator TOML file, mapping catalog ids to
positive decimal strings in wei per GPU-second. Both the standalone server and
embedded service constructors read these rates. Embedded callers can override
them with `PricingEngineService::with_gpu_pricing`. A GPU quote is the configured job base
fee plus `class_rate * gpu_count * duration_secs`; arithmetic overflow is refused.
The requested class must exist in the catalog and have a configured rate.
Unknown classes, invalid allocations and unavailable prices produce errors.
Operators sourcing cloud capacity should derive their class rates from the
SDK remote-provider quotes rather than maintaining provider price tables here.

The response echoes the exact allocation. Its signature commits to class, count,
duration, requester, service, job, price, confidentiality, timestamps and inputs.
The GPU-aware tnt-core `submitJobFromQuote` overload accepts the signed quote and
one `GpuResourceCommitment` shared by every quoted operator. The class on-chain
is the hash of the catalog id.

`blueprint_tangle_extra::job_quote` owns native quote signing and calldata:
`GpuResourceCommitment::new`, `JobQuoteSigner::sign_with_resources`, and
`encode_gpu_job_submission`. Calldata construction refuses mixed allocations or
inputs that differ from the signed hash.

Omitting class, count and duration preserves legacy fixed job pricing and the
legacy EIP-712 signature. Partial GPU allocations are refused. Existing on-chain
entrypoints and old quotes remain verifiable; a legacy signature cannot authorize
a GPU allocation.

## Wire prices

`ResourcePricing.price_per_unit` is an unsigned big-endian integer. Its scale is
owned by [the pricing conversion](src/utils.rs), which also produces the on-chain
amount. The previous floating-point protobuf field is reserved. Decimal resource
configuration is converted directly to integer atoms without passing through a
floating-point value.

Job prices are unsigned big-endian wei. Service `total_cost_rate` remains the
legacy display value; signing uses the exact Decimal-derived on-chain amount.

## Configuration and execution

[OperatorConfig](src/config.rs) owns CLI/environment configuration.
[Default resource configuration](config/default_pricing.toml) and
[job configuration](config/job_pricing.toml) show the maintained TOML shapes.
`load_job_pricing_from_toml` loads fixed job base fees. Programmatic operators
attach those with `with_job_pricing`; class rates come from operator configuration
or `with_gpu_pricing`.
A job without an explicit base-fee entry is unavailable, including GPU jobs.

For a standalone server, run `pricing-engine-server --help` for current options.
The engine reads its keystore, serves gRPC/gRPC-web and signs within the configured
chain and verifying-contract domain. TEE configuration controls whether the
operator can quote confidential execution and how its price changes.

## Testing

`tests/gpu_quote_rpc.rs` uses the public gRPC transport. It checks allocations,
class-specific prices, catalog caps, unavailable classes/rates, legacy pricing,
resource prices beyond floating-point integer precision, and signature binding.

The [on-chain RFQ integration suite](https://github.com/tangle-network/tnt-core/blob/main/test/tangle/JobPricingAndRFQ.t.sol)
redeems GPU and legacy quotes through the production contracts and rejects
class/count/duration substitutions, replay and legacy-signature confusion.

Run the crate checks and the public RPC integration target with the repository's
local Beelink gate.
