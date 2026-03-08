# blueprint

## Purpose
Integration tests for blueprint binary execution, containerization, and QoS endpoint validation. Provides a `BlueprintTestContext` that manages the lifecycle of a running incredible-squaring blueprint process for testing.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Test context struct (`BlueprintTestContext`) managing blueprint process lifecycle, plus tests for binary availability, QoS integration, containerization, and resource requirements analysis.
- `utils.rs` - Shared utilities: blueprint build helper, test keystore setup, QoS health checks via HTTP, Prometheus metrics parsing, Docker containerization workflow, and resource requirements analysis.

## Key APIs
- `BlueprintTestContext::new()` / `start_blueprint()` / `cleanup()` - lifecycle management for blueprint process under test
- `ResourceUsage` / `BlueprintRequirements` - data types for metrics and resource analysis
- `ensure_blueprint_built()` - builds the incredible-squaring example if binary is missing
- `setup_test_keystore()` - populates a temp keystore with Sr25519/Ecdsa test keys
- `check_qos_health()` / `get_blueprint_metrics()` - HTTP probes against QoS endpoints
- `test_docker_containerization()` / `cleanup_docker_resources()` - Docker build/run/verify/cleanup flow

## Relationships
- References the incredible-squaring example blueprint binary at a relative workspace path
- Uses `reqwest` for HTTP health checks and `chrono` for timestamped identifiers
- All tests are `#[serial]` to avoid port conflicts
