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
argv, logs, and Git.

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

## Durable GPU allocation

Canonical `allocate` and no-create `recover_lease` require Unix private state and
checked native accounts. For deliberately new state, call
`initialize_lease_state_dir(path).await`; on restart, use
`with_lease_state_dir(path)`. Initialization never clobbers; opening never repairs
or imports provider metadata. The manager uses its data directory and owner ID.

`blueprint/lease-authorizations.json` preserves original request, public key,
quote, account bindings, and deadline before creation, without environment secrets,
private keys, or raw credentials. An attempted identity never creates again,
including after uncertain responses or proven absence; use a new identity for a
new attempt. Recovery checks every original account before returning `None` for
absence, validates actual billing against the original quote and caller cap,
and keeps the original deadline. Retirement checks private obligations before
listing: missing/changed accounts remain visible failures while correctly bound
leases clean up independently. Native metadata cannot extend retirement.

Without a supplied public key, fresh allocation returns a generated private key
once. Retain the matching key for recovery; lost access is not replaced. Checked
bindings come from `from_credentials`, `with_credentials`, or the environment
factory; custom `with_providers` alone cannot establish a context. Bindings digest
exact credentials, endpoint, region defaults and native configuration, not a
vendor-verified account ID. Changes, including same-account rotation, require
operator reconciliation. After state attachment, `only_providers` narrows fresh
offers while retaining original accounts for recovery and cleanup.

Carry latest SDK state, caller records and keys together when moving, after
stopping the old writer. OS locks do not coordinate independent copies; coherent
owner-controlled rollback is outside this boundary. Missing/corrupt state fails
closed. Lium's relative timeout is a coarse guard; a running `retire_expired`
reconciler enforces the original absolute deadline.

Custom `ManagedProviderAdapter::create` implementations preserve prepared
`ManagedMetadata` exactly; manifests and registry metadata identify the containing
release. Low-level quoted operations retain weaker restart guarantees. Legacy
Lium orphan retries preserve native timestamps and require exact original metadata
before another rent request.
