# blueprint_protocol

## Purpose
Implements the blueprint-specific P2P protocol layer, combining libp2p request/response messaging with gossipsub broadcast. Handles mutual handshake authentication (signature-based, supporting both instance public keys and EVM addresses), protocol message routing between verified peers, and gossip message filtering to only accept messages from handshake-verified peers.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Defines `InstanceMessageRequest`/`InstanceMessageResponse` enums (Handshake, Protocol, Success, Error variants), the `HandshakeMessage` struct with expiry and serialization, and re-exports `BlueprintProtocolBehaviour`/`BlueprintProtocolEvent`.
- `behaviour.rs` - Implements `BlueprintProtocolBehaviour` as a libp2p `NetworkBehaviour`. Wraps a derived behaviour combining request/response (CBOR codec) and gossipsub. Provides `send_request`, `send_response`, `subscribe`, `unsubscribe`, `publish`, `send_handshake`, `verify_handshake`, and `handle_handshake`. On connection establishment, auto-initiates handshakes with unverified peers. On connection close, cleans up peer state and gossipsub membership.
- `handler.rs` - Implements `handle_request_response_event` for processing inbound/outbound handshake and protocol messages. Enforces whitelist checks, handshake verification, and peer ban logic (3 failures triggers a 5-minute ban). Includes `check_expired_handshakes` (30s timeout) and `complete_handshake` which links verification keys and marks peers verified.

## Key APIs (no snippets)
- `BlueprintProtocolBehaviour::new` - Creates the behaviour with gossipsub (signed, strict validation, flood publish) and request/response (CBOR, 30s timeout, 50 concurrent streams).
- `BlueprintProtocolBehaviour::send_handshake` - Initiates a signed handshake with a peer.
- `BlueprintProtocolBehaviour::verify_handshake` - Verifies handshake signature and expiry.
- `BlueprintProtocolBehaviour::send_request` / `send_response` - Direct P2P messaging.
- `BlueprintProtocolBehaviour::subscribe` / `publish` - Gossipsub topic management and broadcasting.
- `BlueprintProtocolEvent` - Events emitted: `Request`, `Response`, `GossipMessage`.
- `InstanceMessageRequest` / `InstanceMessageResponse` - Protocol message enums for handshake and protocol data.

## Relationships
- Uses `discovery::PeerManager` for peer verification, whitelisting, banning, and key-to-peer-id mapping.
- Uses `discovery::peers::VerificationIdentifierKey` for EVM address or public key identity.
- Uses `discovery::utils` for EVM address derivation from compressed public keys.
- Uses `crate::types::ProtocolMessage` for deserializing and forwarding protocol payloads via `crossbeam_channel::Sender`.
- Depends on `blueprint_crypto::KeyType` for generic cryptographic operations (signing, verification).
