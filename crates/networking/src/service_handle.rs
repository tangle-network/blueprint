use crate::types::MessageRouting;
use crate::{
    blueprint_protocol::InstanceMessageRequest,
    discovery::{PeerInfo, PeerManager},
    service::NetworkMessage,
    types::ProtocolMessage,
};
use blueprint_crypto::KeyType;
use crossbeam_channel::{self, Receiver, Sender};
use libp2p::{Multiaddr, PeerId};
use std::sync::Arc;
use tokio::task::JoinHandle;
use tracing::debug;

/// Handle for sending outgoing messages to the network
#[derive(Clone)]
pub struct NetworkSender<K: KeyType> {
    network_message_sender: Sender<NetworkMessage<K>>,
}

impl<K: KeyType> NetworkSender<K> {
    #[must_use]
    pub fn new(network_message_sender: Sender<NetworkMessage<K>>) -> Self {
        Self {
            network_message_sender,
        }
    }

    /// Send a protocol message over the network
    ///
    /// # Errors
    ///
    /// See [`crossbeam_channel::Sender::send`]
    pub fn send_message(&self, message: NetworkMessage<K>) -> Result<(), String> {
        self.network_message_sender
            .send(message)
            .map_err(|e| e.to_string())
    }
}

/// Handle for receiving incoming messages from the network
pub struct NetworkReceiver<K: KeyType> {
    protocol_message_receiver: Receiver<ProtocolMessage<K>>,
}

impl<K: KeyType> NetworkReceiver<K> {
    #[must_use]
    pub fn new(protocol_message_receiver: Receiver<ProtocolMessage<K>>) -> Self {
        Self {
            protocol_message_receiver,
        }
    }

    /// Get the next protocol message
    ///
    /// # Errors
    ///
    /// See [`crossbeam_channel::Receiver::try_recv()`]
    pub fn try_recv(&self) -> Result<ProtocolMessage<K>, crossbeam_channel::TryRecvError> {
        self.protocol_message_receiver.try_recv()
    }
}

/// Combined handle for the network service
pub struct NetworkServiceHandle<K: KeyType> {
    pub local_peer_id: PeerId,
    pub blueprint_protocol_name: Arc<str>,
    pub sender: NetworkSender<K>,
    pub receiver: NetworkReceiver<K>,
    pub peer_manager: Arc<PeerManager<K>>,
}

impl<K: KeyType> Clone for NetworkServiceHandle<K> {
    fn clone(&self) -> Self {
        Self {
            local_peer_id: self.local_peer_id,
            blueprint_protocol_name: self.blueprint_protocol_name.clone(),
            sender: self.sender.clone(),
            receiver: NetworkReceiver::new(self.receiver.protocol_message_receiver.clone()),
            peer_manager: self.peer_manager.clone(),
        }
    }
}

impl<K: KeyType> NetworkServiceHandle<K> {
    #[must_use]
    pub fn new(
        local_peer_id: PeerId,
        blueprint_protocol_name: String,
        peer_manager: Arc<PeerManager<K>>,
        network_message_sender: Sender<NetworkMessage<K>>,
        protocol_message_receiver: Receiver<ProtocolMessage<K>>,
    ) -> Self {
        Self {
            local_peer_id,
            blueprint_protocol_name: Arc::from(blueprint_protocol_name),
            sender: NetworkSender::new(network_message_sender),
            receiver: NetworkReceiver::new(protocol_message_receiver),
            peer_manager,
        }
    }

    pub fn next_protocol_message(&mut self) -> Option<ProtocolMessage<K>> {
        self.receiver.try_recv().ok()
    }

    #[must_use]
    pub fn peers(&self) -> Vec<PeerId> {
        self.peer_manager
            .get_peers()
            .clone()
            .into_read_only()
            .iter()
            .map(|(peer_id, _)| *peer_id)
            .collect()
    }

    #[must_use]
    pub fn peer_info(&self, peer_id: &PeerId) -> Option<PeerInfo> {
        self.peer_manager.get_peer_info(peer_id)
    }

    /// Send a message
    ///
    /// # Errors
    ///
    /// See [`crossbeam_channel::Sender::send`]
    pub fn send(
        &self,
        routing: MessageRouting<K>,
        message: impl Into<Vec<u8>>,
    ) -> Result<(), String> {
        let protocol_message = ProtocolMessage {
            protocol: self.blueprint_protocol_name.clone().to_string(),
            routing,
            payload: message.into(),
        };

        let raw_payload = bincode::serialize(&protocol_message).map_err(|err| err.to_string())?;
        match protocol_message.routing.recipient {
            Some(recipient) => {
                let instance_message_request = InstanceMessageRequest::Protocol {
                    protocol: self.blueprint_protocol_name.clone().to_string(),
                    payload: raw_payload,
                    metadata: None,
                };

                let Some(verification_id_key) = recipient.verification_id_key else {
                    return Ok(());
                };

                let Some(peer_id) = self
                    .peer_manager
                    .get_peer_id_from_verification_id_key(&verification_id_key)
                else {
                    return Ok(());
                };

                self.send_network_message(NetworkMessage::InstanceRequest {
                    peer: peer_id,
                    request: instance_message_request,
                })?;
                debug!("Sent outbound p2p `NetworkMessage` to {:?}", peer_id);
            }
            None => {
                let gossip_message = NetworkMessage::GossipMessage {
                    source: self.local_peer_id,
                    topic: self.blueprint_protocol_name.clone().to_string(),
                    message: raw_payload,
                };
                self.send_network_message(gossip_message)?;
                debug!("Sent outbound gossip `NetworkMessage`");
            }
        }

        Ok(())
    }

    pub(crate) fn send_network_message(&self, message: NetworkMessage<K>) -> Result<(), String> {
        self.sender.send_message(message)
    }

    #[must_use]
    pub fn get_listen_addr(&self) -> Option<Multiaddr> {
        // Get the first peer info for our local peer ID
        if let Some(peer_info) = self.peer_manager.get_peer_info(&self.local_peer_id) {
            // Return the first address from our peer info
            peer_info.addresses.iter().next().cloned()
        } else {
            None
        }
    }

    /// Split the handle into separate sender and receiver
    #[must_use]
    pub fn split(self) -> (NetworkSender<K>, NetworkReceiver<K>) {
        (self.sender, self.receiver)
    }
}

/// We might also bundle a `JoinHandle` so the user can await its completion if needed.
pub struct NetworkServiceTaskHandle {
    /// The join handle for the background service task.
    pub service_task: JoinHandle<()>,
}
