# gRPC Proxy Enablement Plan

## Objectives
- Extend the existing auth proxy to transparently forward gRPC (HTTP/2) requests while preserving today’s HTTP functionality.
- Reuse the current token-based authentication and header sanitization logic with minimal duplication.
- Support unary and streaming RPCs (client, server, bidirectional) from day one.
- Lay groundwork for future mTLS support without blocking the first milestone.

## Scope & Assumptions
- Incoming and outgoing connections terminate on the same Axum/Hyper listener and `hyper_util` client; no dedicated tonic server will be introduced.
- Upstream gRPC services are addressed via the existing `ServiceModel::upstream_url`; no transport-specific URLs needed.
- Clients will send the service identifier using the existing headers (`Authorization`, `X-Service-Id`) and gRPC metadata mirrors these headers.
- mTLS is explicitly out of scope for this milestone but future hooks (e.g., structured peer identity) should not be precluded.

## Deliverables
- Updated proxy logic capable of detecting gRPC traffic (based on HTTP/2 headers) and streaming it end-to-end.
- Transport-aware header sanitization/propagation that preserves required gRPC headers and trailers.
- Metadata-to-header bridge so auth tokens in gRPC metadata are validated and upstream metadata is enriched just like HTTP.
- Integration tests covering unary and streaming flows against a sample gRPC backend service.
- Developer documentation summarizing configuration, usage expectations, and testing instructions (update `docs/` or crate README as appropriate).

## Work Plan
1. **Audit & Refactor Proxy Core**
   - Extract reusable auth + header enrichment logic from `reverse_proxy` into helper(s) shared by HTTP and gRPC paths.
   - Confirm Hyper client configuration enables HTTP/2 pooling and adjust timeouts if needed for long-lived streams.
2. **gRPC Detection & Routing**
   - Introduce detection based on `content-type: application/grpc` (case-insensitive) and presence of `te: trailers`.
   - Route matching requests through a new proxy handler that bypasses HTTP-specific sanitization.
3. **Header & Metadata Handling**
   - Build a metadata adapter: read `authorization` and other required auth headers from gRPC metadata, normalize casing, then feed into existing validators.
   - Update sanitization to allow gRPC-required headers (`content-type`, `te`, `grpc-encoding`, `grpc-accept-encoding`, binary metadata). Keep security-sensitive filtering for forbidden hop-by-hop headers.
   - Ensure additional headers injected by the proxy propagate as gRPC metadata on the upstream request.
4. **Streaming-safe Forwarding**
   - Confirm the proxy forwards request bodies/trailers without buffering. Avoid touching the body `Body` where streaming is necessary.
   - Propagate upstream response trailers (`grpc-status`, `grpc-message`, custom metadata) back to the client.
5. **Testing & Tooling**
   - Create a lightweight test gRPC service (can reuse `tonic` in tests) that supports unary and streaming methods.
   - Add integration tests under `crates/auth/tests/` exercising token validation, metadata propagation, and streaming behaviour.
6. **Documentation & Developer Handoff**
   - Document configuration notes (HTTP/2 requirements, header expectations) and testing steps in `docs/`.
   - Note future mTLS extension points (e.g., where TLS connector config will plug in).

## Testing & Verification Strategy
- **Unit Tests**: Cover metadata parsing utilities, header sanitization changes, and trailer propagation logic.
- **Integration Tests**:
  - Unary call through proxy with valid token → expect success, upstream header injection validated.
  - Bidirectional streaming call (client sends N messages, upstream echoes) → verify stream completes and trailers propagate.
  - Negative cases: missing token, expired token, attempt to inject forbidden headers → expect `UNAUTHENTICATED` / `PERMISSION_DENIED` mapped from HTTP status.
- **gRPC Proxy Harness**:
  - Tests live in `crates/auth/src/tests/grpc_proxy_tests.rs` and spin up an in-process tonic backend plus the auth proxy. They bind to `127.0.0.1:0`; make sure your environment allows local socket binds (some sandboxes block this).
  - Run with `cargo test -p blueprint-auth --lib tests::grpc_proxy_tests::grpc_unary_proxy_round_trip_is_forwarded` and `cargo test -p blueprint-auth --lib tests::grpc_proxy_tests::grpc_streaming_proxy_round_trip_is_forwarded`, or execute the full module via `cargo test -p blueprint-auth grpc_proxy_tests`.
  - Current behaviour (pre-implementation) is **red** with `Status { code: Unavailable, message: "grpc-status header missing, mapped from HTTP status code 502" }` for both unary and streaming paths, because the HTTP-only proxy returns a 502 without the expected gRPC trailers.
  - Passing criteria: tests should succeed without panicking, the unary request should echo the payload, and the streaming call should receive the full sequence of echoed messages.
- **Manual Validation**:
  - Use `grpcurl -d '{"message":"ping"}' -H "authorization: Bearer <token>" -H "x-service-id: <id>" localhost:<port> blueprint.auth.grpcproxytest.EchoService/Echo` once the proxy implementation is in place.
- **Regression Checks**: Re-run existing HTTP proxy integration tests to ensure no regressions in REST flows.

## Review Checklist
1. **Architecture**: Does the change avoid duplicating auth/token logic and keep a single entry point for HTTP + gRPC? Are future mTLS hooks noted?  
2. **Header Handling**: Are gRPC-required headers preserved, and are forbidden headers still sanitized correctly?  
3. **Streaming**: Do handlers avoid buffering bodies and propagate trailers end-to-end?  
4. **Testing**: Are there automated tests for unary + streaming scenarios and failure paths? Manual testing instructions provided?  
5. **Docs**: Is developer documentation updated with configuration steps and known limitations?  
6. **Observability**: Do logs/tracing include request IDs and omit sensitive metadata?  
7. **Backwards Compatibility**: Do existing HTTP endpoints continue to function without changes?
