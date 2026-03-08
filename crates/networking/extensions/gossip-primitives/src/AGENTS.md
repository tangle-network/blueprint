# src

## Purpose
Reusable gossip protocol abstractions for the Blueprint networking stack. Provides protocol-agnostic message deduplication, network trait abstraction, indexed message storage, and mock network implementations for testing without real libp2p dependencies. Supports `no_std` environments.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Crate root with `no_std` support. Declares modules, conditionally includes `mock` (behind `testing` feature), and re-exports all public types.
- `dedup.rs` - LRU-based message deduplication with TTL expiration. `DeduplicationCache` (bounded LRU with `should_process()`, `mark_seen()`, `check_and_mark()` atomic operation, `gc()`). `GossipManager` wraps the cache with `GossipStats` tracking (messages processed, duplicates rejected, re-gossips, send failures). `GossipConfig` controls cache size, TTL, and stats enablement. Includes unit tests.
- `error.rs` - `GossipError` enum covering serialization, send/receive failures, channel closed, timeout, duplicate, validation, peer-not-found, and internal errors. Implements `Display` and `std::error::Error` (feature-gated).
- `network.rs` - Network abstraction layer. `ProtocolNetwork<M>` async trait with `local_peer_id()`, `connected_peers()`, `send_to()`, `broadcast()`, `subscribe()`. `ProtocolNetworkExt<M>` blanket extension adds `send_to_many()` and `broadcast_with_fanout()`. Also defines `PeerId` (32-byte wrapper with optional `libp2p::PeerId` conversion behind `live` feature), `NetworkEvent<M>` enum, and `MessageStream<M>` boxed stream type.
- `store.rs` - Thread-safe indexed message storage. `MessageStore<M>` with primary hash index, secondary sender index, and insertion-order BTreeMap for LRU eviction. Supports `insert()`, `get()`, `get_by_sender()`, `get_unprocessed()`, `mark_processed()`, `remove()`, `remove_older_than()`, and automatic capacity-based eviction. `MessageEntry<M>` and `StoreStats`. Includes unit tests.
- `mock.rs` - In-memory mock network for testing. `MockNetworkHub<M>` manages peer registrations, bidirectional connections, configurable packet loss/latency, and message logging. `MockNetwork<M>` implements `ProtocolNetwork`. `MockNetworkBuilder` provides fluent API for creating multi-peer topologies (`add_peers()`, `fully_connected()`, `ring_topology()`). `MockNetworkConfig` presets: `ideal()`, `with_latency()`, `with_packet_loss()`, `unreliable()`. Includes unit tests.

## Key APIs (no snippets)
- **Traits**: `ProtocolNetwork<M>` (core network abstraction), `ProtocolNetworkExt<M>` (fanout/multi-send)
- **Types**: `DeduplicationCache`, `GossipManager`, `GossipConfig`, `GossipStats`, `MessageStore<M>`, `MessageEntry<M>`, `StoreStats`, `MockNetwork<M>`, `MockNetworkBuilder<M>`, `MockNetworkConfig`, `PeerId`, `NetworkEvent<M>`, `GossipError`
- **Functions**: `GossipManager::hash_message()` (blake3), `DeduplicationCache::check_and_mark()` (atomic dedup)

## Relationships
- **Depends on**: `lru`, `parking_lot`, `hashbrown`, `blake3`, `async-trait`, `futures`, `serde`, `bincode`, `tokio` (mock only)
- **Used by**: `blueprint-agg-sig-gossip` (uses `DeduplicationCache` for re-gossip dedup), other gossip-based protocol extensions
