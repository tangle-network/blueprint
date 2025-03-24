//! Protocol definitions for the RFQ (Request for Quote) system
//!
//! This module defines the protocol constants and utilities for the RFQ system.

/// RFQ protocol name
pub const RFQ_PROTOCOL_NAME: &str = "tangle/cloud/rfq";

/// RFQ protocol version
pub const RFQ_PROTOCOL_VERSION: u16 = 1;

/// Topic name for RFQ gossip messages
pub const RFQ_TOPIC_NAME: &str = "tangle.cloud.rfq.v1";

/// Time-to-live for RFQ requests (in seconds)
pub const DEFAULT_RFQ_REQUEST_TTL: u64 = 60;

/// Time-to-live for price quotes (in seconds)
pub const DEFAULT_QUOTE_TTL: u64 = 300; // 5 minutes

/// Default timeout for waiting for quotes (in seconds)
pub const DEFAULT_QUOTE_COLLECTION_TIMEOUT: u64 = 10;

/// Maximum size of an RFQ message
pub const MAX_RFQ_MESSAGE_SIZE: usize = 1024 * 1024; // 1MB

/// Creates full protocol name with version
pub fn full_protocol_name() -> String {
    format!("{}/{}", RFQ_PROTOCOL_NAME, RFQ_PROTOCOL_VERSION)
}
