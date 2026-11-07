# GPU rental through the Blueprint SDK

The Blueprint SDK owns the [GPU catalog](../crates/core/gpu-catalog.json) and
Tangle's provider implementations. The manager and blueprints use the SDK's
rental layer. [GPU lease orchestration](https://github.com/tangle-network/agent-container/tree/develop/apps/orchestrator/src/gpu-leases)
uses the same implementations through the released `cargo-tangle` binary.

## Catalog contract

`blueprint_core::gpu::CATALOG_JSON` exports the catalog packaged with the SDK.
The JSON declares VRAM in decimal GB and memory bandwidth in GB/s.
Class ids are the GPU identifiers on requests, quotes, registration caps,
Router records, and inference recipes. VRAM per GPU uses decimal GB.

Each class records vendor, generation, VRAM, memory bandwidth, FP8 and FP4
support, interconnect, and the node count cap. `providerSkus` maps provider keys
to exact count strings; `*` represents a caller-selected count. A missing mapping
is unsupported. Hardware descriptions and SKU identifiers contain no prices.
Provider availability and price must be checked against its live API.

Provider names, credential bindings, and capabilities come from the SDK factory
and adapter traits. The released `cloud catalog --json` output generates the
central contract types. Consumers pin a catalog on blueprint's default branch;
they do not maintain a second hardware table.

## Rental contract

The shared resource vocabulary carries class, count, per-device VRAM, image,
ports, storage, lifetime, spot policy, and a spending cap. Node count limits come
from the catalog. Local operator hardware is measured per device rather than
inferred from host RAM.

The managed create/status/list/destroy operations are the provider primitive.
`allocate(spec, ttl)` adds per-lease SSH access and an original authorization in
explicit caller-owned private state; `recover_lease(spec, ttl)` only adopts existing
capacity. `retire_expired(namespace)` enforces the stored original deadlines.
The [state contract](../crates/blueprint-remote-providers/README.md#durable-gpu-allocation)
owns restart, account binding, and portability requirements. The manager uploads and
starts the blueprint it rents capacity for. Blueprints share the same adapters.

The process boundary is `cargo tangle cloud
catalog/quote/create/status/list/destroy --json`. Credentials and requests arrive
on stdin. Quotes return the ranked list from one SDK ranking function. Unknown
prices are unselectable; direct and aggregated offers are deduplicated by
underlying cloud, SKU, and region. Measured throughput can rank cost per token.

Central orchestration owns durable leases, renewal, reconciliation, metering,
and attach. Scoped provider ownership supports recovery after an interrupted
create. Worker and serving endpoints are distinct. Worker credential bootstrap
uses a one-time nonce; provider environment and user-data carry no worker token.

## Quotes, evidence, and delivery

Pricing-engine quotes bind GPU class, count, and duration. Resource prices use
integer wire values. tnt-core owns quote verification and the shared service
manager base. Existing quote entrypoints keep their verified compatibility.
Slashing origins must call the evidence verifier before accepting a claim.

SDK libraries publish through [release-plz](../.github/workflows/release-plz.yml).
The binary uses its [maintained release path](../.github/workflows/release.yml).
Consumers adopt released artifacts and verify the pinned checksums.
