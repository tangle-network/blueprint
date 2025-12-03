# Blueprint SDK: EVM-Only Tangle Migration Plan

## Overview

This document outlines the migration of blueprint-sdk from Tangle Substrate to EVM-only using the new `tnt-core` v2 contracts.

## Current State

### Substrate Dependencies (12 crates affected)

| Crate | Substrate Deps | Migration Status |
|-------|----------------|------------------|
| `blueprint-crypto-sp-core` | `sp-core` | Replace with pure Rust crypto |
| `blueprint-crypto-tangle-pair-signer` | `sp-core`, `sp-runtime`, `subxt` | Replace with alloy signer |
| `blueprint-keystore` | `sp-core`, `sc-keystore` | Add EVM backend |
| `blueprint-client-tangle` | `tangle-subxt`, `sp-core` | Create EVM variant |
| `blueprint-tangle-extra` | `tangle-subxt` | Replace with EVM events |
| `blueprint-clients` | via tangle feature | Update feature flags |
| `blueprint-runner` | `sc-keystore`, `tangle-subxt` | Add EVM runner |
| `blueprint-contexts` | `tangle-subxt` | Add EVM context |
| `blueprint-manager` | `tangle-subxt`, `sp-core` | Add EVM handler |
| `blueprint-qos` | `tangle-subxt`, `sp-core` | Migrate to EVM |
| `blueprint-macros/context-derive` | `tangle-subxt` | Update for EVM |
| `blueprint-testing-utils-tangle` | Full Substrate | Create EVM harness |

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

- [ ] Phase 1: Generate Rust bindings from tnt-core
- [ ] Phase 2: Create blueprint-client-tangle-evm
- [ ] Phase 3: Create blueprint-tangle-evm-extra (producer/consumer)
- [ ] Phase 4: Update keystore for EVM-only mode
- [ ] Phase 5: Update contexts and runner
- [ ] Phase 6: Update macros for EVM event schemas
- [ ] Phase 7: Integration tests with anvil
- [ ] Phase 8: Update examples
- [ ] Phase 9: Update documentation
- [ ] Phase 10: Deprecate substrate-only code paths

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
