# blueprint-gossip-primitives

Reusable gossip protocol primitives for Blueprint SDK networking.

## Overview

This crate provides low-level building blocks for implementing gossip-based protocols:

- **DeduplicationCache**: LRU cache with TTL for message deduplication
- **MessageStore**: Indexed message storage with efficient lookups
- **ProtocolNetwork**: Trait for abstracting network operations
- **MockNetwork**: Testing utilities for protocols without real networking

## Features

| Feature | Description |
|---------|-------------|
| `std` (default) | Standard library support |
| `live` | Enable libp2p-based network implementation |
| `testing` | Enable test utilities |

## Core Components

### DeduplicationCache

An LRU cache with time-based expiration for preventing message re-processing.

```rust
use blueprint_gossip_primitives::DeduplicationCache;
use std::time::Duration;

// Create cache: 1000 entries max, 5 minute TTL
let cache = DeduplicationCache::new(1000, Duration::from_secs(300));

// Check and mark atomically (returns true if new)
let message_hash = blake3_256(&message);
if cache.check_and_mark(message_hash) {
    // First time seeing this message - process it
    process_message(&message);
} else {
    // Already seen - skip
}
```

### GossipManager

Combines deduplication with reliable message delivery.

```rust
use blueprint_gossip_primitives::{GossipManager, GossipConfig};

let config = GossipConfig {
    max_message_size: 65536,
    cache_capacity: 10000,
    cache_ttl: Duration::from_secs(300),
    fanout: 6,
};

let manager = GossipManager::new(config);

// Process incoming message (auto-deduplicates)
if let Some(msg) = manager.receive(raw_bytes)? {
    handle_new_message(msg);
}
```

### MessageStore

Thread-safe message storage with multiple indexes for efficient retrieval.

```rust
use blueprint_gossip_primitives::{MessageStore, MessageEntry, StoreConfig};

let config = StoreConfig {
    max_messages: 10_000,
    index_by_sender: true,
};

let store: MessageStore<MyMessage> = MessageStore::new(config);

// Insert a message
let entry = MessageEntry::new(message, hash, Some(sender_peer_id));
store.insert(entry);

// Query by hash
if let Some(entry) = store.get(&hash) {
    println!("Found: {:?}", entry.message);
}

// Query by sender
let from_peer = store.get_by_sender(&peer_id);

// Get unprocessed messages
let pending = store.get_unprocessed();

// Mark as processed
store.mark_processed(&hash);

// Cleanup old messages
store.remove_older_than(Duration::from_secs(3600));
```

### ProtocolNetwork Trait

Abstraction for network operations, enabling easy testing.

```rust
use blueprint_gossip_primitives::{ProtocolNetwork, NetworkEvent};
use async_trait::async_trait;

#[async_trait]
pub trait ProtocolNetwork: Send + Sync {
    type Message: Send;
    type PeerId: Send + Clone;

    /// Broadcast message to all peers
    async fn broadcast(&self, message: Self::Message) -> Result<(), GossipError>;

    /// Send message to specific peer
    async fn send_to(&self, peer: &Self::PeerId, message: Self::Message)
        -> Result<(), GossipError>;

    /// Receive next network event
    async fn next_event(&self) -> Option<NetworkEvent<Self::Message, Self::PeerId>>;

    /// Get local peer ID
    fn local_peer_id(&self) -> Self::PeerId;

    /// Get connected peers
    fn connected_peers(&self) -> Vec<Self::PeerId>;
}
```

### MockNetwork

Testing utility for protocol development without real networking.

```rust
use blueprint_gossip_primitives::{MockNetwork, MockNetworkConfig};

#[tokio::test]
async fn test_my_protocol() {
    let config = MockNetworkConfig {
        latency: Duration::from_millis(10),
        packet_loss: 0.0,
        message_buffer_size: 100,
    };

    let mock = MockNetwork::new(config);

    // Simulate network operations
    mock.broadcast(my_message).await?;

    // Check received messages
    while let Some(event) = mock.next_event().await {
        match event {
            NetworkEvent::Message { from, message } => {
                // Verify protocol behavior
            }
            _ => {}
        }
    }
}
```

## Message Hashing

Uses BLAKE3 for fast, secure message hashing:

```rust
use blueprint_gossip_primitives::MessageHash;

// MessageHash is [u8; 32]
let hash: MessageHash = blake3_256(&message_bytes);
```

## Thread Safety

All components are designed for concurrent access:

- `DeduplicationCache`: Uses `parking_lot::RwLock`
- `MessageStore`: Uses `parking_lot::RwLock` for all indexes
- `GossipManager`: Thread-safe wrapper around cache

## Usage with agg-sig-gossip

This crate provides the foundation for the signature aggregation protocol:

```rust
use blueprint_gossip_primitives::DeduplicationCache;
use blueprint_networking_agg_sig_gossip_extension::SignatureAggregationProtocol;

// The protocol uses DeduplicationCache internally
let protocol = SignatureAggregationProtocol::new(config, weight_scheme, keys);
// DeduplicationCache is created automatically with appropriate settings
```

## Performance Considerations

- **DeduplicationCache**: O(1) lookups, bounded memory via LRU eviction
- **MessageStore**: O(1) hash lookups, O(n) sender queries (but indexed)
- **Automatic Cleanup**: Expired entries removed on access or via explicit cleanup

## License

Licensed under the Apache License, Version 2.0.
