# eigenlayer

## Purpose
EigenLayer protocol integration for the blueprint runner. Implements operator registration, strategy deposit, quorum staking, and operator-set enrollment for both BLS-based and ECDSA-based AVS contracts.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `bls`, `config`, `ecdsa`, `error` submodules
- `bls.rs` - `EigenlayerBLSConfig` implementing `BlueprintConfig`; full BLS registration flow (register operator, deposit into strategy, set allocation delay, stake to quorum, register for operator sets)
- `config.rs` - `EigenlayerProtocolSettings` with all AVS contract addresses and registration parameters; implements `ProtocolSettingsT` for loading from `BlueprintSettings`; includes default Anvil addresses
- `ecdsa.rs` - `EigenlayerECDSAConfig` implementing `BlueprintConfig`; ECDSA registration flow using `ECDSAStakeRegistry` with signature-based operator enrollment
- `error.rs` - `EigenlayerError` enum covering AVS registry, contract, EL contracts, registration, keystore, crypto, and signature errors

## Key APIs
- `EigenlayerBLSConfig::new(earnings_receiver, delegation_approver)` - creates BLS config; defaults to exit-after-register mode
- `EigenlayerBLSConfig::with_exit_after_register(bool)` - controls whether runner exits after registration
- `EigenlayerECDSAConfig::new(delegation_approver)` - creates ECDSA config
- `EigenlayerProtocolSettings` - all contract addresses plus registration parameters (allocation delay, deposit amount, stake amount, operator sets, staker opt-out window, metadata URL)
- `EigenlayerProtocolSettings::default()` - returns hardcoded Anvil testing addresses

## Relationships
- Implements `BlueprintConfig` trait from parent `crate`
- Depends on `eigensdk` for contract readers/writers, `blueprint_keystore` for key access, `blueprint_evm_extra` for provider/transaction utilities
- `EigenlayerProtocolSettings` is loaded by `ProtocolSettings::eigenlayer()` in the runner config layer
- Used by `crates/testing-utils/eigenlayer/` for test environment setup
