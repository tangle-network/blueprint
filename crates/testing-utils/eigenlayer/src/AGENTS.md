# src

## Purpose
EigenLayer-specific test harness and utilities. Provides `EigenlayerTestHarness` for spinning up an Anvil testnet with EigenLayer core contracts deployed, and `EigenlayerBLSTestEnv` implementing the `TestEnv` trait for BLS-based AVS integration tests.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Re-exports `env`, `harness`, `runner` submodules; re-exports `Error` from `blueprint_core_testing_utils`
- `env.rs` - `setup_eigenlayer_test_environment(http_endpoint)` that creates a quorum on the `RegistryCoordinator` and returns an `EigenlayerProtocolSettings` with default Anvil addresses; `TOKEN_ADDR` constant for the test ERC-20 strategy token
- `harness.rs` - `EigenlayerTestHarness` that boots Anvil, seeds a keystore, optionally deploys EigenLayer contracts, starts an auth proxy and manager bridge, and produces a configured `BlueprintEnvironment`; `deploy_eigenlayer_core_contracts()` for deploying the full EigenLayer core stack; account accessor helpers (`get_owner_account`, `get_aggregator_account`, `get_task_generator_account`)
- `runner.rs` - `EigenlayerBLSTestEnv<Ctx>` implementing `TestEnv` with `EigenlayerBLSConfig`; wraps `TestRunner`, spawns runner in background task, checks for immediate failure, cleans up on drop

## Key APIs
- `EigenlayerTestHarness::setup(owner_key, test_dir, testnet, settings)` - full harness setup with auth proxy and bridge
- `EigenlayerTestHarness::env()` - access the configured `BlueprintEnvironment`
- `setup_eigenlayer_test_environment(http_endpoint)` - creates quorum and returns `EigenlayerProtocolSettings`
- `deploy_eigenlayer_core_contracts(http, key, owner)` - deploys strategy manager, delegation manager, rewards coordinator, etc.
- `EigenlayerBLSTestEnv::new(config, env)` - creates test environment wrapping `TestRunner<Ctx>`
- `EigenlayerBLSTestEnv::run_runner(context)` - spawns runner in background, verifies no immediate failure

## Relationships
- Depends on `blueprint_core_testing_utils` for `TestRunner`, `TestEnv` trait, and `Error` type
- Depends on `blueprint_runner::eigenlayer` for `EigenlayerBLSConfig` and `EigenlayerProtocolSettings`
- Depends on `blueprint_chain_setup` for Anvil container management and key injection
- Depends on `eigenlayer_contract_deployer` for core contract deployment
- Uses `blueprint_auth` for auth proxy and `blueprint_manager_bridge` for bridge setup
