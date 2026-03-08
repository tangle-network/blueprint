# bridge

## Purpose
gRPC bridge crate (`blueprint-manager-bridge`) for communication between the blueprint manager (host) and blueprint services (guest). Provides both client and server implementations of a protobuf-defined protocol for port allocation, auth proxy registration, service owner management, and TLS profile configuration. Supports Unix domain sockets for native deployments and VSOCK for VM guest connections.

## Contents (one hop)
### Subdirectories
- [x] `proto/` - Protobuf3 schema defining the `BlueprintManagerBridge` gRPC service with RPCs for ping, port requests, proxy registration/unregistration, service owner management, and TLS profile updates
- [x] `src/` - Rust client and server implementations of the bridge protocol, including RAII cleanup handles, service-ID pinning, and TLS profile conversions

### Files
- `Cargo.toml` - Crate manifest; depends on `blueprint-core`, `blueprint-auth`, `tonic`, `prost`; features: `client` (for guest side), `server` (for host side), `tracing`
- `build.rs` - Compiles `proto/bridge.proto` via `tonic_build` with proto3 optional field support
- `CHANGELOG.md` - Release history
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `client::Bridge` - async gRPC client for blueprints to request ports, register proxies, manage owners, and update TLS profiles
- `server::Bridge` - server builder; `spawn()` returns a `BridgeHandle` and readiness receiver
- `server::BridgeHandle` - RAII handle that cleans up socket and DB registration on drop
- `server::BridgeService` - service-ID pinning to prevent cross-bridge hijacking
- `api` module - generated protobuf types via `tonic::include_proto!`
- `TlsProfileConfig` / `TlsProfile` conversions in `tls_profile.rs`

## Relationships
- `client` feature consumed by `blueprint-runner` (guest/service side)
- `server` feature consumed by `crates/manager/src/rt/service.rs` to spawn per-service bridge instances
- Depends on `blueprint-auth` for `RocksDb`, `ServiceModel`, `ServiceOwnerModel`, and `TlsProfile`
- Referenced by `crates/manager/src/error.rs` for `Bridge` error variant
