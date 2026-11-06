# Operator RFQ pricing

Run the pricing server beside an operator blueprint, or embed
`PricingEngineService` in its binary. Consumers request quotes over gRPC or
gRPC-Web and submit the signed result through Tangle's RFQ entry points.

## Service and job quotes

`GetPrice` prices service creation from benchmark profiles and configured resource
rates. `GetJobPrice` prices one job from `JobPricingConfig`. Both bind the
requester; job quotes also bind the hash of the exact submitted inputs. A missing
job rate returns `NOT_FOUND`.

The [protobuf definition](proto/pricing.proto) owns the request/response fields.
The [configuration loaders](src/pricing.rs) own resource, subscription, TEE and
per-job pricing input. The [operator configuration](src/config.rs) owns server,
keystore, quote validity and GPU class rates. Rates are operator configuration,
separate from live cloud rental prices.

## GPU job quotes

Provide `gpu_class`, `gpu_count` and `duration_secs` together in `GetJobPrice`.
Class ids and count caps come from the [SDK catalog](../core/gpu-catalog.json).
The operator's `gpu_pricing` TOML table maps each supported class id to a positive
decimal-string integer rate in the settlement asset's smallest units per
GPU-second. The quoted job price is its configured base plus this rate multiplied
by count and duration. Arithmetic overflow fails the request.

A GPU quote signs `GpuJobQuoteDetails` with a nested `GpuResourceCommitment`.
Class, count and duration are returned in the signed quote details. Unknown
classes or invalid allocations are refused; a known class without an operator
rate returns `NOT_FOUND`. Omitting the allocation retains the legacy job schema.

`confidentiality` uses the shared
[`job_quote::Confidentiality`](../tangle-extra/src/job_quote.rs) enum, matching
Tangle's four policies. An explicit policy cannot weaken `require_tee`. Required
TEE fails when unavailable; preferred TEE can use standard execution.

## Exact resource prices

`ResourcePricing.price_per_unit` contains USD18 as exactly 32 big-endian bytes.
Zero is 32 zero bytes. Negative rates and amounts finer than USD18 are refused.
The old floating resource-price field is reserved; regenerate clients from the
proto when adopting this wire change. The legacy service `total_cost_rate`
display field remains a double. Job quote prices are integer base-unit amounts.

The [public gRPC proof](tests/gpu_quote_rpc.rs) verifies the transmitted signatures
with an independent JSON EIP-712 encoder and exercises allocation tampering,
integer precision, invalid requests and policy binding.

## Standalone signing

Enable `blueprint-tangle-extra`'s `keepers` feature and use
[`JobQuoteSigner`](../tangle-extra/src/job_quote.rs). `sign` retains the legacy
job type; `sign_with_resources` accepts a validated `GpuResourceCommitment`.
The [Tangle signature library](https://github.com/tangle-network/tnt-core/blob/main/src/libraries/SignatureLib.sol)
owns the protocol's typed-data names and domain. GPU submission uses the existing
signed job tuple plus the allocation as the overload's fifth argument.

## Running

`pricing-engine-server --help` lists the required operator configuration, pricing
files and chain endpoints. The [binary](src/main.rs) owns its CLI and environment
bindings. Programmatic callers use `PricingEngineService::with_job_pricing`,
with the same operator configuration and signer as the standalone server.

## VM provider prices

[`cloud/vm.rs`](src/cloud/vm.rs) owns VM rate requests. The remote providers
crate re-exports `PricingFetcher`. Each request reads the provider API; it
keeps no price cache and has no handwritten rate or spot multiplier.

Use `with_digitalocean_token`, `with_vultr_token`, or, with the optional `aws`
feature, `with_aws_config`. Library callers supply credentials; the CLI uses
an explicit environment bridge. Credentials are redacted and rejected
credentials remain errors. `with_endpoint` accepts an HTTPS gateway or a
loopback HTTP peer; AWS gateways are supplied in its SDK config.

`quote_instance(provider, region, native_sku)` returns a total in USD per
node-hour. `None` means that allocation cannot be priced. AWS uses signed
[GetProducts](https://docs.aws.amazon.com/aws-cost-management/latest/APIReference/API_pricing_GetProducts.html),
Azure uses the [Retail Prices API](https://learn.microsoft.com/rest/api/cost-management/retail-prices/azure-retail-prices),
and DigitalOcean uses [Sizes](https://docs.digitalocean.com/reference/api/reference/sizes/).
GCP machine types have no complete component SKU mapping here.
[Vultr plans](https://github.com/vultr/govultr/blob/master/plan.go) expose a
monthly amount without a complete regional hourly binding. Both remain
unpriced; monthly caps are never converted into guessed hourly quotes.
