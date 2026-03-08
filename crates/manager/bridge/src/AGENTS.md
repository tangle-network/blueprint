# src

## Purpose
Rust implementation of the manager-to-service bridge, providing both client and server sides of the gRPC protocol defined in `bridge/proto/bridge.proto`. Blueprints use the client to register with the auth proxy, request ports, and manage service owners and TLS profiles. The manager spawns the server side for each service instance.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root; re-exports `error`, `api`, and conditionally `client`/`server` modules. Defines `VSOCK_PORT` constant (8000) for VM guest connections.
- `api.rs` - Generated protobuf types via `tonic::include_proto!("blueprint_manager_bridge")`.
- `client.rs` - `Bridge` client struct that connects over Unix domain sockets (native) or VSOCK (VM guest). Provides async methods: `ping`, `request_port`, `register_blueprint_service_proxy`, `unregister_blueprint_service_proxy`, `add_owner_to_service`, `remove_owner_from_service`, `update_blueprint_service_tls_profile`.
- `server.rs` - `Bridge` server struct that binds a Unix socket and spawns a gRPC server implementing `BlueprintManagerBridge`. Includes `BridgeHandle` (RAII cleanup of socket and DB registration on drop), `BridgeService` (service-ID pinning and cross-bridge hijack protection via `save_if_absent`), TLS profile validation, and host port allocation.
- `error.rs` - `Error` enum wrapping `std::io::Error`, `tonic::transport::Error`, and `tonic::Status`.
- `tls_profile.rs` - Bidirectional `From` conversions between protobuf `TlsProfileConfig` and `blueprint_auth::models::TlsProfile`.

## Key APIs (no snippets)
- `client::Bridge` - async gRPC client for blueprints to communicate with the manager
- `server::Bridge` - server builder; `spawn()` returns a `BridgeHandle` and a readiness `oneshot::Receiver`
- `server::BridgeHandle` - RAII handle that cleans up the service registration and socket on drop
- `server::BridgeService` - implements service-ID pinning to prevent one bridge from operating on another bridge's service

## Relationships
- Depends on `bridge/proto/bridge.proto` for generated types
- `client` feature used by `blueprint-runner` (guest side)
- `server` feature used by `crates/manager/src/rt/service.rs` to spawn per-service bridge instances
- Depends on `blueprint-auth` for `RocksDb`, `ServiceModel`, `ServiceOwnerModel`, and `TlsProfile`
