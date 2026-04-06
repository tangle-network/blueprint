# GPU Marketplace Integration Strategy

How Tangle's Blueprint system should integrate with third-party GPU cloud marketplaces.

## Current State

### What exists

**`blueprint-remote-providers`** supports 5 traditional cloud providers:
- AWS EC2 (p4d/g4dn GPU instances), GCP GCE, Azure, DigitalOcean, Vultr
- Each has a `CloudProviderAdapter` implementation
- GPU instance type mapping in `infra/mapper.rs`

**`modal-inference-blueprint`** wraps Modal's GPU infrastructure:
- Operators deploy models to Modal via `modal deploy`
- Blueprint serves as Tangle-compatible HTTP proxy + billing layer
- 116 models across 17 task types

**`tangle-router`** provides operator discovery and scoring:
- DB-backed operator registry
- Scoring: reputation (40%) + latency (30%) + price (30%)

### What doesn't exist

No integrations with GPU-specific marketplaces:
- Vast.ai (spot GPU marketplace)
- RunPod (serverless/pod GPU)
- Prime Intellect (decentralized training)
- Lambda Labs (GPU cloud)
- CoreWeave (GPU-native cloud)
- Together AI (inference platform)
- Akash Network (decentralized compute)
- io.net (decentralized GPU aggregator)

## Architecture Options

### Option 1: Extend `blueprint-remote-providers` with marketplace adapters

Add GPU marketplace providers alongside existing cloud adapters.

```
CloudProvider enum gains:
  VastAi, RunPod, LambdaLabs, CoreWeave, TogetherAi

Each implements CloudProviderAdapter:
  provision_instance() → call marketplace API to rent GPU
  terminate_instance() → release GPU
  deploy_blueprint() → SSH/container deploy to rented instance
```

**Pros:**
- Reuses existing provisioning pipeline
- Manager handles lifecycle automatically (TTL, cleanup)
- Single abstraction for all providers

**Cons:**
- `CloudProvider` enum is closed (in pricing-engine crate, requires cross-crate changes)
- Marketplace APIs are diverse — Vast.ai is bid-based, RunPod is pod-based, Modal is serverless
- Tight coupling: every new provider requires SDK changes + release

**Verdict:** Good for 2-3 strategic providers. Doesn't scale to many.

### Option 2: Gateway adapter pattern

Create a single `GpuMarketplaceGateway` adapter that routes to multiple providers through a unified API.

```
CloudProvider::GpuMarketplace → GpuMarketplaceAdapter
  → GpuMarketplaceGateway (our service)
    → routes to: Vast.ai, RunPod, Lambda, etc.
    → selects cheapest/fastest provider matching ResourceSpec
    → handles auth, billing reconciliation, instance lifecycle
```

**Pros:**
- Single adapter in SDK, many providers behind the gateway
- Gateway can do cross-provider price comparison
- New providers added to gateway without SDK changes
- Can aggregate spot market pricing

**Cons:**
- New infrastructure to build and operate
- Single point of failure
- Adds latency to provisioning path

**Verdict:** Right approach if we want to be a compute aggregator. Significant build.

### Option 3: Blueprint-level integration (current Modal pattern)

Each GPU provider gets its own inference blueprint. Operators choose their provider.

```
llm-inference-blueprint      → operator self-hosts LLM via vLLM (formerly vllm-inference-blueprint)
modal-inference-blueprint    → operator uses Modal
runpod-inference-blueprint   → operator uses RunPod (new)
vastai-inference-blueprint   → operator uses Vast.ai (new)
```

**Pros:**
- Already works (Modal blueprint is production)
- No SDK changes needed
- Each provider's quirks handled in its own blueprint
- Operators choose the economics that work for them
- Customers don't care — they see operators with models and prices

**Cons:**
- Blueprint proliferation
- Shared logic (billing, SpendAuth, metrics) duplicated across blueprints
- No cross-provider optimization (can't auto-select cheapest GPU)

**Verdict:** This is the current approach and it works. Scale concern is manageable with shared crates.

### Option 4: Hybrid — shared crate + provider-specific blueprints

Extract shared inference logic into a crate, let blueprints focus on provider integration.

```
tangle-inference-core (shared crate):
  - SpendAuth billing
  - OpenAI-compatible HTTP server
  - Prometheus metrics
  - Health checks
  - GPU detection

Provider blueprints (thin wrappers):
  llm-inference-blueprint     → spawns local vLLM subprocess (renamed from vllm-inference-blueprint)
  modal-inference-blueprint   → proxies to Modal API
  runpod-inference-blueprint  → proxies to RunPod serverless
  vastai-inference-blueprint  → provisions Vast.ai instance + deploys vLLM
```

**Pros:**
- Shared billing, auth, metrics — no duplication
- Each blueprint is small (provider integration only)
- New providers are easy to add
- Operators choose provider, customers choose model+price

**Cons:**
- Shared crate becomes a dependency — breaking changes affect all blueprints
- Need to design the abstraction carefully

**Verdict:** Best long-term approach. Extracts value without over-engineering.

## Recommendation

**Start with Option 3 (current pattern), evolve to Option 4.**

### Phase 1: More provider blueprints (now)

Create blueprints for 2-3 high-value providers:

1. **RunPod** — serverless GPU, pay-per-second, good for bursty inference
   - Integration: RunPod Serverless API (`/run`, `/status`)
   - Operator deploys model to RunPod, blueprint proxies requests
   - Similar to Modal pattern

2. **Vast.ai** — spot GPU marketplace, cheapest A100s
   - Integration: Vast.ai API (`/offers`, `/instances`)
   - Operator bids on GPU instances, deploys vLLM
   - More complex: bid management, instance lifecycle

3. **Together AI** — managed inference, no GPU management
   - Integration: Together API (OpenAI-compatible)
   - Simplest: pure proxy, operator has Together API key
   - Good for operators without GPU expertise

### Phase 2: Extract shared crate (next)

Once 3+ inference blueprints exist, extract common code:

```toml
[dependencies]
tangle-inference-core = { path = "../tangle-inference-core" }
```

The crate provides:
- `InferenceServer` — Axum HTTP server with OpenAI-compatible endpoints
- `BillingClient` — ShieldedCredits integration
- `SpendAuthVerifier` — EIP-712 verification
- `MetricsCollector` — Prometheus metrics
- `GpuDetector` — nvidia-smi parsing
- `InferenceBackend` trait — abstraction over vLLM, Modal, RunPod, etc.

### Phase 3: Gateway service (later, if needed)

Build only if cross-provider optimization becomes valuable:
- Automatic provider selection based on price/latency/availability
- Spot market arbitrage across Vast.ai, RunPod, Lambda
- Single operator registration, multiple backend providers

This is a separate service (not in the SDK), operated by Tangle or community.

## Provider API Comparison

| Provider | API Style | GPU Access | Billing | Integration Complexity |
|----------|-----------|------------|---------|----------------------|
| Modal | Python SDK, serverless | Auto-provisioned | Per-second | Low (Python CLI) |
| RunPod | REST API, serverless | Pod or serverless | Per-second | Low (REST) |
| Vast.ai | REST API, marketplace | Bid-based spot | Per-hour | Medium (bidding) |
| Lambda Labs | REST API, cloud | On-demand instances | Per-hour | Low (EC2-like) |
| CoreWeave | K8s API, cloud | K8s GPU scheduling | Per-hour | Medium (K8s) |
| Together AI | REST API, managed | Abstracted | Per-token | Lowest (pure proxy) |
| Prime Intellect | Decentralized | P2P marketplace | Per-task | High (protocol) |
| Akash Network | Decentralized | SDL-based bidding | Per-block | High (protocol) |

## What NOT to build

- **Our own GPU cloud** — capital intensive, no moat. Let operators use existing providers.
- **A unified GPU API** — too many provider quirks. The blueprint abstraction already solves this at a higher level.
- **Provider lock-in** — the blueprint pattern is intentionally provider-agnostic. Customers choose operators, not providers. Keep it that way.

## Key Insight

Tangle's value is not in provisioning GPUs — it's in the **decentralized operator marketplace** with:
- On-chain operator registration and capability discovery
- Cryptographic billing (ShieldedCredits, x402)
- Reputation-based routing (tangle-router)
- Provider-agnostic abstraction (any GPU source, same customer API)

The right integration pattern is thin blueprint wrappers over provider APIs, with shared billing/auth infrastructure. The `blueprint-remote-providers` crate should stay focused on infrastructure provisioning (VMs, K8s), not application-level GPU marketplace integration.
