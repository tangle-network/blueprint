# Architectural Spec: First-Class TEE Support in Blueprint SDK

## 1. Executive Summary

Blueprint should treat TEE as a first-class runtime capability, not an optional `kms_url` knob.

Today, `blueprint-runner` exposes a `tee` feature with no runtime behavior (`crates/runner/Cargo.toml:82-83`) and only one TEE-related config field (`kms_url`) in `BlueprintEnvironment` (`crates/runner/src/config.rs:227-229`, `:954-963`).

By contrast, the sandbox blueprint already demonstrates production-grade TEE patterns across five backends (AWS Nitro, Azure CVM/SKR, GCP Confidential Space, Phala/TDX, Direct), with lifecycle, attestation, and sealed-secret flows (`sandbox-runtime/src/tee/*.rs`). We should not copy that code into SDK, but we should codify its successful patterns into SDK-native abstractions.

### What we are building

A new first-class TEE capability for Blueprint SDK with:
- Runtime deployment modes: `Remote`, `Direct`, `Hybrid`
- Standard attestation types and verifier APIs
- A key-exchange background service for secret handoff
- Runner/manager/middleware integration points
- Optional policy gates for x402 and aggregation flows

### Deployment modes

1. **Remote TEE Provider**
- Manager runs outside TEE and provisions blueprint runtimes in cloud confidential compute.
- Aligns with existing remote-provider abstractions (`DeploymentTarget`, provider adapters, tracker).

2. **Direct TEE**
- Manager itself runs inside a TEE host/enclave and launches blueprint workloads locally in that trust domain.
- Highest integrity path with fewer network trust links.

3. **Hybrid**
- Manager runs outside TEE; selected jobs/services are scheduled into TEE runtimes, others run normally.
- Enables incremental adoption and cost/perf control.

---

## 2. Crate Structure

## 2.1 Recommended shape

Create a new crate: `crates/tee` (`blueprint-tee`) and integrate it into runner/manager/tangle-extra/x402.

Rationale:
- Keeps TEE concerns cohesive and testable
- Avoids bloating `blueprint-runner`
- Mirrors how x402 was productized as its own crate while integrating via runner background services (`crates/x402/src/gateway.rs:21`, `:349-389`)

## 2.2 Proposed modules

```text
crates/tee/
  src/
    lib.rs
    config.rs
    errors.rs
    attestation/
      mod.rs
      report.rs
      claims.rs
      verifier.rs
      providers/
        aws_nitro.rs
        azure_skr.rs
        gcp_confidential.rs
        tdx.rs
        sev_snp.rs
    runtime/
      mod.rs
      backend.rs
      direct.rs
      remote.rs
      hybrid.rs
      registry.rs
    exchange/
      mod.rs
      service.rs      # BackgroundService
      protocol.rs
      store.rs
    middleware/
      tee_layer.rs
      tee_context.rs
    policy/
      attestation_policy.rs
      measurement_policy.rs
```

## 2.3 Integration crates/files

- `crates/runner`
  - add real `tee` feature dependency and builder integration
  - current baseline is no-op feature (`Cargo.toml:82-83`)
- `crates/manager`
  - add TEE runtime mode + scheduling config; currently only VM/container/remote/native (`crates/manager/src/rt/service.rs:45-53`, `:103-147`)
- `crates/tangle-extra`
  - extend metadata layer model (current `TangleLayer` injects `call_id` and `service_id`, `crates/tangle-extra/src/layers.rs:82-86`)
- `crates/x402`
  - add optional attestation policy gate before enqueueing paid calls (current gateway already has policy pipeline and background service lifecycle, `crates/x402/src/gateway.rs:295-346`, `:349-389`)

## 2.4 Dependency direction

- `blueprint-tee` depends on:
  - `blueprint-core` (metadata/result types)
  - `blueprint-runner` traits only where necessary (or a small trait crate if cycle risk)
  - `blueprint-remote-providers` for remote mode provisioning adapters
- `blueprint-runner`, `blueprint-manager`, `x402`, `tangle-extra` depend on `blueprint-tee` behind `tee` feature

---

## 3. API Design

## 3.1 Runner ergonomics

```rust
use blueprint_runner::BlueprintRunner;
use blueprint_tee::{TeeConfig, TeeMode, TeeRequirement};

let tee = TeeConfig::builder()
    .requirement(TeeRequirement::Required)
    .mode(TeeMode::Direct)
    .allow_providers(["tdx", "sev_snp", "nitro"])
    .build()?;

BlueprintRunner::builder(config, env)
    .tee(tee)
    .router(router)
    .run()
    .await?;
```

### Builder extension

```rust
impl<F> BlueprintRunnerBuilder<F> {
    pub fn tee(mut self, cfg: TeeConfig) -> Self;
}
```

Implementation pattern should mirror existing background-service wiring in runner (`background_service()` at `crates/runner/src/lib.rs:594-597`; startup/select handling `:891-908`, `:1118-1141`).

## 3.2 Core types

```rust
pub enum TeeMode {
    Disabled,
    Direct,
    Remote,
    Hybrid,
}

pub enum TeeRequirement {
    Preferred,
    Required,
}

pub struct TeeConfig {
    pub requirement: TeeRequirement,
    pub mode: TeeMode,
    pub provider_selector: TeeProviderSelector,
    pub attestation_policy: AttestationPolicy,
    pub key_exchange: TeeKeyExchangeConfig,
}
```

## 3.3 Attestation type system

```rust
pub trait TeeAttestation: Send + Sync {
    fn provider(&self) -> TeeProvider;
    fn format(&self) -> AttestationFormat;
    fn report(&self) -> &AttestationReport;
}

pub struct AttestationReport {
    pub provider: TeeProvider,
    pub issued_at_unix: u64,
    pub measurement: Measurement,
    pub public_key_binding: Option<PublicKeyBinding>,
    pub claims: AttestationClaims,
    pub evidence: Vec<u8>,
}

pub trait AttestationVerifier {
    fn verify(&self, report: &AttestationReport, policy: &AttestationPolicy)
        -> Result<VerifiedAttestation, TeeError>;
}
```

This replaces sandbox’s minimal raw tuple (`tee_type/evidence/measurement/timestamp`) (`sandbox-runtime/src/tee/mod.rs:58-67`) with richer typed claims and explicit provider formats.

## 3.4 Middleware and extractor

```rust
pub struct TeeLayer;

pub struct TeeContext {
    pub attestation: VerifiedAttestation,
    pub deployment: TeeDeploymentRef,
}

pub const TEE_ATTESTATION_DIGEST_KEY: &str = "tee.attestation.digest";
pub const TEE_PROVIDER_KEY: &str = "tee.provider";
pub const TEE_MEASUREMENT_KEY: &str = "tee.measurement";
```

`TeeLayer` should follow the same result-metadata mutation model as `TangleLayer` (`crates/tangle-extra/src/layers.rs:77-86`).

## 3.5 Background service for key exchange

```rust
pub struct TeeAuthService {
    pub config: TeeKeyExchangeConfig,
}

impl blueprint_runner::BackgroundService for TeeAuthService {
    async fn start(&self) -> Result<oneshot::Receiver<Result<(), RunnerError>>, RunnerError>;
}
```

This aligns with the x402 pattern (`crates/x402/src/gateway.rs:349-389`) and avoids hiding key exchange inside ad hoc API handlers.

---

## 4. Integration Architecture

## 4.1 Runner

Current runner behavior:
- Supports pluggable background services (`crates/runner/src/lib.rs:127-136`, `:594-597`)
- Monitors service lifecycle and fails fast on service errors (`:1118-1129`)

TEE integration:
- `BlueprintRunnerBuilder::tee(TeeConfig)` registers:
  - `TeeAuthService` (key exchange/session)
  - `TeeRuntimeHealthService` (optional attestation freshness polling)
- `TeeLayer` injected where `TangleLayer` is used for result metadata

## 4.2 Manager runtime targets

Current manager runtime selection has no TEE branch (`vm` fallback then `native`, `crates/manager/src/rt/service.rs:115-147`).

Add runtime variant:

```rust
enum Runtime {
    Hypervisor(...),
    Container(...),
    Remote(...),
    Tee(TeeServiceInstance),
    Native(...),
}
```

`TeeServiceInstance` strategy:
- `Direct`: local confidential host launcher
- `Remote`: provider-backed deployment via `blueprint-remote-providers`
- `Hybrid`: per-job routing policy (`tee_required_jobs`, labels, or policy file)

## 4.3 Remote providers

Use existing abstractions instead of parallel TEE deployment stacks:
- `DeploymentTarget` as extension point (`crates/blueprint-remote-providers/src/core/deployment_target.rs:9-37`)
- adapter contract (`deploy_blueprint_with_target`, `infra/traits.rs:72-78`)
- provisioner orchestration (`infra/provisioner.rs:215-257`)

Proposed extension:

```rust
pub enum DeploymentTarget {
    VirtualMachine { ... },
    ManagedKubernetes { ... },
    GenericKubernetes { ... },
    TeeVirtualMachine { provider: TeeProvider, profile: TeeVmProfile },
    TeeContainer { provider: TeeProvider, profile: TeeContainerProfile },
}
```

## 4.4 x402 integration

x402 currently verifies payment then enqueues job calls (`crates/x402/src/gateway.rs:330-346`).

Add optional `AttestationPolicyGate`:
- Payment accepted only if requested TEE policy is satisfiable
- For result-settlement workflows, require valid attestation digest in result metadata

## 4.5 Aggregation integration

Aggregation consumer already parses metadata and routes submission (`crates/tangle-extra/src/aggregating_consumer.rs:538-579`, `:663-699`).

TEE extension:
- Include `tee.attestation.digest` and `tee.measurement` in signed payload domain
- Aggregation service can enforce “only results from approved measurements” before threshold acceptance
- Strongest guarantee: multi-operator threshold + consistent enclave measurements

---

## 5. Key Exchange Flow

## 5.1 Required flow (two-phase)

```text
Client                      Manager/Runner                TEE Runtime
  |                               |                           |
  |-- request ephemeral key ----->|                           |
  |                               |-- generate keypair ------>|
  |                               |<-(pubkey, attestation)----|
  |<-- pubkey + attestation ------|                           |
  |                                                       (bind pubkey
  |                                                        to measurement)
  |-- verify attestation (local verifier lib)               |
  |-- encrypt secrets to pubkey ---------------------------->|
  |                               |-- forward sealed blob -->|
  |                               |<-- decrypt+ack ----------|
  |<-- injection status -----------|                          |
```

## 5.2 Service design

`TeeAuthService` responsibilities:
- Manage ephemeral session keys with TTL
- Expose local control-plane endpoints for key retrieval and sealed-secret submission
- Enforce one-time/limited-use handoff tokens
- Record attestation hash + key fingerprint for audit trail

## 5.3 Inspiration from sandbox `session_auth.rs`

Adopt:
- Challenge/nonce pattern (`create_challenge`, `session_auth.rs:86-112`)
- Signature verification separation (`verify_eip191_signature`, `:136-176`)
- TTL/capacity controls (`MAX_CHALLENGES/MAX_SESSIONS`, `:32-34`)

Avoid in SDK core:
- In-memory global stores for production auth (`:65-69`)
- Random fallback secret for session encryption in production (`:200-216`)

---

## 6. Provider Implementations

## 6.1 Provider SPI

Define stable interface in `blueprint-tee`:

```rust
#[async_trait]
pub trait TeeRuntimeBackend {
    async fn deploy(&self, req: TeeDeployRequest) -> Result<TeeDeploymentHandle, TeeError>;
    async fn get_attestation(&self, handle: &TeeDeploymentHandle) -> Result<AttestationReport, TeeError>;
    async fn stop(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError>;
    async fn destroy(&self, handle: &TeeDeploymentHandle) -> Result<(), TeeError>;
}
```

This is conceptually similar to sandbox `TeeBackend` (`sandbox-runtime/src/tee/mod.rs:158-173`) but should avoid global singleton access (`:216-245`) in favor of injected dependencies.

## 6.2 Per-provider requirements

### AWS Nitro
- Enclave-enabled EC2 provisioning and userdata bootstrap (`aws_nitro.rs:230-243`, `:111-167`)
- Health/attestation fetch after enclave startup (`:268-279`)
- KMS recipient-attestation-based release path (documented in module header `:30-37`)

### GCP Confidential Space
- `confidentialInstanceConfig` + launcher metadata (`gcp.rs:173-211`)
- machine-family-derived TEE type (`:77-94`)
- post-launch health/attestation path (`:310-321`)

### Azure CVM/SKR
- Multi-resource orchestration: Public IP + NIC + CVM (`azure.rs:465-477`)
- OAuth token cache for ARM (`:110-173`)
- destroy-time resource cleanup from stored metadata (`:577-650`)

### Phala/TDX
- Compose-based app deployment through provider SDK (`phala.rs:37-64`, `:76-87`)
- Attestation fetch and network endpoint lookup (`:97-121`)

### Direct (local TEE)
- Device passthrough by TEE type (`direct.rs:60-66`, `:101-107`)
- hardened runtime defaults (cap drop, readonly rootfs, tmpfs, limits, `:109-130`)
- native ioctl attestation with sidecar fallback (`:234-249`, `attestation.rs:151-165`)

## 6.3 Lessons for SDK provider layer

Keep:
- provider-specific env/config loaders
- explicit wait-for-running and health probes
- metadata capture for stop/destroy lifecycle

Improve:
- strongly typed provider profile structs instead of stringly env-only control
- typed attestation claims vs raw byte vectors
- avoid store scans for lookup (`sidecar_info_for_deployment` currently scans store, `tee/mod.rs:251-267`)

---

## 7. Migration Path

## Phase 0: Foundation (S)
- Keep existing `tee` feature, add `blueprint-tee` crate skeleton
- Add `TeeConfig` type and parser in runner/manager without behavior changes

## Phase 1: Attestation SDK types + verifier (M)
- Ship `AttestationReport`, provider enums, verifier traits
- Add client-side verification API first

## Phase 2: Direct mode GA (L)
- Implement direct backend first (lowest cloud dependency)
- Add native attestation support (TDX/SEV)
- Add `TeeAuthService` and sealed-secret handoff

## Phase 3: Remote provider beta (XL)
- Extend `blueprint-remote-providers` with TEE targets
- Add AWS/GCP/Azure adapters for confidential targets
- Integrate deployment tracker and secure bridge with TEE metadata

## Phase 4: Hybrid scheduling (L)
- Manager-level per-job/service policy routing
- Fallback semantics (`required` fails closed, `preferred` degrades)

## Phase 5: x402 + aggregation policy hooks (M)
- x402 attestation gate
- aggregation domain binding to attestation digest

## Phase 6: Deprecate stub config (S)
- Replace standalone `kms_url`-only behavior with full TEE config model

---

## 8. Gap Analysis & Checklist

Priority key: P0 critical, P1 high, P2 medium

| Priority | Item | Scope | Size |
|---|---|---|---|
| P0 | Add `blueprint-tee` crate with core config + error + traits | New crate | M |
| P0 | Runner API: `.tee(TeeConfig)` + service registration | `crates/runner` | M |
| P0 | Manager config: tee mode/provider/policy flags | `crates/manager/src/config` | M |
| P0 | Manager runtime variant `Runtime::Tee` + lifecycle | `crates/manager/src/rt/service.rs` | L |
| P0 | Attestation report and verifier APIs | `crates/tee/attestation` | L |
| P0 | `TeeAuthService` key exchange + sealed handoff | `crates/tee/exchange` | L |
| P1 | `TeeLayer` + `TeeContext` middleware/extractor | `crates/tee` + `tangle-extra` | M |
| P1 | Extend remote `DeploymentTarget` for TEE targets | `blueprint-remote-providers` | M |
| P1 | AWS/GCP/Azure TEE adapter implementations | provider adapters | XL |
| P1 | Direct backend implementation | local runtime | L |
| P1 | Tracker schema extension for TEE deployment refs | remote tracker | M |
| P2 | x402 attestation policy gate | `crates/x402` | M |
| P2 | Aggregation payload binding to attestation digest | `tangle-extra` + agg service | M |
| P2 | docs/examples/operator guides | docs + examples | M |
| P2 | conformance test suite across providers/modes | integration testing | XL |

### Notable current gaps observed in code

- Runner TEE feature has no behavior (`crates/runner/Cargo.toml:82-83`)
- Runner TEE config is only `kms_url` (`crates/runner/src/config.rs:227-229`, `:954-963`)
- Manager has no TEE runtime path (`crates/manager/src/rt/service.rs:115-147`)
- Remote runtime path still returns a local native service handle in one path (`crates/manager/src/remote/service.rs:406-418`)
- Secure bridge endpoint policy currently conflicts with public-IP registration flow (`secure_bridge.rs:211-239` vs `:392-400`)

---

## 9. What We Learned from Sandbox Blueprint

## 9.1 Patterns to adopt

1. **Clear backend lifecycle contract**
- `deploy/attestation/stop/destroy` is the right base shape (`sandbox-runtime/src/tee/mod.rs:158-173`)

2. **Persist backend deployment metadata for cleanup**
- `tee_deployment_id`, metadata JSON, attestation snapshot are critical (`runtime.rs:276-284`, `:845-847`)

3. **Direct backend hardening defaults**
- device mapping + reduced privileges + readonly rootfs + tmpfs (`tee/direct.rs:101-130`)

4. **Native attestation path where possible**
- ioctl-based attestation reduces coupling to sidecar implementation (`tee/attestation.rs:151-165`, `:171-261`)

5. **Health gate before attestation fetch**
- deployment waits for sidecar readiness before report collection (`tee/mod.rs:325-352`)

6. **Sealed-secret API shape**
- public-key fetch + sealed upload + attestation fetch is a practical surface (`tee/sealed_secrets_api.rs:47-52`, `:98-102`, `:161-165`)

## 9.2 Patterns to avoid

1. **Global singleton backend state**
- `OnceCell` global backend (`tee/mod.rs:216-245`) makes composition/testing harder in SDK-level multi-runner contexts.

2. **Stringly typed env-driven backend selection as primary API**
- `TEE_BACKEND` switch is useful for bootstrap, but SDK API should be typed first (`tee/backend_factory.rs:25-35`).

3. **Attestation report too weakly modeled**
- raw `Vec<u8>` fields without typed claim semantics (`tee/mod.rs:58-67`).

4. **Store scan lookups for deployment routing**
- `sidecar_info_for_deployment()` scans persistent records (`tee/mod.rs:251-267`); SDK should index by deployment handle.

5. **Auth/session state fully in-memory for production control plane**
- challenge/session maps and random fallback secrets (`session_auth.rs:65-69`, `:200-216`) are not robust for distributed manager deployments.

6. **Incomplete runtime abstraction boundaries**
- Remote deployment path creating local native handles (`crates/manager/src/remote/service.rs:406-418`) indicates runtime/API boundaries should be tightened before adding TEE hybrid complexity.

---

## Appendix A: Concrete code anchors reviewed

- Sandbox TEE core: `sandbox-runtime/src/tee/mod.rs`
- Sandbox providers: `tee/aws_nitro.rs`, `tee/gcp.rs`, `tee/azure.rs`, `tee/phala.rs`, `tee/direct.rs`
- Native attestation: `tee/attestation.rs`
- Sealed secrets/API: `tee/sealed_secrets.rs`, `tee/sealed_secrets_api.rs`
- Sandbox runtime integration: `sandbox-runtime/src/runtime.rs`
- Sandbox auth inspiration: `sandbox-runtime/src/session_auth.rs`
- Runner TEE stub: `crates/runner/Cargo.toml`, `crates/runner/src/config.rs`
- Manager config/runtime: `crates/manager/src/config/mod.rs`, `crates/manager/src/rt/service.rs`, `crates/manager/src/rt/remote.rs`, `crates/manager/src/remote/service.rs`
- Remote providers architecture: `crates/blueprint-remote-providers/src/core/deployment_target.rs`, `infra/traits.rs`, `infra/provisioner.rs`, `secure_bridge.rs`, `deployment/*`
- x402 background-service model: `crates/x402/src/gateway.rs`
- Tangle metadata layer model: `crates/tangle-extra/src/layers.rs`
