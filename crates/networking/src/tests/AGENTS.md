# tests

## Purpose
Integration tests for the networking crate covering peer discovery, handshake authentication, gossip messaging, and multi-round protocol execution. All tests use `K256Ecdsa` keys and are marked `#[serial_test::serial]` to avoid port/resource conflicts.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module root that declares the four test submodules.
- `blueprint_protocol.rs` - Tests multi-round summation protocol: two-node and three-node variants. Round 1 broadcasts numbers via gossip, Round 2 verifies sums via targeted P2P messages. Validates end-to-end protocol message serialization, routing, and delivery.
- `discovery.rs` - Tests mDNS peer discovery (`test_peer_discovery_mdns`), Kademlia bootstrap-based discovery with three nodes (`test_peer_discovery_kademlia`), and peer info/identify updates (`test_peer_info_updates`).
- `gossip.rs` - Tests gossip between verified peers (two-node and three-node), and verifies that unverified peers cannot receive gossip messages.
- `handshake.rs` - Tests automatic handshake on connection with whitelisted keys, and verifies that non-whitelisted peers get banned after failed handshake attempts.

## Key APIs (no snippets)
- Test functions: `test_summation_protocol_basic`, `test_summation_protocol_multi_node`, `test_peer_discovery_mdns`, `test_peer_discovery_kademlia`, `test_peer_info_updates`, `test_gossip_between_verified_peers`, `test_multi_node_gossip`, `test_unverified_peer_gossip`, `test_automatic_handshake`, `test_handshake_with_invalid_peer`.

## Relationships
- Uses `crate::test_utils` for node creation, startup, and wait helpers.
- Uses `crate::service::AllowedKeys` and `crate::types::{MessageRouting, ProtocolMessage}`.
- All tests use `blueprint_crypto::k256::K256Ecdsa` as the key type.
