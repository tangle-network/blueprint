# rt

## Purpose
Runtime layer providing multiple execution backends for blueprint services. Manages service lifecycle (create, start, status, shutdown) across native processes, VM-sandboxed instances, container deployments, and remote cloud instances. Each service is paired with a gRPC bridge for manager-to-service communication.

## Contents (one hop)
### Subdirectories
- [x] `container/` - Kubernetes/Kata Containers runtime for running blueprints as containerized workloads. Feature-gated on `containers`.
- [x] `hypervisor/` - cloud-hypervisor VM sandbox runtime for running blueprints in isolated VMs with TAP networking and nftables firewall rules. Feature-gated on `vm-sandbox`. Contains `net/` (network manager, nftables) and `assets/` subdirectories.

### Files
- `mod.rs` - Module declarations for `container`, `hypervisor`, `native`, `remote`, and `service` (feature-gated). Defines `ResourceLimits` struct (storage_space, memory_size, cpu_count, gpu_count, network_bandwidth) with defaults (20GB storage, 4GB memory, 2 CPUs).
- `native.rs` - `ProcessHandle` for unsandboxed native process execution. Wraps a status channel and abort handle. Methods: `status()` (non-blocking poll), `wait_for_status_change()` (async), `abort()` (sends kill signal).
- `remote.rs` - `RemoteServiceInstance` for cloud-deployed services (feature-gated on `remote-providers`). Tracks deployment status via `DeploymentTracker`, provides `start()`, `status()`, `shutdown()`, and `logs()` methods mapping cloud deployment states to the `Status` enum.
- `service.rs` - `Service` struct unifying all runtime backends behind a common interface. `Runtime` enum dispatches to Hypervisor, Container, Remote, or Native. Factory methods: `from_binary()` (auto-selects VM or native based on config), `new_vm()`, `new_container()`, `new_native()`, `new_remote()`. Each factory spawns a `Bridge` server for the service. `Status` enum (NotStarted, Pending, Running, Finished, Error, Unknown). `start()` returns a health-check future that waits for bridge connectivity (480s timeout). `shutdown()` cleans up runtime and bridge. `generate_running_process_status_handle()` monitors native child processes.

## Key APIs (no snippets)
- `Service` - unified service instance with `from_binary()`, `start()`, `status()`, `shutdown()`
- `ResourceLimits` - resource allocation spec passed to all runtime backends
- `Status` enum - service lifecycle states
- `ProcessHandle` - native process monitor with abort capability
- `RemoteServiceInstance` - cloud deployment wrapper

## Relationships
- `Service` created by `sources/` handlers (`BlueprintSourceHandler::spawn()`) and `remote/serverless.rs`
- `Service` stored in `blueprint::ActiveBlueprints` by protocol event handlers
- Each `Service` owns a `BridgeHandle` from `bridge/src/server.rs`
- `from_binary()` checks `BlueprintManagerContext::vm_sandbox_options.no_vm` to choose runtime
- `ResourceLimits` sourced from on-chain service metadata by protocol handlers
