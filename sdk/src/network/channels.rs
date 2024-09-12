use crate::network::{deserialize, IdentifierInfo, Network, ProtocolMessage};
use futures::StreamExt;
use gadget_io::tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use round_based::{Incoming, MessageDestination, MessageType, MsgId, Outgoing, PartyIndex};
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use sp_core::ecdsa;

use std::collections::HashMap;
use std::sync::Arc;

use crate::error::Error;
use crate::logger::Logger;

pub type UserID = u16;

/// Represent a message transmitting between parties on wire
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Msg<B> {
    /// Index of the sender
    ///
    /// Lies in range `[1; n]` where `n` is number of parties involved in computation
    pub sender: u16,
    /// Index of receiver
    ///
    /// `None` indicates that it's broadcast message. Receiver index, if set, lies in range `[1; n]`
    /// where `n` is number of parties involved in computation
    pub receiver: Option<u16>,
    /// Message body
    pub body: B,
}

impl<B> Msg<B> {
    /// Applies closure to message body
    pub fn map_body<T, F>(self, f: F) -> Msg<T>
    where
        F: FnOnce(B) -> T,
    {
        Msg {
            sender: self.sender,
            receiver: self.receiver,
            body: f(self.body),
        }
    }
}

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments, clippy::let_underscore_future)]
pub fn create_job_manager_to_async_protocol_channel_split<
    N: Network,
    C1: Serialize + DeserializeOwned + MaybeSenderReceiver + Send + 'static,
    C2: Serialize + DeserializeOwned + MaybeSenderReceiver + Send + 'static,
>(
    mut rx_gadget: UnboundedReceiver<ProtocolMessage>,
    identifier_info: IdentifierInfo,
    user_id_mapping: Arc<HashMap<u16, ecdsa::Public>>,
    my_account_id: ecdsa::Public,
    network: N,
    logger: Logger,
) -> (
    futures::channel::mpsc::UnboundedSender<C1>,
    futures::channel::mpsc::UnboundedReceiver<std::io::Result<C1>>,
    UnboundedSender<C2>,
    UnboundedReceiver<C2>,
) {
    let (tx_to_async_proto_1, rx_for_async_proto_1) = futures::channel::mpsc::unbounded();
    let (tx_to_async_proto_2, rx_for_async_proto_2) =
        gadget_io::tokio::sync::mpsc::unbounded_channel();
    let logger_outgoing = logger.clone();
    // Take the messages from the gadget and send them to the async protocol
    let _ = gadget_io::tokio::task::spawn(async move {
        while let Some(msg) = rx_gadget.recv().await {
            match deserialize::<MultiplexedChannelMessage<C1, C2>>(&msg.payload[..]) {
                Ok(msg) => match msg {
                    MultiplexedChannelMessage::Channel1(msg) => {
                        if tx_to_async_proto_1.unbounded_send(Ok(msg)).is_err() {
                            logger.error("Failed to send message to C1 protocol");
                        }
                    }
                    MultiplexedChannelMessage::Channel2(msg) => {
                        if tx_to_async_proto_2.send(msg).is_err() {
                            logger.error("Failed to send message to C2 protocol");
                        }
                    }

                    _ => {
                        unreachable!("We only have two channels")
                    }
                },
                Err(err) => {
                    logger.error(format!("Failed to deserialize message: {err:?}"));
                }
            }
        }
    });

    let (tx_to_outbound_1, mut rx_to_outbound_1) = futures::channel::mpsc::unbounded::<C1>();
    let (tx_to_outbound_2, mut rx_to_outbound_2) =
        gadget_io::tokio::sync::mpsc::unbounded_channel::<C2>();
    let network_clone = network.clone();
    let user_id_mapping_clone = user_id_mapping.clone();
    let my_user_id = user_id_mapping
        .iter()
        .find_map(|(user_id, account_id)| {
            if *account_id == my_account_id {
                Some(*user_id)
            } else {
                None
            }
        })
        .expect("Failed to find my user id");

    // Take the messages the async protocol sends to the outbound channel and send them to the gadget
    let _ = gadget_io::tokio::task::spawn(async move {
        let logger = &logger_outgoing;
        let channel_1_task = async move {
            while let Some(msg) = rx_to_outbound_1.next().await {
                if let Err(err) = wrap_message_and_forward_to_network::<_, C1, C2, (), _>(
                    msg,
                    &network,
                    &user_id_mapping,
                    my_user_id,
                    identifier_info,
                    MultiplexedChannelMessage::Channel1,
                    logger,
                )
                .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }
        };

        let channel_2_task = async move {
            while let Some(msg) = rx_to_outbound_2.recv().await {
                if let Err(err) = wrap_message_and_forward_to_network::<_, C1, C2, (), _>(
                    msg,
                    &network_clone,
                    &user_id_mapping_clone,
                    my_user_id,
                    identifier_info,
                    MultiplexedChannelMessage::Channel2,
                    logger,
                )
                .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }
        };

        gadget_io::tokio::join!(channel_1_task, channel_2_task);
    });

    (
        tx_to_outbound_1,
        rx_for_async_proto_1,
        tx_to_outbound_2,
        rx_for_async_proto_2,
    )
}

impl<T> MaybeSenderReceiver for Msg<T> {
    fn maybe_sender(&self) -> MaybeSender {
        MaybeSender::SomeoneElse(self.sender as UserID)
    }

    fn maybe_receiver(&self) -> MaybeReceiver {
        match self.receiver {
            None => MaybeReceiver::Broadcast,
            Some(i) => MaybeReceiver::P2P(i as UserID),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MultiplexedChannelMessage<C1, C2, C3 = ()> {
    Channel1(C1),
    Channel2(C2),
    Channel3(C3),
}

/// All possible senders of a message
#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub enum MaybeSender {
    /// We are the sender of the message
    Myself,
    /// The sender is someone else
    /// it could also be us, double check the [`UserID`]
    SomeoneElse(UserID),
    /// The sender is unknown.
    #[default]
    Unknown,
}

impl MaybeSender {
    /// Returns `true` if the maybe sender is [`Myself`].
    ///
    /// [`Myself`]: MaybeSender::Myself
    #[must_use]
    pub fn is_myself(&self) -> bool {
        matches!(self, Self::Myself)
    }

    /// Returns `true` if the maybe sender is [`Myself`].
    /// Or if the sender is [`SomeoneElse`] but the [`UserID`] is the same as `my_user_id`
    ///
    /// [`Myself`]: MaybeSender::Myself
    /// [`SomeoneElse`]: MaybeSender::SomeoneElse
    #[must_use]
    pub fn is_myself_check(&self, my_user_id: UserID) -> bool {
        match self {
            Self::Myself => true,
            Self::SomeoneElse(id) if (*id == my_user_id) => true,
            _ => false,
        }
    }

    /// Returns `true` if the maybe sender is [`SomeoneElse`].
    ///
    /// [`SomeoneElse`]: MaybeSender::SomeoneElse
    #[must_use]
    pub fn is_someone_else(&self) -> bool {
        matches!(self, Self::SomeoneElse(..))
    }

    /// Returns `true` if the maybe sender is [`Unknown`].
    ///
    /// [`Unknown`]: MaybeSender::Unknown
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns the sender as [`UserID`] if it is knwon.
    #[must_use]
    pub fn as_user_id(&self) -> Option<UserID> {
        match self {
            Self::Myself => None,
            Self::SomeoneElse(id) => Some(*id),
            Self::Unknown => None,
        }
    }
}

#[derive(Debug, Default, Serialize, Deserialize, Copy, Clone)]
pub enum MaybeReceiver {
    /// The message is broadcasted to everyone
    Broadcast,
    /// The message is sent to a specific party
    P2P(UserID),
    /// The receiver is us.
    Myself,
    /// The receiver is unknown.
    #[default]
    Unknown,
}

impl MaybeReceiver {
    /// Returns `true` if the maybe receiver is [`Broadcast`].
    ///
    /// [`Broadcast`]: MaybeReceiver::Broadcast
    #[must_use]
    pub fn is_broadcast(&self) -> bool {
        matches!(self, Self::Broadcast)
    }

    /// Returns `true` if the maybe receiver is [`P2P`].
    ///
    /// [`P2P`]: MaybeReceiver::P2P
    #[must_use]
    pub fn is_p2p(&self) -> bool {
        matches!(self, Self::P2P(..))
    }

    /// Returns `true` if the maybe receiver is [`Myself`].
    ///
    /// [`Myself`]: MaybeReceiver::Myself
    #[must_use]
    pub fn is_myself(&self) -> bool {
        matches!(self, Self::Myself)
    }

    /// Returns `true` if the maybe receiver is [`Myself`]
    /// Or if the receiver is [`P2P`] but the [`UserID`] is the same as `my_user_id`
    ///
    /// [`Myself`]: MaybeReceiver::Myself
    /// [`P2P`]: MaybeReceiver::P2P
    #[must_use]
    pub fn is_myself_check(&self, my_user_id: UserID) -> bool {
        match self {
            Self::Myself => true,
            Self::P2P(id) if (*id == my_user_id) => true,
            _ => false,
        }
    }

    /// Returns `true` if the maybe receiver is [`Unknown`].
    ///
    /// [`Unknown`]: MaybeReceiver::Unknown
    #[must_use]
    pub fn is_unknown(&self) -> bool {
        matches!(self, Self::Unknown)
    }

    /// Returns the receiver as [`UserID`] if it is knwon.
    #[must_use]
    pub fn as_user_id(&self) -> Option<UserID> {
        match self {
            Self::Broadcast => None,
            Self::P2P(id) => Some(*id),
            Self::Myself => None,
            Self::Unknown => None,
        }
    }
}

pub trait InnerMessage {
    type Inner: Serialize + DeserializeOwned + Send + 'static;
    fn inner_message(self) -> Self::Inner;
}

pub trait InnerMessageFromInbound: Sized + InnerMessage {
    fn from_inbound(
        id: MsgId,
        sender: PartyIndex,
        msg_type: MessageType,
        msg: <Self as InnerMessage>::Inner,
    ) -> Self;
}

impl<M: Serialize + DeserializeOwned + Send + 'static> InnerMessage for Outgoing<M> {
    type Inner = M;

    fn inner_message(self) -> Self::Inner {
        self.msg
    }
}

impl<M: Serialize + DeserializeOwned + Send + 'static> InnerMessage for Incoming<M> {
    type Inner = M;

    fn inner_message(self) -> Self::Inner {
        self.msg
    }
}

/// A Simple trait to extract the sender and the receiver from a message
pub trait MaybeSenderReceiver {
    fn maybe_sender(&self) -> MaybeSender;
    fn maybe_receiver(&self) -> MaybeReceiver;
}

impl<M> MaybeSenderReceiver for Outgoing<M> {
    fn maybe_sender(&self) -> MaybeSender {
        MaybeSender::Myself
    }

    fn maybe_receiver(&self) -> MaybeReceiver {
        match self.recipient {
            MessageDestination::AllParties => MaybeReceiver::Broadcast,
            MessageDestination::OneParty(i) => MaybeReceiver::P2P(i as UserID),
        }
    }
}

impl<M> MaybeSenderReceiver for Incoming<M> {
    fn maybe_sender(&self) -> MaybeSender {
        MaybeSender::SomeoneElse(self.sender as UserID)
    }

    fn maybe_receiver(&self) -> MaybeReceiver {
        match self.msg_type {
            MessageType::Broadcast => MaybeReceiver::Broadcast,
            MessageType::P2P => MaybeReceiver::Myself,
        }
    }
}

impl<M: Serialize + DeserializeOwned + Send + 'static> InnerMessageFromInbound for Incoming<M> {
    fn from_inbound(
        id: MsgId,
        sender: PartyIndex,
        msg_type: MessageType,
        msg: <Self as InnerMessage>::Inner,
    ) -> Self {
        Incoming {
            id,
            sender,
            msg_type,
            msg,
        }
    }
}

impl MaybeSenderReceiver for () {
    fn maybe_sender(&self) -> MaybeSender {
        MaybeSender::Unknown
    }

    fn maybe_receiver(&self) -> MaybeReceiver {
        MaybeReceiver::Unknown
    }
}

pub type DuplexedChannel<O, I, C2> = (
    futures::channel::mpsc::UnboundedSender<O>,
    futures::channel::mpsc::UnboundedReceiver<Result<I, futures::channel::mpsc::TryRecvError>>,
    futures::channel::mpsc::UnboundedSender<C2>,
    futures::channel::mpsc::UnboundedReceiver<C2>,
);

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments, clippy::let_underscore_future)]
pub fn create_job_manager_to_async_protocol_channel_split_io<
    N: Network,
    C2: Serialize + DeserializeOwned + MaybeSenderReceiver + Send + 'static,
    O: InnerMessage<Inner = I::Inner> + MaybeSenderReceiver + Send + 'static,
    I: InnerMessage + InnerMessageFromInbound + MaybeSenderReceiver + Send + 'static,
>(
    mut rx_gadget: UnboundedReceiver<ProtocolMessage>,
    identifier_info: IdentifierInfo,
    user_id_mapping: Arc<HashMap<u16, ecdsa::Public>>,
    my_account_id: ecdsa::Public,
    network: N,
    logger: Logger,
    i: u16,
) -> DuplexedChannel<O, I, C2> {
    let (tx_to_async_proto_1, rx_for_async_proto_1) = futures::channel::mpsc::unbounded();
    let (tx_to_async_proto_2, rx_for_async_proto_2) = futures::channel::mpsc::unbounded();
    let logger_outgoing = logger.clone();
    let mapping_clone = user_id_mapping.clone();

    let my_user_id = user_id_mapping
        .iter()
        .find_map(|(user_id, account_id)| {
            if *account_id == my_account_id {
                Some(*user_id)
            } else {
                None
            }
        })
        .expect("Failed to find my user id");

    if my_user_id != i {
        logger.error(format!(
            "My user id is not equal to i: {} != {}",
            my_user_id, i
        ));
    }

    // Take the messages from the gadget and send them to the async protocol
    let _ = gadget_io::tokio::task::spawn(async move {
        let mut id = 0;
        while let Some(msg_orig) = rx_gadget.recv().await {
            if msg_orig.payload.is_empty() {
                logger.warn(format!(
                    "Received empty message from Peer {:?}",
                    msg_orig.sender.user_id
                ));
                continue;
            }

            match deserialize::<MultiplexedChannelMessage<O::Inner, C2>>(&msg_orig.payload[..]) {
                Ok(msg) => match msg {
                    MultiplexedChannelMessage::Channel1(msg) => {
                        logger.trace(format!("Received message count: {id}", id = id + 1));
                        logger.trace(format!(
                            "Received message from {:?} as {:?}",
                            msg_orig.sender.user_id, msg_orig.recipient
                        ));
                        let msg_type = if let Some(to) = msg_orig.recipient {
                            if let Some(to_account_id) = mapping_clone.get(&to.user_id) {
                                if *to_account_id != my_account_id {
                                    logger.error("Invalid message received");
                                    continue;
                                }
                            } else {
                                logger
                                    .error("Invalid message received (`to` not found in mapping)");
                                continue;
                            }

                            MessageType::P2P
                        } else {
                            MessageType::Broadcast
                        };

                        let incoming = I::from_inbound(id, msg_orig.sender.user_id, msg_type, msg);

                        if tx_to_async_proto_1.unbounded_send(Ok(incoming)).is_err() {
                            logger.error("Failed to send Incoming message to protocol");
                        }

                        id += 1;
                    }
                    MultiplexedChannelMessage::Channel2(msg) => {
                        if tx_to_async_proto_2.unbounded_send(msg).is_err() {
                            logger.error("Failed to send C2 message to protocol");
                        }
                    }
                    _ => {
                        unreachable!("We only have two channels")
                    }
                },
                Err(err) => {
                    logger.error(format!("Failed to deserialize message: {err:?}"));
                }
            }
        }
    });

    let (tx_to_outbound_1, mut rx_to_outbound_1) = futures::channel::mpsc::unbounded::<O>();
    let (tx_to_outbound_2, mut rx_to_outbound_2) = futures::channel::mpsc::unbounded::<C2>();
    let network_clone = network.clone();
    let user_id_mapping_clone = user_id_mapping.clone();

    // Take the messages from the async protocol and send them to the gadget
    let _ = gadget_io::tokio::task::spawn(async move {
        let logger = &logger_outgoing;
        let channel_1_task = async move {
            while let Some(msg) = rx_to_outbound_1.next().await {
                if let Err(err) = wrap_message_and_forward_to_network::<_, O::Inner, C2, (), _>(
                    msg,
                    &network,
                    &user_id_mapping,
                    my_user_id,
                    identifier_info,
                    |m| MultiplexedChannelMessage::Channel1(m.inner_message()),
                    logger,
                )
                .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }

            logger.trace("Channel 1 outgoing task closing")
        };

        let channel_2_task = async move {
            while let Some(msg) = rx_to_outbound_2.next().await {
                if let Err(err) = wrap_message_and_forward_to_network::<_, O::Inner, C2, (), _>(
                    msg,
                    &network_clone,
                    &user_id_mapping_clone,
                    my_user_id,
                    identifier_info,
                    |m| MultiplexedChannelMessage::Channel2(m),
                    logger,
                )
                .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }

            logger.trace("Channel 2 outgoing task closing")
        };

        gadget_io::tokio::join!(channel_1_task, channel_2_task);
    });

    (
        tx_to_outbound_1,
        rx_for_async_proto_1,
        tx_to_outbound_2,
        rx_for_async_proto_2,
    )
}

pub type TriplexedChannel<O1, I1, O2, I2, C2> = (
    futures::channel::mpsc::UnboundedSender<O1>,
    futures::channel::mpsc::UnboundedReceiver<Result<I1, futures::channel::mpsc::TryRecvError>>,
    futures::channel::mpsc::UnboundedSender<O2>,
    futures::channel::mpsc::UnboundedReceiver<Result<I2, futures::channel::mpsc::TryRecvError>>,
    futures::channel::mpsc::UnboundedSender<C2>,
    futures::channel::mpsc::UnboundedReceiver<C2>,
);

#[cfg(feature = "std")]
#[allow(clippy::too_many_arguments, clippy::let_underscore_future)]
pub fn create_job_manager_to_async_protocol_channel_split_io_triplex<
    N: Network + 'static,
    C3: Serialize + DeserializeOwned + MaybeSenderReceiver + Send + 'static,
    O1: InnerMessage<Inner = I1::Inner> + MaybeSenderReceiver + Send + 'static,
    I1: InnerMessage + InnerMessageFromInbound + MaybeSenderReceiver + Send + 'static,
    O2: InnerMessage<Inner = I2::Inner> + MaybeSenderReceiver + Send + 'static,
    I2: InnerMessage + InnerMessageFromInbound + MaybeSenderReceiver + Send + 'static,
>(
    mut rx_gadget: UnboundedReceiver<ProtocolMessage>,
    identifier_info: IdentifierInfo,
    user_id_mapping: Arc<HashMap<u16, ecdsa::Public>>,
    my_account_id: ecdsa::Public,
    network: N,
    logger: Logger,
) -> TriplexedChannel<O1, I1, O2, I2, C3> {
    let (tx_to_async_proto_1, rx_for_async_proto_1) = futures::channel::mpsc::unbounded();
    let (tx_to_async_proto_2, rx_for_async_proto_2) = futures::channel::mpsc::unbounded();
    let (tx_to_async_proto_3, rx_for_async_proto_3) = futures::channel::mpsc::unbounded();

    let logger_outgoing = logger.clone();
    // Take the messages from the gadget and send them to the async protocol
    let _ = gadget_io::tokio::task::spawn(async move {
        let mut id = 0;
        while let Some(msg_orig) = rx_gadget.recv().await {
            if msg_orig.payload.is_empty() {
                logger.warn(format!(
                    "Received empty message from Peer {:?}",
                    msg_orig.sender.user_id
                ));
                continue;
            }

            match deserialize::<MultiplexedChannelMessage<O1::Inner, O2::Inner, C3>>(
                &msg_orig.payload[..],
            ) {
                Ok(msg) => match msg {
                    MultiplexedChannelMessage::Channel1(msg) => {
                        let msg_type = if msg_orig.recipient.is_some() {
                            MessageType::P2P
                        } else {
                            MessageType::Broadcast
                        };

                        let incoming = I1::from_inbound(id, msg_orig.sender.user_id, msg_type, msg);

                        if tx_to_async_proto_1.unbounded_send(Ok(incoming)).is_err() {
                            logger.error("Failed to send Incoming message to protocol");
                        }

                        id += 1;
                    }
                    MultiplexedChannelMessage::Channel2(msg) => {
                        let msg_type = if msg_orig.recipient.is_some() {
                            MessageType::P2P
                        } else {
                            MessageType::Broadcast
                        };

                        let incoming = I2::from_inbound(id, msg_orig.sender.user_id, msg_type, msg);

                        if tx_to_async_proto_2.unbounded_send(Ok(incoming)).is_err() {
                            logger.error("Failed to send Incoming message to protocol");
                        }

                        id += 1;
                    }
                    MultiplexedChannelMessage::Channel3(msg) => {
                        if tx_to_async_proto_3.unbounded_send(msg).is_err() {
                            logger.error("Failed to send C2 message to protocol");
                        }
                    }
                },

                Err(err) => {
                    logger.error(format!("Failed to deserialize message: {err:?}"));
                }
            }
        }
    });

    let (tx_to_outbound_1, mut rx_to_outbound_1) = futures::channel::mpsc::unbounded::<O1>();
    let (tx_to_outbound_2, mut rx_to_outbound_2) = futures::channel::mpsc::unbounded::<O2>();
    let (tx_to_outbound_3, mut rx_to_outbound_3) = futures::channel::mpsc::unbounded::<C3>();

    let my_user_id = user_id_mapping
        .iter()
        .find_map(|(user_id, account_id)| {
            if *account_id == my_account_id {
                Some(*user_id)
            } else {
                None
            }
        })
        .expect("Failed to find my user id");
    // Take the messages from the async protocol and send them to the gadget
    let _ = gadget_io::tokio::task::spawn(async move {
        let user_id_mapping = &user_id_mapping;
        let network = &network;
        let logger = &logger_outgoing;
        let task0 = async move {
            while let Some(msg) = rx_to_outbound_1.next().await {
                if let Err(err) =
                    wrap_message_and_forward_to_network::<_, O1::Inner, O2::Inner, C3, _>(
                        msg,
                        network,
                        user_id_mapping,
                        my_user_id,
                        identifier_info,
                        |m| MultiplexedChannelMessage::Channel1(m.inner_message()),
                        logger,
                    )
                    .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }
        };

        let task1 = async move {
            while let Some(msg) = rx_to_outbound_2.next().await {
                if let Err(err) =
                    wrap_message_and_forward_to_network::<_, O1::Inner, O2::Inner, C3, _>(
                        msg,
                        network,
                        user_id_mapping,
                        my_user_id,
                        identifier_info,
                        |m| MultiplexedChannelMessage::Channel2(m.inner_message()),
                        logger,
                    )
                    .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }
        };

        let task2 = async move {
            while let Some(msg) = rx_to_outbound_3.next().await {
                if let Err(err) =
                    wrap_message_and_forward_to_network::<_, O1::Inner, O2::Inner, C3, _>(
                        msg,
                        network,
                        user_id_mapping,
                        my_user_id,
                        identifier_info,
                        |m| MultiplexedChannelMessage::Channel3(m),
                        logger,
                    )
                    .await
                {
                    logger.error(format!("Failed to send message to outbound: {err:?}"));
                }
            }
        };

        gadget_io::tokio::join!(task0, task1, task2);
    });

    (
        tx_to_outbound_1,
        rx_for_async_proto_1,
        tx_to_outbound_2,
        rx_for_async_proto_2,
        tx_to_outbound_3,
        rx_for_async_proto_3,
    )
}

#[allow(clippy::too_many_arguments)]
#[cfg(feature = "std")]
async fn wrap_message_and_forward_to_network<
    N: Network,
    C1: Serialize,
    C2: Serialize,
    C3: Serialize,
    M,
>(
    msg: M,
    network: &N,
    user_id_mapping: &HashMap<u16, ecdsa::Public>,
    my_user_id: u16,
    identifier_info: IdentifierInfo,
    splitter: impl FnOnce(M) -> MultiplexedChannelMessage<C1, C2, C3>,
    logger: &Logger,
) -> Result<(), Error>
where
    M: MaybeSenderReceiver + Send + 'static,
{
    let from = msg.maybe_sender();
    let to = msg.maybe_receiver();
    logger.trace(format!("Sending message from {:?} to {:?}", from, to));

    let from_account_id = user_id_mapping
        .get(&from.as_user_id().unwrap_or(my_user_id))
        .cloned();
    let to_account_id = to
        .as_user_id()
        .and_then(|to| user_id_mapping.get(&to).cloned());

    let msg = N::build_protocol_message(
        identifier_info,
        my_user_id,
        to.as_user_id(),
        &splitter(msg),
        from_account_id,
        to_account_id,
    );

    network.send_message(msg).await
}
