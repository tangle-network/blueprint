# src

## Purpose
Main source directory for the `blueprint-manager` crate -- the network connection layer that discovers, fetches, and runs blueprint services. Provides the CLI binary entry point, the executor loop that monitors blockchain state and spawns/despawns service processes, protocol adapters for Tangle and EigenLayer, multiple runtime backends (native, container, hypervisor, remote), and blueprint source fetching from GitHub, containers, and test fixtures.

## Contents (one hop)
### Subdirectories
- [x] `blueprint/` - Blueprint data types and active-blueprint tracking (`ActiveBlueprints` map of blueprint_id -> service_id -> Service)
- [x] `config/` - CLI argument parsing (`BlueprintManagerCli`, `BlueprintManagerConfig`) and runtime context (`BlueprintManagerContext`) with keystore, paths, and auth proxy options
- [x] `executor/` - Main execution loop (`run_blueprint_manager`); monitors blockchain finality, spawns/cleans up services, integrates remote providers
- [x] `protocol/` - Protocol abstraction layer with `ProtocolManager` enum dispatch over Tangle and EigenLayer; handles blockchain event listening and service lifecycle
- [x] `remote/` - Remote provider integration (feature-gated `remote-providers`); blueprint analysis, fetching, policy loading, pricing service, provider selection, and serverless deployment
- [x] `rt/` - Runtime backends: native process spawning, container (Kubernetes), hypervisor (VM sandbox), remote TEE; defines `ResourceLimits` and per-service `Service` lifecycle
- [x] `sdk/` - SDK entry helpers: logger setup (`setup_blueprint_manager_logger`) and utility functions
- [x] `sources/` - Blueprint binary acquisition: GitHub release downloads with attestation, container image references, remote URLs, and test-mode stubs; includes safe archive unpacking

### Files
- `lib.rs` - Crate root; declares all public modules, re-exports `run_blueprint_manager` and `blueprint_auth`
- `main.rs` - Binary entry point; parses CLI, loads config TOML, sets up logger, runs the executor with CTRL-C shutdown
- `error.rs` - Unified `Error` enum covering fetcher failures, hash mismatches, download errors, bridge errors, TEE unavailability, and protocol-specific errors

## Key APIs (no snippets)
- `run_blueprint_manager` - Top-level async function that starts the manager executor loop
- `BlueprintManagerCli` / `BlueprintManagerConfig` - clap-derived CLI and config types
- `BlueprintManagerContext` - Runtime context holding keystore, data dir, test mode flags
- `ProtocolManager` - Enum-dispatched protocol handler for Tangle and EigenLayer
- `ResourceLimits` - Per-service resource allocation (storage, memory, CPU, GPU, network)

## Relationships
- Depends on `blueprint-manager-bridge` for service-to-manager communication
- Depends on `blueprint-runner` for `BlueprintEnvironment` and protocol settings
- Depends on `blueprint-auth` for auth proxy, keystore, and DB operations
- Depends on `blueprint-client-tangle` for Tangle blockchain client
- The binary is installed as `blueprint-manager` and orchestrates the full service lifecycle
