# config

## Purpose
CLI configuration and runtime context for the blueprint manager. Defines all command-line arguments (paths, verbosity, source type, sandbox options, auth proxy, remote deployment) and the `BlueprintManagerContext` that holds initialized runtime state (Kubernetes client, VM network manager, RocksDB handle, cloud config).

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - `BlueprintManagerCli` and `BlueprintManagerConfig` structs (clap-derived). Sub-structs: `Paths` (blueprint_config, keystore_uri, data_dir, cache_dir, runtime_dir), `VmSandboxOptions` (no_vm, default_address_pool, network_interface; feature-gated on `vm-sandbox`), `ContainerOptions` (kube_service_port; feature-gated on `containers`), `AuthProxyOpts` (host, port), `RemoteDeploymentOptions` (enable, auto_select_cheapest, preferred_provider, max_hourly_cost, prefer_kubernetes; feature-gated on `remote-providers`). `SourceType` enum (Container, Native, Wasm). Directory verification and network interface detection helpers.
- `ctx.rs` - `BlueprintManagerContext` struct wrapping `BlueprintManagerConfig` with runtime state: `ContainerContext` (kube client, port lock, local IP), `VmContext` (network manager, interface name), `RocksDb` handle (lazy-set via mutex), and optional `CloudConfig`. Constructor performs directory creation, VM image download, Kubernetes client init, and network setup. Implements `Deref`/`DerefMut` to `BlueprintManagerConfig`.

## Key APIs (no snippets)
- `BlueprintManagerConfig` - all CLI-configurable options for the manager
- `BlueprintManagerContext` - initialized runtime context; derefs to config
- `BlueprintManagerContext::new(config)` - async constructor that validates directories, sets up networking, and initializes clients
- `SourceType` - enum controlling preferred blueprint execution method (Container, Native, Wasm)

## Relationships
- Used by `executor/mod.rs` as the entry point to `run_blueprint_manager`
- Passed to `protocol/` managers, `sources/` handlers, and `rt/service.rs` for spawning services
- `BlueprintManagerContext::db()` provides the auth proxy database to `bridge/src/server.rs`
- Cloud config loaded from env and used by `executor/remote_provider_integration.rs`
