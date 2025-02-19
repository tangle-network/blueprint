use crossbeam_channel::{self, Receiver, Sender};
use dashmap::DashMap;
use futures::Future;
use futures::{Sink, Stream};
use gadget_networking::{
    key_types::InstanceMsgPublicKey,
    service_handle::NetworkServiceHandle,
    types::{ParticipantInfo, ProtocolMessage},
};
use round_based::{
    Delivery, Incoming, MessageDestination, MessageType, MsgId, Outgoing, PartyIndex,
};
use serde::{de::DeserializeOwned, Serialize};
use std::{
    collections::HashMap,
    pin::Pin,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    task::{Context, Poll},
};

/// Wrapper to adapt NetworkServiceHandle to round-based protocols
pub struct RoundBasedNetworkAdapter<M> {
    /// The underlying network handle
    handle: NetworkServiceHandle,
    /// Current party's index
    party_index: PartyIndex,
    /// Mapping of party indices to their public keys
    parties: Arc<DashMap<PartyIndex, InstanceMsgPublicKey>>,
    /// Counter for message IDs
    next_msg_id: Arc<AtomicU64>,
    /// Channel for forwarding messages
    forward_tx: Sender<(PartyIndex, M)>,
    /// Channel for receiving forwarded messages
    forward_rx: Receiver<(PartyIndex, M)>,
    /// Protocol identifier
    protocol_id: String,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> RoundBasedNetworkAdapter<M>
where
    M: Clone + Send + Sync + Unpin + 'static,
    M: Serialize + DeserializeOwned,
    M: round_based::ProtocolMessage,
{
    pub fn new(
        handle: NetworkServiceHandle,
        party_index: PartyIndex,
        parties: HashMap<PartyIndex, InstanceMsgPublicKey>,
        protocol_id: impl Into<String>,
    ) -> Self {
        let (forward_tx, forward_rx) = crossbeam_channel::unbounded();

        Self {
            handle,
            party_index,
            parties: Arc::new(DashMap::from_iter(parties)),
            next_msg_id: Arc::new(AtomicU64::new(0)),
            forward_tx,
            forward_rx,
            protocol_id: protocol_id.into(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<M> Delivery<M> for RoundBasedNetworkAdapter<M>
where
    M: Clone + Send + Sync + Unpin + 'static,
    M: Serialize + DeserializeOwned,
    M: round_based::ProtocolMessage,
{
    type Send = RoundBasedSender<M>;
    type Receive = RoundBasedReceiver<M>;
    type SendError = NetworkError;
    type ReceiveError = NetworkError;

    fn split(self) -> (Self::Receive, Self::Send) {
        let RoundBasedNetworkAdapter {
            handle,
            party_index,
            parties,
            next_msg_id,
            forward_tx,
            forward_rx,
            protocol_id,
            ..
        } = self;

        let sender = RoundBasedSender {
            handle: handle.clone(),
            party_index,
            parties: parties.clone(),
            next_msg_id: next_msg_id.clone(),
            protocol_id: protocol_id.clone(),
            forward_tx,
            _phantom: std::marker::PhantomData,
        };

        let receiver = RoundBasedReceiver::new(handle, party_index, forward_rx);

        (receiver, sender)
    }
}

pub struct RoundBasedSender<M> {
    handle: NetworkServiceHandle,
    party_index: PartyIndex,
    parties: Arc<DashMap<PartyIndex, InstanceMsgPublicKey>>,
    next_msg_id: Arc<AtomicU64>,
    protocol_id: String,
    forward_tx: Sender<(PartyIndex, M)>,
    _phantom: std::marker::PhantomData<M>,
}

impl<M> Sink<Outgoing<M>> for RoundBasedSender<M>
where
    M: Serialize + round_based::ProtocolMessage + Clone,
{
    type Error = NetworkError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, outgoing: Outgoing<M>) -> Result<(), Self::Error> {
        let this = unsafe { self.get_unchecked_mut() };
        let msg_id = this.next_msg_id.fetch_add(1, Ordering::Relaxed);
        let round = outgoing.msg.round();

        // Handle local message forwarding for self-messages
        if outgoing.recipient == MessageDestination::OneParty(this.party_index) {
            return this
                .forward_tx
                .send((this.party_index, outgoing.msg.clone()))
                .map_err(|_| NetworkError::Send("Failed to forward local message".into()));
        }

        let (recipient, recipient_key) = match outgoing.recipient {
            MessageDestination::AllParties => (None, None),
            MessageDestination::OneParty(p) => {
                let key = this.parties.get(&p).map(|k| k.clone());
                (Some(p), key)
            }
        };

        let protocol_message = ProtocolMessage {
            protocol: format!("{}/{}", this.protocol_id, round),
            routing: gadget_networking::types::MessageRouting {
                message_id: msg_id,
                round_id: round,
                sender: ParticipantInfo {
                    id: gadget_networking::types::ParticipantId(this.party_index),
                    public_key: this.parties.get(&this.party_index).map(|k| k.clone()),
                },
                recipient: recipient.map(|p| ParticipantInfo {
                    id: gadget_networking::types::ParticipantId(p),
                    public_key: recipient_key,
                }),
            },
            payload: serde_json::to_vec(&outgoing.msg)
                .map_err(|e| NetworkError::Serialization(e))?,
        };

        this.handle
            .send_protocol_message(protocol_message)
            .map_err(|e| NetworkError::Send(e))
    }

    fn poll_flush(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }
}

pub struct RoundBasedReceiver<M> {
    handle: NetworkServiceHandle,
    party_index: PartyIndex,
    forward_rx: Receiver<(PartyIndex, M)>,
    _phantom: std::marker::PhantomData<M>,
    next_message_future: Option<Pin<Box<dyn Future<Output = Option<ProtocolMessage>> + Send>>>,
}

impl<M> RoundBasedReceiver<M> {
    fn new(
        handle: NetworkServiceHandle,
        party_index: PartyIndex,
        forward_rx: Receiver<(PartyIndex, M)>,
    ) -> Self {
        Self {
            handle,
            party_index,
            forward_rx,
            _phantom: std::marker::PhantomData,
            next_message_future: None,
        }
    }
}

impl<M> Stream for RoundBasedReceiver<M>
where
    M: DeserializeOwned + round_based::ProtocolMessage + Unpin,
{
    type Item = Result<Incoming<M>, NetworkError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        // First check forwarded messages
        if let Ok((sender, msg)) = self.forward_rx.try_recv() {
            return Poll::Ready(Some(Ok(Incoming {
                msg,
                sender,
                id: 0,
                msg_type: MessageType::P2P,
            })));
        }

        // Get a mutable reference to self
        let this = self.get_mut();

        // Create and store the future if we don't have one
        if this.next_message_future.is_none() {
            let mut handle = this.handle.clone();
            this.next_message_future =
                Some(Box::pin(
                    async move { handle.next_protocol_message().await },
                ));
        }

        // Poll the stored future
        if let Some(future) = &mut this.next_message_future {
            match future.as_mut().poll(cx) {
                Poll::Ready(Some(msg)) => {
                    // Clear the future so we create a new one next time
                    this.next_message_future = None;

                    let msg_type = if msg.routing.recipient.is_some() {
                        MessageType::P2P
                    } else {
                        MessageType::Broadcast
                    };

                    let sender = msg.routing.sender.id.0;
                    let id = msg.routing.message_id;

                    match serde_json::from_slice(&msg.payload) {
                        Ok(msg) => Poll::Ready(Some(Ok(Incoming {
                            msg,
                            sender,
                            id,
                            msg_type,
                        }))),
                        Err(e) => Poll::Ready(Some(Err(NetworkError::Serialization(e)))),
                    }
                }
                Poll::Ready(None) => {
                    this.next_message_future = None;
                    Poll::Ready(None)
                }
                Poll::Pending => Poll::Pending,
            }
        } else {
            // This shouldn't happen because we create the future above if it's None
            Poll::Ready(None)
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
