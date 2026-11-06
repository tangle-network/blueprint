# Blueprint remote providers

This crate owns provider implementations for Blueprint deployments, including
CPU, GPU, and Kubernetes workloads. The Blueprint SDK exposes it through its
`remote-providers` feature as `blueprint_sdk::remote`.

## GPU catalog

[blueprint-core's catalog](../core/gpu-catalog.json) owns class ids, hardware
properties, decimal GB per GPU, node caps, and count-specific provider SKU ids.
Use `blueprint_core::gpu::{CATALOG_JSON, catalog, find}` to read it without std.
Prices and current stock come from the provider's API.

Provider credential bindings and capabilities belong to the factory and adapter
traits. The binary's generated descriptors are the source for clients; a static
support matrix cannot establish that a provider implements a requested shape.
CPU and non-Blackwell deployment consumers remain part of this crate's scope.

## Interfaces and configuration

Read the [public exports](src/lib.rs), [adapter trait](src/infra/traits.rs), and
[manifest](Cargo.toml) for available methods and feature gates. Provider-specific
configuration is owned by its implementation. Keep credential values out of
argv, logs, and Git. `managed::BoundaryRequest` and `BoundaryResponse` own the
versioned process contract; `execute_boundary` runs the same managed SDK verbs.
The CLI catalog emits their derived schema and its digest. Credentials are
provider-tagged objects supplied on stdin; the process never discovers them
from an inherited environment.

The [rental design](../../docs/gpu-marketplace-strategy.md) defines managed
allocation, lease retirement, ranking, and the shared process boundary. Central
orchestration owns durable leases and billing. Blueprints share these provider
implementations.

TEE policy and workload binding are documented in [AGENTS.md](AGENTS.md) and
[attestation](src/attestation/). Read those contracts before changing enforcement.

## Verification

Use crate-scoped tests and the supported features from the manifest. Integration
proofs fake provider HTTP or process boundaries. Live cloud proofs require an
explicit budget and provider-confirmed retirement.
