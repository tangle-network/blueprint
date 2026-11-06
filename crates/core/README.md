# blueprint-core

Core runtime primitives and the canonical GPU catalog for the Blueprint SDK.
Most applications use the SDK umbrella's `core` export.

`gpu::{CATALOG_JSON, catalog, find}` expose the [packaged catalog](gpu-catalog.json)
without std. It owns class ids, hardware properties, decimal GB per GPU, node
caps and provider SKU mappings. Prices come from live provider APIs. Provider
descriptors are generated from the credential factory and adapter traits.

The [source exports](src/lib.rs) own the runtime API. The
[GPU rental design](../../docs/gpu-marketplace-strategy.md) defines the catalog,
provider, orchestration and consumer contracts.
