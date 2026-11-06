# Rent GPUs through the Blueprint SDK

The Blueprint SDK owns decentralized GPU rental for the manager, GPU lease
blueprints, and inference blueprints. Its
[GPU catalog](../crates/core/gpu-catalog.json) also supplies the pinned copy in
[centralized GPU leases](https://github.com/tangle-network/agent-container/tree/develop/apps/orchestrator/src/gpu-leases).
Blueprints use SDK adapters whenever they need cloud capacity.

## Describe hardware once

`blueprint_core::gpu` reads the published JSON catalog. Each class records its
vendor, generation, VRAM per GPU, memory bandwidth, FP8/FP4 support,
interconnect, and maximum GPUs per node. `providerSkus` maps a provider to exact
node counts; `*` denotes a provider SKU with a caller-selected count. A missing
count is unsupported. Prices belong to provider quote APIs.

`blueprint_core::resources::{ResourceSpec, CloudProvider}` is the shared
vocabulary, re-exported by remote providers and the manager. Requests preserve
`gpu_class`, `gpu_count`, and `gpu_vram_gb` independently of host RAM. Validation
uses the selected catalog class's node cap. Existing execution metadata remains
valid; `GpuRequirements.class` optionally selects a canonical catalog id.

For operator hardware, measure each device's name and VRAM through `nvidia-smi`.
For cloud inventory, use the selected catalog SKU's hardware description and
validate provider-reported capacity. Kubernetes placement uses the GPU class
and a numeric minimum-VRAM constraint.

## Allocate and retire

Enable `blueprint-sdk/remote-providers` and use `blueprint_sdk::remote`.
Construct adapters with caller-supplied credentials and pass them to public
`CloudProvisioner::with_providers`. Environment-based construction remains
available for operators.

`quotes(spec)` compares current configured providers. Unknown prices are
unselectable. `allocate(spec, ttl)` tries priced candidates in order and returns
a `Lease` with its endpoint and per-lease SSH access. Provider account keys stay
with the allocator. The deployment tracker persists expiry and owns retirement;
`Lease::terminate` retires early. Reopen the same state directory and call
`resume_leases` after restarting the allocator process.

The manager's remote runtime deploys its workload onto the selected machine.
The duplicate event hook that provisioned an idle VM is removed. Operator-owned
GPU allocation in gpu-lease-blueprint remains local; cloud capacity uses this SDK.

## Bind quotes to the allocation

Pricing-engine `GetJobPrice` accepts GPU class, count, and duration. Resource
prices use integer wire values. GPU job quotes bind the allocation in EIP-712;
tnt-core's GPU-aware JobsRFQ overload verifies those fields. Legacy job quote
entrypoints retain their original typed-data and call selectors.

`tangle-slashing` authenticates GPU evidence using its configured collector's
signature over the envelope and service context. Blueprint policy still owns
which evidence warrants a claim; tnt-core owns claim execution.

## Consume releases

SDK libraries publish through [release-plz](../.github/workflows/release-plz.yml)
and its batch publisher. Consumers pin the published SDK. The centralized
runtime-contracts copy pins the catalog source commit in its header. Inference
engine supervision and family recipes belong to
[tangle-inference-core](https://github.com/tangle-network/tangle-inference-core).
