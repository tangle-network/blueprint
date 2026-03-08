# src

## Purpose
Core implementation of the `blueprint-networking` crate, providing a libp2p-based P2P networking stack with blueprint-specific protocol extensions. Combines Kademlia DHT discovery, mDNS, gossipsub broadcast, and request/response messaging with mutual handshake authentication and peer management.

## Contents (one hop)
### Subdirectories
- [x] `blueprint_protocol/` - Blueprint-specific P2P protocol layer combining request/response (CBOR codec) with gossipsub; handles mutual handshake authentication (signature-based), protocol message routing, peer banning (3 failures = 5-minute ban), and gossip filtering to verified peers only
- [x] `discovery/` - Peer discovery and management: `DiscoveryBehaviour` wrapping Kademlia and mDNS, `PeerManager` for verification/whitelisting/banning, `PeerInfo` tracking, and EVM address derivation utilities
- [x] `test_utils/` - Test utilities for networking (feature-gated behind `testing`)
- [x] `tests/` - Integration tests covering blueprint protocol messaging, discovery, gossip, and handshake flows

### Files
- `lib.rs` - Crate root; declares modules, re-exports `KeyType`, `NetworkConfig`, `NetworkEvent`, `NetworkService`, `AllowedKeys`
- `behaviours.rs` - `BlueprintBehaviour` composite `NetworkBehaviour` combining connection limits, discovery, blueprint protocol, and ping; `BlueprintBehaviourConfig` for initialization
- `service.rs` - `NetworkService` that runs the libp2p swarm event loop; `NetworkConfig` for startup; `AllowedKeys` enum (EVM addresses or instance public keys); `NetworkEvent` enum for inbound/outbound messages and gossip
- `service_handle.rs` - `NetworkServiceHandle` and `NetworkSender` for sending commands to the running network service from outside the event loop
- `error.rs` - Networking error types
- `types.rs` - Shared types including `ProtocolMessage` and `MessageRouting`

## Key APIs (no snippets)
- `NetworkService` - Main service managing the libp2p swarm lifecycle
- `NetworkConfig` - Configuration for network name, keys, peer targets, discovery options
- `NetworkEvent` - Events emitted: inbound/outbound requests/responses, gossip messages
- `NetworkServiceHandle` / `NetworkSender` - Handles for external interaction with the running service
- `BlueprintBehaviour` - Composite libp2p behaviour with discovery, protocol, ping, and connection limits
- `AllowedKeys` - Enum for whitelisting peers by EVM address or instance public key

## Relationships
- Depends on `blueprint-crypto` for `KeyType` generic cryptographic operations
- Used by `blueprint-runner` and blueprint services for P2P communication
- Extensions in `crates/networking/extensions/` build on top of this core layer
