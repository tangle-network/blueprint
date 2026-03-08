# filters

## Purpose
Provides Tower `Predicate`-based filters for routing EVM job calls and utility functions for creating Alloy RPC log filters. Filters check job call extensions for matching contract addresses or event signatures, while the utility functions produce `alloy_rpc_types::Filter` instances for node-level event subscription.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Top-level filter creation utilities: `create_event_filter<E>` (filter by address + event type), `create_contract_filter` (filter by address + multiple event signatures), `create_event_type_filter<E>` (filter by event type from any contract). Re-exports submodules.
- `contract.rs` - `MatchesContract` predicate that implements `tower::filter::Predicate<JobCall>`. Checks whether any log in the job call's extensions has a matching contract address. Defines `MatchesContractError` for missing/empty logs or no match.
- `event.rs` - `MatchesEvent` predicate that checks whether any log's `topic0` matches a given event signature hash (`B256`). Defines `MatchesEventError` with similar variants to `MatchesContractError`.

## Key APIs
- `MatchesContract(Address)` - Tower predicate filtering job calls by contract address
- `MatchesEvent(B256)` - Tower predicate filtering job calls by event signature
- `create_event_filter<E: SolEvent>(address)` - builds an Alloy RPC `Filter`
- `create_contract_filter(address, signatures)` - builds a multi-event RPC `Filter`
- `create_event_type_filter<E: SolEvent>()` - builds an address-agnostic RPC `Filter`

## Relationships
- Predicates operate on `blueprint_core::JobCall` and inspect `Vec<alloy_rpc_types::Log>` stored in extensions by producers in `crate::producer`
- Used with `tower::filter::Filter` to gate which job calls reach handlers
- `MatchesContract` pairs with the `ContractAddress` extractor in `crate::extract::contract`
