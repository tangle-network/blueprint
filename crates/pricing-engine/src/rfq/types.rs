//! Type definitions for the RFQ (Request for Quote) module
//!
//! This module defines the core data types used for the request-for-quote system.

use crate::models::PricingModel;
use crate::types::{Price, ResourceRequirement, TimePeriod};
use blueprint_crypto::KeyType;
use blueprint_crypto::hashing::blake3_256;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_core::crypto::Pair;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use thiserror::Error;
use uuid::Uuid;

/// Result type for RFQ operations
pub type RfqResult<T> = Result<T, RfqError>;

/// Error types for the RFQ module
#[derive(Debug, Error)]
pub enum RfqError {
    /// Error serializing or deserializing messages
    #[error("Serialization error: {0}")]
    Serialization(#[from] bincode::Error),

    /// Error with networking operations
    #[error("Network error: {0}")]
    Network(String),

    /// Error with signature operations
    #[error("Signature error: {0}")]
    Signature(String),

    /// Error with quote generation
    #[error("Quote generation error: {0}")]
    QuoteGeneration(String),

    /// Error with timing or scheduling
    #[error("Timing error: {0}")]
    Timing(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// Quote expired
    #[error("Quote expired")]
    QuoteExpired,

    /// Timeout waiting for quotes
    #[error("Timeout waiting for quotes")]
    Timeout,

    /// Other error
    #[error("{0}")]
    Other(String),
}

/// Unique identifier for a quote request
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode, TypeInfo,
)]
pub struct QuoteRequestId(pub [u8; 16]);

impl QuoteRequestId {
    /// Generate a new random request ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().into_bytes())
    }

    /// Get the ID as a string
    pub fn to_string(&self) -> String {
        Uuid::from_bytes(self.0).to_string()
    }
}

impl Default for QuoteRequestId {
    fn default() -> Self {
        Self::new()
    }
}

/// A request for price quotes from service operators
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct QuoteRequest {
    /// ID of the request
    pub id: QuoteRequestId,
    /// Public key of the requester
    pub requester_id: Vec<u8>,
    /// Blueprint ID for which the request is made
    pub blueprint_id: String,
    /// Resource requirements
    pub requirements: Vec<ResourceRequirement>,
    /// Optional maximum price willing to pay
    pub max_price: Option<Price>,
    /// When the request was created
    pub created_at: u64,
    /// When the request expires
    pub expires_at: u64,
}

impl QuoteRequest {
    /// Create a new quote request
    pub fn new(
        requester_id: Vec<u8>,
        blueprint_id: impl Into<String>,
        requirements: Vec<ResourceRequirement>,
        max_price: Option<Price>,
        ttl: Duration,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let expires_at = now + ttl.as_secs();
        let id = QuoteRequestId::new();

        Self {
            id,
            requester_id,
            blueprint_id: blueprint_id.into(),
            requirements,
            max_price,
            created_at: now,
            expires_at,
        }
    }

    /// Check if the request is expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.expires_at < now
    }
}

/// Filters for limiting which operators should respond to an RFQ
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct RequestFilters {
    /// Maximum price the requester is willing to pay
    pub max_price: Option<Price>,

    /// Specific regions the requester is interested in
    pub regions: Option<Vec<String>>,

    /// Specific providers to target
    pub providers: Option<Vec<Vec<u8>>>,

    /// Minimum reputation score for providers
    pub min_reputation: Option<u32>,
}

/// A price quote from an operator
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode, TypeInfo)]
pub struct PriceQuote {
    /// Request ID this quote is responding to
    pub request_id: QuoteRequestId,

    /// Provider ID offering this quote
    pub provider_id: Vec<u8>,

    /// Provider's name
    pub provider_name: String,

    /// The quoted price
    pub price: Price,

    /// Billing period for this price
    pub billing_period: Option<TimePeriod>,

    /// Timestamp when this quote was created
    pub timestamp: u64,

    /// When this quote expires
    pub expires_at: u64,

    /// Pricing model used for the quote
    pub model_id: String,

    /// Additional terms or information about the quote
    pub additional_info: Option<String>,
}

impl PriceQuote {
    /// Create a new price quote
    pub fn new(
        request_id: QuoteRequestId,
        provider_id: Vec<u8>,
        provider_name: String,
        price: Price,
        model_id: String,
        billing_period: Option<TimePeriod>,
        ttl: Duration,
    ) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            request_id,
            provider_id,
            provider_name,
            price,
            billing_period,
            timestamp: now,
            expires_at: now + ttl.as_secs(),
            model_id,
            additional_info: None,
        }
    }

    /// Check if this quote has expired
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.expires_at < now
    }

    /// Add additional information to the quote
    pub fn with_additional_info(mut self, info: impl Into<String>) -> Self {
        self.additional_info = Some(info.into());
        self
    }
}

/// A signed price quote that can be verified and used on-chain
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignedPriceQuote<K: KeyType> {
    /// The price quote
    pub quote: PriceQuote,

    /// The provider's signature
    pub signature: K::Signature,
}

impl<K: KeyType> SignedPriceQuote<K> {
    /// Create a new signed price quote
    pub fn new(quote: PriceQuote, key_pair: &K::Secret) -> RfqResult<Self>
    where
        K::Secret: Pair<Public = K::Public, Signature = K::Signature>,
    {
        // Serialize the quote to bytes for signing
        let quote_bytes = bincode::serialize(&quote)?;

        // Hash the quote
        let quote_hash = blake3_256(&quote_bytes);

        // Sign the quote
        let signature = key_pair.sign(&quote_hash);

        Ok(Self { quote, signature })
    }

    /// Verify the signature on this quote
    pub fn verify(&self, public_key: &K::Public) -> RfqResult<bool>
    where
        K::Signature: AsRef<[u8]>,
    {
        // Serialize the quote to bytes for verification
        let quote_bytes = bincode::serialize(&self.quote)?;

        // Hash the quote
        let quote_hash = blake3_256(&quote_bytes);

        // Verify using the public key
        let verified = K::verify(public_key, &quote_hash, &self.signature);

        Ok(verified)
    }
}

/// Response containing multiple price quotes
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "K: KeyType")]
pub struct PriceQuoteResponse<K: KeyType> {
    /// Request ID these quotes are responding to
    pub request_id: QuoteRequestId,

    /// List of signed quotes
    pub quotes: Vec<SignedPriceQuote<K>>,

    /// Timestamp when this response was created
    pub timestamp: u64,
}

/// Message types for the RFQ protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RfqMessageType {
    /// Request for quotes
    QuoteRequest(QuoteRequest),

    /// Quote response (encoded to hide from others)
    QuoteResponse(Vec<u8>),

    /// Cancellation of a previous request
    CancelRequest(QuoteRequestId),
}

/// Complete RFQ message with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RfqMessage {
    /// Protocol version
    pub version: u16,

    /// Message type
    pub message_type: RfqMessageType,

    /// Timestamp for this message
    pub timestamp: u64,
}

impl RfqMessage {
    /// Create a new RFQ message
    pub fn new(message_type: RfqMessageType) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            version: 1,
            message_type,
            timestamp: now,
        }
    }
}
