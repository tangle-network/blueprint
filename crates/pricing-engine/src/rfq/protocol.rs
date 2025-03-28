//! Protocol definitions for the RFQ (Request for Quote) system
//!
//! This module defines the protocol constants and utilities for the RFQ system.
//! The RFQ protocol allows clients to request price quotes from service operators
//! for specific blueprints and resource requirements. It uses a combination of
//! gossip messaging (for broadcasts) and direct P2P communication (for responses).

/// RFQ protocol name used for identification in the network layer
pub const RFQ_PROTOCOL_NAME: &str = "tangle/cloud/rfq";

/// RFQ protocol version for compatibility checking
/// Increment this when making breaking changes to the protocol
pub const RFQ_PROTOCOL_VERSION: u16 = 1;

/// Topic name for RFQ gossip messages
/// Used for subscribing to RFQ broadcasts via the gossip protocol
pub const RFQ_TOPIC_NAME: &str = "tangle.cloud.rfq.v1";

/// Time-to-live for RFQ requests (in seconds)
/// Requests older than this are considered expired and will not be processed
pub const DEFAULT_RFQ_REQUEST_TTL: u64 = 60;

/// Time-to-live for price quotes (in seconds)
/// Quotes older than this are considered expired and should not be used
pub const DEFAULT_QUOTE_TTL: u64 = 300; // 5 minutes

/// Default timeout for waiting for quotes (in seconds)
/// After this time, the client will return whatever quotes have been collected
pub const DEFAULT_QUOTE_COLLECTION_TIMEOUT: u64 = 10;

/// Maximum size of an RFQ message
/// Messages larger than this will be rejected to prevent DoS attacks
pub const MAX_RFQ_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

/// Creates the full protocol name with version
///
/// # Returns
/// A string in the format "{PROTOCOL_NAME}/{VERSION}"
pub fn full_protocol_name() -> String {
    format!("{}/{}", RFQ_PROTOCOL_NAME, RFQ_PROTOCOL_VERSION)
}

/// Checks if a protocol name and version matches our supported protocol
///
/// # Arguments
/// * `protocol_name` - The protocol name to check
/// * `version` - The protocol version to check
///
/// # Returns
/// `true` if the protocol name and version are supported
pub fn is_supported_protocol(protocol_name: &str, version: u16) -> bool {
    protocol_name == RFQ_PROTOCOL_NAME && version <= RFQ_PROTOCOL_VERSION
}

/// Checks if a message size is within the allowed limits
///
/// # Arguments
/// * `size` - Size of the message in bytes
///
/// # Returns
/// `true` if the message size is within limits
pub fn is_valid_message_size(size: usize) -> bool {
    size <= MAX_RFQ_MESSAGE_SIZE
}
