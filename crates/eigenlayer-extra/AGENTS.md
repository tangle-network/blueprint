# eigenlayer-extra

## Purpose
High-level EigenLayer AVS framework (`blueprint-eigenlayer-extra`). Provides production-ready abstractions on top of `eigensdk-rs` for building Actively Validated Services: multi-AVS registration management, on-chain operator discovery, rewards/slashing monitoring, task aggregation, and configurable runtime targets (native, hypervisor, container).

## Contents (one hop)
### Subdirectories
- [x] `src/` - Crate source: discovery, registration, services (lifecycle/rewards/slashing), sidecar helpers, generic task aggregation, error types, and utilities

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-eigenlayer-extra`); depends on `blueprint-core`, `blueprint-keystore`, `blueprint-client-eigenlayer`, `blueprint-evm-extra`, `blueprint-runner`, `blueprint-crypto-bn254`, `eigensdk`, and Alloy EVM libraries
- `README.md` - Architecture overview, multi-AVS design, runtime targets, API examples for discovery/registration/rewards
- `example-avs-config.json` - Sample AVS registration configuration with contract addresses and runtime settings

## Key APIs (no snippets)
- `AvsDiscoveryService` / `DiscoveredAvs` / `OperatorStatus` -- on-chain AVS discovery and operator status queries
- `RegistrationStateManager` / `AvsRegistration` / `AvsRegistrationConfig` / `AvsRegistrations` -- persistent registration state management (stored in `~/.tangle/eigenlayer_registrations.json`)
- `OperatorLifecycleManager` -- operator lifecycle management
- `RewardsManager` -- claimable rewards checking
- `SlashingMonitor` -- slashing event monitoring
- `RuntimeTarget` -- execution environment selection (native/hypervisor/container)
- `generic_task_aggregation` module -- generic task aggregation utilities

## Relationships
- Depends on `blueprint-core`, `blueprint-keystore`, `blueprint-client-eigenlayer`, `blueprint-evm-extra`, `blueprint-runner`, `blueprint-crypto-bn254`
- Used by `blueprint-manager` for EigenLayer protocol handling
- Used by `cli/` for EigenLayer registration and deployment commands
- Test utilities provided by `blueprint-eigenlayer-testing-utils`
