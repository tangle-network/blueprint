# blueprint-core

Core runtime primitives, shared resource requests and the canonical GPU catalog.
Applications use the SDK umbrella's `core` export.

`gpu::{CATALOG_JSON, catalog, find}` expose the [packaged catalog](gpu-catalog.json)
without std. It owns class ids, hardware properties, decimal GB per GPU, node
caps and provider SKU mappings. `gpu::detect` reads actual device memory through
`nvidia-smi`. `resources` owns `ResourceSpec`, `CloudProvider`, weight identity
and normalized GPU arms. Prices and stock come from live provider APIs;
credential and capability descriptors come from the provider implementations.

The [source exports](src/lib.rs) own the runtime API. The
[GPU rental design](../../docs/gpu-marketplace-strategy.md) defines the catalog,
provider, orchestration and consumer contracts.
