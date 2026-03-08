# src

## Purpose
Adapter bridging `blueprint-networking`'s `NetworkServiceHandle` to the `round-based` crate's `Delivery` trait, enabling round-based MPC (multi-party computation) protocols to run over the Blueprint P2P network.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `lib.rs` - Single-file crate containing the full adapter implementation:
  - `RoundBasedNetworkAdapter<M, K>` wraps a `NetworkServiceHandle<K>` with party-index-to-PeerId mappings and implements `round_based::Delivery<M>`. `split()` produces a sender/receiver pair.
  - `RoundBasedSender<M, K>` implements `futures::Sink<Outgoing<M>>`, serializing messages to JSON and routing via `NetworkServiceHandle::send()`. Supports broadcast (recipient=None) and point-to-point delivery.
  - `RoundBasedReceiver<M, K>` implements `futures::Stream<Item=Result<Incoming<M>>>`, polling `NetworkServiceHandle::next_protocol_message()`, deserializing JSON payloads, and mapping sender PeerId to PartyIndex. Ignores messages from unknown senders.
  - `NetworkError` enum for serialization and send failures.

## Key APIs (no snippets)
- **Types**: `RoundBasedNetworkAdapter<M, K>`, `RoundBasedSender<M, K>`, `RoundBasedReceiver<M, K>`, `NetworkError`
- **Constructor**: `RoundBasedNetworkAdapter::new(handle, party_index, parties_map, protocol_id)`
- **Trait impls**: `Delivery<M>` (from `round-based`), `Sink<Outgoing<M>>` (sender), `Stream<Item=Incoming<M>>` (receiver)

## Relationships
- **Depends on**: `blueprint-networking` (`NetworkServiceHandle`, `ProtocolMessage`, `MessageRouting`), `blueprint-crypto` (`KeyType`), `round-based` crate (`Delivery`, `Incoming`, `Outgoing`, `PartyIndex`, `ProtocolMessage`), `serde_json` (message serialization), `libp2p` (`PeerId`)
- **Used by**: Any round-based MPC protocol (e.g., distributed key generation, threshold signing) running on the Blueprint network; tested via `tests/rand_protocol.rs`
