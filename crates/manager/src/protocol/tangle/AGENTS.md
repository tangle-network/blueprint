# tangle

## Purpose
Tangle protocol integration for the blueprint manager. Provides on-chain event observation via Tangle EVM contracts, on-chain metadata resolution, and service lifecycle management driven by `ServiceActivated`/`ServiceTerminated`/`OperatorPreRegistered` events.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations and re-exports: `TangleProtocolClient`, `TangleEventHandler`.
- `client.rs` - `TangleProtocolClient` struct wrapping `TangleClient` from `blueprint-client-tangle`. Builds client from `BlueprintEnvironment` (keystore, RPC endpoints, contract addresses). `next_event()` maps `TangleEvent` into `ProtocolEvent::Tangle`.
- `event_handler.rs` - `TangleEventHandler` struct with pluggable `BlueprintMetadataProvider`. `initialize()` syncs with latest block, scans contract state for active services on fallback. `handle_event()` processes `ServiceActivated` (start service), `ServiceTerminated` (stop service), `OperatorPreRegistered` (start registration-mode service) logs. `ensure_service_running()` resolves metadata, orders sources by preference/confidentiality policy, attempts each source with fallback, supports `BLUEPRINT_CARGO_BIN` local fallback. Contains source ordering logic: `ordered_source_indices()` with priority by `SourceCategory` (Native/Container/Testing) and TEE-awareness. `BlueprintMetadata` struct, `BlueprintMetadataProvider` async trait.
- `metadata.rs` - `OnChainMetadataProvider` implementing `BlueprintMetadataProvider`. Fetches blueprint definitions from Tangle contracts, converts on-chain source types (Container/Native/WASM) to manager `BlueprintSource` variants. Handles GitHub and remote fetcher metadata (JSON artifact descriptors), on-chain binary conversion (arch/OS discriminators), confidentiality policy resolution from `profilingData`. Validates container source fields. `GithubArtifactMetadata`, `RemoteArtifactMetadata` deserialization structs.

## Key APIs
- `TangleProtocolClient::new(env, ctx)` -- build client with keystore and RPC config
- `TangleProtocolClient::next_event()` -- stream Tangle block events
- `TangleEventHandler::initialize()` / `handle_event()` -- service lifecycle orchestration
- `BlueprintMetadataProvider` trait -- pluggable metadata resolution (default: `OnChainMetadataProvider`)
- `OnChainMetadataProvider` -- reads blueprint definitions and sources from Tangle contracts
- `ordered_source_indices()` -- deterministic source fallback ordering with TEE policy support

## Relationships
- Depends on `blueprint-client-tangle` for `TangleClient`, contract bindings (`ITangle`), `ConfidentialityPolicy`
- Depends on `crate::sources` for `GithubBinaryFetcher`, `RemoteBinaryFetcher`, `TestSourceFetcher`, `ContainerSource`
- Depends on `crate::rt::service::Service` for process lifecycle
- Peer module to `crate::protocol::eigenlayer`
