# discovery

## Purpose
Implements the peer discovery subsystem for the P2P network, combining Kademlia DHT, mDNS, identify, autonat, UPnP, and circuit relay protocols into a unified `DiscoveryBehaviour`. Manages peer lifecycle (connection, info tracking, disconnection), periodic random Kademlia queries for peer discovery, and peer identity management with whitelist-based verification.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Module root that re-exports `PeerEvent`, `PeerInfo`, `PeerManager` from `peers`, and provides `new_kademlia` which creates a pre-configured Kademlia behaviour with optimized settings (60s query timeout, replication factor 3, 24h TTL, server mode).
- `behaviour.rs` - Defines `DerivedDiscoveryBehaviour` (Kademlia, mDNS, identify, autonat, UPnP, relay - all togglable) and `DiscoveryBehaviour<K>` which wraps it. Implements `NetworkBehaviour` with peer connection/disconnection tracking, exponential-backoff random Kademlia queries (1s to 60s), identify info processing, mDNS discovered peer injection into Kademlia, and relay/UPnP event handling. Emits `DiscoveryEvent` variants (PeerConnected, PeerDisconnected, Discovery).
- `config.rs` - Builder-pattern `DiscoveryConfig<K>` for constructing `DiscoveryBehaviour`. Configurable: bootstrap peers, relay nodes, target peer count (default 25), enable/disable mDNS/Kademlia/UPnP/relay, network name, protocol version. The `build` method assembles all sub-behaviours and bootstraps Kademlia.
- `peers.rs` - Core peer management. `VerificationIdentifierKey<K>` enum (EvmAddress or InstancePublicKey) with signature verification (secp256k1 ECDSA recovery for EVM, generic KeyType for instance keys). `PeerInfo` tracks addresses, identify info, latency, success/failure counts, response times. `PeerManager<K>` provides concurrent peer state (DashMap/DashSet), verified peer tracking, whitelisted key management with dynamic updates via channel, ban management with expiry (background cleanup task), peer event broadcasting, and whitelist-indexed key/peer lookups.
- `utils.rs` - Cryptographic utilities: `secp256k1_ecdsa_recover` for public key recovery from ECDSA signatures, `get_address_from_pubkey` and `get_address_from_compressed_pubkey` for Ethereum address derivation using keccak256.

## Key APIs (no snippets)
- `DiscoveryConfig::new` / `DiscoveryConfig::build` - Builder for constructing the discovery behaviour.
- `DiscoveryBehaviour::bootstrap` - Triggers Kademlia bootstrap.
- `DiscoveryBehaviour::get_peers` / `get_peer_info` / `nat_status` - Peer and NAT status queries.
- `PeerManager::new` - Creates manager with initial allowed keys (EVM addresses or instance public keys).
- `PeerManager::verify_peer` / `is_peer_verified` - Handshake verification state.
- `PeerManager::ban_peer` / `unban_peer` / `is_banned` - Ban management with optional duration.
- `PeerManager::link_peer_id_to_verification_id_key` - Associates peer IDs with cryptographic identities.
- `PeerManager::run_allowed_keys_updater` - Blocking loop that receives whitelist updates via channel.
- `VerificationIdentifierKey::verify` - Verifies signatures against EVM addresses or instance public keys.

## Relationships
- Used by `blueprint_protocol::BlueprintProtocolBehaviour` for peer verification and whitelist checks.
- Uses `crate::service::AllowedKeys` enum for initializing whitelisted keys.
- Depends on `blueprint_crypto::KeyType` for generic key operations and `libsecp256k1` for ECDSA recovery.
- `PeerManager` is shared (via `Arc`) between discovery and blueprint protocol layers.
