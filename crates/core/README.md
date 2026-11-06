# blueprint-core

Core runtime primitives, GPU catalog, and shared provisioning requirements.
Most applications use the `blueprint-sdk` re-export.

The [GPU catalog](gpu-catalog.json) owns class identifiers, hardware properties,
node count caps, and provider SKUs. `gpu::{catalog, find}` read the embedded JSON
without requiring `std`. With `std`, `gpu::detect` measures operator devices
through `nvidia-smi`; host RAM is independent of per-device VRAM.

`resources::{ResourceSpec, CloudProvider}` supplies the shared vocabulary for
pricing, cloud adapters, and the manager. Provider quotes own current prices.

Job calls, extractors, metadata, and errors remain available through the source
exports. See the [decentralized GPU design](../../docs/gpu-marketplace-strategy.md)
for rental, quote, and consumer boundaries.
