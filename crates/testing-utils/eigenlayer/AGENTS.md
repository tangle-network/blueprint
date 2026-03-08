# eigenlayer

## Purpose
EigenLayer-specific testing utilities crate (`blueprint-eigenlayer-testing-utils`) providing test harnesses for running blueprint integration tests against EigenLayer contract deployments on local Anvil instances. Handles EigenLayer core contract deployment, operator registration, and bridge/auth proxy setup.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Implementation: `EigenlayerTestHarness` for end-to-end EigenLayer testing, environment setup helpers, and runner integration

### Files
- `Cargo.toml` - Crate manifest; depends on `blueprint-runner` (eigenlayer), `blueprint-evm-extra`, `blueprint-auth`, `blueprint-manager-bridge` (server), `blueprint-core-testing-utils`, `blueprint-chain-setup` (anvil), `eigensdk`, `eigenlayer-contract-deployer`, alloy crates
- `CHANGELOG.md` - Release history
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `EigenlayerTestHarness::setup()` - Create a test harness with Anvil testnet, deployed EigenLayer contracts, bridge, and auth proxy
- `EigenlayerTestHarness::setup_with_context()` - Setup with custom job context injection
- Environment setup functions for EigenLayer protocol settings
- Runner integration for EigenLayer-protocol blueprint testing

## Relationships
- Depends on `blueprint-core-testing-utils` for `TestRunner` and shared infrastructure
- Depends on `blueprint-chain-setup` with `anvil` feature for testnet management
- Depends on `eigenlayer-contract-deployer` for deploying EigenLayer core contracts (DelegationManager, StrategyManager, etc.)
- Provides the standard test harness for EigenLayer-protocol blueprint testing
