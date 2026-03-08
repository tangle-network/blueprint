# anvil

## Purpose
Anvil-based testing utilities crate (`blueprint-anvil-testing-utils`) providing test harnesses for running blueprint integration tests against local Anvil EVM testnets with Tangle contract deployments. Supports multi-operator setups, seeded testnets, and the full runner lifecycle.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Implementation: `BlueprintHarness` and `SeededTangleTestnet` for end-to-end testing, Anvil instance management, multi-operator test configurations, and Tangle contract interaction helpers

### Files
- `Cargo.toml` - Crate manifest; depends on `blueprint-runner` (tangle), `blueprint-router`, `blueprint-client-tangle`, `blueprint-tangle-extra`, `blueprint-core-testing-utils`, `blueprint-manager-bridge`, `blueprint-keystore`, `blueprint-crypto` (k256), alloy crates, and test infrastructure
- `CHANGELOG.md` - Release history
- `README.md` - Crate documentation

## Key APIs (no snippets)
- `BlueprintHarness` - Primary test harness managing Anvil instance, bridge, auth proxy, and runner lifecycle
- `SeededTangleTestnet` - Pre-configured testnet with operator keys and contract deployments
- Anvil instance management and contract deployment helpers
- Multi-operator test configuration support

## Relationships
- Depends on `blueprint-core-testing-utils` for `TestRunner` and shared test infrastructure
- Depends on `blueprint-chain-setup-anvil` for Anvil testnet setup
- Used by `blueprint-qos/tests/` and other integration test suites
- Provides the standard test harness for Tangle-protocol blueprint testing
