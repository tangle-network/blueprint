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

`CloudProvisioner::allocate` and the no-create `recover_lease` require explicit
private SDK state on Unix (other platforms fail closed without a protected-file
implementation). For a deliberately new caller state directory, call
`initialize_lease_state_dir(path).await`; on every subsequent start use
`with_lease_state_dir(path)`. Initialization refuses an existing file; opening
never initializes, repairs, or imports authorizations from provider metadata.
At least one checked native account is required before initialization or attachment;
unsupported configurations create no SDK files. The manager wires this to its
existing data directory and persisted owner ID.

The private `blueprint/lease-authorizations.json` binds the normalized request,
public SSH key, original candidate/price/deadline, and configured account digests
before native creation. It contains no private SSH key, workload environment, or
raw account credential. Once an attempt is recorded, absence or an uncertain
response never authorizes another create for that lease identity. Use a new
identity for a new intentional attempt. `recover_lease` never requests capacity;
`None` means all original configured accounts were successfully inspected.
An unavailable account remains an error. Recovery checks the actual billed rate
against the original quote and caller cap, and keeps the original deadline.
`retire_expired` uses that deadline even if mutable native metadata extends it.
It checks original account obligations before listing; removed or changed accounts
remain visible failures even when a new account lists nothing. Correctly bound
authorizations continue cleaning up independently of failed historical bindings.

Fresh allocation can generate an SSH key and return its private half once.
Recovery requires the matching caller-owned public key; a lost generated private
key cannot be replaced and is never persisted by the SDK. Keep the key yourself
if recovery access is required. Exact credential/config digests identify inputs,
not vendor-verified accounts: changing credentials, endpoint, region defaults, or
native launch configuration fails recovery even for a same-account rotation.
Operator reconciliation is required; the SDK does not infer account continuity.
Use `from_credentials`, `with_credentials`, or the environment factory for checked
native bindings; custom `with_providers` alone cannot authorize strong recovery.
Calling `only_providers` after attaching state selects fresh offers while retaining
original accounts for recovery and cleanup.

Carry the latest SDK state, caller records, and caller-owned key together when
moving hosts, after stopping the old writer. OS locks coordinate one shared
filesystem, not independently copied stores. Missing/corrupt state fails closed;
coherent owner-controlled rollback is outside this storage boundary. Lium's
relative `termination_hours` is a coarse vendor guard; SDK retirement is the
original absolute deadline and requires a running reconciler.

This changes the allocation default and the `ManagedProviderAdapter::create`
argument to prepared `ManagedMetadata`; custom implementations must migrate and
preserve it exactly. `ManagedResource` and its native allocation fingerprint are
unchanged. The low-level quoted create/status/list/destroy path remains available
without strong restart guarantees. These APIs require a new SDK release; read
manifests and registry metadata for the containing release before adopting it.
