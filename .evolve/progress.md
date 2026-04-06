# Pursue Progress — blueprint

## Latest Generation

**Gen 2 (2026-04-06) — Hardening + Decentralized Providers + Inference Core Adoption** — ADVANCED

See [.evolve/pursuits/2026-04-06-gen2-hardening-decentralized-inference.md](pursuits/2026-04-06-gen2-hardening-decentralized-inference.md) for the full spec and results.

### What shipped in Gen 2
- **Hardened gpu_adapter.rs**: `retry_with_backoff`, `provision_with_cleanup`, `classify_status_code`, `parse_retry_after`, `RetryPolicy` with jitter — 532 LOC of shared primitives, 21 new tests
- **Gen 1 adapter hardening**: all 7 existing adapters wrapped with `provision_with_cleanup` to prevent billing leaks on failed deploys
- **7 new decentralized providers**: Akash (Cosmos), io.net (REST), Spheron (REST), Nosana (Solana), Prime Intellect (aggregator), Render/Dispersed (young AI), Bittensor Lium (subnet 51) — 4,512 LOC total, 82 new tests
- **`tangle-inference-core` redesign**: `AppStateBuilder`, decoupled `BillingClient::new_with_params`, 6 `CostModel` implementations (per-token, per-char, per-second, per-image, flat-request, task-type), integration test, `examples/minimal_operator.rs`, 360-line adoption README
- **vllm-inference-blueprint migrated** to `tangle-inference-core`: 3 files deleted (billing.rs, metrics.rs, health.rs), ~1,800 LOC removed, tests still passing

### Baseline after Gen 2
- **19 cloud providers** supported (was 12)
- **14 GPU-focused providers** (was 7)
- **7 decentralized providers** (was 0)
- **270 lib tests** in blueprint-remote-providers (was 159, +111)
- **19 tests** in tangle-inference-core (was 9, +10)
- **5 tests** in vllm-inference-blueprint (still passing after migration)
- **0 known resource leaks** across adapters
- **0 adapters missing retry/backoff**
- Clippy clean, rustfmt clean
- Manager unit tests: 21 passing

### Operator workflow (post-Gen-2)
```bash
# Operator can now choose from 19 providers, including decentralized ones
export AKASH_RPC_URL=https://rpc.akash.forbole.com
export AKASH_WALLET_MNEMONIC="..."
# OR io.net, Spheron, Nosana, Prime Intellect, Render, Bittensor Lium
# OR traditional clouds: AWS, GCP, Azure, DO, Vultr
# OR GPU-focused: Lambda, RunPod, Vast, CoreWeave, Paperspace, Fluidstack, TensorDock

blueprint-manager --enable-remote-deployments

# When a GPU blueprint activates:
# 1. Manager picks provider (GPU workloads prefer GPU marketplaces first)
# 2. Adapter provisions instance via REST API (with retry+backoff)
# 3. Deploy via Docker-over-SSH (with automatic cleanup on failure)
# 4. TTL-based auto-cleanup on service termination
```

### Inference blueprint migration pattern (proven in Gen 2)
vllm-inference-blueprint is the reference. The pattern works:
```rust
// Before: ~3,200 LOC with billing.rs, metrics.rs, health.rs, full server boilerplate
// After: ~1,750 LOC — those files deleted, server shrunk 35%

use tangle_inference_core::{
    AppState, AppStateBuilder, BillingClient, NonceStore, SpendAuthPayload,
    BillingConfig, ServerConfig, GpuConfig, TangleConfig,
    PerTokenCostModel, CostModel, validate_spend_auth, extract_x402_spend_auth,
    payment_required, error_response, RequestGuard, gather, detect_gpus,
};

let state = AppStateBuilder::new()
    .billing(Arc::new(billing_client))
    .nonce_store(Arc::new(nonce_store))
    .server_config(Arc::new(config.server))
    .billing_config(Arc::new(config.billing))
    .operator_address(operator_addr)
    .backend(vllm_process)  // blueprint-specific backend
    .build()?;
```

## Seeds for Gen 3
- **Migrate remaining 6 inference blueprints** (voice, embedding, modal, image-gen, video-gen, distributed) — pattern proven in vllm
- **Live integration tests** with real credentials via GitHub secrets
- **Cross-provider arbitrage** — query all enabled providers, pick cheapest within reliability threshold
- **SKU catalog validation** — fetch live catalog, fail-fast on unknown SKUs
- **Native Cosmos SDK for Akash** (replace REST-over-relay with real tx construction)
- **Solana native signer for Nosana**
- **Akash SDL template library** generated from ResourceSpec

## Historical Generations
- **Gen 1 (2026-04-06)**: Added 7 GPU marketplace providers (Lambda Labs, RunPod, Vast.ai, CoreWeave, Paperspace, Fluidstack, TensorDock). Audit revealed resource leak bugs, leading to Gen 2 hardening.
