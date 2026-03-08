# manager

## Purpose
Blueprint runtime manager (`blueprint-manager`). Orchestrates the full lifecycle of Blueprint services: configuration parsing, protocol-specific event handling (Tangle and EigenLayer), blueprint source resolution (GitHub releases, container images, local binaries), runtime execution (native processes, containers via Kubernetes/Kata, hypervisor VMs), and optional remote cloud deployment. Ships as both a library and a standalone `blueprint-manager` binary.

## Contents (one hop)
### Subdirectories
- [x] `bridge/` - Separate crate (`blueprint-manager-bridge`) providing gRPC-based IPC between manager and service processes; uses tonic/prost with proto definitions in `proto/`; supports Unix socket and vsock transports
- [x] `src/` - Main source: `config/` (CLI args and context), `executor/` (main orchestration loop via `run_blueprint_manager`), `protocol/` (Tangle and EigenLayer event handlers and metadata), `rt/` (runtime backends: `container/` for K8s/Kata, `hypervisor/` for cloud-hypervisor VMs with networking), `sources/` (blueprint binary resolution: GitHub, container, remote, testing), `blueprint/` (blueprint definition types), `sdk/` (logging/entry helpers), `remote/` (cloud deployment, gated on `remote-providers`), `error.rs`, `main.rs`
- [x] `tests/` - Integration and E2E tests: protocol flow, EigenLayer E2E, service lifecycle, blueprint sources, multi-service, runtime target, remote fetcher, serverless integration, source handler integration

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-manager`); binary target `blueprint-manager`; extensive deps including `blueprint-core`, `blueprint-runner`, `blueprint-clients`, `blueprint-keystore`, `blueprint-auth`, `blueprint-manager-bridge`, `blueprint-eigenlayer-extra`, Alloy/EigenSDK for on-chain interaction, `docktopus` for containers, `clap` for CLI, `axum`/`prometheus` for metrics endpoint, optional `kube`/`k8s-openapi` for containers, optional `cloud-hypervisor-client`/`fatfs`/`nix` for VM sandbox
- `README.md` - Overview of manager orchestration, protocol/runtime source handling, `run_blueprint_manager` entry point

## Key APIs (no snippets)
- `run_blueprint_manager()` -- main entry point that starts the orchestration loop
- `config::BlueprintManagerCli` / `BlueprintManagerContext` -- CLI argument parsing and runtime context
- `protocol::tangle` / `protocol::eigenlayer` -- protocol-specific event handlers for service activation/termination
- `sources` module -- `GithubBinaryFetcher`, `ContainerSourceHandler`, `RemoteSourceHandler`, `TestSourceHandler`
- `rt::container` -- Kubernetes/Kata container runtime
- `rt::hypervisor` -- cloud-hypervisor VM runtime (Linux only, gated on `vm-sandbox`)
- `blueprint_manager_bridge` (sub-crate) -- gRPC bridge for manager-to-service IPC

## Relationships
- Depends on `blueprint-core`, `blueprint-runner`, `blueprint-clients`, `blueprint-keystore`, `blueprint-auth`, `blueprint-eigenlayer-extra`, `blueprint-testing-utils`, `blueprint-chain-setup`
- Optional deps: `blueprint-remote-providers`, `blueprint-qos`, `blueprint-pricing-engine`, `blueprint-profiling`, `blueprint-faas`
- Consumed by operators running the `blueprint-manager` binary
- Bridge sub-crate used by `blueprint-runner` (client side) for service-to-manager communication
- Feature flags: `containers` (default, K8s), `vm-sandbox` (Linux-only hypervisor), `remote-providers` (cloud deployment), `tee` (trusted execution), `qos`, `aws`/`gcp`/`azure`/`digitalocean`/`custom` (FaaS)
