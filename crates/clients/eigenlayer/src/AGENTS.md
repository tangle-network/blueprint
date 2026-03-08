# src

## Purpose
EigenLayer client providing access to AVS registry, operator stake management, BLS aggregation, allocation management, and delegation queries through EigenLayer smart contracts.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root declaring `client`, `error`, and `tests` modules.
- `client.rs` - `EigenlayerClient` struct wrapping `BlueprintEnvironment`, providing methods for: HTTP/WS/wallet providers, AVS registry reader/writer, operator info service (in-memory), AVS registry service chain caller, BLS aggregation service, operator stake queries (at block, current block, by ID, history, latest update), total stake queries, registered operator public key queries, strategy allocations, max magnitude, slashable shares in queue, operators for service, and slashable assets for AVS.
- `error.rs` - Error enum wrapping IO, parse, URL, Alloy contract, AVS registry, EL contracts, operator service info, transport, and config errors. Converts into `blueprint_client_core::error::Error`.
- `tests.rs` - Integration tests using Anvil testnet: deploys EigenLayer core and AVS contracts via `eigenlayer_contract_deployer`, creates quorums, and tests provider creation (HTTP/WS), AVS registry reader/writer, operator info service, operator stake queries, and operator ID lookup.

## Key APIs
- `EigenlayerClient::new(config)` - create client from `BlueprintEnvironment`
- `avs_registry_reader()` / `avs_registry_writer()` - AVS registry access
- `bls_aggregation_service_in_memory()` - BLS signature aggregation
- `get_operator_stake_in_quorums_at_block()` / `get_operator_stake_in_quorums_at_current_block()` - stake queries
- `get_strategies_in_operator_set()` / `get_strategy_allocations()` / `get_max_magnitude()` - allocation management
- `get_slashable_shares_in_queue()` / `get_slashable_assets_for_avs()` - slashing queries
- `query_existing_registered_operator_pub_keys()` - operator discovery

## Relationships
- Wraps `eigensdk` crate types for registry, operator info, and BLS aggregation
- Uses `alloy_provider` / `alloy_primitives` for EVM interaction
- Depends on `blueprint_runner::config::BlueprintEnvironment` for configuration
- Tests use `blueprint_chain_setup_anvil`, `blueprint_eigenlayer_testing_utils`, and `eigenlayer_contract_deployer`
- Error type converts into `blueprint_client_core::error::Error`
