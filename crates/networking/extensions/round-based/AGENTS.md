# round-based

## Purpose
Adapter bridge integrating the `round-based` crate (multi-round MPC protocol framework) with `blueprint-networking`'s authenticated P2P layer. Enables multi-party computation protocols targeting the `round_based` API to execute over the Blueprint P2P network with peer authentication and routing.

## Contents (one hop)
### Subdirectories
- [x] `src/` - Single `lib.rs` implementing adapter types: `RoundBasedNetworkAdapter`, `RoundBasedSender`, `RoundBasedReceiver`, `NetworkError`
- [x] `tests/` - Integration test `rand_protocol.rs` with in-memory simulation, async simulation, and P2P network test variants

### Files
- `Cargo.toml` - Package manifest (v0.1.0-alpha.16). Depends on `blueprint-core`, `blueprint-crypto`, `blueprint-networking`, `round-based` (v0.4.1), `futures`, `serde_json`.
  - **Key items**: workspace resolver v3, edition 2024
- `README.md` - Quick overview: provides `RoundBasedNetworkAdapter` implementing `round_based::Delivery`.
  - **Key items**: usage pattern, GitHub source link
- `CHANGELOG.md` - 16 alpha releases (alpha.1 through alpha.16); mostly dependency updates.
  - **Key items**: release dates, QoS addition in alpha.11

## Key APIs (no snippets)
- **Types**: `RoundBasedNetworkAdapter<M, K>` (main wrapper, implements `Delivery<M>`), `RoundBasedSender<M, K>` (implements `Sink<Outgoing<M>>`), `RoundBasedReceiver<M, K>` (implements `Stream<Item = Incoming<M>>`), `NetworkError` (Serialization, Send variants)
- **Functions**: `RoundBasedNetworkAdapter::new(handle, party_index, parties_map, protocol_id)`, `split() -> (Receiver, Sender)`

## Relationships
- **Depends on**: `blueprint-networking` (NetworkServiceHandle, ProtocolMessage), `blueprint-crypto` (KeyType trait), `round-based` v0.4.1 (Delivery, Incoming, Outgoing), `futures` (Sink, Stream), `serde_json`
- **Used by**: `blueprint-sdk` (re-exported as `blueprint::networking::round_based_compat`, gated on `round-based-compat` feature); any round-based MPC protocol (threshold signing, DKG, randomness generation)
- **Siblings**: `agg-sig-gossip/`, `gossip-primitives/` (independent extensions)
- **Data/control flow**:
  - Construction requires upfront party-index-to-PeerId mapping
  - Sender serializes `Outgoing<M>` to JSON, routes via `NetworkServiceHandle::send()` with protocol ID namespacing
  - Receiver polls `next_protocol_message()`, deserializes, maps sender PeerId to PartyIndex
  - Messages from unknown peers silently dropped (trace log)

## Notes
- Generic over message type `M` and key type `K: KeyType`
- Message routing uses `"{protocol_id}/{round_number}"` for multiplexing
- `p2p_networking` test requires `--test-threads=1` due to mDNS port binding (marked `#[serial_test::serial]`)
- JSON serialization for transport; could add bincode/CBOR for performance
