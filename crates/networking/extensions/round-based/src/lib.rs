use blueprint_core::warn;
use blueprint_crypto::KeyType;
use blueprint_networking::{
    discovery::PeerManager,
    service_handle::NetworkServiceHandle,
    types::{ParticipantId, ParticipantInfo, ProtocolMessage},
};
use blueprint_std::{
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
};
use futures::{Sink, Stream};
use round_based::{Delivery, Incoming, MessageDestination, MessageType, Outgoing, PartyIndex};
use serde::{Serialize, de::DeserializeOwned};

/// Wrapper to adapt [`NetworkServiceHandle`] to round-based protocols
pub struct RoundBasedNetworkAdapter<M, K: KeyType> {
    /// The underlying network handle
    handle: NetworkServiceHandle<K>,
    /// Current party's index
    party_index: PartyIndex,
    /// Peer manager
    peer_manager: Arc<PeerManager<K>>,
    /// Counter for message IDs
    next_msg_id: Arc<AtomicU64>,
    /// Protocol identifier
    protocol_id: String,
    _phantom: std::marker::PhantomData<M>,
}

impl<M, K: KeyType> RoundBasedNetworkAdapter<M, K>
where
    M: Clone + Send + Sync + Unpin + 'static,
    M: Serialize + DeserializeOwned,
    M: round_based::ProtocolMessage,
{
    pub fn new(
        handle: NetworkServiceHandle<K>,
        party_index: PartyIndex,
        peer_manager: Arc<PeerManager<K>>,
        protocol_id: impl Into<String>,
    ) -> Self {
        Self {
            handle,
            party_index,
            peer_manager,
            next_msg_id: Arc::new(AtomicU64::new(0)),
            protocol_id: protocol_id.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M, K: KeyType> Delivery<M> for RoundBasedNetworkAdapter<M, K>
where
    M: Clone + Send + Sync + Unpin + 'static,
    M: Serialize + DeserializeOwned,
    M: round_based::ProtocolMessage,
    K::Public: Unpin,
{
    type Send = RoundBasedSender<M, K>;
    type Receive = RoundBasedReceiver<M, K>;
    type SendError = NetworkError;
    type ReceiveError = NetworkError;

    fn split(self) -> (Self::Receive, Self::Send) {
        let RoundBasedNetworkAdapter {
            handle,
            party_index,
            peer_manager,
            next_msg_id,
            protocol_id,
            ..
        } = self;

        let sender = RoundBasedSender {
            handle: handle.clone(),
            party_index,
            peer_manager: peer_manager.clone(),
            next_msg_id: next_msg_id.clone(),
            protocol_id: protocol_id.clone(),
            _phantom: std::marker::PhantomData,
        };

        let receiver = RoundBasedReceiver::new(handle, party_index);

        (receiver, sender)
    }
}

pub struct RoundBasedSender<M, K: KeyType> {
    handle: NetworkServiceHandle<K>,
    party_index: PartyIndex,
    peer_manager: Arc<PeerManager<K>>,
    next_msg_id: Arc<AtomicU64>,
    protocol_id: String,
    _phantom: std::marker::PhantomData<M>,
}

impl<M, K: KeyType> Sink<Outgoing<M>> for RoundBasedSender<M, K>
where
    M: Serialize + round_based::ProtocolMessage + Clone + Unpin,
    K::Public: Unpin,
{
    type Error = NetworkError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, outgoing: Outgoing<M>) -> Result<(), Self::Error> {
        let this = self.get_mut();
        let msg_id = this.next_msg_id.fetch_add(1, Ordering::Relaxed);
        let round = outgoing.msg.round();

        tracing::trace!(
            i = %this.party_index,
            recipient = ?outgoing.recipient,
            %round,
            %msg_id,
            protocol_id = %this.protocol_id,
            "Sending message",
        );

        let recipient_info = match outgoing.recipient {
            MessageDestination::AllParties => None,
            MessageDestination::OneParty(p) => {
                match this
                    .peer_manager
                    .get_peer_id_from_party_index(&ParticipantId(p))
                {
                    Some(peer_id) => Some((p, peer_id)),
                    None => {
                        warn!(party_index = %p, "Could not find PeerId for party index");
                        None
                    }
                }
            }
        };

        if let Some((recipient_index, recipient_peer_id)) = recipient_info {
            let protocol_message = ProtocolMessage {
                protocol: format!("{}/{}", this.protocol_id, round),
                routing: blueprint_networking::types::MessageRouting {
                    sender: ParticipantInfo {
                        id: blueprint_networking::types::ParticipantId(this.party_index),
                        verification_id_key: this
                            .peer_manager
                            .get_verification_id_key_from_peer_id(&this.handle.local_peer_id),
                    },
                    message_id: msg_id,
                    round_id: round,
                    recipient: Some(ParticipantInfo {
                        id: blueprint_networking::types::ParticipantId(recipient_index),
                        verification_id_key: this
                            .peer_manager
                            .get_verification_id_key_from_peer_id(&recipient_peer_id),
                    }),
                },
                payload: serde_json::to_vec(&outgoing.msg).map_err(NetworkError::Serialization)?,
            };

            tracing::trace!(
                %round,
                %msg_id,
                protocol_id = %this.protocol_id,
                recipient_peer_id = %recipient_peer_id,
                "Sending unicast message to network",
            );

            this.handle
                .send(protocol_message.routing, protocol_message.payload)
                .map_err(NetworkError::Send)?;
        } else if outgoing.recipient == MessageDestination::AllParties {
            let protocol_message = ProtocolMessage {
                protocol: format!("{}/{}", this.protocol_id, round),
                routing: blueprint_networking::types::MessageRouting {
                    sender: ParticipantInfo {
                        id: blueprint_networking::types::ParticipantId(this.party_index),
                        verification_id_key: this
                            .peer_manager
                            .get_verification_id_key_from_peer_id(&this.handle.local_peer_id),
                    },
                    message_id: msg_id,
                    round_id: round,
                    recipient: None,
                },
                payload: serde_json::to_vec(&outgoing.msg).map_err(NetworkError::Serialization)?,
            };

            tracing::trace!(
                %round,
                %msg_id,
                protocol_id = %this.protocol_id,
                "Sending broadcast message to network",
            );

            this.handle
                .send(protocol_message.routing, protocol_message.payload)
                .map_err(NetworkError::Send)?;
        }

        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct RoundBasedReceiver<M, K: KeyType> {
    handle: NetworkServiceHandle<K>,
    party_index: PartyIndex,
    _phantom: std::marker::PhantomData<M>,
}

impl<M, K: KeyType> RoundBasedReceiver<M, K> {
    fn new(handle: NetworkServiceHandle<K>, party_index: PartyIndex) -> Self {
        Self {
            handle,
            party_index,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M, K: KeyType> Stream for RoundBasedReceiver<M, K>
where
    M: DeserializeOwned + round_based::ProtocolMessage + Unpin,
    K::Public: Unpin,
{
    type Item = Result<Incoming<M>, NetworkError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Get a mutable reference to self
        let this = self.get_mut();

        let next_protocol_message = this.handle.next_protocol_message();
        match next_protocol_message {
            Some(protocol_message) => {
                let msg_type = if protocol_message.routing.recipient.is_some() {
                    MessageType::P2P
                } else {
                    MessageType::Broadcast
                };

                let Some(sender_verification_id) =
                    protocol_message.routing.sender.verification_id_key
                else {
                    warn!("Received message from unknown participant");
                    return Poll::Ready(Some(Err(NetworkError::UnknownPeer(
                        "Failed to find verification ID key".to_string(),
                    ))));
                };
                let Some(sender_peer_id) = this
                    .handle
                    .peer_manager
                    .get_peer_id_from_verification_id_key(&sender_verification_id)
                else {
                    warn!("Received message from unknown participant");
                    return Poll::Ready(Some(Err(NetworkError::UnknownPeer(format!(
                        "Failed to find peer ID for verification ID key: {:?}",
                        sender_verification_id
                    )))));
                };

                let sender_party_index = match this
                    .handle
                    .peer_manager
                    .get_party_index_from_peer_id(&sender_peer_id)
                {
                    Some(sender_party_index) => sender_party_index.0,
                    None => {
                        warn!("Received message from unknown peer: {}", sender_peer_id);
                        return Poll::Ready(Some(Err(NetworkError::UnknownPeer(
                            sender_peer_id.to_string(),
                        ))));
                    }
                };
                let id = protocol_message.routing.message_id;

                tracing::trace!(
                    i = %this.party_index,
                    sender = ?sender_party_index,
                    %id,
                    protocol_id = %protocol_message.protocol,
                    ?msg_type,
                    size = %protocol_message.payload.len(),
                    "Received message",
                );
                match serde_json::from_slice(&protocol_message.payload) {
                    Ok(msg) => Poll::Ready(Some(Ok(Incoming {
                        msg,
                        sender: sender_party_index,
                        id,
                        msg_type,
                    }))),
                    Err(e) => Poll::Ready(Some(Err(NetworkError::Serialization(e)))),
                }
            }
            None => {
                //tracing::trace!(i = %this.party_index, "No message received; the waker will wake us up when there is a new message");
                // In this case, tell the waker to wake us up when there is a new message
                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum NetworkError {
    #[error("Failed to serialize message: {0}")]
    Serialization(#[from] serde_json::Error),
    #[error("Network error: {0}")]
    Send(String),
    #[error("Unknown peer: {0}")]
    UnknownPeer(String),
}
