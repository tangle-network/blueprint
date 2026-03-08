# tangle

## Purpose
Tangle protocol integration for the blueprint runner. Provides operator registration against Tangle EVM contracts and protocol-specific configuration loaded from environment variables.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `config` and `error` submodules
- `config.rs` - `TangleProtocolSettings` (blueprint ID, service ID, contract addresses) implementing `ProtocolSettingsT`; `TangleConfig` runtime config implementing `BlueprintConfig` with registration-input support and exit-after-register control
- `error.rs` - `TangleError` enum covering transport, contract, keystore, ECDSA decompression, missing config, and transaction errors

## Key APIs
- `TangleProtocolSettings` - loaded from `BLUEPRINT_ID`, `SERVICE_ID`, `TANGLE_CONTRACT`, `RESTAKING_CONTRACT`, `STATUS_REGISTRY_CONTRACT` env vars
- `TangleConfig::new(rpc_address)` - creates config with an RPC endpoint for operator registration announcements
- `TangleConfig::with_exit_after_register(bool)` - controls post-registration behavior
- `TangleConfig::with_registration_inputs(bytes)` - attaches custom TLV payload to the `registerOperator` call
- `TangleConfig` implements `BlueprintConfig` with `register()` that calls `ITangle::registerOperator` on-chain and `requires_registration()` that queries `TangleClient::is_operator_registered`

## Relationships
- Implements `BlueprintConfig` trait from parent `crate`
- Uses `blueprint_client_tangle::TangleClient` to check registration status
- Uses `alloy` provider + signer for on-chain transaction submission
- `TangleProtocolSettings` is consumed by `ProtocolSettings::tangle()` in the runner config layer
- Used by `crates/testing-utils/anvil/` for the `BlueprintHarness` test environment
