//! Type definitions for the RFQ (Request for Quote) module
//!
//! This module defines the core data types used for the request-for-quote system.
//! These types support:
//! - Secure, cryptographically verifiable price quotes
//! - Efficient serialization using SCALE codec
//! - Automatic expiry of requests and quotes
//! - On-chain compatibility for quotes

use crate::types::{Price, ResourceRequirement, TimePeriod};
use blueprint_crypto::KeyType;
use blueprint_crypto::hashing::blake3_256;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
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
///
/// This is a UUID used to correlate requests and responses, and to
/// identify duplicate requests in the network.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Encode, Decode, TypeInfo,
)]
pub struct QuoteRequestId(pub [u8; 16]);

impl QuoteRequestId {
    /// Generate a new random request ID
    ///
    /// # Returns
    /// A random UUID-based identifier
    pub fn new() -> Self {
        Self(Uuid::new_v4().into_bytes())
    }

    /// Get the ID as a string
    ///
    /// # Returns
    /// The UUID as a string in standard format
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
///
/// This is broadcast via gossip protocol to all potential operators,
/// who can then respond with quotes if they support the requested blueprint.
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
#[serde(bound = "K: KeyType")]
pub struct QuoteRequest<K: KeyType> {
    /// ID of the request
    pub id: QuoteRequestId,

    /// Public key of the requester
    /// Used for operator verification and response targeting
    pub requester_id: K::Public,

    /// Blueprint ID for which the request is made
    /// Operators only respond if they support this blueprint
    pub blueprint_id: String,

    /// Resource requirements for the service
    /// Used by operators to calculate prices
    pub requirements: Vec<ResourceRequirement>,

    /// Optional maximum price willing to pay
    /// If set, operators should not respond if their price exceeds this
    pub max_price: Option<Price>,

    /// When the request was created (Unix timestamp in seconds)
    pub created_at: u64,

    /// When the request expires (Unix timestamp in seconds)
    pub expires_at: u64,
}

impl<K: KeyType> QuoteRequest<K> {
    /// Create a new quote request
    ///
    /// # Arguments
    /// * `requester_id` - Public key bytes of the requester
    /// * `blueprint_id` - ID of the blueprint being requested
    /// * `requirements` - Resource requirements for the service
    /// * `max_price` - Optional maximum price the requester will pay
    /// * `ttl` - Time-to-live for this request
    ///
    /// # Returns
    /// A new quote request with a random ID and current timestamp
    pub fn new(
        requester_id: K::Public,
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
    ///
    /// # Returns
    /// `true` if the current time is past the expiration time
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.expires_at < now
    }
}

/// A price quote from an operator
///
/// This quote is generated in response to a quote request and contains
/// the price and terms offered by the operator.
#[derive(Debug, Clone, Serialize, Deserialize, Encode, Decode, TypeInfo)]
#[serde(bound = "K: KeyType")]
pub struct PriceQuote<K: KeyType> {
    /// Request ID this quote is responding to
    pub request_id: QuoteRequestId,

    /// Provider ID offering this quote
    /// This is typically the public key or peer ID of the operator
    pub provider_id: K::Public,

    /// Provider's name
    pub provider_name: String,

    /// The quoted price
    pub price: Price,

    /// Billing period for this price
    pub billing_period: Option<TimePeriod>,

    /// Timestamp when this quote was created (Unix timestamp in seconds)
    pub timestamp: u64,

    /// When this quote expires (Unix timestamp in seconds)
    pub expires_at: u64,

    /// Pricing model used for the quote
    pub model_id: String,

    /// Additional terms or information about the quote
    pub additional_info: Option<String>,
}

impl<K: KeyType> PriceQuote<K> {
    /// Create a new price quote
    ///
    /// # Arguments
    /// * `request_id` - ID of the request this quote is responding to
    /// * `provider_id` - ID of the provider giving this quote
    /// * `provider_name` - Name of the provider
    /// * `price` - The quoted price
    /// * `model_id` - ID of the pricing model used
    /// * `billing_period` - Optional billing period for the service
    /// * `ttl` - Time-to-live for this quote
    ///
    /// # Returns
    /// A new price quote with current timestamp and calculated expiration
    pub fn new(
        request_id: QuoteRequestId,
        provider_id: K::Public,
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
    ///
    /// # Returns
    /// `true` if the current time is past the expiration time
    pub fn is_expired(&self) -> bool {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        self.expires_at < now
    }

    /// Add additional information to the quote
    ///
    /// # Arguments
    /// * `info` - Additional information to include with the quote
    ///
    /// # Returns
    /// The quote with additional information added
    pub fn with_additional_info(mut self, info: impl Into<String>) -> Self {
        self.additional_info = Some(info.into());
        self
    }
}

/// A signed price quote that can be verified and used on-chain
///
/// This wraps a price quote with a cryptographic signature from the
/// provider, allowing it to be verified on-chain or by other parties.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "K: KeyType")]
pub struct SignedPriceQuote<K: KeyType> {
    /// The price quote being signed
    pub quote: PriceQuote<K>,

    /// The provider's signature for the quote
    pub signature: K::Signature,
}

impl<K: KeyType> SignedPriceQuote<K> {
    /// Create a new signed price quote
    ///
    /// # Arguments
    /// * `quote` - The price quote to sign
    /// * `key_pair` - The keypair to sign with
    ///
    /// # Returns
    /// A signed price quote that can be verified
    ///
    /// # Errors
    /// Returns an error if serialization or signing fails
    pub fn new(quote: PriceQuote<K>, key_pair: &K::Secret) -> RfqResult<Self> {
        // Serialize the quote to bytes for signing
        let quote_bytes = bincode::serialize(&quote)?;

        // Hash the quote to create a message digest
        let quote_hash = blake3_256(&quote_bytes);

        // Sign the quote hash with the provider's key
        let signature = K::sign_with_secret(&mut key_pair.clone(), &quote_hash)
            .map_err(|e| RfqError::Signature(format!("{:?}", e)))?;

        Ok(Self { quote, signature })
    }

    /// Verify the signature on this quote
    ///
    /// # Arguments
    /// * `public_key` - The public key to verify against
    ///
    /// # Returns
    /// `true` if the signature is valid for this quote
    ///
    /// # Errors
    /// Returns an error if serialization fails
    pub fn verify(&self, public_key: &K::Public) -> RfqResult<bool>
    where
        K::Signature: AsRef<[u8]>,
    {
        // Serialize the quote to bytes for verification
        let quote_bytes = bincode::serialize(&self.quote)?;

        // Hash the quote to create a message digest
        let quote_hash = blake3_256(&quote_bytes);

        // Verify using the public key
        let verified = K::verify(public_key, &quote_hash, &self.signature);

        Ok(verified)
    }
}

/// Response containing multiple price quotes
///
/// Used to send one or more signed quotes back to the requester.
/// This is sent as a direct P2P message, not via gossip.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "K: KeyType")]
pub struct PriceQuoteResponse<K: KeyType> {
    /// Request ID these quotes are responding to
    pub request_id: QuoteRequestId,

    /// List of signed quotes from this provider
    pub quotes: Vec<SignedPriceQuote<K>>,

    /// Timestamp when this response was created (Unix timestamp in seconds)
    pub timestamp: u64,
}

/// Message types for the RFQ protocol
///
/// These represent the different types of messages that can be sent
/// through the RFQ network protocol.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "K: KeyType")]
pub enum RfqMessageType<K: KeyType> {
    /// Request for quotes (broadcast via gossip)
    QuoteRequest(QuoteRequest<K>),

    /// Quote response (direct P2P message to requester)
    /// The bytes are serialized PriceQuoteResponse data
    QuoteResponse(Vec<u8>),

    /// Cancellation of a previous request (broadcast via gossip)
    CancelRequest(QuoteRequestId),
}

/// Complete RFQ message with metadata
///
/// This is the top-level message format for the RFQ protocol,
/// including protocol version and timestamp information.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "K: KeyType")]
pub struct RfqMessage<K: KeyType> {
    /// Protocol version
    pub version: u16,

    /// Message timestamp (Unix timestamp in seconds)
    pub timestamp: u64,

    /// Message type and contents
    pub message_type: RfqMessageType<K>,
}

impl<K: KeyType> RfqMessage<K> {
    /// Create a new RFQ message
    ///
    /// # Arguments
    /// * `message_type` - The type of message and its contents
    ///
    /// # Returns
    /// A new RFQ message with the current timestamp
    pub fn new(message_type: RfqMessageType<K>) -> Self {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        Self {
            version: super::protocol::RFQ_PROTOCOL_VERSION,
            timestamp: now,
            message_type,
        }
    }

    /// Get a human-readable name for the message type
    ///
    /// This is useful for logging and debugging.
    ///
    /// # Returns
    /// A string representation of the message type
    pub fn message_type_name(&self) -> &'static str {
        match self.message_type {
            RfqMessageType::QuoteRequest(_) => "QuoteRequest",
            RfqMessageType::QuoteResponse(_) => "QuoteResponse",
            RfqMessageType::CancelRequest(_) => "CancelRequest",
        }
    }

    /// Get detailed information about the message for debugging
    ///
    /// This provides more context about the message contents, such as
    /// request IDs, blueprint IDs, or the number of quotes in a response.
    ///
    /// # Returns
    /// A string with detailed message information
    pub fn debug_info(&self) -> String {
        match &self.message_type {
            RfqMessageType::QuoteRequest(req) => {
                format!(
                    "QuoteRequest(id: {}, blueprint: {}, reqs: {})",
                    req.id.to_string(),
                    req.blueprint_id,
                    req.requirements.len()
                )
            }
            RfqMessageType::QuoteResponse(bytes) => {
                match bincode::deserialize::<PriceQuoteResponse<K>>(bytes) {
                    Ok(resp) => format!(
                        "QuoteResponse(id: {}, quotes: {})",
                        resp.request_id.to_string(),
                        resp.quotes.len()
                    ),
                    Err(_) => "QuoteResponse(failed to deserialize)".to_string(),
                }
            }
            RfqMessageType::CancelRequest(id) => {
                format!("CancelRequest(id: {})", id.to_string())
            }
        }
    }
}
