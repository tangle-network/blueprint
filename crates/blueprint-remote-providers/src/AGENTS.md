# src

## Purpose
Source directory for the `blueprint-remote-providers` crate, providing multi-cloud infrastructure provisioning for the Blueprint Manager. Handles cloud VM provisioning, deployment orchestration, health monitoring, pricing, and secure communication between the local Blueprint auth proxy and remote instances across AWS, GCP, Azure, DigitalOcean, Vultr, and Kubernetes.

## Contents (one hop)
### Subdirectories
- [x] `core/` - Core abstractions including `CloudProvider` trait, `Error`/`Result` types, `ResourceSpec` for resource requirements, `DeploymentTarget` for remote deployment targets, and test utilities.
- [x] `deployment/` - Deployment orchestration including SSH and Kubernetes deployment clients, deployment tracking, error recovery, QoS tunneling, secure command execution, update management, and Blueprint Manager integration. Has `ssh/` and `tracker/` subdirectories.
- [x] `infra/` - Infrastructure provisioning layer with `CloudProvisioner` trait, instance status tracking, auto-deployment logic, provider adapters, and resource-to-instance type mapping.
- [x] `monitoring/` - Health monitoring with `HealthMonitor` and `HealthCheckResult` types, service discovery, log streaming, and Loki integration.
- [x] `pricing/` - Pricing service for cost estimation and reporting across cloud providers. Has a `public/` subdirectory for public pricing APIs.
- [x] `providers/` - Cloud provider implementations for AWS, Azure, DigitalOcean, GCP, Vultr, and Kubernetes. Each has its own subdirectory with provider-specific provisioning logic. Contains shared `common/` utilities.
- [x] `security/` - Security layer with authentication, AES-GCM encrypted credential storage (`SecureCredentialManager`), and secure HTTP client configuration.
- [x] `shared/` - Shared utilities for Kubernetes deployment specs, SSH deployment helpers, and security helpers used across modules.

### Files
- `auth_integration.rs` - Auth proxy integration for remote services; `SecureCloudCredentials` with AES-GCM encryption, `RemoteServiceAuth` with JWT access token generation/validation, and `AuthProxyRemoteExtension` for registering/forwarding authenticated requests
- `config.rs` - Cloud provider configuration structs (`CloudConfig`, `AwsConfig`, `GcpConfig`, `AzureConfig`, `DigitalOceanConfig`, `VultrConfig`) with environment variable loading
- `lib.rs` - Crate root; re-exports primary types, declares all modules, provides legacy compatibility aliases, and helper functions for creating HTTP clients
- `observability.rs` - `MetricsCollector` for recording and retrieving named metrics values
- `secure_bridge.rs` - `SecureBridge` for secure mTLS-authenticated tunneling between Blueprint Manager and remote instances; endpoint registration with SSRF protection, health checks, and request forwarding

## Key APIs
- `CloudProvider` trait -- core abstraction for cloud provider operations
- `SecureBridge` -- secure communication bridge with mTLS, endpoint management, SSRF validation, and request forwarding
- `AuthProxyRemoteExtension` -- auth proxy extension for registering and routing to remote services with JWT authentication
- `SecureCloudCredentials` -- AES-GCM encrypted credential storage with blake3 key derivation
- `CloudConfig::from_env()` -- loads multi-cloud configuration from environment variables
- `create_provider_client(timeout)` -- creates a configured reqwest HTTP client

## Relationships
- Depends on `blueprint-core` for logging macros, `blueprint-std` for standard library types
- Used by `blueprint-manager` to provision and manage remote infrastructure for blueprint services
- Integrates with `blueprint-auth` patterns for JWT-based service authentication
- The `secure_bridge` module coordinates with `deployment/tracker` for deployment record updates
