# proto

## Purpose
Protobuf service definition for the gRPC bridge between the blueprint manager (host) and blueprint services (guest). Defines the wire protocol for port allocation, auth proxy registration, service owner management, and TLS profile configuration.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `bridge.proto` - Protobuf3 schema defining the `BlueprintManagerBridge` gRPC service with RPCs for `Ping`, `RequestPort`, `RegisterBlueprintServiceProxy`, `UnregisterBlueprintServiceProxy`, `AddOwnerToService`, `RemoveOwnerFromService`, and `UpdateBlueprintServiceTlsProfile`. Also defines message types including `ServiceOwner` (with `KeyType` enum), `TlsProfileConfig` (envelope-encrypted TLS assets, mTLS settings, SNI, SAN templates), and request/response messages.

## Key APIs (no snippets)
- `BlueprintManagerBridge` service - the full gRPC interface between manager and blueprint
- `TlsProfileConfig` message - encrypted TLS certificate/key material with mTLS and SNI configuration
- `ServiceOwner` message - public key identity (ECDSA or SR25519) for service access control

## Relationships
- Consumed by `bridge/src/api.rs` via `tonic::include_proto!` to generate Rust types
- Client side implemented in `bridge/src/client.rs` (`Bridge` struct)
- Server side implemented in `bridge/src/server.rs` (`BridgeService` struct)
- TLS profile types bridged to `blueprint_auth::models::TlsProfile` via `bridge/src/tls_profile.rs`
