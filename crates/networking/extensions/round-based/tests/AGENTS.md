# tests

## Purpose
Integration and simulation tests for the round-based network adapter. Implements a complete commit-reveal randomness generation protocol to validate that `RoundBasedNetworkAdapter` correctly bridges Blueprint P2P networking with `round-based` MPC semantics.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `rand_protocol.rs` - Two-round commit-reveal randomness generation protocol and its tests.
  - **Protocol**: `protocol_of_random_generation<R, M>(party, i, n, rng)` -- each party generates random bytes, commits (SHA-256 hash), broadcasts, decommits, verifies, and XORs all randomness to produce shared output. Detects cheating via `Blame`.
  - **Message types**: `Msg` enum (CommitMsg, DecommitMsg), derives `ProtocolMessage`.
  - **Tests**:
    - `simulation` -- pure in-memory sync simulation via `round_based::sim::run_with_setup`, 5 parties
    - `simulation_async` -- async simulation via `round_based::sim::async_env`, 5 parties
    - `p2p_networking` -- full P2P integration test using `TestNode`, `K256Ecdsa` keys, `wait_for_all_handshakes()`, and `RoundBasedNetworkAdapter` over real libp2p. 2 parties with bootstrap peer discovery. Requires `#[serial_test::serial]`.

## Key APIs (no snippets)
- **Functions**: `protocol_of_random_generation()` (generic async MPC protocol)
- **Types**: `Msg`, `CommitMsg`, `DecommitMsg`, `Error<RecvErr, SendErr>`, `Blame`

## Relationships
- **Depends on**: `RoundBasedNetworkAdapter` (from parent crate), `blueprint-networking` (`TestNode`, `NetworkServiceHandle`, `wait_for_all_handshakes`), `blueprint-crypto` (`K256Ecdsa`), `round-based` (`MpcParty`, `RoundsRouter`), `sha2`, `hex`
- **Used by**: CI test suite; `p2p_networking` test requires `--test-threads=1` due to mDNS
