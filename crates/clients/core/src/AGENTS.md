# src

## Purpose
Defines the `BlueprintServicesClient` trait, the core abstraction for querying operators, operator identity, and blueprint identity across all supported networks (Tangle, EigenLayer, EVM).

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Defines the `BlueprintServicesClient` trait with associated types (`PublicApplicationIdentity`, `PublicAccountIdentity`, `Id`, `Error`) and async methods: `get_operators()`, `operator_id()`, `blueprint_id()`, plus default implementations `get_operators_and_operator_id()` and `get_operator_index()`. Also defines the `OperatorSet<K, V>` type alias as `BTreeMap<K, V>`. Uses `#[auto_impl(&, Arc)]` for automatic trait implementations.
- `error.rs` - Error enum with variants for Eigenlayer, EVM, Tangle, Network, GetOperators, OperatorId, UniqueId, GetOperatorsAndOperatorId, GetOperatorIndex, and Other.

## Key APIs
- `BlueprintServicesClient` trait - the main abstraction implemented by network-specific clients
- `OperatorSet<K, V>` - type alias for operator collections
- `Error` - shared error type convertible from network-specific errors

## Relationships
- Implemented by `blueprint_client_tangle::TangleClient`, `blueprint_client_eigenlayer::EigenlayerClient`, and EVM clients
- `Error` type is used as a conversion target via `From<Error>` in network-specific error types
- Uses `auto_impl` to generate implementations for `&T` and `Arc<T>` automatically
