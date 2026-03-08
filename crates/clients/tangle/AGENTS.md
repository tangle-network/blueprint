# tangle

## Purpose
Crate (`blueprint-client-tangle`) providing the Tangle network client for Blueprint SDK. Connects to Tangle EVM contracts to query blueprints, services, and operators; monitors events (job submissions, service lifecycle); submits job results; and interacts with the restaking system.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Source code with `client.rs` (main `TangleClient` implementation), `blueprint_metadata.rs` (execution profile resolution and confidentiality policy), `config.rs` (client configuration), `contracts.rs` (Solidity contract bindings for ITangle, IMultiAssetDelegation, IOperatorStatusRegistry), `error.rs` (error types), `lib.rs` (re-exports and `EventsClient` trait), and `services.rs` (service/blueprint info types).
- [x] `tests/` - Integration tests with `anvil.rs` for testing against a local Anvil instance.

### Files
- `Cargo.toml` - Crate manifest; depends on `blueprint-core`, `blueprint-keystore`, `blueprint-crypto` (k256), `tnt-core-bindings`, alloy crates (provider, contract, signer), `tokio`, `serde`, `tracing`
- `README.md` - Crate documentation

## Key APIs
- `TangleClient` -- main client for Tangle EVM contract interaction; event monitoring, blueprint/service/operator queries, job result submission
- `EventsClient<E>` trait -- generic trait for event-driven clients with `next_event()` and `latest_event()`
- `TangleClientConfig` / `TangleSettings` -- client configuration types
- Blueprint metadata types: `ConfidentialityPolicy`, `ExecutionProfile`, `resolve_execution_profile()`
- Service types: `BlueprintInfo`, `ServiceInfo`, `ServiceStatus`, `OperatorSecurityCommitment`
- Contract bindings: `ITangle`, `IMultiAssetDelegation`, `IOperatorStatusRegistry`, `IBlueprintServiceManager`

## Relationships
- Implements `blueprint-client-core::BlueprintServicesClient` for Tangle networks
- Depends on `blueprint-keystore` and `blueprint-crypto` for key management and signing
- Uses `tnt-core-bindings` for Tangle-specific contract ABIs
- Used by `blueprint-runner` and `blueprint-manager` for Tangle protocol integration
- Dev-tested against local Anvil instances via `blueprint-chain-setup-anvil`
