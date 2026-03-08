# proto

## Purpose
Protocol Buffer definitions for testing the auth crate's gRPC proxy functionality. Provides a minimal echo service used in integration tests to verify that authenticated gRPC proxying works correctly.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `grpc_proxy_test.proto` - Defines `EchoService` with unary `Echo` and bidirectional streaming `EchoStream` RPCs, each using simple `EchoRequest`/`EchoResponse` messages containing a single `string message` field.

## Key APIs (no snippets)
- `EchoService.Echo` -- unary RPC for testing basic gRPC proxy passthrough.
- `EchoService.EchoStream` -- bidirectional streaming RPC for testing streamed proxy connections.
- Package: `blueprint.auth.grpcproxytest`.

## Relationships
- Used by the auth crate's gRPC proxy tests to generate Rust client/server stubs via `tonic-build` or similar codegen.
- The generated code validates that the auth proxy correctly forwards gRPC traffic after authentication.
