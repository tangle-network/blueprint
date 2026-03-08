# src

## Purpose
Tangle network client providing connectivity to Tangle EVM contracts for blueprint operators. Supports querying blueprints, services, and operators; monitoring events; submitting job results; managing restaking; and reading/writing blueprint execution metadata.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root with module declarations, re-exports of all public types, and `EventsClient<TangleEvent>` implementation for `TangleClient`. Includes comprehensive doc comments with usage examples.
- `client.rs` - `TangleClient` struct backed by `DynProvider<Ethereum>`, implementing `BlueprintServicesClient`. Provides methods for: blueprint/service/operator queries, event monitoring, job result submission, restaking operations (stake, delegation, deposits, locks, unstake, withdrawals), operator status heartbeats, and service request management. Defines key types: `TangleEvent`, `EcdsaPublicKey`, `RestakingMetadata`, `RestakingStatus`, `OperatorMetadata`, `OperatorStatusSnapshot`, `DelegationInfo`, `DepositInfo`, `LockInfo`, `AssetInfo`, `JobSubmissionResult`, `TransactionResult`.
- `config.rs` - `TangleClientConfig` (HTTP/WS endpoints, keystore URI, data dir, test/dry-run modes) and `TangleSettings` (blueprint ID, service ID, contract addresses for Tangle core, restaking, and status registry).
- `contracts.rs` - Re-exports TNT core contract bindings: `ITangle`, `IMultiAssetDelegation`, `IOperatorStatusRegistry`, `IBlueprintServiceManager`, `ITangleServices`, and their associated types.
- `error.rs` - Error enum covering transport, transaction, contract, config, keystore, blueprint/service/operator not found, missing ECDSA key, party not found, missing status registry, serialization, and provider-not-initialized errors.
- `services.rs` - Service-specific query types: `ServiceInfo`, `BlueprintInfo`, `BlueprintConfig`, `MembershipModel`, `PricingModel`, `ServiceStatus`, `ServiceRequestInfo`, `ServiceRequestParams`, `OperatorSecurityCommitment`.
- `blueprint_metadata.rs` - Helpers for execution metadata on blueprints: `ConfidentialityPolicy` (Any/TeeRequired/StandardRequired/TeePreferred), `ExecutionProfile`, `ExecutionProfileError`, and functions to resolve/inject execution profiles and job profiling data from/into contract metadata fields.

## Key APIs
- `TangleClient::new()` / `TangleClient::with_keystore()` - client construction
- `get_blueprint_info()` / `get_service_info()` / `get_operator_metadata()` - queries
- `submit_result()` - job result submission
- `next_event()` / `latest_event()` - block event monitoring
- `resolve_execution_profile()` / `inject_execution_profile()` - metadata management
- `ConfidentialityPolicy` / `ExecutionProfile` - TEE placement policy types
- `TangleClientConfig` / `TangleSettings` - configuration
- `EventsClient<E>` trait - generic event source abstraction

## Relationships
- Implements `BlueprintServicesClient` from `blueprint_client_core`
- Uses `tnt_core_bindings` for Solidity contract ABI types
- Uses `blueprint_keystore` for ECDSA key management
- Uses `alloy_provider` / `alloy_primitives` for EVM interaction
- Consumed by `blueprint_manager` for Tangle event handling and metadata resolution
