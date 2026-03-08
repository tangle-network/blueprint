# tests

## Purpose
Integration and property-based test suite for the `blueprint-remote-providers` crate. Validates cloud provisioning, deployment orchestration, networking resilience, security hardening, and end-to-end blueprint deployment flows across multiple cloud providers.

## Contents (one hop)
### Subdirectories
- [x] `blueprint/` - Blueprint-specific test utilities and helpers (`mod.rs`, `utils.rs`).
- [x] `deployment/` - Deployment integration tests covering SSH deployment, Kubernetes deployment and simulation, QoS Docker/Kubernetes tests, and deployment integration flows.
- [x] `integration/` - Broad integration tests including auth integration, SSH/container deployment, chaos engineering, core functionality, critical flows, manager bridge, observability, property tests, QoS integration, real blueprint tests, and remote deployment end-to-end.
- [x] `networking/` - Network resilience and communication tests including failure resilience, proxy integration, resilience tests, and secure communication tests.
- [x] `providers/` - Provider-specific integration tests for AWS and pricing API validation.
- [x] `security/` - Security-focused tests covering cloud API security, command injection prevention, container security, and network security.

### Files
- `deployment_decision_tests.rs` - Tests for deployment decision logic: provider selection algorithms, instance type mapping, cost comparison, and resource requirement translation (requires `aws` feature)
- `integration_tests.rs` - Core integration tests for multi-cloud provisioning workflows
- `log_streaming_tests.rs` - Tests for log streaming from remote instances
- `managed_kubernetes_e2e.rs` - End-to-end tests for managed Kubernetes deployments
- `property_tests.rs` - Property-based tests for configuration and resource spec invariants
- `provider_k8s_integration.rs` - Kubernetes provider integration tests
- `providers_integration.rs` - Cross-provider integration tests
- `real_blueprint_deployment.rs` - Tests deploying real blueprint binaries to cloud infrastructure
- `sdk_provisioning_tests.rs` - SDK-level provisioning workflow tests
- `update_rollback_tests.rs` - Tests for update and rollback deployment flows

## Key APIs
- No public APIs; this is a test-only directory

## Relationships
- Tests the `blueprint-remote-providers` crate (`src/`)
- Some tests are feature-gated (e.g., `aws`) and require cloud credentials to run
- Security tests validate SSRF prevention, command injection hardening, and container isolation
