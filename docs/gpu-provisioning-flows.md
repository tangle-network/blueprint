# GPU provisioning ownership

The [SDK design](gpu-marketplace-strategy.md) owns the rental and quote contracts.
The [catalog](../crates/core/gpu-catalog.json) owns class ids, per-GPU hardware,
node caps, and provider SKU mappings. It is exported by
`blueprint_core::gpu::CATALOG_JSON` and packaged in `blueprint-core`.

The manager, GPU lease blueprints, and inference blueprints share SDK provider
adapters. Central orchestration uses the released `cargo-tangle` process
boundary. Provider-specific SKU, count, TLS, firewall, and lifecycle behavior
belongs in the SDK; orchestration owns durable leases and billing.

Operator hardware is distinct from cloud capacity. Its registration describes
measured devices. Cloud requests select a catalog class and exact count, then
check the provider's current stock and price. Neither path treats host RAM as
GPU memory.

Inference engine supervision, recipes, and weight staging belong to
[tangle-inference-core](https://github.com/tangle-network/tangle-inference-core).
