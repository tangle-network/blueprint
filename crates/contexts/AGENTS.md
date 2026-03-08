# contexts

## Purpose
Crate `blueprint-contexts`: Context provider traits that give Blueprint job handlers access to infrastructure services (keystore, Tangle client, EigenLayer client, EVM client, P2P networking). Implements these traits on `BlueprintEnvironment` from `blueprint-runner`, bridging the gap between the runner's configuration and the clients that jobs need.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Feature-gated context modules: `keystore.rs` (keystore access), `tangle.rs` (Tangle client access), `eigenlayer.rs` (EigenLayer context), `instrumented_evm_client.rs` (EVM client context). Main `lib.rs` conditionally exposes modules based on features.

### Files
- `CHANGELOG.md` - Version history.
- `Cargo.toml` - Crate manifest (`blueprint-contexts`). Deps: `blueprint-runner`, `blueprint-clients`, optional `blueprint-networking`, `blueprint-keystore`, `blueprint-client-tangle`. Features: `evm`, `eigenlayer`, `networking`, `keystore` (default), `tangle`.
- `README.md` - Crate documentation.

## Key APIs (no snippets)
- `KeystoreContext` trait -- provides `keystore()` method returning a `Keystore` instance. Implemented on `BlueprintEnvironment`.
- `TangleClientContext` trait -- provides async `tangle_client()` method returning a `TangleClient`. Implemented on `BlueprintEnvironment`.
- `eigenlayer` module -- EigenLayer-specific context (feature-gated on `eigenlayer`).
- `instrumented_evm_client` module -- instrumented EVM client context (feature-gated on `evm`).
- `tangle` module -- re-exports `TangleClient`, `TangleClientConfig`, `TangleSettings` (feature-gated on `tangle`).

## Relationships
- Depends on `blueprint-runner` for `BlueprintEnvironment` (the type these traits are implemented on).
- Depends on `blueprint-clients` for the underlying client types.
- Depends on `blueprint-keystore` for keystore creation and configuration.
- Used by blueprint job handlers to access infrastructure without manual client construction.
- Re-exported through `blueprint-sdk` for end-user consumption.
