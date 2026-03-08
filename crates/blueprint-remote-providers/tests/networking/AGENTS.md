# networking

## Purpose
Tests for network communication patterns including failure resilience, proxy-to-remote forwarding, circuit breakers, rate limiting, retry logic, and secure communication (mTLS, authentication, credential management).

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations for all networking test submodules.
- `failure_resilience.rs` - Simulations of distributed system failure modes: circuit breaker state machine, adaptive timeouts, concurrent operation deadlock prevention, multi-provider failover, partial failure handling in bulk operations, exponential backoff with jitter, and resource cleanup tracking.
- `proxy_integration_tests.rs` - Integration tests for the `SecureBridge` proxy layer: health checks, request forwarding to mock TCP services, circuit breaker integration (fast-fail on open circuit), auth proxy integration with encrypted credentials (`SecureCloudCredentials`), and retry with intermittent failures.
- `resilience_tests.rs` - Unit tests for the `resilience` module: `CircuitBreaker` state transitions (closed/open/half-open/recovery), `with_retry()` backoff logic, and `RateLimiter` token bucket (burst, refill, blocking acquire).
- `secure_communication_tests.rs` - Security-focused tests: credential encryption/decryption lifecycle, secure bridge endpoint registration, deployment-to-bridge integration, auth proxy remote extension, end-to-end secure flow with JWT token generation, concurrent remote operations, credential rotation, observability instrumentation, localhost-only network isolation, external access blocking, configurable port exposure, JWT bypass prevention, certificate validation enforcement, mTLS production enforcement, authentication bypass prevention, token replay attack prevention, container security hardening, network security validation, and SSRF endpoint validation.

## Key APIs
- `SecureBridge` / `SecureBridgeConfig` / `RemoteEndpoint` - secure proxy for remote instance communication
- `CircuitBreaker` / `CircuitBreakerConfig` / `CircuitState` - circuit breaker pattern
- `RateLimiter` - token bucket rate limiter
- `with_retry()` / `RetryConfig` - configurable retry with backoff
- `SecureCloudCredentials` / `RemoteServiceAuth` / `AuthProxyRemoteExtension` - credential management and auth proxy
- `DeploymentTracker` / `DeploymentRecord` - deployment state integration

## Relationships
- Depends on `blueprint_remote_providers` for `secure_bridge`, `auth_integration`, `resilience` modules
- Uses `blueprint_auth` (`RocksDb`) for credential storage in tests
- Uses `mockito` for mock HTTP services, raw TCP listeners for proxy tests
- Tests enforce critical security properties: localhost-only binding, mTLS in production, SSRF prevention
