# src

## Purpose
Defines context traits that extract protocol-specific clients and services from the `BlueprintEnvironment`. Each trait provides a single async accessor method, enabling blueprint jobs to obtain typed clients for their target network.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `eigenlayer.rs` - Defines `EigenlayerContext` trait with `eigenlayer_client()` method; implemented for `BlueprintEnvironment`.
- `instrumented_evm_client.rs` - Defines `EvmInstrumentedClientContext` trait with `evm_client()` method; re-exports `InstrumentedClient`.
- `keystore.rs` - Defines `KeystoreContext` trait with `keystore()` method; implemented for `BlueprintEnvironment` using filesystem-backed keystore configuration.
- `lib.rs` - Feature-gated module declarations for `eigenlayer`, `instrumented_evm_client`, `keystore`, and `tangle`.
- `p2p.rs` - Empty placeholder for future P2P networking context.
- `tangle.rs` - Defines `TangleClientContext` trait with `tangle_client()` method; implemented for `BlueprintEnvironment`, constructing a `TangleClient` from environment protocol settings and keystore.

## Key APIs (no snippets)
- `EigenlayerContext::eigenlayer_client(&self)` -- returns an `EigenlayerClient`.
- `TangleClientContext::tangle_client(&self)` -- returns a `TangleClient` configured from environment settings.
- `KeystoreContext::keystore(&self)` -- returns a filesystem-backed `Keystore`.
- `EvmInstrumentedClientContext::evm_client(&self)` -- returns an `InstrumentedClient`.

## Relationships
- Depends on `blueprint-clients` for the underlying client types and errors.
- Depends on `blueprint-runner` for `BlueprintEnvironment` (the type these traits are implemented on).
- Depends on `blueprint-keystore` for keystore creation.
- Consumed by blueprint job handlers that need network client access via the context/extractor pattern.
