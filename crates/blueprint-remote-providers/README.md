# Blueprint remote providers

Rent cloud GPUs from a blueprint or the Blueprint Manager. The public SDK
exports this crate as `blueprint_sdk::remote` with its `remote-providers` feature.

The [GPU catalog](../core/gpu-catalog.json) owns GPU classes and provider SKUs.
`ResourceSpec` preserves GPU class, count, and per-device VRAM separately from
system RAM. A provider's fixed node size must match the request exactly.

## Rent with caller-owned credentials

```rust,no_run
use blueprint_remote_providers::{CloudProvider, CloudProvisioner, ResourceSpec};
use blueprint_remote_providers::infra::traits::CloudProviderAdapter;
use blueprint_remote_providers::providers::runpod::RunPodAdapter;
use std::{collections::HashMap, time::Duration};

# async fn rent(api_key: String) -> Result<(), Box<dyn std::error::Error>> {
let adapter = RunPodAdapter::with_credentials(api_key)?;
let providers = HashMap::from([
    (CloudProvider::RunPod, Box::new(adapter) as Box<dyn CloudProviderAdapter>),
]);
let cloud = CloudProvisioner::with_providers(providers)
    .with_lease_store(".tangle/gpu-leases");
cloud.resume_leases().await?;
let spec = ResourceSpec {
    gpu_class: Some("nvidia-b200".into()),
    gpu_count: Some(8),
    gpu_vram_gb: Some(180),
    ..ResourceSpec::recommended()
};
let lease = cloud.allocate(&spec, Duration::from_secs(3600)).await?;
println!("{}", lease.endpoint);
// Transfer lease.access to the authorized lessee over an authenticated channel.
lease.terminate().await?;
# Ok(())
# }
```

`quotes` compares current provider prices. Unknown prices and unavailable SKUs
are excluded. Allocation tries the next priced candidate if creation fails.
Access uses a per-lease SSH key; provider account credentials stay in the adapter.
An optional public key lets the lessee keep their existing private key.

Expiry records use the deployment tracker. Keep the allocating process running
and reopen the same store with `resume_leases` after restart. Failed retirement
keeps its record for cleanup. Explicit termination verifies the provider no
longer has the resource.

`CloudProvisioner::new` constructs adapters from the environment. Use adapter
`with_credentials` constructors and `with_providers` when another application
owns credentials. `with_endpoint` accepts an explicit provider gateway, scopes
requests to its origin, and permits HTTP only for loopback test peers.

External orchestrators use the same adapters through
[`cargo tangle cloud … --json`](../../cli/README.md). SDK `create_managed` returns
the native id immediately; `status_managed`, `list_managed`, and `destroy_managed`
reconcile resources by namespace and lease identity. Managed creation has no
SDK retirement timer: the caller owns its durable lease lifecycle. Unknown
observed billing rates and start times remain absent.

## Deploy workloads

The adapter deployment API supports SSH/container and Kubernetes workloads.
Use the allocated instance's actual access endpoint. The manager's remote
runtime uploads and starts its binary on the machine it rents. Azure allocation
requires a caller-owned existing NIC. AWS callers supply the machine image.
GCP machine types without a complete billing-SKU mapping remain unselectable.

`require_tee` fails closed through [attestation](src/attestation/); read
[AGENTS.md](AGENTS.md) before changing that boundary. A hardware attestation
alone does not prove workload binding.

## Check the public boundary

```sh
cargo test -p blueprint-remote-providers --no-default-features --features std --test gpu_catalog_adapters -- --test-threads=1
cargo test -p blueprint-remote-providers --no-default-features --features std --test gpu_lease_adversarial -- --test-threads=1
```

These integration targets use HTTP provider peers and create no billable GPU
resources. Live cloud tests require credentials and an explicit spending budget.
See the [decentralized design](../../docs/gpu-marketplace-strategy.md) for
catalog, manager, quote, and consumer ownership.
