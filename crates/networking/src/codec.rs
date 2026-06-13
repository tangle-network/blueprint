//! Canonical wire codec for [`ProtocolMessage`] envelopes.
//!
//! Every encode site and every decode site MUST go through this module.
//! With bincode 1.x, `bincode::serialize` (fixint encoding) and
//! `bincode::options().deserialize` (varint encoding) are **incompatible**
//! wire formats. Mixing them silently breaks the transport: senders produce
//! bytes that no receiver can decode. Centralizing the `Options` construction
//! here makes that divergence impossible.

use crate::types::ProtocolMessage;
use bincode::Options;
use serde::{Serialize, de::DeserializeOwned};

/// Maximum allowed size for a message payload
pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

/// The single source of truth for the wire format: varint encoding with a
/// size limit of [`MAX_MESSAGE_SIZE`], enforced on both encode and decode.
fn wire_options() -> impl Options {
    bincode::options().with_limit(MAX_MESSAGE_SIZE as u64)
}

/// Serialize a value with the canonical wire options.
///
/// # Errors
///
/// Fails if the encoded size would exceed [`MAX_MESSAGE_SIZE`] (so oversized
/// sends fail fast at the sender instead of being dropped by receivers) or if
/// serialization itself fails.
pub fn serialize<T: Serialize + ?Sized>(value: &T) -> Result<Vec<u8>, bincode::Error> {
    wire_options().serialize(value)
}

/// Deserialize a value with the canonical wire options.
///
/// # Errors
///
/// Fails if the input exceeds [`MAX_MESSAGE_SIZE`] or is not a valid encoding
/// of `T`.
pub fn deserialize<T: DeserializeOwned>(bytes: &[u8]) -> Result<T, bincode::Error> {
    wire_options().deserialize(bytes)
}

/// Encode a [`ProtocolMessage`] envelope for the wire (gossip and direct P2P).
///
/// # Errors
///
/// See [`serialize`]
pub fn encode_protocol_message(msg: &ProtocolMessage) -> Result<Vec<u8>, bincode::Error> {
    serialize(msg)
}

/// Decode a [`ProtocolMessage`] envelope received from the wire.
///
/// # Errors
///
/// See [`deserialize`]
pub fn decode_protocol_message(bytes: &[u8]) -> Result<ProtocolMessage, bincode::Error> {
    deserialize(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::MessageRouting;
    use libp2p::PeerId;

    fn message(recipient: Option<PeerId>) -> ProtocolMessage {
        ProtocolMessage {
            protocol: "/test-protocol/1.0.0".to_string(),
            routing: MessageRouting {
                message_id: 42,
                round_id: 7,
                sender: PeerId::random(),
                recipient,
            },
            payload: b"protocol payload bytes".to_vec(),
        }
    }

    fn assert_round_trip(msg: &ProtocolMessage) {
        let bytes = encode_protocol_message(msg).expect("encode failed");
        let decoded = decode_protocol_message(&bytes).expect("decode failed");
        assert_eq!(decoded.protocol, msg.protocol);
        assert_eq!(decoded.payload, msg.payload);
        assert_eq!(decoded.routing.message_id, msg.routing.message_id);
        assert_eq!(decoded.routing.round_id, msg.routing.round_id);
        assert_eq!(decoded.routing.sender, msg.routing.sender);
        assert_eq!(decoded.routing.recipient, msg.routing.recipient);
    }

    #[test]
    fn round_trip_broadcast_routing() {
        assert_round_trip(&message(None));
    }

    #[test]
    fn round_trip_direct_routing() {
        assert_round_trip(&message(Some(PeerId::random())));
    }

    /// Regression test for the fixint/varint codec mismatch: bytes produced by
    /// the send path must decode through the exact `Options` incantation the
    /// receive paths (gossip + request-response) use. Before this codec module
    /// existed, the sender used `bincode::serialize` (fixint) while receivers
    /// used `bincode::options().with_limit(..)` (varint), so every message
    /// sent through `NetworkServiceHandle::send` failed to decode.
    #[test]
    fn send_path_bytes_decode_via_receive_path_options() {
        let msg = message(Some(PeerId::random()));
        let bytes = encode_protocol_message(&msg).expect("encode failed");
        let decoded = bincode::options()
            .with_limit(MAX_MESSAGE_SIZE as u64)
            .deserialize::<ProtocolMessage>(&bytes)
            .expect("receive-path options must decode send-path bytes");
        assert_eq!(decoded.payload, msg.payload);
        assert_eq!(decoded.routing.message_id, msg.routing.message_id);
    }

    #[test]
    fn oversized_message_fails_on_encode() {
        let mut msg = message(None);
        msg.payload = vec![0u8; MAX_MESSAGE_SIZE + 1];
        assert!(
            encode_protocol_message(&msg).is_err(),
            "encoding must fail fast when the payload exceeds MAX_MESSAGE_SIZE"
        );
    }
}
