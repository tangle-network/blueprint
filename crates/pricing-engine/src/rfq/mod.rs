//! Request for Quote (RFQ) implementation for the Tangle Cloud Pricing Engine
//!
//! This module implements a Request for Quote (RFQ) system for the pricing engine,
//! allowing clients to request pricing information from multiple operators
//! simultaneously using the existing networking infrastructure.

mod processor;
mod protocol;
mod types;

pub use processor::{RfqProcessor, RfqProcessorConfig};
pub use protocol::{RFQ_PROTOCOL_NAME, RFQ_PROTOCOL_VERSION, RFQ_TOPIC_NAME};
pub use types::{
    PriceQuote, PriceQuoteResponse, QuoteRequest, QuoteRequestId, RfqError, RfqMessage,
    RfqMessageType, RfqResult, SignedPriceQuote,
};
