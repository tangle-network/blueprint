/// Common types used across the protocol abstraction layer
use blueprint_runner::config::{Protocol, ProtocolSettings};
use serde::{Deserialize, Serialize};

/// The type of protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtocolType {
    /// Tangle Network
    Tangle,
    /// EigenLayer AVS
    Eigenlayer,
}

impl From<Protocol> for ProtocolType {
    fn from(protocol: Protocol) -> Self {
        #[allow(unreachable_patterns)]
        match protocol {
            Protocol::Tangle => ProtocolType::Tangle,
            Protocol::Eigenlayer => ProtocolType::Eigenlayer,
            _ => unreachable!("Protocol not supported"),
        }
    }
}

impl From<ProtocolType> for Protocol {
    fn from(protocol_type: ProtocolType) -> Self {
        match protocol_type {
            ProtocolType::Tangle => Protocol::Tangle,
            ProtocolType::Eigenlayer => Protocol::Eigenlayer,
        }
    }
}

impl From<&ProtocolSettings> for ProtocolType {
    fn from(settings: &ProtocolSettings) -> Self {
        match settings {
            ProtocolSettings::Eigenlayer(_) => ProtocolType::Eigenlayer,
            ProtocolSettings::Tangle(_) | ProtocolSettings::None => ProtocolType::Tangle, // Default to Tangle
        }
    }
}

/// A protocol event containing block/event information
///
/// This is a unified type that can hold events from any protocol.
/// Protocol-specific handlers extract the data they need.
#[derive(Debug, Clone)]
pub enum ProtocolEvent {
    /// A Tangle network event (finality notification)
    Tangle(TangleProtocolEvent),
    /// An EigenLayer event (new tasks, responses, etc.)
    Eigenlayer(EigenlayerProtocolEvent),
}

/// Tangle-specific event data
#[derive(Debug, Clone)]
pub struct TangleProtocolEvent {
    /// Block number
    pub block_number: u64,
    /// Block hash
    pub block_hash: [u8; 32],
    /// The raw event (contains substrate events)
    pub inner: blueprint_clients::tangle::client::TangleEvent,
}

/// EigenLayer-specific event data
#[derive(Debug, Clone)]
pub struct EigenlayerProtocolEvent {
    /// Block number
    pub block_number: u64,
    /// Block hash
    pub block_hash: Vec<u8>,
    /// EVM logs for this block
    pub logs: Vec<alloy_rpc_types::Log>,
}

impl ProtocolEvent {
    /// Extract Tangle event data if this is a Tangle event
    #[must_use]
    pub fn as_tangle(&self) -> Option<&TangleProtocolEvent> {
        match self {
            ProtocolEvent::Tangle(evt) => Some(evt),
            _ => None,
        }
    }

    /// Extract EigenLayer event data if this is an EigenLayer event
    #[must_use]
    pub fn as_eigenlayer(&self) -> Option<&EigenlayerProtocolEvent> {
        match self {
            ProtocolEvent::Eigenlayer(evt) => Some(evt),
            _ => None,
        }
    }

    /// Get the block number for any protocol event
    #[must_use]
    pub fn block_number(&self) -> u64 {
        match self {
            ProtocolEvent::Tangle(evt) => evt.block_number,
            ProtocolEvent::Eigenlayer(evt) => evt.block_number,
        }
    }
}
