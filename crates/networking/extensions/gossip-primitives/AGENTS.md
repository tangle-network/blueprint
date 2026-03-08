# gossip-primitives

## Purpose
Reusable gossip protocol primitives for Blueprint SDK networking. Provides protocol-agnostic building blocks: message deduplication via bounded LRU cache with TTL, async network trait abstraction, thread-safe indexed message storage, and mock network for testing. Supports `no_std` environments.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Source code with 6 Rust modules: dedup, network, store, mock, error, lib

### Files
- `Cargo.toml` - Crate manifest (v0.1.0-alpha.15). Dependencies: `blueprint-core`, `blueprint-std`, `lru`, `parking_lot`, `blake3`, `tokio`, `futures`, `async-trait`, `serde`, `bincode`, `hashbrown`. Features: `std` (default), `live` (libp2p integration), `testing` (mock network).
  - **Key items**: version `0.1.0-alpha.15`, edition 2024, three feature flags
- `README.md` - Documentation with component overview, usage examples, performance notes, and thread safety considerations.
  - **Key items**: DeduplicationCache, GossipManager, MessageStore, ProtocolNetwork, MockNetwork

## Key APIs (no snippets)
- **Traits**: `ProtocolNetwork<M>` (async network abstraction: `send_to`, `broadcast`, `subscribe`, `connected_peers`), `ProtocolNetworkExt<M>` (blanket extension: `send_to_many`, `broadcast_with_fanout`)
- **Types**: `DeduplicationCache` (LRU + TTL dedup), `GossipManager` (cache + stats wrapper), `MessageStore<M>` (thread-safe indexed storage), `PeerId` (32-byte peer identity), `NetworkEvent<M>` (Message/PeerConnected/PeerDisconnected)
- **Functions**: `GossipManager::hash_message()` (blake3), `DeduplicationCache::check_and_mark()` (atomic dedup)
- **Testing**: `MockNetwork<M>` (implements `ProtocolNetwork`), `MockNetworkBuilder<M>` (fluent topology builder: `fully_connected()`, `ring_topology()`)
- **Errors**: `GossipError` enum (Serialization, SendFailed, ChannelClosed, Timeout, Duplicate, etc.)

## Relationships
- **Depends on**: `blueprint-core`, `blueprint-std`, `lru` (0.13), `parking_lot`, `blake3`, `futures`, `async-trait`; optionally `libp2p` and `blueprint-networking` (behind `live` feature)
- **Used by**: `blueprint-networking-agg-sig-gossip-extension` (uses DeduplicationCache for signature aggregation), `blueprint-sdk` (optional `gossip` feature gate)
- **Siblings**: `agg-sig-gossip/` (depends on this), `round-based/` (independent)

## Notes
- Memory bounded: LRU default 10,000 entries; MessageStore evicts oldest 10% on overflow
- TTL default 5 minutes for dedup cache entries
- `check_and_mark()` is atomic (single lock hold) to prevent TOCTOU races
- All public types are Send + Sync; uses parking_lot for fine-grained locking
- `no_std` support via `blueprint_std` fallbacks
