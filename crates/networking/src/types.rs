use libp2p::{PeerId, gossipsub::IdentTopic};
use serde::{Deserialize, Serialize};
use std::fmt::Display;

/// Maximum allowed size for a message payload
pub const MAX_MESSAGE_SIZE: usize = 16 * 1024 * 1024;

/// Type of message delivery mechanism
#[derive(Debug, Clone)]
pub enum MessageDelivery {
    /// Broadcast to all peers via gossipsub
    Broadcast {
        /// The topic to broadcast on
        topic: IdentTopic,
    },
    /// Direct P2P message to a specific peer
    Direct {
        /// The target peer ID
        peer_id: PeerId,
    },
}

/// Message routing information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MessageRouting {
    /// Unique identifier for this message
    pub message_id: u64,
    /// The round/sequence number this message belongs to
    pub round_id: u16,
    /// The sender's information
    pub sender: PeerId,
    /// Optional recipient information for direct messages
    pub recipient: Option<PeerId>,
}

/// A protocol message that can be sent over the network
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProtocolMessage {
    /// The protocol name
    pub protocol: String,
    /// Routing information for the message
    pub routing: MessageRouting,
    /// The actual message payload
    pub payload: Vec<u8>,
}

impl Display for MessageRouting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "msg={} round={} from={} to={:?}",
            self.message_id, self.round_id, self.sender, self.recipient
        )
    }
}

impl Display for ProtocolMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}] {} payload_size={}",
            self.protocol,
            self.routing,
            self.payload.len()
        )
    }
}
