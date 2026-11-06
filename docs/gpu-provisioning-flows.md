# GPU provisioning

[The decentralized GPU design](gpu-marketplace-strategy.md) owns rental,
catalog, quoting, and retirement. Blueprint code uses the SDK's remote provider
feature for cloud capacity and detects available GPUs when using operator hardware.

## Declare requirements

Set `profilingData.execution_profile.gpu` in blueprint metadata:

```json
{"execution_profile":{"gpu":{"policy":"required","class":"nvidia-b200","min_count":8,"min_vram_gb":180}}}
```

`class` is optional for existing metadata. `required` refuses placement without
the requested hardware. `preferred` allows CPU fallback. `none` requests no GPU.
Use canonical ids from the [catalog](../crates/core/gpu-catalog.json).

## Run on operator hardware

The manager detects each local NVIDIA device with
`nvidia-smi --query-gpu=name,memory.total --format=csv,noheader,nounits`.
Host system RAM is independent of VRAM. Required placement checks the class,
count, and actual memory of each qualifying GPU.

For Kubernetes, install the NVIDIA device plugin. Required GPU pods request
`nvidia.com/gpu`, select the class label `gpu.tangle.tools/class`, and use a
numeric minimum against `gpu.tangle.tools/vram-gb`. Labels describe actual
operator hardware; larger-memory nodes can satisfy smaller minimums.

## Rent cloud capacity

Construct provider adapters with caller-owned credentials, pass them to
`CloudProvisioner::with_providers`, and allocate with the shared `ResourceSpec`
and lifetime. Current provider prices determine selection. The SDK installs
per-lease access and persists the retirement deadline before returning access.
Resume the same lease store after process restart.

The manager's remote runtime deploys the blueprint onto the rented machine.
GPU lease and inference blueprints share these adapters. Application policy,
serving engines, and operator registration stay with their owning blueprint or
[tangle-inference-core](https://github.com/tangle-network/tangle-inference-core).
