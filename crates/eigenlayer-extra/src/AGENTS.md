# src

## Purpose
Source directory for the EigenLayer AVS (Actively Validated Service) framework crate. Provides production-ready abstractions on top of `eigensdk-rs` for building AVS operators, including registration management, on-chain discovery, operator lifecycle, rewards, slashing monitoring, task aggregation, and sidecar RPC communication.

## Contents (one hop)
### Subdirectories
- [x] `services/` - High-level operator services: lifecycle management (`OperatorLifecycleManager`), rewards claiming (`RewardsManager`), and slashing monitoring (`SlashingMonitor`)
- [x] `sidecar/` - JSON-RPC client and types for communicating with the EigenLayer operator sidecar process

### Files
- `client.rs` - `AggregatorClient` for submitting signed task responses to an aggregator RPC server with retry logic
- `contract_conversions.rs` - Type conversions between eigensdk contract types and local representations
- `discovery.rs` - `AvsDiscoveryService` for querying on-chain AVS state and operator registration status
- `error.rs` - `EigenlayerExtraError` enum covering keystore, contract, transaction, and provider errors
- `generic_task_aggregation.rs` - Generic task/response framework (`TaskResponse`, `SignedTaskResponse`) for AVS task processing
- `lib.rs` - Crate root; declares modules and re-exports key types
- `registration.rs` - `RegistrationStateManager` for AVS operator registration, deregistration, and state tracking
- `rpc_server.rs` - JSON-RPC server infrastructure for exposing aggregator endpoints
- `util.rs` - Utility functions for EigenLayer AVS development

## Key APIs (no snippets)
- `AvsDiscoveryService` / `DiscoveredAvs` / `OperatorStatus` - On-chain AVS discovery and status queries
- `AvsRegistration` / `AvsRegistrationConfig` / `RegistrationStateManager` - Registration lifecycle management
- `OperatorLifecycleManager` - Operator startup, heartbeat, and shutdown coordination
- `RewardsManager` - Rewards claiming and distribution
- `SlashingMonitor` - Monitors and responds to slashing events
- `AggregatorClient` - HTTP-based RPC client for task aggregation
- `TaskResponse` / `SignedTaskResponse` - Generic task processing types

## Relationships
- Depends on `blueprint-core`, `blueprint-keystore`, `eigensdk`, `alloy-*`
- Mirrors `blueprint-tangle-extra` but for EigenLayer protocol
- Shared registration logic used by both CLI and manager
- Consumed by `blueprint-runner` and `blueprint-manager` for EigenLayer protocol handling

## Notes
- Async-first design with real contract integration (no mocks)
- Framework approach: augments eigensdk rather than wrapping it
