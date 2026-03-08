# integration

## Purpose
End-to-end integration tests for the remote providers system, covering SSH deployment workflows, blueprint containerization, QoS integration, chaos engineering, core functionality, observability, and property-based testing. Many submodules are currently disabled due to missing dependencies or compiler issues.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module declarations; only `blueprint_ssh_deployment_tests`, `real_blueprint_tests`, `ssh_container_tests`, and `ssh_deployment_integration` are currently enabled.
- `auth_integration.rs` - Authentication integration tests (disabled).
- `blueprint_ssh_deployment_tests.rs` - Blueprint SSH deployment test scenarios.
- `chaos_engineering_tests.rs` - Chaos engineering and fault injection tests (disabled due to compiler cycle error).
- `core_functionality.rs` - Core functionality verification (disabled).
- `critical_flows.rs` - Critical deployment flow tests (disabled).
- `manager_bridge.rs` - Manager bridge integration tests (disabled).
- `observability.rs` - Observability and metrics tests (disabled).
- `property_tests.rs` - Property-based tests (disabled).
- `qos_integration.rs` - QoS integration tests (disabled).
- `real_blueprint_tests.rs` - Tests using real blueprint binaries.
- `remote_deployment_e2e.rs` - End-to-end remote deployment tests.
- `ssh_container_tests.rs` - SSH container lifecycle tests.
- `ssh_deployment_integration.rs` - SSH deployment integration scenarios.

## Key APIs
- Tests exercise the complete remote deployment stack from provisioning through deployment and monitoring
- Active tests focus on SSH-based deployment and container management workflows

## Relationships
- Depends on `blueprint_remote_providers` crate for deployment, SSH, and provisioning APIs
- Several modules disabled with comments indicating missing dependencies or compiler issues
- Active tests overlap with `../deployment/ssh_deployment.rs` in SSH testing scenarios
