# tracker

## Purpose
Deployment tracking and lifecycle management for Blueprint service instances. Maps service instances to their deployed infrastructure across multiple targets (local Docker/Kubernetes, cloud VMs, managed Kubernetes clusters, SSH/bare metal), handles graceful cleanup with retry logic when services are terminated or TTLs expire, and provides persistent state management with webhook notifications.

## Contents (one hop)
### Subdirectories
- [x] `cleanup/` - Platform-specific cleanup handler implementations for 14 deployment types (local runtimes, cloud VMs, managed K8s, SSH/bare metal)

### Files
- `mod.rs` - Module coordinator re-exporting public API from core and types; includes comprehensive test suite.
  - **Key items**: `DeploymentTracker`, `ttl_checker_task`, `CleanupHandler`, `DeploymentRecord`, `DeploymentStatus`, `DeploymentType`
- `core.rs` - Central tracker logic for managing deployment lifecycle with in-memory state, disk persistence, and cleanup dispatch.
  - **Key items**: `DeploymentTracker`, `register_deployment()`, `handle_termination()`, `handle_ttl_expiry()`, `check_all_ttls()`, `ttl_checker_task()`
  - **Interactions**: Loads/saves state via `serde_json` to `deployment_state.json`; dispatches cleanup via `CleanupHandler` trait; sends webhook notifications via `reqwest`
- `types.rs` - Domain model types for deployment tracking.
  - **Key items**: `DeploymentRecord`, `DeploymentType` (14 variants), `DeploymentStatus` (5 states), `CleanupHandler` trait
  - **Interactions**: Uses `uuid::Uuid` for IDs, `chrono::DateTime<Utc>` for timestamps, references `crate::core::resources::ResourceSpec`

## Key APIs (no snippets)
- **Types**: `DeploymentTracker` (main orchestrator), `DeploymentRecord` (metadata), `CleanupHandler` (async trait), `DeploymentType` (14-variant enum), `DeploymentStatus` (5-state enum)
- **Functions**: `register_deployment()`, `handle_termination()`, `handle_ttl_expiry()`, `check_all_ttls()`, `list_deployments()`, `list_active()`, `get_by_instance_id()`, `update_instance_id()`
- **Background task**: `ttl_checker_task(tracker)` - Runs TTL checks every 60 seconds

## Relationships
- **Depends on**: `crate::core` (Error, CloudProvider, ResourceSpec), `crate::deployment::ssh` (SshConnection, SshDeploymentClient), `crate::providers` (GcpProvisioner), `crate::infra` (CloudProvisioner), `blueprint_core` (logging)
- **Used by**: `crate::deployment::manager_integration` (RemoteDeploymentRegistry integrates tracker as `Arc<DeploymentTracker>`), `crate::deployment::mod` (re-exports public API)
- **Data/control flow**:
  - Registration: app registers deployment -> tracker stores in-memory + persists to JSON -> schedules TTL task
  - TTL checking: background task every 60s -> `check_all_ttls()` -> triggers `handle_ttl_expiry()` for expired entries
  - Termination: event -> `handle_termination()` -> dispatch cleanup handler by type -> retry up to 3x (5s backoff) -> remove record + send webhook
  - State recovery: loads `deployment_state.json` on startup for restart resilience

## Notes
- Cleanup retries up to 3 times with 5-second backoff; failure preserves record in Active state
- State persisted as JSON in configurable state directory
- Feature-gated cleanup handlers: `#[cfg(feature = "aws")]`, `#[cfg(feature = "gcp")]`
- `DeploymentTracker` implements Clone via Arc-sharing for async task distribution
