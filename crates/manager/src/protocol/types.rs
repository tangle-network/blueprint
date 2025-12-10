/// Common types used across the protocol abstraction layer
use blueprint_client_tangle_evm::TangleEvmEvent;
use blueprint_runner::config::{Protocol, ProtocolSettings};
use serde::{Deserialize, Serialize};

/// The type of protocol
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ProtocolType {
    /// Tangle EVM Network
    TangleEvm,
    /// EigenLayer AVS
    Eigenlayer,
}

impl From<Protocol> for ProtocolType {
    fn from(protocol: Protocol) -> Self {
        match protocol {
            Protocol::TangleEvm => ProtocolType::TangleEvm,
            Protocol::Eigenlayer => ProtocolType::Eigenlayer,
            _ => unreachable!("Protocol not supported"),
        }
    }
}

impl From<ProtocolType> for Protocol {
    fn from(protocol_type: ProtocolType) -> Self {
        match protocol_type {
            ProtocolType::TangleEvm => Protocol::TangleEvm,
            ProtocolType::Eigenlayer => Protocol::Eigenlayer,
        }
    }
}

impl From<&ProtocolSettings> for ProtocolType {
    fn from(settings: &ProtocolSettings) -> Self {
        match settings {
            ProtocolSettings::Eigenlayer(_) => ProtocolType::Eigenlayer,
            _ => ProtocolType::TangleEvm,
        }
    }
}

/// A protocol event containing block/event information
///
/// This is a unified type that can hold events from any protocol.
/// Protocol-specific handlers extract the data they need.
#[derive(Debug, Clone)]
pub enum ProtocolEvent {
    /// A Tangle EVM network event
    TangleEvm(TangleEvmProtocolEvent),
    /// An EigenLayer event (new tasks, responses, etc.)
    Eigenlayer(EigenlayerProtocolEvent),
}

/// Tangle EVM-specific event data
#[derive(Debug, Clone)]
pub struct TangleEvmProtocolEvent {
    /// Block number
    pub block_number: u64,
    /// Block hash
    pub block_hash: alloy_primitives::B256,
    /// Timestamp
    pub timestamp: u64,
    /// Logs in the block
    pub logs: Vec<alloy_rpc_types::Log>,
    /// Full event details
    pub inner: TangleEvmEvent,
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
    /// Extract Tangle EVM event data if this is a Tangle EVM event
    #[must_use]
    pub fn as_tangle_evm(&self) -> Option<&TangleEvmProtocolEvent> {
        match self {
            ProtocolEvent::TangleEvm(evt) => Some(evt),
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
            ProtocolEvent::TangleEvm(evt) => evt.block_number,
            ProtocolEvent::Eigenlayer(evt) => evt.block_number,
        }
    }
}
