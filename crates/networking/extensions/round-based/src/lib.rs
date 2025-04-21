use blueprint_crypto::KeyType;
use blueprint_networking::{service_handle::NetworkServiceHandle, types::ProtocolMessage};
use dashmap::DashMap;
use futures::{Sink, Stream};
use libp2p::PeerId;
use round_based::{Delivery, Incoming, MessageDestination, MessageType, Outgoing, PartyIndex};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    collections::HashMap,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    task::{Context, Poll},
};

/// Wrapper to adapt [`NetworkServiceHandle`] to round-based protocols
pub struct RoundBasedNetworkAdapter<M, K: KeyType> {
    /// The underlying network handle
    handle: NetworkServiceHandle<K>,
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
        parties: HashMap<PartyIndex, PeerId>,
        protocol_id: impl Into<String>,
    ) -> Self {
        Self {
            handle,
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
    K::Secret: Unpin,
{
    type Send = RoundBasedSender<M, K>;
    type Receive = RoundBasedReceiver<M, K>;
    type SendError = NetworkError;
    type ReceiveError = NetworkError;

    fn split(self) -> (Self::Receive, Self::Send) {
        let RoundBasedNetworkAdapter {
            handle,
            next_msg_id,
            protocol_id,
            ..
        } = self;

        let sender = RoundBasedSender {
            handle: handle.clone(),
            next_msg_id: next_msg_id.clone(),
            protocol_id: protocol_id.clone(),
            _phantom: std::marker::PhantomData,
        };

        let receiver = RoundBasedReceiver::new(handle);

        (receiver, sender)
    }
}

pub struct RoundBasedSender<M, K: KeyType> {
    handle: NetworkServiceHandle<K>,
    next_msg_id: Arc<AtomicU64>,
    protocol_id: String,
    _phantom: std::marker::PhantomData<M>,
}

impl<M, K: KeyType> Sink<Outgoing<M>> for RoundBasedSender<M, K>
where
    M: Serialize + round_based::ProtocolMessage + Clone + Unpin,
    K::Public: Unpin,
    K::Secret: Unpin,
{
    type Error = NetworkError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, outgoing: Outgoing<M>) -> Result<(), Self::Error> {
        let this = self.get_mut();
        let msg_id = this.next_msg_id.fetch_add(1, Ordering::Relaxed);
        let round = outgoing.msg.round();
        let party_index = this
            .handle
            .peer_manager
            .get_whitelist_index_from_peer_id(&this.handle.local_peer_id)
            .unwrap_or_default();

        tracing::trace!(
            i = %party_index,
            recipient = ?outgoing.recipient,
            %round,
            %msg_id,
            protocol_id = %this.protocol_id,
            "Sending message",
        );

        let (recipient, _) = match outgoing.recipient {
            MessageDestination::AllParties => (None, None),
            MessageDestination::OneParty(p) => {
                let key = this
                    .handle
                    .peer_manager
                    .get_peer_id_from_whitelist_index(p as usize);
                (Some(p), key)
            }
        };

        let protocol_message = ProtocolMessage {
            protocol: format!("{}/{}", this.protocol_id, round),
            routing: blueprint_networking::types::MessageRouting {
                message_id: msg_id,
                round_id: round,
                sender: this.handle.local_peer_id,
                recipient: recipient.and_then(|p| {
                    this.handle
                        .peer_manager
                        .get_peer_id_from_whitelist_index(p as usize)
                }),
            },
            payload: serde_json::to_vec(&outgoing.msg).map_err(NetworkError::Serialization)?,
        };

        tracing::trace!(
            %round,
            %msg_id,
            protocol_id = %this.protocol_id,
            "Sending message to network",
        );

        this.handle
            .send(protocol_message.routing, protocol_message.payload)
            .map_err(NetworkError::Send)
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
    _phantom: std::marker::PhantomData<M>,
}

impl<M, K: KeyType> RoundBasedReceiver<M, K> {
    fn new(handle: NetworkServiceHandle<K>) -> Self {
        Self {
            handle,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M, K: KeyType> Stream for RoundBasedReceiver<M, K>
where
    M: DeserializeOwned + round_based::ProtocolMessage + Unpin,
    K::Public: Unpin,
    K::Secret: Unpin,
{
    type Item = Result<Incoming<M>, NetworkError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // Get a mutable reference to self
        let this = self.get_mut();
        let party_index = this
            .handle
            .peer_manager
            .get_whitelist_index_from_peer_id(&this.handle.local_peer_id)
            .unwrap_or_default();
        let next_protocol_message = this.handle.next_protocol_message();
        match next_protocol_message {
            Some(protocol_message) => {
                let msg_type = if protocol_message.routing.recipient.is_some() {
                    MessageType::P2P
                } else {
                    MessageType::Broadcast
                };

                let sender = protocol_message.routing.sender;
                let sender_index = this
                    .handle
                    .peer_manager
                    .get_whitelist_index_from_peer_id(&sender);
                let id = protocol_message.routing.message_id;

                match sender_index {
                    Some(sender_index) => match serde_json::from_slice(&protocol_message.payload) {
                        Ok(msg) => {
                            tracing::trace!(
                                i = %party_index,
                                sender = ?sender_index,
                                %id,
                                protocol_id = %protocol_message.protocol,
                                ?msg_type,
                                size = %protocol_message.payload.len(),
                                "Received message",
                            );
                            Poll::Ready(Some(Ok(Incoming {
                                msg,
                                sender: sender_index as u16,
                                id,
                                msg_type,
                            })))
                        }
                        Err(e) => Poll::Ready(Some(Err(NetworkError::Serialization(e)))),
                    },
                    None => {
                        tracing::warn!(
                            i = %party_index,
                            sender = ?sender,
                            %id,
                            protocol_id = %protocol_message.protocol,
                            "Received message from unknown sender; ignoring",
                        );
                        cx.waker().wake_by_ref();
                        Poll::Pending
                    }
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
}
