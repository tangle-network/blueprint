# src

## Purpose
BLS signature aggregation protocol over P2P gossip. Nodes sign a shared message, broadcast signature shares, collect them weighted against a configurable threshold, and produce an aggregated signature result. Includes malicious peer detection (invalid signatures, equivocation) and deterministic hash-based aggregator election.

## Contents (one hop)
### Subdirectories
- [x] `tests/` - Integration tests validating distributed signature aggregation across multiple nodes using BLS381, BLS377, and BN254 schemes with threshold-based weight verification.

### Files
- `lib.rs` - Crate root; declares modules and re-exports all public types.
- `protocol.rs` - Core protocol logic. `SignatureAggregationProtocol<S,W>` drives the main `async run()` loop: sign-and-broadcast, poll for incoming messages, check weighted threshold, and emit `AggregationResult`. Also defines `ProtocolConfig` (network handle, timeout, poll intervals) and `AggregationError` enum.
- `messages.rs` - Wire-format types: `AggSigMessage<S>` enum (SignatureShare, MaliciousReport, ProtocolComplete) and `AggregationResult<S>` (aggregated signature, contributors set, total weight, malicious set).
- `protocol_state.rs` - `AggregationState<S>` tracks per-message signature sets, seen signatures, malicious peers, and verified completion. `ProtocolRound` enum (Initialization, SignatureCollection, Completion) enforces forward-only state transitions.
- `aggregator_selection.rs` - `AggregatorSelector` uses hash-based deterministic election: hashes each participant's public key with message context and selects nodes below a ratio threshold. Also extends `SignatureAggregationProtocol` with `check_threshold()`, `aggregate_and_verify()`, `build_result()`, and `verify_result()`.
- `signature_weight.rs` - `SignatureWeight` trait with `weight()`, `total_weight()`, `threshold_weight()`, `calculate_weight()`, `meets_threshold()`. Implementations: `EqualWeight` (percentage-based), `CustomWeight` (per-peer map), `DynamicWeight` (runtime dispatch enum).
- `malicious.rs` - `MaliciousEvidence<S>` enum (InvalidSignature, Equivocation). Extends protocol with `handle_malicious_report()`, `verify_malicious_evidence()`, and `check_for_equivocation()`.

## Key APIs (no snippets)
- **Types**: `SignatureAggregationProtocol<S, W>`, `ProtocolConfig<S>`, `AggregationState<S>`, `AggregatorSelector`, `AggregationResult<S>`, `AggSigMessage<S>`, `MaliciousEvidence<S>`, `ProtocolRound`, `AggregationError`
- **Traits**: `SignatureWeight` (with `EqualWeight`, `CustomWeight`, `DynamicWeight` implementations)
- **Functions**: `SignatureAggregationProtocol::new()`, `.run(&[u8])` (main async entrypoint), `.is_aggregator()`, `.check_threshold()`, `.aggregate_and_verify()`, `.build_result()`, `ProtocolConfig::for_testing()`

## Relationships
- **Depends on**: `blueprint-networking` (`NetworkServiceHandle`, `ProtocolMessage`, `MessageRouting`), `blueprint-crypto` (`AggregatableSignature` trait, `blake3_256`), `blueprint-gossip-primitives` (`DeduplicationCache`), `libp2p` (`PeerId`), `bincode` (wire serialization)
- **Used by**: Blueprint protocols requiring distributed threshold signature aggregation over P2P gossip networks
