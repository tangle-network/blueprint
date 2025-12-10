//! Network abstraction for gossip protocols
//!
//! This module provides the [`ProtocolNetwork`] trait that abstracts network operations,
//! allowing gossip protocols to be tested without real network dependencies.

use crate::error::GossipError;
use alloc::{boxed::Box, vec::Vec};
use async_trait::async_trait;
use core::fmt::Debug;
use futures::Stream;
use serde::{Serialize, de::DeserializeOwned};

/// A peer identifier (abstracted from libp2p::PeerId)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PeerId(pub [u8; 32]);

impl PeerId {
    /// Create a new peer ID from bytes
    #[must_use]
    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// Get the bytes of this peer ID
    #[must_use]
    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

impl core::fmt::Display for PeerId {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Display first 8 bytes as hex
        write!(f, "Peer({}...)", hex::encode(&self.0[..4]))
    }
}

#[cfg(feature = "live")]
impl From<libp2p::PeerId> for PeerId {
    fn from(peer_id: libp2p::PeerId) -> Self {
        let bytes = peer_id.to_bytes();
        let mut arr = [0u8; 32];
        let len = bytes.len().min(32);
        arr[..len].copy_from_slice(&bytes[..len]);
        Self(arr)
    }
}

/// Event from the network
#[derive(Debug, Clone)]
pub enum NetworkEvent<M> {
    /// Received a message from a peer
    Message {
        /// The peer that sent the message
        from: PeerId,
        /// The message content
        message: M,
    },
    /// A peer connected
    PeerConnected(PeerId),
    /// A peer disconnected
    PeerDisconnected(PeerId),
}

/// A stream of network events
pub type MessageStream<M> = Box<dyn Stream<Item = NetworkEvent<M>> + Send + Unpin>;

/// Trait for network operations in gossip protocols
///
/// This trait abstracts the network layer, allowing protocols to be tested
/// with mock implementations that don't require real network connections.
///
/// # Type Parameters
///
/// * `M` - The message type that can be sent and received
#[async_trait]
pub trait ProtocolNetwork<M>: Send + Sync
where
    M: Serialize + DeserializeOwned + Send + Sync + Clone + Debug + 'static,
{
    /// Get the local peer ID
    fn local_peer_id(&self) -> PeerId;

    /// Get all connected peers
    fn connected_peers(&self) -> Vec<PeerId>;

    /// Send a message to a specific peer
    ///
    /// # Errors
    ///
    /// Returns an error if the send fails
    async fn send_to(&self, peer: PeerId, message: M) -> Result<(), GossipError>;

    /// Broadcast a message to all connected peers
    ///
    /// # Arguments
    ///
    /// * `message` - The message to broadcast
    /// * `exclude` - Optional peer to exclude from broadcast (typically the sender)
    ///
    /// # Errors
    ///
    /// Returns an error if the broadcast fails
    async fn broadcast(&self, message: M, exclude: Option<PeerId>) -> Result<(), GossipError>;

    /// Subscribe to incoming network events
    ///
    /// Returns a stream of network events that can be polled asynchronously.
    fn subscribe(&self) -> MessageStream<M>;

    /// Check if a peer is connected
    fn is_peer_connected(&self, peer: &PeerId) -> bool {
        self.connected_peers().contains(peer)
    }
}

/// Extension trait for common network operations
#[async_trait]
pub trait ProtocolNetworkExt<M>: ProtocolNetwork<M>
where
    M: Serialize + DeserializeOwned + Send + Sync + Clone + Debug + 'static,
{
    /// Send a message to multiple peers
    ///
    /// # Errors
    ///
    /// Returns the first error encountered, but attempts to send to all peers
    async fn send_to_many(&self, peers: &[PeerId], message: M) -> Result<(), GossipError> {
        let mut last_error = None;
        for peer in peers {
            if let Err(e) = self.send_to(*peer, message.clone()).await {
                last_error = Some(e);
            }
        }
        match last_error {
            Some(e) => Err(e),
            None => Ok(()),
        }
    }

    /// Broadcast with fanout limit (send to random subset of peers)
    ///
    /// # Errors
    ///
    /// Returns an error if the broadcast fails
    async fn broadcast_with_fanout(
        &self,
        message: M,
        fanout: usize,
        exclude: Option<PeerId>,
    ) -> Result<(), GossipError> {
        let mut peers = self.connected_peers();

        // Remove excluded peer
        if let Some(excluded) = exclude {
            peers.retain(|p| p != &excluded);
        }

        // If we have fewer peers than fanout, send to all
        if peers.len() <= fanout {
            return self.broadcast(message, exclude).await;
        }

        // Select random subset
        // Note: In production, use a proper RNG. This is deterministic for testing.
        let selected: Vec<_> = peers.into_iter().take(fanout).collect();
        self.send_to_many(&selected, message).await
    }
}

// Blanket implementation
impl<M, T> ProtocolNetworkExt<M> for T
where
    T: ProtocolNetwork<M>,
    M: Serialize + DeserializeOwned + Send + Sync + Clone + Debug + 'static,
{
}

// Hex encoding helper for Display
mod hex {
    pub fn encode(bytes: &[u8]) -> alloc::string::String {
        use alloc::string::String;
        let mut s = String::with_capacity(bytes.len() * 2);
        for byte in bytes {
            use core::fmt::Write;
            let _ = write!(s, "{byte:02x}");
        }
        s
    }
}
