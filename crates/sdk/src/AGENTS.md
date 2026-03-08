# src

## Purpose
Meta-crate re-export hub aggregating 25+ Blueprint internal crates into a unified, feature-gated public API for building blockchain services. Serves as the primary entry point for the Blueprint SDK.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Master re-export hub (245 lines): core re-exports (always), protocol modules (feature-gated: tangle, evm, eigenlayer), and advanced subsystems (testing, networking, tee, x402, webhooks, remote-providers).
  - **Key items**: `pub use blueprint_core as core`, `pub use blueprint_runner as runner`, `pub use blueprint_router as router`, `pub use blueprint_crypto as crypto`, `pub use blueprint_keystore as keystore`, 19 feature flags
  - **Interactions**: Documents logging targets (evm-polling-producer, tangle-producer, blueprint-runner, etc.)
- `error.rs` - Unified error type (90 lines) aggregating component errors with feature-gated variants.
  - **Key items**: `Error` enum (Client, Keystore, Runner, Config, Other, + feature-gated Alloy, Eigenlayer, Networking, Stores), `AlloyError` nested enum, `implement_client_error!` macro
- `registration.rs` - Async utility (28 lines) for writing blueprint registration payloads to file.
  - **Key items**: `write_registration_inputs(env, payload) -> Result<PathBuf>`, uses `tokio::fs`
  - **Interactions**: Runner polls output file and forwards to manager during pre-registration

## Key APIs (no snippets)
- **Core re-exports** (always): `core`, `runner`, `router`, `crypto`, `keystore`, `clients`, `auth`, `contexts`, `qos`, `std`
- **Protocol modules** (feature-gated): `tangle` (blueprint-tangle-extra), `evm` (blueprint-evm-extra), `eigenlayer` (blueprint-eigenlayer-extra)
- **Subsystems** (feature-gated): `testing`, `networking` (P2P + gossip + round-based), `tee`, `x402`, `webhooks`, `remote` (cloud deployment)
- **Types**: `Error` (unified SDK error), `write_registration_inputs()` (registration utility)

## Relationships
- **Depends on**: 25+ workspace crates organized in 3 tiers: always (core, runner, router, crypto, keystore, clients, auth), protocol (tangle-extra, evm-extra, eigenlayer-extra), optional (macros, networking, stores, tee, x402, webhooks, remote-providers)
- **Used by**: All end-user blueprint service implementations

## Files (detailed)

### `lib.rs`
- **Role**: Feature-gated re-export hub with comprehensive rustdoc.
- **Key items**: 19 feature flags, module docs with logging targets and use cases (oracles, bridges, ZK, AI agents)
- **Knobs / invariants**: Feature cascade controls conditional compilation and transitive dependencies

### `error.rs`
- **Role**: Single source of truth for SDK-level error consolidation.
- **Key items**: `Error` enum, `AlloyError` sub-enum, `From<T>` impls via macros
- **Knobs / invariants**: Feature-gated variants only compiled when corresponding features enabled

### `registration.rs`
- **Role**: Writes TLV registration payloads to file for runner/manager consumption.
- **Key items**: `write_registration_inputs()`, async tokio::fs
- **Knobs / invariants**: Generic payload type via `AsRef<[u8]>`

## End-to-end flow
1. Developer adds `blueprint-sdk` with desired features to Cargo.toml
2. `lib.rs` conditionally compiles and re-exports relevant crates
3. Developer accesses unified API: `blueprint_sdk::core::Job`, `blueprint_sdk::tangle::*`, etc.
4. Errors from any component flow through `blueprint_sdk::Error`

## Notes
- 363 total lines of code; pure aggregation with no business logic
- 19 feature flags enable selective compilation
- Comprehensive rustdoc with logging targets table and use case examples
