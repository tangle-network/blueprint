# GPU Provisioning Flows

How GPU resources are provisioned, validated, and used across the Tangle Blueprint ecosystem. Written for blueprint developers, operators, and customers.

## Overview

GPU provisioning operates at three layers:

```
Layer 1: On-chain declaration     Blueprint developer sets min GPU requirements
Layer 2: Manager enforcement      Blueprint Manager provisions and enforces GPU hardware
Layer 3: Application usage        Blueprint binary detects and uses available GPUs
```

## For Blueprint Developers

### Declaring GPU requirements

When creating a blueprint, set GPU requirements in the `profilingData` field of your `BlueprintDefinition.metadata`:

```json
{
  "execution_profile": {
    "gpu": {
      "policy": "required",
      "min_count": 1,
      "min_vram_gb": 16
    }
  }
}
```

**Policies:**
- `required` — service won't activate without GPU hardware. K8s pods get `nvidia.com/gpu` hard resource constraint.
- `preferred` — GPU preferred, CPU fallback allowed. K8s pods get soft node affinity.
- `none` — no GPU requirements (default).

These are **minimums**. Operators can exceed them and charge accordingly via RFQ pricing.

### BSM-level GPU validation

Your Blueprint Service Manager contract can add application-level GPU validation on top. Example from `InferenceBSM`:

```solidity
function onRegister(address operator, bytes calldata registrationInputs)
    external payable override onlyFromTangle
{
    (string memory model, uint32 gpuCount, uint32 totalVramMib,
     string memory gpuModel, string memory endpoint)
        = abi.decode(registrationInputs, (string, uint32, uint32, string, string));

    ModelConfig storage mc = modelConfigs[keccak256(bytes(model))];
    if (totalVramMib < mc.minGpuVramMib)
        revert InsufficientGpuCapability(mc.minGpuVramMib, totalVramMib);

    operatorCaps[operator] = OperatorCapabilities({
        model: model, gpuCount: gpuCount, totalVramMib: totalVramMib,
        gpuModel: gpuModel, endpoint: endpoint, active: true
    });
}
```

This validates that the operator's *declared* capabilities meet the model's requirements. The operator self-declares GPU specs at registration. Lying is deterred by staking/slashing — not by hardware attestation (which is a future TEE-in-GPU concern).

### Detecting GPUs in your blueprint binary

Your operator binary runs on hardware the manager already provisioned. Detect available GPUs at startup:

```rust
// nvidia-smi --query-gpu=index,name,memory.total --format=csv,noheader,nounits
pub async fn detect_gpus() -> Result<Vec<GpuInfo>> {
    let output = Command::new("nvidia-smi")
        .args(["--query-gpu=index,name,memory.total,memory.used,memory.free,temperature.gpu,utilization.gpu,driver_version",
               "--format=csv,noheader,nounits"])
        .output().await?;
    // parse CSV rows into GpuInfo structs
}
```

Expose GPU status via health endpoint (e.g., `GET /health/gpu`) for monitoring.

## For Operators

### What happens when you register

```
1. You call registerOperator(blueprintId, registrationPayload)
   registrationPayload = abi.encode(model, gpuCount, totalVramMib, gpuModel, endpoint)

2. BSM.onRegister validates:
   - Model is enabled (configureModel called by blueprint owner)
   - Your declared VRAM >= model's minGpuVramMib
   - Stores your OperatorCapabilities on-chain

3. You start the Blueprint Manager:
   blueprint-manager --preferred-source container  # or native

4. Manager receives ServiceActivated event and:
   - Reads GPU requirements from on-chain profilingData
   - Selects runtime (K8s, VM, native, remote cloud)
   - Provisions hardware with GPU if required
   - Deploys your blueprint binary
   - Your binary detects GPUs, spawns inference backend, starts serving
```

### Runtime paths

**Kubernetes (production):**
Your blueprint runs as a K8s pod. If GPU is `required`:
- Pod spec includes `nvidia.com/gpu` resource request
- Node selector: `gpu.tangle.tools/enabled=true`
- VRAM filter: `gpu.tangle.tools/min-vram-gb=<value>`
- Pod stays Pending until a GPU node is available

Prerequisites: NVIDIA device plugin installed, nodes labeled.

**Remote cloud (auto-provisioned):**
Run manager with `--enable-remote-deployments` and cloud credentials configured. The manager auto-provisions a cloud instance:
- GPU workloads → GCP (A100, T4) or AWS (p4d, g4dn)
- TEE required → AWS, GCP, or Azure
- Standard → DigitalOcean or Vultr
- 1-hour TTL auto-cleanup (configurable)

**Native (local/dev):**
Blueprint binary runs directly on the host. No GPU enforcement by the manager — your binary detects GPUs via `nvidia-smi`. Use this for development with local GPUs.

**VM sandbox (Linux):**
Blueprint runs inside a cloud-hypervisor VM. CPU and memory limits are applied. GPU passthrough via VFIO is supported by cloud-hypervisor (`DeviceConfig` with `x_nv_gpudirect_clique`) but not yet wired in the manager.

### Pricing

You set prices at two levels:

**Per-resource (infrastructure cost):**
Configure via Pricing Engine sidecar TOML:
```toml
[resources]
gpu = { kind = "GPU", count = 1, price_per_unit_rate = 0.005 }
cpu = { kind = "CPU", count = 4, price_per_unit_rate = 0.001 }
```

**Per-token (application pricing):**
Set via BSM `configureModel()` or override with RFQ quotes:
```
pricePerInputToken = 1    # base units (tsUSD wei)
pricePerOutputToken = 3
```

Customers see BSM defaults. You can override per-request via EIP-712 signed RFQ quotes — charge more for premium hardware, less during off-peak.

### Visibility: what the customer sees

Customers can discover operators and their capabilities:
- BSM contract: `operatorCaps[operator]` → model, gpuCount, totalVramMib, gpuModel, endpoint
- Health endpoint: `GET /health/gpu` → real-time GPU status, temperature, utilization
- Metrics: `GET /metrics` → Prometheus metrics (tokens/sec, latency, GPU util)
- tcloud SDK: operator discovery with reputation scoring (latency 30%, reputation 40%, price 30%)

## For Customers

### Requesting a service

```
1. Query available operators for a blueprint:
   - Read operatorCaps from BSM contract
   - Or use tcloud SDK for discovery + reputation scoring

2. Create a service:
   createServiceFromQuotes(blueprintId, quotes[], config, callers, ttl)
   - config bytes: opaque blueprint-specific params (model selection, context length)
   - quotes: EIP-712 signed price quotes from operators

3. Use the service:
   Option A (on-chain): submitJobFromQuote(serviceId, jobIndex, inputs, quotes)
   Option B (x402 HTTP): POST /v1/chat/completions with SpendAuth header
```

### Payment paths

**On-chain jobs:** Submit job → operator processes → result posted on-chain. Verifiable, recorded. Pay per job via RFQ quote.

**x402 HTTP (low-latency):** Sign EIP-712 SpendAuth off-chain → operator serves immediately → async settlement. Private, fast. Pay per request via ShieldedCredits.

### What you can verify

- Operator's declared GPU specs (on-chain, immutable after registration)
- Real-time GPU health and utilization (health endpoint)
- Inference throughput and latency (metrics endpoint)
- Payment amounts (SpendAuth signatures are deterministic from token counts)
- Service availability (operator uptime via QoS heartbeats)

## Flow Diagrams

### Flow A: Kubernetes Container Runtime

```
Blueprint Developer                    Tangle Chain
    │                                      │
    │── createBlueprint(definition) ──────►│  profilingData: { gpu: { policy: required, min_count: 1, min_vram_gb: 16 } }
    │                                      │
    │                                      │
Operator                                   │
    │── registerOperator(blueprintId, ─────►│  abi.encode(model, gpuCount=2, totalVramMib=81920, gpuModel="A100", endpoint)
    │   registrationPayload)               │
    │                                      │  BSM.onRegister: validates 81920 >= 16384 ✓
    │                                      │
Customer                                   │
    │── createServiceFromQuotes(...) ──────►│
    │                                      │  emits ServiceActivated(serviceId)
    │                                      │
Blueprint Manager (on Operator node)       │
    │◄── ServiceActivated ─────────────────│
    │                                      │
    │  resolve_service(serviceId)
    │    → GpuRequirements { policy: Required, min_count: 1, min_vram_gb: 16 }
    │  apply_gpu_limits → ResourceLimits { gpu_count: Some(1), gpu_policy: Required }
    │
    │  source = Container(image: "ghcr.io/tangle/llm-inference:latest")
    │    → ContainerInstance::new(limits)
    │    → K8s Pod spec:
    │        resources.limits["nvidia.com/gpu"] = "1"
    │        node_selector: { gpu.tangle.tools/enabled: "true", gpu.tangle.tools/min-vram-gb: "16" }
    │    → Pod scheduled on GPU node
    │
    │  Blueprint binary starts inside pod:
    │    → detect_gpus() via nvidia-smi
    │    → spawn vLLM subprocess on GPU
    │    → start HTTP server on :8080
    │
Customer                              Operator Pod
    │── POST /v1/chat/completions ────►│
    │   + SpendAuth (EIP-712)          │
    │                                  │  verify signature → authorize spend → proxy to vLLM → claim payment
    │◄── inference response ───────────│
```

### Flow B: Remote Cloud Auto-Provisioning

```
Blueprint Manager (--enable-remote-deployments)
    │
    │◄── ServiceActivated
    │
    │  1. Local spawn (same as Flow A — container, native, or VM)
    │
    │  2. notify_remote_service_initiated(blueprint_id, service_id, gpu_requirements)
    │     → resource_spec_from_limits(): ResourceSpec { gpu_count: Some(1), cpu: 2.0, memory_gb: 4.0 }
    │     → RemoteProviderManager::on_service_initiated()
    │
    │     Provider selection:
    │       gpu_count.is_some() → GCP (preferred)
    │       select_configured_provider() → filters by available credentials
    │
    │     → CloudProvisioner::provision_with_requirements(GCP, resource_spec, "us-central1", false)
    │       → GCP adapter: create n1-standard-4 + T4 GPU instance
    │       → returns ProvisionedInstance { id, ip, instance_type }
    │
    │     → RemoteDeploymentRegistry::register(blueprint_id, service_id, config)
    │     → TtlManager::register_ttl(3600)  // 1-hour auto-cleanup
    │
    │  On ServiceTerminated:
    │     → stop_service() locally
    │     → notify_remote_service_terminated()
    │       → TtlManager::unregister_ttl()
    │       → RemoteDeploymentRegistry::cleanup() → terminate cloud instance
```

### Flow C: Native Process (Development)

```
Blueprint Manager (--no-vm or no vm-sandbox feature)
    │
    │◄── ServiceActivated
    │
    │  limits = ResourceLimits { gpu_count: Some(1), ... }
    │  source = Testing(cargo_bin: "llm-operator") or Github(binary)
    │
    │  → Service::from_binary() → Service::new_native()
    │    → tokio::process::Command::new(binary_path).spawn()
    │    → limits stored but NOT enforced (no cgroups)
    │
    │  Blueprint binary runs on host:
    │    → detect_gpus() finds host GPUs via nvidia-smi
    │    → spawns vLLM on local GPU
    │    → serves at localhost:8080
```

## GPU Cloud Marketplace Integration

See [gpu-marketplace-strategy.md](./gpu-marketplace-strategy.md) for analysis of integrating with third-party GPU providers.
