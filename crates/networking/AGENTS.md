# networking

## Purpose
P2P networking layer (`blueprint-networking`). Built on libp2p, provides authenticated peer-to-peer communication for Blueprint operators: identity-verified handshakes, direct request-response messaging, gossipsub broadcast, peer discovery (mDNS, Kademlia, relay, AutoNAT), and protocol extensions for MPC and threshold cryptography workflows.

## Contents (one hop)
### Subdirectories
- [x] `extensions/` - Networking protocol extension sub-crates: `round-based/` (round-based MPC protocol adapter over libp2p), `agg-sig-gossip/` (aggregated signature gossip protocol), `gossip-primitives/` (shared gossip protocol types)
- [x] `src/` - Core networking source: `blueprint_protocol/` (handshake, verified messaging), `discovery/` (peer discovery strategies), `behaviours.rs` (libp2p behaviour composition), `service.rs` / `service_handle.rs` (network service lifecycle), `types.rs` (message/peer types), `error.rs`, `test_utils/` (testing helpers, gated on `testing`), `tests/`

### Files
- `CHANGELOG.md` - Release history
- `Cargo.toml` - Crate manifest (`blueprint-networking`); depends on `blueprint-core`, `blueprint-std`, `blueprint-crypto` (k256, hashing), `libp2p` (gossipsub, mDNS, noise, yamux, TCP, QUIC, request-response, Kademlia, relay, AutoNAT, UPnP), `dashmap`, `bincode`, `crossbeam-channel`; features: `std`, `testing`
- `README.md` - Detailed protocol documentation: handshake flow, direct P2P messaging, broadcast gossip, security features, error handling, timeouts

## Key APIs (no snippets)
- `NetworkService` -- main service that runs the libp2p swarm event loop
- `NetworkConfig` -- configuration for network identity, listen addresses, bootstrap peers
- `NetworkEvent` -- events emitted by the network service (messages received, peer connected/disconnected)
- `AllowedKeys` -- access control for which peer keys are permitted
- `KeyType` (re-exported from `blueprint_crypto`) -- key type used for peer identity
- `blueprint_protocol` module -- handshake verification, authenticated message exchange
- `discovery` module -- peer discovery via mDNS, Kademlia, relay

## Relationships
- Depends on `blueprint-core`, `blueprint-std`, `blueprint-crypto`
- Used by `blueprint-runner` (networking feature) for operator P2P communication
- Extension crates used for MPC protocols: `round-based` adapter, `agg-sig-gossip`, `gossip-primitives`
- Used by `blueprint-pricing-engine` for P2P price quote distribution
- Requires serial test execution due to port/resource conflicts
