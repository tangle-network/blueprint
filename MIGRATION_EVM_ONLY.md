# Blueprint SDK: EVM-Only Tangle Migration Plan

## Overview

This document outlines the migration of blueprint-sdk from Tangle Substrate to EVM-only using the new `tnt-core` v2 contracts.

## Current State

### Substrate Dependencies (12 crates affected)

| Crate | Substrate Deps | Migration Status |
|-------|----------------|------------------|
| `blueprint-crypto-sp-core` | `sp-core` | ✅ Removed (k256/BLS only) |
| `blueprint-crypto-tangle-pair-signer` | `sp-core`, `sp-runtime`, `subxt` | ✅ Removed (alloy signer everywhere) |
| `blueprint-keystore` | `sp-core`, `sc-keystore` | ✅ Filesystem + remote EVM backends |
| `blueprint-client-tangle` | `tangle-subxt`, `sp-core` | ✅ Superseded by `tangle-evm` client |
| `blueprint-tangle-extra` | `tangle-subxt` | ✅ Superseded by `tangle-evm-extra` |
| `blueprint-clients` | via tangle feature | ✅ Only exports Tangle EVM + EigenLayer |
| `blueprint-runner` | `sc-keystore`, `tangle-subxt` | ✅ Tangle EVM runner finalized |
| `blueprint-contexts` | `tangle-subxt` | ✅ `tangle_evm` context only |
| `blueprint-manager` | `tangle-subxt`, `sp-core` | ✅ Protocol layer handles Tangle EVM + EigenLayer |
| `blueprint-qos` | `tangle-subxt`, `sp-core` | ✅ Heartbeats/dashboards wired to OperatorStatusRegistry metrics |
| `blueprint-macros/context-derive` | `tangle-subxt` | ✅ EVM-only derive paths |
| `blueprint-testing-utils-tangle` | Full Substrate | ✅ Replaced with `blueprint-anvil-testing-utils` seeded via `LocalTestnet.s.sol` |

### Remaining cleanup

- Move `cargo check --workspace` and example integration tests into CI so EVM blueprints stay buildable. ✅ Example harness now runs via `hello-tangle-blueprint` in CI; workspace `cargo check` still tracked.
- Track removal of any dormant Substrate context helpers (`crates/contexts/src/tangle.rs`, legacy macros) if they resurface downstream.
- Wire the new OAuth/API-key EVM blueprints into CI or operator docs so users have verified EigenLayer + Tangle EVM flows.
- Continue guarding against `sp-core`, `tangle_subxt`, or `tangle-pair` dependencies via the existing `rg` CI gate.
- Publish clear instructions for using `TNT_BROADCAST_PATH` when overriding the bundled LocalTestnet broadcast.

### Latest additions

- Added `BlueprintHarness` in `blueprint-anvil-testing-utils` to wrap Anvil + runner wiring.
- Introduced `examples/hello-tangle` as the canonical EVM blueprint harness demo (see [`examples/hello-tangle/README.md`](examples/hello-tangle/README.md)).
- Documented the operator workflow in [`docs/operators/anvil.md`](docs/operators/anvil.md) covering key generation, env vars, and the harness commands.

### Known issues

- All Anvil-backed integration tests rely on the snapshot under `crates/chain-setup/anvil/snapshots/`. The bundled
  `localtestnet-broadcast.json` is used as the fallback deployment artifact when the snapshot drifts.
- Export `RUN_TNT_E2E=1` before running these suites locally so the harness behavior matches CI (the workflow sets this automatically after cloning `tnt-core`).

### Snapshots

- The canonical Anvil snapshot lives at `crates/chain-setup/anvil/snapshots/localtestnet-state.json`; override it with
  `ANVIL_SNAPSHOT_PATH` when testing alternate states.
- Regenerate snapshots via `scripts/update-anvil-snapshot.sh`. The script spins up Anvil (`--hardfork cancun`,
  `--disable-code-size-limit`), runs `LocalTestnet.s.sol`, and dumps the resulting state. Preserve the printed Forge/Anvil
  logs when debugging (`KEEP_SNAPSHOT_LOGS=1`) so you can tie a snapshot back to the `tnt-core` commit that produced it.
- Harnesses automatically prefer the snapshot and only fall back to replaying the bundled broadcast when the snapshot is
  missing or fails validation. Override the broadcast path with `TNT_BROADCAST_PATH` if needed.

## Target Architecture

### tnt-core v2 Contract Interfaces

```
Tangle.sol (Main Entry Point)
├── createBlueprint(uri, manager) → blueprintId
├── registerOperator(blueprintId, preferences)
├── requestService(blueprintId, operators, config, ...) → requestId
├── approveService(requestId, restakingPercent)
├── submitJob(serviceId, jobIndex, inputs) → callId
├── submitResult(serviceId, callId, output)
└── Events:
    ├── BlueprintCreated(blueprintId, owner, manager)
    ├── OperatorRegistered(blueprintId, operator, preferences)
    ├── ServiceRequested(requestId, blueprintId, owner, operators)
    ├── ServiceApproved(requestId, operator)
    ├── ServiceActivated(serviceId, blueprintId, owner)
    ├── JobSubmitted(serviceId, callId, jobIndex, caller, inputs)
    └── JobResultSubmitted(serviceId, callId, operator, output)

MultiAssetDelegation.sol (Restaking)
├── registerOperator() payable
├── depositAndDelegate(operator) payable
├── scheduleUnstake(operator, token, amount)
└── Events: OperatorRegistered, Delegated, UnstakeScheduled, etc.

OperatorStatusRegistry.sol (QoS/Heartbeat)
├── submitHeartbeat(serviceId, blueprintId, statusCode, metrics, signature)
├── isOnline(serviceId, operator) → bool
├── configureHeartbeat(serviceId, interval, maxMissed)
└── Events: HeartbeatSubmitted, OperatorOffline, etc.
```

## Migration Phases

### Phase 1: Generate Rust Bindings

```bash
# In tnt-core
forge bind --crate-name tnt-core-bindings \
  --contracts src/v2/Tangle.sol \
  --contracts src/v2/restaking/MultiAssetDelegation.sol \
  --contracts src/v2/restaking/OperatorStatusRegistry.sol \
  -o ../blueprint-sdk/crates/clients/tangle-evm/bindings
```

**Output:**
- `tnt-core-bindings` crate with alloy-generated types
- `Tangle`, `MultiAssetDelegation`, `OperatorStatusRegistry` contract bindings
- Event types for all contract events

### Phase 2: Create `blueprint-client-tangle-evm`

**New crate:** `crates/clients/tangle-evm/`

```rust
pub struct TangleEvmClient {
    provider: Arc<RootProvider<Http<Client>>>,
    signer: PrivateKeySigner,
    tangle: TangleInstance<...>,
    delegation: MultiAssetDelegationInstance<...>,
    status_registry: OperatorStatusRegistryInstance<...>,
    config: BlueprintEnvironment,
}

impl TangleEvmClient {
    pub async fn new(config: &BlueprintEnvironment) -> Result<Self>;

    // Blueprint operations
    pub async fn create_blueprint(&self, uri: &str, manager: Option<Address>) -> Result<u64>;
    pub async fn get_blueprint(&self, id: u64) -> Result<Blueprint>;

    // Operator operations
    pub async fn register_operator(&self, blueprint_id: u64, preferences: &[u8]) -> Result<()>;
    pub async fn is_operator_registered(&self, blueprint_id: u64, operator: Address) -> Result<bool>;

    // Service operations
    pub async fn request_service(&self, req: ServiceRequest) -> Result<u64>;
    pub async fn approve_service(&self, request_id: u64, restaking_percent: u8) -> Result<()>;
    pub async fn get_service(&self, id: u64) -> Result<Service>;

    // Job operations
    pub async fn submit_job(&self, service_id: u64, job_index: u8, inputs: &[u8]) -> Result<u64>;
    pub async fn submit_result(&self, service_id: u64, call_id: u64, output: &[u8]) -> Result<()>;
}
```

### Phase 3: Create `blueprint-tangle-evm-extra`

**New crate:** `crates/tangle-evm-extra/`

```rust
// Producer: Subscribe to EVM events
pub struct TangleEvmProducer {
    provider: Arc<RootProvider<...>>,
    tangle_address: Address,
    filter: Filter,
}

impl Stream for TangleEvmProducer {
    type Item = JobCall;

    // Converts JobSubmitted events to JobCall
}

// Consumer: Submit results via EVM transactions
pub struct TangleEvmConsumer {
    client: TangleEvmClient,
}

impl Sink<JobResult> for TangleEvmConsumer {
    // Calls submitResult on Tangle contract
}

// Extractors (same interface as substrate version)
pub mod extract {
    pub struct CallId;
    pub struct ServiceId;
    pub struct Caller;
    pub struct Args;
    pub struct BlockNumber;
    // etc.
}
```

**Event Mapping (Substrate → EVM):**

| Substrate Event | EVM Event | Notes |
|----------------|-----------|-------|
| `Services::JobCalled` | `JobSubmitted(serviceId, callId, jobIndex, caller, inputs)` | Direct mapping |
| `Services::JobResultSubmitted` | `JobResultSubmitted(serviceId, callId, operator, output)` | Direct mapping |
| `Services::ServiceInitiated` | `ServiceActivated(serviceId, blueprintId, owner)` | Different name |
| `Services::Registered` | `OperatorRegistered(blueprintId, operator, preferences)` | Similar |
| `Services::Unregistered` | `OperatorUnregistered(blueprintId, operator)` | Similar |

### Phase 4: Update Keystore

_Status: ✅ Filesystem + remote signers now back the EVM stack; Substrate storage backends were removed. The notes below document the original design decisions._

**Option A: Keep substrate keystore optional**
```rust
// crates/keystore/src/storage/mod.rs
pub enum KeystoreBackend {
    #[cfg(feature = "substrate")]
    Substrate(LocalKeystore),
    Evm(EvmKeystore),
}

pub struct EvmKeystore {
    keys: HashMap<Address, PrivateKeySigner>,
    path: PathBuf,
}
```

**Option B: Make EVM-only the default**
- Remove `sc-keystore` dependency
- Use `alloy-signer-local` for key storage
- Store keys as JSON keystore files (compatible with geth)

### Phase 5: Update Contexts and Runner

_Status: ✅ `BlueprintEnvironment` exposes only `tangle_evm`/`eigenlayer` protocol settings and the manager/runtime now speak Alloy-only clients._

**`crates/contexts/src/tangle_evm.rs`:**
```rust
pub struct TangleEvmContext {
    client: Arc<TangleEvmClient>,
    config: BlueprintEnvironment,
}

impl TangleEvmContext {
    pub async fn client(&self) -> &TangleEvmClient;
    pub fn config(&self) -> &BlueprintEnvironment;
    pub fn operator_address(&self) -> Address;
}

// Derive macro support
#[derive(TangleEvmContext)]
pub struct MyContext {
    #[tangle_evm]
    tangle: TangleEvmContext,
}
```

**`crates/runner/src/tangle_evm/`:**
```rust
pub struct TangleEvmRunner {
    client: TangleEvmClient,
    producer: TangleEvmProducer,
    consumer: TangleEvmConsumer,
}

impl TangleEvmRunner {
    pub async fn run<H: JobHandler>(self, handler: H) -> Result<()>;
}
```

### Phase 6: Update Macros

_Status: ✅ `context-derive` now exposes `TangleEvmClientContext`, `KeystoreContext`, and `EigenlayerContext` only._

**`crates/macros/src/job.rs`:** Support EVM event schemas

```rust
#[job(
    id = 0,
    event = "JobSubmitted(uint64,uint64,uint8,address,bytes)",
    result_event = "JobResultSubmitted(uint64,uint64,address,bytes)",
    verifier(evm = "0x1234...")
)]
async fn my_job(ctx: TangleEvmContext, args: MyArgs) -> Result<MyResult> {
    // ...
}
```

### Phase 7: Integration Testing

_Status: ✅ `blueprint-anvil-testing-utils` now seeds LocalTestnet for the client, pricing engine, and blueprint-runner integration suites. `crates/pricing-engine/tests/evm_listener.rs` verifies the QoS path, while `crates/manager/tests/tangle_evm_runner.rs` runs a full BlueprintRunner stack that consumes on-chain jobs end-to-end._

**Test harness using anvil:**

```rust
#[tokio::test]
async fn test_full_workflow() {
    // Start anvil
    let anvil = Anvil::new().spawn();

    // Deploy contracts
    let deployer = LocalTestnetSetup::new(&anvil).await;
    let (tangle, delegation, registry) = deployer.deploy().await;

    // Create blueprint
    let blueprint_id = tangle.createBlueprint("ipfs://...", None).await?;

    // Register operator
    delegation.registerOperator().value(10.ether()).send().await?;
    tangle.registerOperator(blueprint_id, b"").send().await?;

    // Request and approve service
    let request_id = tangle.requestService(...).send().await?;
    tangle.approveService(request_id, 50).send().await?;

    // Submit job
    let call_id = tangle.submitJob(service_id, 0, args).send().await?;

    // Verify producer receives event
    let job_call = producer.next().await.unwrap();
    assert_eq!(job_call.call_id, call_id);

    // Submit result via consumer
    consumer.send(JobResult { call_id, output: result }).await?;

    // Verify result on chain
    let result_event = ...;
}
```

## File Changes Summary

### New Files
- `crates/clients/tangle-evm/` (new crate)
- `crates/tangle-evm-extra/` (new crate)
- `crates/testing-utils/tangle-evm/` (new crate)
- `crates/contexts/src/tangle_evm.rs`
- `crates/runner/src/tangle_evm/`

### Modified Files
- `Cargo.toml` - Add new crates, update features
- `crates/keystore/Cargo.toml` - Make substrate optional
- `crates/keystore/src/storage/mod.rs` - Add EVM backend
- `crates/clients/Cargo.toml` - Add tangle-evm feature
- `crates/sdk/Cargo.toml` - Add tangle-evm feature
- `crates/macros/context-derive/src/lib.rs` - Support EVM context

### Feature Flags
```toml
[features]
default = ["tangle-evm"]

# EVM-only Tangle (new default)
tangle-evm = [
    "blueprint-client-tangle-evm",
    "blueprint-tangle-evm-extra",
]

# Legacy Substrate Tangle (optional)
tangle-substrate = [
    "blueprint-client-tangle",
    "blueprint-tangle-extra",
    "sp-core",
    "sc-keystore",
]
```

## Migration Checklist

- [x] Phase 1: Generate Rust bindings from tnt-core
- [x] Phase 2: Create blueprint-client-tangle-evm
- [x] Phase 3: Create blueprint-tangle-evm-extra (producer/consumer)
- [x] Phase 4: Update keystore for EVM-only mode
- [x] Phase 5: Update contexts and runner
- [x] Phase 6: Update macros for EVM event schemas
- [x] Phase 7: Integration tests with anvil
- [x] Phase 8: Update examples
- [ ] Phase 9: Update documentation (operator/QoS runbooks + README polish)
- [x] Phase 10: Deprecate substrate-only code paths

## Preserved Concepts

### Producer/Consumer Pattern
The existing pattern is preserved:
- Producer subscribes to chain events → emits `JobCall`
- Handler processes jobs → produces `JobResult`
- Consumer submits results back to chain

### Generalized Event Schemas
EVM events can still use arbitrary schemas via ABI encoding:
- Job inputs/outputs are `bytes` that can encode any structure
- Blueprint metadata (JSON/IPFS) defines the schema
- Same flexibility as Substrate but with ABI instead of SCALE

### Blueprint SDK API
The high-level SDK API remains similar:
```rust
// Before (Substrate)
#[job(id = 0, event = "JobCalled")]
async fn my_job(ctx: TangleContext, ...) -> Result<...>

// After (EVM)
#[job(id = 0, event = "JobSubmitted")]
async fn my_job(ctx: TangleEvmContext, ...) -> Result<...>
```

## Timeline Estimate

| Phase | Effort | Dependencies |
|-------|--------|--------------|
| Phase 1 | 1 day | tnt-core contracts finalized |
| Phase 2 | 2-3 days | Phase 1 |
| Phase 3 | 2-3 days | Phase 2 |
| Phase 4 | 1 day | - |
| Phase 5 | 1-2 days | Phase 2, 3 |
| Phase 6 | 1 day | Phase 3 |
| Phase 7 | 2-3 days | All phases |

**Total: ~10-14 days**

## Remaining Workstreams: EVM CLI & Runtime Parity

The substrate removal unblocked the EVM harness, but multiple CLI workflows still assume Devnet-only execution or legacy managers. The following sections describe the remaining scope (ordered by immediacy) and the architectural hooks each feature must rely on.

### 1. Blueprint Deployment to Production EVM (Highest Priority)

**Goal:** make `cargo tangle blueprint deploy tangle --network {devnet,testnet,mainnet}` work end-to-end against either the harness or live contracts.

- **CLI flow:** `apps/cargo-tangle/src/cli/blueprint/deploy/tangle.rs` should hydrate a `TangleDeployment` plan from CLI args + `.env`. The flow: (1) collect blueprint artifacts via `blueprint_manager::sources`, (2) upload metadata via the `blueprint_manager_bridge::client` (IPFS/S3/GitHub), (3) use `TangleEvmClient` to call `requestService`/`registerOperator`, then (4) emit structured progress lines and transaction hashes.
- **TangleDeployment abstraction:** extract today’s `DevnetStack` orchestration into `crates/devnet-stack/src/deployment.rs`. Implement variants:
  - `DevnetHarness`: existing behavior (start an Anvil-based stack, inject artifacts into the manager, run debug containers locally).
  - `RemoteNetwork`: parameterized by RPC URL, chain ID, contract addresses, and operator keystore path; load defaults from `settings.env` (fall back to CLI flags).
  - Shared interface: `prepare_artifacts()`, `register_blueprint()`, `request_service()`, `wait_until_active(timeout)`.
- **Artifact registry support:** leverage `blueprint_manager::sources::{BinarySource, ContainerSource, WasmSource}` to normalize uploads. For on-chain manager registry lookups, translate CLI flags like `--container ghcr.io/org/image:tag` into the `SourceDescriptor` that `manager_bridge` already understands.
- **Progress logging:** ✅ every RPC transaction now emits `tx_submitted` / `tx_confirmed` events that include the hash, confirmation block, and gas used. When `--json` is passed the logs are structured JSON lines so CI can scrape them.
- **CI smoke-test:** add `cargo test -p cargo-tangle deploys_blueprint_to_devnet` that spins up `DevnetStack`, runs the deploy command against it, and asserts the resulting service status equals `Active`. Gate it behind `RUN_TNT_E2E` to avoid default local runs.
- **Blueprint definition manifest:** the CLI now accepts `--definition path/to/definition.json` when targeting `--network testnet|mainnet`. The file mirrors the `Types.BlueprintDefinition` schema: metadata URI + manager, job descriptors, optional config, and an array of implementation sources (container images or native binaries). Minimal example:
  ```jsonc
  {
    "metadata_uri": "ipfs://bafy...",
    "manager": "0x0000000000000000000000000000000000000001",
    "jobs": [
      { "name": "square", "description": "Squares u64 inputs" }
    ],
    "sources": [
      {
        "kind": "container",
        "registry": "ghcr.io",
        "image": "tangle-network/incredible-squaring",
        "tag": "v0.1.0"
      }
    ],
    "supported_memberships": ["fixed", "dynamic"]
  }
  ```
  Schemas (`params_schema`, `result_schema`, `registration_schema`, `request_schema`) are optional hex strings. Container sources describe the OCI image, while native sources take a fetcher (`github`, `ipfs`, or `http`), an `artifact_uri`, and an `entrypoint`.

### 2. CLI Service Management Parity (Operator Workflows)

**Goal:** every service/job subcommand supports both Devnet and production EVM networks.

- **Service requests/approval:** ✅ `cargo tangle blueprint service {request,approve,reject}` now accepts operator-specific exposure BPS plus asset security requirements/commitments that map 1:1 with `ITangleServices`. TTLs are interpreted as seconds (matching `Tangle.sol`), and approvals optionally attach explicit security commitments. Added `cargo tangle blueprint service list` and `cargo tangle blueprint service requests` (both with `--json`) powered by `TangleEvmClient::{list_services,list_requests}` so parity with the Substrate-era listing helpers is restored.
- **Jobs commands:** resurrect `cargo tangle blueprint jobs submit` + `jobs watch`. Use `TangleEvmClient::submit_job` and a new `wait_for_job_result` helper that streams `JobResultSubmitted` logs via alloy filters. `--watch` should stream incremental job status, falling back to polling via `get_service` when the log subscription drops.
- **Manual spawn parity:** ✅ `cargo tangle blueprint service spawn` now shells into the same manager harness used by `run`/`deploy`, letting operators target any RPC endpoint with an explicit `--spawn-method` (native/container/vm), `--data-dir`, and `--allow-unchecked-attestations` flags.
- **Heartbeat/status:** ✅ `cargo tangle operator status` queries `IOperatorStatusRegistry` via `TangleEvmClient::operator_status` and reports last heartbeat, raw status code, and whether the registry considers the operator online (`--json` supported).
- **Integration coverage:** wrap each CLI command in harness tests inside `apps/cargo-tangle/tests/`. Use `DevnetStack::operator_key()` to sign transactions so tests don’t depend on developer keys.

### 3. Container & VM Runtime Options (User Experience)

**Goal:** expose runtime selection as a first-class concept instead of only in debug/devnet commands.

- **Global prefs:** ✅ `cargo tangle blueprint run|preregister` accept `--preferred-source`, `--vm/--no-vm`, and `--save-runtime-prefs` which update `settings.env` so subsequent runs inherit the preferred source + VM setting. `service spawn` can override the same knobs per invocation.
- **Per-command overrides:** `cargo tangle blueprint run`, `deploy`, `service spawn`, and `jobs submit` should accept overrides that temporarily supersede the stored defaults.
- **Runtime propagation:** update `RunOpts` → `BlueprintRunner` wiring so the spawn method flows into both the harness (Devnet) and manager bridge payloads (production). Ensure new enums map to the underlying hypervisor/container switches we already pass to the debug path.
- **Docs:** describe why/when to pick each runtime (native developer loops, container reproducibility, VM isolation) and highlight any prerequisites (Docker socket, Hypervisor config, Nitro VM, etc.).

### 4. Documentation & Examples

**Goal:** keep users/operators unblocked with clear instructions.

- Update `README.md`, `CLAUDE.md`, and `docs/operators/*.md` to explain the new CLI commands, runtime flags, and deployment steps. Include explicit CLI snippets showing `--spawn-method`, `deploy tangle --network mainnet`, and `bridge` commands.
- Extend `examples/hello-tangle/README.md` (or add a new `examples/full-cli-flow.md`) with a walkthrough: create blueprint → deploy to devnet → submit job → watch result → inspect logs.
- Document `settings.env` keys for production: RPC URLs, contract addresses, keystore path, spawn prefs, manager bridge endpoint, artifact registries (GitHub releases, S3 bucket), and expectations for secrets handling.

### 6. Observability & Tests

**Goal:** ensure CLI behavior is verifiable and debuggable.

- **CLI integration tests:** under `apps/cargo-tangle/tests/` add suites that execute CLI binaries via `assert_cmd` while a `DevnetStack` (Anvil) is running. Cover `blueprint run`, `jobs submit/watch`, deployment smoke test, bridge listing, and operator status commands. Gate behind `RUN_TNT_E2E=1` and mark with `serial` (or annotate for `cargo nextest`).
- **Logging knobs:** every CLI entry point should respect `RUST_LOG`, `--log-format json`, and `--quiet`. Document the env vars and provide templates for piping logs into `jq`.
- **Nextest config:** update `nextest.toml` to run CLI suites serially to avoid Anvil port clashes and set generous timeouts (deployment flows can take >60s).
- **Harness telemetry:** expose additional metrics (container stdout, deploy progress) via structured logs so flaky runs can be triaged quickly.

### 7. Future: Remote Deployment Pipeline

**Goal:** design the automation path for blueprint repos/CI.

- Offer `cargo tangle blueprint deploy tangle --artifacts <dir>` that zips the artifacts, uploads them (GitHub Releases, S3, OCI registry), and registers the metadata through the manager bridge before hitting the on-chain contracts. This command should be non-interactive and suitable for CI with `--yes`.
- Provide a GitHub Actions workflow template (in `docs/ci/blueprint-deploy.md`) that (1) builds artifacts, (2) runs unit + harness tests, (3) calls the deploy command against testnet, and (4) optionally promotes to mainnet upon manual approval.
- Define secrets/layout expectations: AWS/GitHub tokens for uploads, operator keystore path in CI, RPC endpoints, spawn preferences. Recommend storing them in repository environments or HashiCorp Vault.
- Track dependencies: requires production-ready manager bridge endpoints, hardened artifact storage, and the CLI features above.

These workstreams complete the parity gap between the restored devnet/debug experience and the production-grade EVM stack. Prioritize production deployment support and CLI parity first; the remaining sections can proceed in parallel once those foundations land.
