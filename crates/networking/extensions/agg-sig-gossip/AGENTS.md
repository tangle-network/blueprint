# agg-sig-gossip

## Purpose
Gossip-based BLS signature aggregation protocol for distributed consensus. Operators collect and verify cryptographic signatures, deterministically elect aggregators via hash-based selection, apply configurable weight thresholds (equal or stake-weighted), and detect malicious peers (invalid signatures, equivocation).

## Contents (one hop)
### Subdirectories
- [x] `src/` - Protocol modules: aggregation state machine, aggregator election, message types, weight schemes, malicious detection, error handling

### Files
- `Cargo.toml` - Package manifest (v0.1.0-alpha.15). Deps: `blueprint-crypto` (aggregation), `blueprint-networking`, `blueprint-gossip-primitives` (dedup), `libp2p`, `bincode`, `tokio`, `crossbeam`, `dashmap`.
  - **Key items**: edition 2024, workspace resolver v3
- `README.md` - Usage guide with examples.
  - **Key items**: protocol overview, configuration examples
- `CHANGELOG.md` - v0.1.0-alpha.1 (2025-04-08) to v0.1.0-alpha.15 (2025-11-03).
  - **Key items**: release dates, QoS additions

## Key APIs (no snippets)
- **Types**: `SignatureAggregationProtocol<S, W>` (main orchestrator with `run()`, `handle_message()`, `sign_and_broadcast()`), `ProtocolConfig<S>` (with `new()`, `for_testing()`), `AggregationState<S>` (state machine with `try_transition_to()`), `AggregationResult<S>` (contributors, total_weight, malicious)
- **Messages**: `AggSigMessage<S>` enum (SignatureShare, MaliciousReport, ProtocolComplete)
- **Aggregator Election**: `AggregatorSelector` with `is_aggregator()`, `select_aggregators()` - deterministic hash-based selection
- **Weight Schemes**: `SignatureWeight` trait with `EqualWeight`, `CustomWeight`, `DynamicWeight` implementations
- **Malicious Detection**: `MaliciousEvidence<S>` enum (InvalidSignature, Equivocation)
- **Errors**: `AggregationError` (ThresholdNotMet, KeyNotFound, NetworkError, Timeout, etc.)

## Relationships
- **Depends on**: `blueprint-crypto` (AggregatableSignature, blake3_256), `blueprint-networking` (NetworkServiceHandle, ProtocolMessage), `blueprint-gossip-primitives` (DeduplicationCache LRU+TTL), `libp2p`, `bincode`, `dashmap`
- **Used by**: `blueprint-tangle-extra` (via P2PGossipConfig, AggregationStrategy::P2PGossip), Blueprint services needing distributed threshold signatures
- **Siblings**: `gossip-primitives/` (dependency), `round-based/` (independent)

## Notes
- Generic over any `AggregatableSignature` scheme (BLS381, BLS377, BN254, W3F BLS)
- Dual-interval polling: fast message drain (25ms), slower threshold check (50ms)
- Hash-based aggregator election without centralized coordinator
- Deduplication via LRU + blake3 cache prevents re-gossip storms
- Tests require `--test-threads=1` due to mDNS port conflicts
