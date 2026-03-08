# services

## Purpose
High-level service managers for EigenLayer operator operations. Provides abstractions over the EigenLayer smart contract interactions for operator lifecycle management, rewards claiming, and slashing monitoring. Each manager wraps a `BlueprintEnvironment` and accesses ECDSA keys from the keystore.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports the `lifecycle`, `rewards`, and `slashing` submodules.
- `lifecycle.rs` - `OperatorLifecycleManager` for operator registration/deregistration and metadata updates. `OperatorMetadata` struct for update payloads. Methods: `deregister_operator()` (removes from AVS operator sets), `update_operator_metadata()` (updates metadata URL and delegation approver), `get_operator_status()` (checks registration status). Uses `ELChainWriter`/`ELChainReader` from eigensdk.
- `rewards.rs` - `RewardsManager` for rewards claiming and earnings tracking. Methods: `get_claimable_rewards()` (queries current claimable distribution root), `calculate_earnings_per_strategy()` (returns strategy-to-shares mapping), `claim_rewards()` (single claim via `processClaim`), `claim_rewards_batch()` (gas-optimized batch claiming via `processClaims`), `is_operator_registered()`. Interacts with `RewardsCoordinator` contract.
- `slashing.rs` - `SlashingMonitor` for slashing detection. `SlashingStatus` and `SlashingEvent` data types. Methods: `is_operator_slashed()` (checks via `DelegationManager.isOperator`), `get_slashing_status()` (returns detailed status), `get_slashable_shares()` (queries slashable shares for a given strategy). Interacts with `DelegationManager` contract.

## Key APIs
- `OperatorLifecycleManager` -- deregister, update metadata, check status
- `OperatorMetadata` struct -- metadata URL + delegation approver address
- `RewardsManager` -- query claimable rewards, calculate per-strategy earnings, claim single/batch
- `SlashingMonitor` -- check slashing status, query slashable shares
- `SlashingStatus` / `SlashingEvent` -- slashing data types

## Relationships
- Depends on `blueprint-runner` for `BlueprintEnvironment`
- Depends on `blueprint-keystore` for ECDSA key access (`K256Ecdsa`, `EigenlayerBackend`)
- Depends on `eigensdk` for `ELChainWriter`, `ELChainReader`, `RewardsCoordinator`, `DelegationManager`
- Depends on `blueprint-evm-extra` for provider utilities
- Uses `crate::error::EigenlayerExtraError` for error handling
- Sibling to `crate::sidecar` (which provides the Sidecar API client for rewards data)
