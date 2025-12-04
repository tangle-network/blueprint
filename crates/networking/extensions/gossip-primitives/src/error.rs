//! Error types for gossip primitives

use alloc::string::{String, ToString};
use core::fmt;

/// Errors that can occur in gossip operations
#[derive(Debug)]
pub enum GossipError {
    /// Message serialization failed
    Serialization(String),
    /// Message deserialization failed
    Deserialization(String),
    /// Network send failed
    SendFailed(String),
    /// Network receive failed
    ReceiveFailed(String),
    /// Channel closed unexpectedly
    ChannelClosed,
    /// Operation timed out
    Timeout,
    /// Message was a duplicate
    Duplicate,
    /// Message validation failed
    ValidationFailed(String),
    /// Peer not found
    PeerNotFound(String),
    /// Internal error
    Internal(String),
}

impl fmt::Display for GossipError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Serialization(msg) => write!(f, "serialization error: {msg}"),
            Self::Deserialization(msg) => write!(f, "deserialization error: {msg}"),
            Self::SendFailed(msg) => write!(f, "send failed: {msg}"),
            Self::ReceiveFailed(msg) => write!(f, "receive failed: {msg}"),
            Self::ChannelClosed => write!(f, "channel closed"),
            Self::Timeout => write!(f, "operation timed out"),
            Self::Duplicate => write!(f, "duplicate message"),
            Self::ValidationFailed(msg) => write!(f, "validation failed: {msg}"),
            Self::PeerNotFound(id) => write!(f, "peer not found: {id}"),
            Self::Internal(msg) => write!(f, "internal error: {msg}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for GossipError {}

impl From<bincode::Error> for GossipError {
    fn from(e: bincode::Error) -> Self {
        Self::Serialization(e.to_string())
    }
}
