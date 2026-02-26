# blueprint-chain-setup-anvil

Anvil-specific chain setup and state-management utilities.

## What it includes

- `anvil`: process/bootstrap helpers.
- `keys`: deterministic key helpers for test environments.
- `snapshot` and `state`: snapshot/state persistence and recovery helpers.

## Typical usage

Use this crate in integration tests where you need repeatable local EVM state for Blueprint services.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/chain-setup/anvil
