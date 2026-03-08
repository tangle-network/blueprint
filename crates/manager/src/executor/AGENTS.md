# executor

## Purpose
Top-level blueprint manager execution loop. Provides the public API to start the manager, wire up the auth proxy, initialize the protocol layer, and run the event loop. Also integrates remote cloud provider deployment when the `remote-providers` feature is enabled.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `BlueprintManagerHandle` struct (start/shutdown/await pattern with `Future` impl). `run_blueprint_manager()` and `run_blueprint_manager_with_keystore()` functions that create the auth proxy, initialize a `ProtocolManager` based on `env.protocol_settings`, and run the protocol event loop under `tokio::select!` with shutdown signals. `run_auth_proxy()` sets up RocksDB and spawns the `blueprint_auth::proxy::AuthenticatedProxy` on the configured host/port. Firewall cleanup on shutdown when `vm-sandbox` is enabled.
- `remote_provider_integration.rs` - `RemoteProviderManager` struct (feature-gated on `remote-providers`) that handles cloud deployment lifecycle: `on_service_initiated()` provisions instances via `CloudProvisioner` with intelligent provider selection (TEE-aware, GPU/CPU/memory heuristics), registers with `RemoteDeploymentRegistry`, and sets up TTL management. `on_service_terminated()` cleans up deployments. Supports AWS, GCP, Azure, DigitalOcean, and Vultr with configurable regions.

## Key APIs (no snippets)
- `run_blueprint_manager(ctx, env, shutdown_cmd)` - main entry point returning a `BlueprintManagerHandle`
- `BlueprintManagerHandle` - start/shutdown handle that implements `Future` for awaiting completion
- `run_auth_proxy(data_dir, opts)` - spawns the auth proxy server, returns `(RocksDb, Future)`
- `RemoteProviderManager` - cloud deployment lifecycle manager with `on_service_initiated`/`on_service_terminated`

## Relationships
- Calls `protocol::ProtocolManager::new()` and `run()` to drive the event loop
- Uses `config::BlueprintManagerContext` for runtime state
- `RemoteProviderManager` uses `blueprint_remote_providers` crate for provisioning and tracking
- Consumed by `blueprint-manager` binary and test harnesses
