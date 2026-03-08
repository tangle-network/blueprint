# eigenlayer

## Purpose
Crate (`blueprint-client-eigenlayer`) providing the EigenLayer client for Tangle Blueprints. Implements the `BlueprintServicesClient` trait for EigenLayer, enabling operator registration, AVS registry queries, BLS aggregation services, and EigenLayer contract interaction.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Source code with `client.rs` (EigenLayer client implementation), `error.rs` (error types), `lib.rs` (module declarations), and `tests.rs` (unit tests).

### Files
- `CHANGELOG.md` - Version history
- `Cargo.toml` - Crate manifest; depends on `blueprint-runner` (with eigenlayer feature), `blueprint-client-core`, `blueprint-evm-extra`, `eigensdk` (AVS registry, BLS aggregation, operator info services), alloy crates, and `tokio`
- `README.md` - Crate documentation

## Key APIs
- EigenLayer client implementing `BlueprintServicesClient` with ECDSA public keys as operator identities
- Integration with eigensdk for AVS registry reads, EL contract interaction, BLS aggregation, and operator info services

## Relationships
- Implements `blueprint-client-core::BlueprintServicesClient` for EigenLayer networks
- Depends on `blueprint-runner` with `eigenlayer` feature for environment configuration
- Uses `blueprint-evm-extra` for EVM utilities
- Dev-dependencies include `blueprint-chain-setup-anvil` and `blueprint-eigenlayer-testing-utils` for integration testing
