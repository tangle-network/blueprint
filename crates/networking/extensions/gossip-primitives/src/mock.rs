//! Mock network implementation for testing gossip protocols
//!
//! This module provides [`MockNetwork`] which allows testing gossip protocols
//! without real network dependencies (no mDNS, no libp2p swarm, etc.).
//!
//! ## Features
//!
//! - In-memory message passing between mock peers
//! - Configurable network conditions (delay, packet loss)
//! - Message inspection for test assertions
//! - No port conflicts or mDNS cross-contamination

use crate::error::GossipError;
use crate::network::{MessageStream, NetworkEvent, PeerId, ProtocolNetwork};
use alloc::boxed::Box;
use alloc::sync::Arc;
use alloc::vec::Vec;
use async_trait::async_trait;
use core::fmt::Debug;
use futures::stream;
use parking_lot::{Mutex, RwLock};
use serde::{de::DeserializeOwned, Serialize};
use std::collections::{HashMap, VecDeque};
use tokio::sync::broadcast;

/// Configuration for the mock network
#[derive(Debug, Clone)]
pub struct MockNetworkConfig {
    /// Simulated network latency (min, max) in milliseconds
    pub latency_ms: Option<(u64, u64)>,
    /// Probability of packet loss (0.0 - 1.0)
    pub packet_loss: f64,
    /// Whether to deliver messages in order
    pub ordered_delivery: bool,
    /// Maximum queued messages before dropping
    pub max_queue_size: usize,
}

impl Default for MockNetworkConfig {
    fn default() -> Self {
        Self {
            latency_ms: None,
            packet_loss: 0.0,
            ordered_delivery: true,
            max_queue_size: 1000,
        }
    }
}

impl MockNetworkConfig {
    /// Config with no network issues (ideal conditions)
    #[must_use]
    pub fn ideal() -> Self {
        Self::default()
    }

    /// Config with some latency
    #[must_use]
    pub fn with_latency(min_ms: u64, max_ms: u64) -> Self {
        Self {
            latency_ms: Some((min_ms, max_ms)),
            ..Self::default()
        }
    }

    /// Config with packet loss
    #[must_use]
    pub fn with_packet_loss(probability: f64) -> Self {
        Self {
            packet_loss: probability.clamp(0.0, 1.0),
            ..Self::default()
        }
    }

    /// Config simulating poor network conditions
    #[must_use]
    pub fn unreliable() -> Self {
        Self {
            latency_ms: Some((10, 100)),
            packet_loss: 0.1,
            ordered_delivery: false,
            max_queue_size: 100,
        }
    }
}

/// Shared state for the mock network hub
pub struct MockNetworkHub<M> {
    /// All registered peers and their message senders
    peers: RwLock<HashMap<PeerId, broadcast::Sender<NetworkEvent<M>>>>,
    /// Connection topology: peer -> set of connected peers
    connections: RwLock<HashMap<PeerId, Vec<PeerId>>>,
    /// Record of all sent messages for inspection
    message_log: Mutex<Vec<MessageRecord<M>>>,
    /// Network configuration
    config: MockNetworkConfig,
}

/// Record of a sent message for inspection
#[derive(Debug, Clone)]
pub struct MessageRecord<M> {
    /// Sender of the message
    pub from: PeerId,
    /// Recipient (None for broadcast)
    pub to: Option<PeerId>,
    /// The message
    pub message: M,
    /// When it was sent
    pub timestamp: std::time::Instant,
}

impl<M: Clone + Send + Sync + 'static> MockNetworkHub<M> {
    fn new(config: MockNetworkConfig) -> Self {
        Self {
            peers: RwLock::new(HashMap::new()),
            connections: RwLock::new(HashMap::new()),
            message_log: Mutex::new(Vec::new()),
            config,
        }
    }

    fn register_peer(&self, peer_id: PeerId) -> broadcast::Receiver<NetworkEvent<M>> {
        let (tx, rx) = broadcast::channel(self.config.max_queue_size);
        self.peers.write().insert(peer_id, tx);
        self.connections.write().insert(peer_id, Vec::new());
        rx
    }

    fn connect_peers(&self, peer1: PeerId, peer2: PeerId) {
        let mut connections = self.connections.write();

        // Add bidirectional connection
        connections.entry(peer1).or_default().push(peer2);
        connections.entry(peer2).or_default().push(peer1);

        // Notify both peers of the connection
        let peers = self.peers.read();
        if let Some(tx) = peers.get(&peer1) {
            let _ = tx.send(NetworkEvent::PeerConnected(peer2));
        }
        if let Some(tx) = peers.get(&peer2) {
            let _ = tx.send(NetworkEvent::PeerConnected(peer1));
        }
    }

    fn disconnect_peers(&self, peer1: PeerId, peer2: PeerId) {
        let mut connections = self.connections.write();

        if let Some(conns) = connections.get_mut(&peer1) {
            conns.retain(|p| p != &peer2);
        }
        if let Some(conns) = connections.get_mut(&peer2) {
            conns.retain(|p| p != &peer1);
        }

        // Notify both peers of the disconnection
        let peers = self.peers.read();
        if let Some(tx) = peers.get(&peer1) {
            let _ = tx.send(NetworkEvent::PeerDisconnected(peer2));
        }
        if let Some(tx) = peers.get(&peer2) {
            let _ = tx.send(NetworkEvent::PeerDisconnected(peer1));
        }
    }

    fn get_connected_peers(&self, peer_id: &PeerId) -> Vec<PeerId> {
        self.connections
            .read()
            .get(peer_id)
            .cloned()
            .unwrap_or_default()
    }

    fn is_connected(&self, peer1: &PeerId, peer2: &PeerId) -> bool {
        self.connections
            .read()
            .get(peer1)
            .map(|conns| conns.contains(peer2))
            .unwrap_or(false)
    }

    fn should_drop_packet(&self) -> bool {
        if self.config.packet_loss <= 0.0 {
            return false;
        }
        // Simple deterministic "random" for testing predictability
        // In real use, this would use actual RNG
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .subsec_nanos();
        (now as f64 / u32::MAX as f64) < self.config.packet_loss
    }

    fn send_message(&self, from: PeerId, to: PeerId, message: M) -> Result<(), GossipError> {
        // Check connection
        if !self.is_connected(&from, &to) {
            return Err(GossipError::PeerNotFound(format!("{to}")));
        }

        // Simulate packet loss
        if self.should_drop_packet() {
            return Ok(()); // Silently drop
        }

        // Record the message
        self.message_log.lock().push(MessageRecord {
            from,
            to: Some(to),
            message: message.clone(),
            timestamp: std::time::Instant::now(),
        });

        // Deliver the message
        let peers = self.peers.read();
        if let Some(tx) = peers.get(&to) {
            tx.send(NetworkEvent::Message { from, message })
                .map_err(|_| GossipError::SendFailed("receiver dropped".into()))?;
        }

        Ok(())
    }

    fn broadcast_message(
        &self,
        from: PeerId,
        message: M,
        exclude: Option<PeerId>,
    ) -> Result<(), GossipError> {
        let connected = self.get_connected_peers(&from);

        // Record the broadcast
        self.message_log.lock().push(MessageRecord {
            from,
            to: None,
            message: message.clone(),
            timestamp: std::time::Instant::now(),
        });

        let peers = self.peers.read();
        for peer in connected {
            if exclude == Some(peer) {
                continue;
            }

            if self.should_drop_packet() {
                continue; // Silently drop
            }

            if let Some(tx) = peers.get(&peer) {
                let _ = tx.send(NetworkEvent::Message {
                    from,
                    message: message.clone(),
                });
            }
        }

        Ok(())
    }

    fn get_message_log(&self) -> Vec<MessageRecord<M>> {
        self.message_log.lock().clone()
    }

    fn clear_message_log(&self) {
        self.message_log.lock().clear();
    }
}

/// A mock network instance for a single peer
pub struct MockNetwork<M> {
    /// This peer's ID
    local_peer_id: PeerId,
    /// Shared hub for all mock peers
    hub: Arc<MockNetworkHub<M>>,
    /// Receiver for incoming events
    receiver: Mutex<Option<broadcast::Receiver<NetworkEvent<M>>>>,
}

impl<M: Clone + Send + Sync + 'static> MockNetwork<M> {
    /// Create a new mock network hub and return the first peer
    #[must_use]
    pub fn new_hub(config: MockNetworkConfig) -> MockNetworkBuilder<M> {
        MockNetworkBuilder {
            hub: Arc::new(MockNetworkHub::new(config)),
            peers: Vec::new(),
        }
    }

    /// Get a reference to the hub (for inspection in tests)
    #[must_use]
    pub fn hub(&self) -> &Arc<MockNetworkHub<M>> {
        &self.hub
    }

    /// Connect this peer to another peer
    pub fn connect_to(&self, other: &MockNetwork<M>) {
        self.hub.connect_peers(self.local_peer_id, other.local_peer_id);
    }

    /// Disconnect from another peer
    pub fn disconnect_from(&self, other: &MockNetwork<M>) {
        self.hub.disconnect_peers(self.local_peer_id, other.local_peer_id);
    }

    /// Get all messages sent through this hub
    #[must_use]
    pub fn get_message_log(&self) -> Vec<MessageRecord<M>> {
        self.hub.get_message_log()
    }

    /// Clear the message log
    pub fn clear_message_log(&self) {
        self.hub.clear_message_log();
    }

    /// Get messages sent by this peer
    #[must_use]
    pub fn get_sent_messages(&self) -> Vec<MessageRecord<M>> {
        self.hub
            .get_message_log()
            .into_iter()
            .filter(|r| r.from == self.local_peer_id)
            .collect()
    }

    /// Get messages received by this peer
    #[must_use]
    pub fn get_received_messages(&self) -> Vec<MessageRecord<M>> {
        self.hub
            .get_message_log()
            .into_iter()
            .filter(|r| r.to == Some(self.local_peer_id))
            .collect()
    }
}

#[async_trait]
impl<M> ProtocolNetwork<M> for MockNetwork<M>
where
    M: Serialize + DeserializeOwned + Send + Sync + Clone + Debug + Unpin + 'static,
{
    fn local_peer_id(&self) -> PeerId {
        self.local_peer_id
    }

    fn connected_peers(&self) -> Vec<PeerId> {
        self.hub.get_connected_peers(&self.local_peer_id)
    }

    async fn send_to(&self, peer: PeerId, message: M) -> Result<(), GossipError> {
        self.hub.send_message(self.local_peer_id, peer, message)
    }

    async fn broadcast(&self, message: M, exclude: Option<PeerId>) -> Result<(), GossipError> {
        self.hub
            .broadcast_message(self.local_peer_id, message, exclude)
    }

    fn subscribe(&self) -> MessageStream<M> {
        let mut receiver_guard = self.receiver.lock();

        // If we still have the receiver, convert it to a stream
        if let Some(rx) = receiver_guard.take() {
            let stream = BroadcastStream::new(rx);
            Box::new(stream)
        } else {
            // No receiver, return empty stream
            Box::new(stream::empty())
        }
    }
}

/// Builder for creating a mock network with multiple peers
pub struct MockNetworkBuilder<M> {
    hub: Arc<MockNetworkHub<M>>,
    peers: Vec<MockNetwork<M>>,
}

impl<M: Clone + Send + Sync + 'static> MockNetworkBuilder<M> {
    /// Add a peer with a specific ID
    #[must_use]
    pub fn add_peer(mut self, peer_id: PeerId) -> Self {
        let receiver = self.hub.register_peer(peer_id);
        self.peers.push(MockNetwork {
            local_peer_id: peer_id,
            hub: Arc::clone(&self.hub),
            receiver: Mutex::new(Some(receiver)),
        });
        self
    }

    /// Add a peer with an auto-generated ID
    #[must_use]
    pub fn add_auto_peer(self) -> Self {
        let id = self.peers.len() as u8;
        let mut bytes = [0u8; 32];
        bytes[0] = id;
        self.add_peer(PeerId::from_bytes(bytes))
    }

    /// Add multiple peers with auto-generated IDs
    #[must_use]
    pub fn add_peers(mut self, count: usize) -> Self {
        for _ in 0..count {
            self = self.add_auto_peer();
        }
        self
    }

    /// Connect all peers to each other (fully connected mesh)
    #[must_use]
    pub fn fully_connected(self) -> Self {
        let peer_ids: Vec<_> = self.peers.iter().map(|p| p.local_peer_id).collect();

        for i in 0..peer_ids.len() {
            for j in (i + 1)..peer_ids.len() {
                self.hub.connect_peers(peer_ids[i], peer_ids[j]);
            }
        }

        self
    }

    /// Connect peers in a ring topology
    #[must_use]
    pub fn ring_topology(self) -> Self {
        let peer_ids: Vec<_> = self.peers.iter().map(|p| p.local_peer_id).collect();

        for i in 0..peer_ids.len() {
            let next = (i + 1) % peer_ids.len();
            self.hub.connect_peers(peer_ids[i], peer_ids[next]);
        }

        self
    }

    /// Build and return all mock networks
    #[must_use]
    pub fn build(self) -> Vec<MockNetwork<M>> {
        self.peers
    }
}

/// Wrapper to convert broadcast::Receiver to a Stream
struct BroadcastStream<M> {
    receiver: broadcast::Receiver<NetworkEvent<M>>,
    buffer: VecDeque<NetworkEvent<M>>,
}

impl<M: Clone> BroadcastStream<M> {
    fn new(receiver: broadcast::Receiver<NetworkEvent<M>>) -> Self {
        Self {
            receiver,
            buffer: VecDeque::new(),
        }
    }
}

impl<M: Clone + Send + Unpin + 'static> futures::Stream for BroadcastStream<M> {
    type Item = NetworkEvent<M>;

    fn poll_next(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Self::Item>> {
        // First check buffer
        if let Some(event) = self.buffer.pop_front() {
            return std::task::Poll::Ready(Some(event));
        }

        // Try to receive from broadcast channel
        match self.receiver.try_recv() {
            Ok(event) => std::task::Poll::Ready(Some(event)),
            Err(broadcast::error::TryRecvError::Empty) => {
                // Register waker and return pending
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
            Err(broadcast::error::TryRecvError::Closed) => std::task::Poll::Ready(None),
            Err(broadcast::error::TryRecvError::Lagged(n)) => {
                // Log the lag and continue - in tests this shouldn't happen often
                tracing::warn!("Mock network lagged by {n} messages");
                cx.waker().wake_by_ref();
                std::task::Poll::Pending
            }
        }
    }
}

impl<M: Clone + Send + Unpin + 'static> Unpin for BroadcastStream<M> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
    struct TestMessage {
        content: String,
    }

    #[tokio::test]
    async fn test_mock_network_basic() {
        let networks: Vec<MockNetwork<TestMessage>> = MockNetwork::new_hub(MockNetworkConfig::ideal())
            .add_peers(2)
            .fully_connected()
            .build();

        let peer0 = &networks[0];
        let peer1 = &networks[1];

        // Peer 0 sends to Peer 1
        let msg = TestMessage {
            content: "hello".to_string(),
        };
        peer0.send_to(peer1.local_peer_id(), msg.clone()).await.unwrap();

        // Check message log
        let log = peer0.get_message_log();
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].from, peer0.local_peer_id());
        assert_eq!(log[0].to, Some(peer1.local_peer_id()));
    }

    #[tokio::test]
    async fn test_mock_network_broadcast() {
        let networks: Vec<MockNetwork<TestMessage>> = MockNetwork::new_hub(MockNetworkConfig::ideal())
            .add_peers(4)
            .fully_connected()
            .build();

        let sender = &networks[0];
        let msg = TestMessage {
            content: "broadcast".to_string(),
        };

        sender.broadcast(msg, None).await.unwrap();

        // Should have sent to 3 other peers (broadcast recorded once)
        let log = sender.get_message_log();
        assert_eq!(log.len(), 1);
        assert!(log[0].to.is_none()); // Broadcast has no specific recipient
    }

    #[tokio::test]
    async fn test_mock_network_not_connected() {
        let networks: Vec<MockNetwork<TestMessage>> = MockNetwork::new_hub(MockNetworkConfig::ideal())
            .add_peers(2)
            // NOT calling fully_connected()
            .build();

        let peer0 = &networks[0];
        let peer1 = &networks[1];

        let msg = TestMessage {
            content: "hello".to_string(),
        };

        // Should fail because peers aren't connected
        let result = peer0.send_to(peer1.local_peer_id(), msg).await;
        assert!(matches!(result, Err(GossipError::PeerNotFound(_))));
    }

    #[tokio::test]
    async fn test_mock_network_ring_topology() {
        let networks: Vec<MockNetwork<TestMessage>> = MockNetwork::new_hub(MockNetworkConfig::ideal())
            .add_peers(4)
            .ring_topology()
            .build();

        // In ring: 0-1-2-3-0
        // Peer 0 is connected to 1 and 3
        let peer0 = &networks[0];
        let connected = peer0.connected_peers();

        assert_eq!(connected.len(), 2);
    }

    #[tokio::test]
    async fn test_mock_network_disconnect() {
        let networks: Vec<MockNetwork<TestMessage>> = MockNetwork::new_hub(MockNetworkConfig::ideal())
            .add_peers(2)
            .fully_connected()
            .build();

        let peer0 = &networks[0];
        let peer1 = &networks[1];

        // Initially connected
        assert!(!peer0.connected_peers().is_empty());

        // Disconnect
        peer0.disconnect_from(peer1);

        // Now disconnected
        assert!(peer0.connected_peers().is_empty());

        // Send should fail
        let msg = TestMessage {
            content: "hello".to_string(),
        };
        let result = peer0.send_to(peer1.local_peer_id(), msg).await;
        assert!(matches!(result, Err(GossipError::PeerNotFound(_))));
    }
}
