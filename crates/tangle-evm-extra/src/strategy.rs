//! Aggregation Strategy for BLS Signature Aggregation
//!
//! This module provides a strategy pattern for choosing between different
//! signature aggregation methods:
//! - HTTP Service: Uses a centralized aggregation service (simpler deployment)
//! - P2P Gossip: Uses peer-to-peer gossip protocol (fully decentralized)
//!
//! ## Threshold Types
//!
//! The aggregation system supports two threshold types matching the on-chain configuration:
//! - `CountBased`: Threshold is based on the number of operators (e.g., 67% of operators must sign)
//! - `StakeWeighted`: Threshold is based on operator stake exposure (e.g., 67% of total stake must sign)
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_evm_extra::strategy::{AggregationStrategy, HttpServiceConfig};
//!
//! // Use HTTP service (recommended for most cases)
//! let strategy = AggregationStrategy::HttpService(HttpServiceConfig::new(
//!     "http://localhost:8080",
//!     bls_secret,
//!     operator_index,
//! ));
//!
//! // Or use P2P gossip (for fully decentralized setups)
//! #[cfg(feature = "p2p-aggregation")]
//! let strategy = AggregationStrategy::P2PGossip(P2PGossipConfig::new(
//!     network_handle,
//!     bls_secret,
//!     participant_keys,
//! ));
//! ```

use alloy_primitives::Bytes;
use blueprint_std::string::String;
#[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
use blueprint_std::time::Duration;
use blueprint_std::vec::Vec;

/// Threshold type for BLS aggregation (matches on-chain Types.ThresholdType)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThresholdType {
    /// Percentage of operator count (each operator has weight 1)
    #[default]
    CountBased,
    /// Percentage of total stake/exposure (operators weighted by their stake exposure)
    StakeWeighted,
}

impl From<u8> for ThresholdType {
    fn from(value: u8) -> Self {
        match value {
            1 => ThresholdType::StakeWeighted,
            _ => ThresholdType::CountBased,
        }
    }
}

#[cfg(feature = "aggregation")]
use blueprint_std::sync::Arc;

#[cfg(feature = "aggregation")]
use blueprint_crypto_bn254::{ArkBlsBn254, ArkBlsBn254Public, ArkBlsBn254Secret};

#[cfg(all(feature = "p2p-aggregation", not(feature = "aggregation")))]
use blueprint_crypto_bn254::{ArkBlsBn254, ArkBlsBn254Public};

#[cfg(feature = "p2p-aggregation")]
use blueprint_std::collections::HashMap;

/// Strategy for how to aggregate BLS signatures
///
/// Blueprint developers can choose between:
/// - `HttpService`: Uses a centralized aggregation service (recommended)
/// - `P2PGossip`: Uses peer-to-peer gossip protocol (advanced)
#[derive(Clone)]
pub enum AggregationStrategy {
    /// Use an HTTP aggregation service (recommended)
    ///
    /// This is simpler to deploy and more reliable. Any operator can run
    /// the aggregation service, making it semi-decentralized.
    #[cfg(feature = "aggregation")]
    HttpService(HttpServiceConfig),

    /// Use peer-to-peer gossip protocol
    ///
    /// This is fully decentralized but requires P2P connectivity between
    /// operators. More complex to set up and debug.
    #[cfg(feature = "p2p-aggregation")]
    P2PGossip(P2PGossipConfig),
}

/// Configuration for HTTP-based aggregation service
#[cfg(feature = "aggregation")]
#[derive(Clone)]
pub struct HttpServiceConfig {
    /// HTTP client for the aggregation service
    pub client: blueprint_tangle_aggregation_svc::AggregationServiceClient,
    /// BLS secret key for signing
    pub bls_secret: Arc<ArkBlsBn254Secret>,
    /// BLS public key (derived from secret)
    pub bls_public: Arc<ArkBlsBn254Public>,
    /// Operator index in the service
    pub operator_index: u32,
    /// Whether to wait for threshold to be met before returning
    pub wait_for_threshold: bool,
    /// Timeout for waiting for threshold
    pub threshold_timeout: Duration,
    /// Poll interval when waiting for threshold
    pub poll_interval: Duration,
}

#[cfg(feature = "aggregation")]
impl HttpServiceConfig {
    /// Create a new HTTP service config
    pub fn new(
        service_url: impl Into<String>,
        bls_secret: ArkBlsBn254Secret,
        operator_index: u32,
    ) -> Self {
        use blueprint_crypto_core::KeyType;

        let bls_public = ArkBlsBn254::public_from_secret(&bls_secret);
        Self {
            client: blueprint_tangle_aggregation_svc::AggregationServiceClient::new(service_url),
            bls_secret: Arc::new(bls_secret),
            bls_public: Arc::new(bls_public),
            operator_index,
            wait_for_threshold: false,
            threshold_timeout: Duration::from_secs(60),
            poll_interval: Duration::from_secs(1),
        }
    }

    /// Set whether to wait for threshold to be met
    #[must_use]
    pub fn with_wait_for_threshold(mut self, wait: bool) -> Self {
        self.wait_for_threshold = wait;
        self
    }

    /// Set the timeout for waiting for threshold
    #[must_use]
    pub fn with_threshold_timeout(mut self, timeout: Duration) -> Self {
        self.threshold_timeout = timeout;
        self
    }

    /// Set the poll interval when waiting for threshold
    #[must_use]
    pub fn with_poll_interval(mut self, interval: Duration) -> Self {
        self.poll_interval = interval;
        self
    }
}

/// Configuration for P2P gossip-based aggregation
#[cfg(feature = "p2p-aggregation")]
#[derive(Clone)]
pub struct P2PGossipConfig {
    /// Network service handle for P2P communication
    pub network_handle: blueprint_networking::service_handle::NetworkServiceHandle<ArkBlsBn254>,
    /// Number of aggregators to select
    pub num_aggregators: u16,
    /// Timeout for the aggregation protocol
    pub timeout: Duration,
    /// Threshold in basis points (e.g., 6700 for 67%)
    /// This matches the on-chain thresholdBps format
    pub threshold_bps: u16,
    /// Threshold type (CountBased or StakeWeighted)
    pub threshold_type: ThresholdType,
    /// Map of participant peer IDs to their public keys
    pub participant_public_keys: HashMap<libp2p::PeerId, ArkBlsBn254Public>,
    /// Map of participant peer IDs to their stake weights (exposure in basis points)
    /// Only used when threshold_type is StakeWeighted
    /// Values are in basis points (10000 = 100%)
    pub operator_weights: HashMap<libp2p::PeerId, u64>,
}

#[cfg(feature = "p2p-aggregation")]
impl P2PGossipConfig {
    /// Create a new P2P gossip config with default count-based threshold (67%)
    pub fn new(
        network_handle: blueprint_networking::service_handle::NetworkServiceHandle<ArkBlsBn254>,
        participant_public_keys: HashMap<libp2p::PeerId, ArkBlsBn254Public>,
    ) -> Self {
        Self {
            network_handle,
            num_aggregators: 2,
            timeout: Duration::from_secs(30),
            threshold_bps: 6700, // 67% in basis points
            threshold_type: ThresholdType::CountBased,
            participant_public_keys,
            operator_weights: HashMap::new(),
        }
    }

    /// Set the number of aggregators
    #[must_use]
    pub fn with_num_aggregators(mut self, num: u16) -> Self {
        self.num_aggregators = num;
        self
    }

    /// Set the protocol timeout
    #[must_use]
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set the threshold percentage (convenience method that converts to basis points)
    #[must_use]
    pub fn with_threshold_percentage(mut self, percentage: u8) -> Self {
        self.threshold_bps = u16::from(percentage) * 100;
        self
    }

    /// Set the threshold in basis points (e.g., 6700 for 67%)
    /// This matches the on-chain thresholdBps format directly
    #[must_use]
    pub fn with_threshold_bps(mut self, bps: u16) -> Self {
        self.threshold_bps = bps;
        self
    }

    /// Set the threshold type (CountBased or StakeWeighted)
    #[must_use]
    pub fn with_threshold_type(mut self, threshold_type: ThresholdType) -> Self {
        self.threshold_type = threshold_type;
        self
    }

    /// Set the operator weights for stake-weighted thresholds
    ///
    /// The weights should be the operator's exposure in basis points (from ServiceOperator.exposureBps).
    /// This is only used when `threshold_type` is `StakeWeighted`.
    ///
    /// # Example
    ///
    /// ```rust,ignore
    /// // Operator 1 has 5000 bps (50%) exposure, operator 2 has 3000 bps (30%)
    /// let weights = HashMap::from([
    ///     (peer_id_1, 5000),
    ///     (peer_id_2, 3000),
    /// ]);
    /// config.with_operator_weights(weights)
    /// ```
    #[must_use]
    pub fn with_operator_weights(mut self, weights: HashMap<libp2p::PeerId, u64>) -> Self {
        self.operator_weights = weights;
        self
    }

    /// Configure for stake-weighted threshold with operator weights
    ///
    /// This is a convenience method that sets both the threshold type and weights.
    #[must_use]
    pub fn with_stake_weighted_threshold(
        mut self,
        threshold_bps: u16,
        weights: HashMap<libp2p::PeerId, u64>,
    ) -> Self {
        self.threshold_bps = threshold_bps;
        self.threshold_type = ThresholdType::StakeWeighted;
        self.operator_weights = weights;
        self
    }
}

/// Result of a successful aggregation
#[derive(Debug, Clone)]
pub struct AggregatedSignatureResult {
    /// The service ID
    pub service_id: u64,
    /// The call ID
    pub call_id: u64,
    /// The job output
    pub output: Bytes,
    /// Aggregated BLS signature (G1 point, serialized)
    pub aggregated_signature: Vec<u8>,
    /// Aggregated BLS public key (G2 point, serialized)
    pub aggregated_pubkey: Vec<u8>,
    /// Bitmap indicating which operators signed
    pub signer_bitmap: alloy_primitives::U256,
    /// Indices of operators who did not sign
    pub non_signer_indices: Vec<u32>,
}

/// Error type for aggregation strategies
#[derive(Debug, thiserror::Error)]
pub enum StrategyError {
    /// HTTP service error
    #[cfg(feature = "aggregation")]
    #[error("HTTP service error: {0}")]
    HttpService(#[from] blueprint_tangle_aggregation_svc::ClientError),

    /// P2P protocol error
    #[cfg(feature = "p2p-aggregation")]
    #[error("P2P protocol error: {0}")]
    P2PProtocol(String),

    /// BLS crypto error
    #[error("BLS error: {0}")]
    Bls(String),

    /// No aggregation strategy configured
    #[error(
        "No aggregation strategy configured - enable 'aggregation' or 'p2p-aggregation' feature"
    )]
    NoAggregationStrategy,

    /// Threshold not met
    #[error("Threshold not met: got {got}, need {need}")]
    ThresholdNotMet { got: usize, need: usize },

    /// Timeout
    #[error("Aggregation timed out")]
    Timeout,

    /// Serialization error
    #[error("Serialization error: {0}")]
    Serialization(String),
}

#[cfg(any(feature = "aggregation", feature = "p2p-aggregation"))]
impl AggregationStrategy {
    /// Execute the aggregation strategy for a job result
    ///
    /// This signs the output, coordinates with other operators (via HTTP or P2P),
    /// and returns the aggregated result ready for on-chain submission.
    ///
    /// # Arguments
    ///
    /// * `service_id` - The service ID from the job system
    /// * `call_id` - The call ID for this specific job invocation
    /// * `output` - The job output to sign
    /// * `total_operators` - Total number of operators in the service
    /// * `threshold` - Minimum number of signatures required (from job system)
    pub async fn aggregate(
        &self,
        service_id: u64,
        call_id: u64,
        output: Bytes,
        total_operators: u32,
        threshold: u32,
    ) -> Result<AggregatedSignatureResult, StrategyError> {
        match self {
            #[cfg(feature = "aggregation")]
            AggregationStrategy::HttpService(config) => {
                aggregate_via_http(
                    config,
                    service_id,
                    call_id,
                    output,
                    total_operators,
                    threshold,
                )
                .await
            }
            #[cfg(feature = "p2p-aggregation")]
            AggregationStrategy::P2PGossip(config) => {
                aggregate_via_p2p(
                    config.clone(),
                    service_id,
                    call_id,
                    output,
                    total_operators,
                    threshold,
                )
                .await
            }
            #[allow(unreachable_patterns)]
            _ => Err(StrategyError::NoAggregationStrategy),
        }
    }
}

#[cfg(not(any(feature = "aggregation", feature = "p2p-aggregation")))]
impl AggregationStrategy {
    pub async fn aggregate(
        &self,
        _service_id: u64,
        _call_id: u64,
        _output: Bytes,
        _total_operators: u32,
        _threshold: u32,
    ) -> Result<AggregatedSignatureResult, StrategyError> {
        Err(StrategyError::NoAggregationStrategy)
    }
}

/// Aggregate via HTTP service
#[cfg(feature = "aggregation")]
async fn aggregate_via_http(
    config: &HttpServiceConfig,
    service_id: u64,
    call_id: u64,
    output: Bytes,
    total_operators: u32,
    threshold: u32,
) -> Result<AggregatedSignatureResult, StrategyError> {
    use blueprint_crypto_core::{BytesEncoding, KeyType};
    use blueprint_tangle_aggregation_svc::{
        SubmitSignatureRequest, ThresholdConfig, create_signing_message,
    };

    blueprint_core::debug!(
        target: "aggregation-strategy",
        "Aggregating via HTTP service for service {} call {}",
        service_id, call_id
    );

    // Create the message to sign
    let message = create_signing_message(service_id, call_id, &output);

    // Sign with BLS key
    let mut secret_clone = (*config.bls_secret).clone();
    let signature = ArkBlsBn254::sign_with_secret(&mut secret_clone, &message)
        .map_err(|e| StrategyError::Bls(e.to_string()))?;

    // Get bytes
    let pubkey_bytes = config.bls_public.to_bytes();
    let sig_bytes = signature.to_bytes();

    // Try to initialize the task (may already exist)
    let threshold_config = ThresholdConfig::Count {
        required_signers: threshold.max(1),
    };
    let _ = config
        .client
        .init_task(
            service_id,
            call_id,
            &output,
            total_operators,
            threshold_config,
        )
        .await;

    // Submit our signature
    let submit_request = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: config.operator_index,
        output: output.to_vec(),
        signature: sig_bytes,
        public_key: pubkey_bytes,
    };

    let response = config.client.submit_signature(submit_request).await?;

    blueprint_core::info!(
        target: "aggregation-strategy",
        "Submitted signature: {}/{} (threshold met: {})",
        response.signatures_collected,
        response.threshold_required,
        response.threshold_met
    );

    // Wait for threshold if configured
    let result = if config.wait_for_threshold {
        if response.threshold_met {
            config
                .client
                .get_aggregated(service_id, call_id)
                .await?
                .ok_or_else(|| StrategyError::Bls("Aggregated result not available".into()))?
        } else {
            config
                .client
                .wait_for_threshold(
                    service_id,
                    call_id,
                    config.poll_interval,
                    config.threshold_timeout,
                )
                .await?
        }
    } else if response.threshold_met {
        config
            .client
            .get_aggregated(service_id, call_id)
            .await?
            .ok_or_else(|| StrategyError::Bls("Aggregated result not available".into()))?
    } else {
        // Return early - threshold not met and not waiting
        return Err(StrategyError::ThresholdNotMet {
            got: response.signatures_collected,
            need: response.threshold_required,
        });
    };

    Ok(AggregatedSignatureResult {
        service_id: result.service_id,
        call_id: result.call_id,
        output: Bytes::from(result.output),
        aggregated_signature: result.aggregated_signature,
        aggregated_pubkey: result.aggregated_pubkey,
        signer_bitmap: result.signer_bitmap,
        non_signer_indices: result.non_signer_indices,
    })
}

/// Aggregate via P2P gossip protocol
#[cfg(feature = "p2p-aggregation")]
async fn aggregate_via_p2p(
    config: P2PGossipConfig,
    service_id: u64,
    call_id: u64,
    output: Bytes,
    _total_operators: u32,
    _threshold: u32,
) -> Result<AggregatedSignatureResult, StrategyError> {
    use blueprint_crypto::hashing::blake3_256;
    use blueprint_crypto_core::BytesEncoding;
    use blueprint_networking_agg_sig_gossip_extension::{
        DynamicWeight, ProtocolConfig, SignatureAggregationProtocol,
    };

    blueprint_core::debug!(
        target: "aggregation-strategy",
        "Aggregating via P2P gossip for service {} call {} (threshold_bps: {}, type: {:?})",
        service_id, call_id, config.threshold_bps, config.threshold_type
    );

    // Create the message to sign (same format as HTTP)
    let message = crate::aggregating_consumer::integration::create_signing_message(
        service_id, call_id, &output,
    );

    // Hash the message for the protocol
    let message_hash = blake3_256(&message);

    // Create protocol config
    let protocol_config = ProtocolConfig::new(
        config.network_handle.clone(),
        config.num_aggregators,
        config.timeout,
    );

    // Create weight scheme based on threshold type
    // This matches the on-chain logic in Jobs.sol _validateSignersAndThreshold
    let weight_scheme = match config.threshold_type {
        ThresholdType::CountBased => {
            // CountBased: each operator has weight 1, threshold is percentage of operator count
            let num_participants = config.participant_public_keys.len();
            let threshold_percentage = (config.threshold_bps / 100) as u8;
            blueprint_core::debug!(
                target: "aggregation-strategy",
                "Using CountBased threshold: {}% of {} participants",
                threshold_percentage, num_participants
            );
            DynamicWeight::equal(num_participants, threshold_percentage)
        }
        ThresholdType::StakeWeighted => {
            // StakeWeighted: operators weighted by their stake exposure (exposureBps)
            // threshold_weight = (total_weight * threshold_bps) / 10000
            let total_weight: u64 = config.operator_weights.values().sum();
            let threshold_weight = (total_weight * u64::from(config.threshold_bps)) / 10000;

            blueprint_core::debug!(
                target: "aggregation-strategy",
                "Using StakeWeighted threshold: {} / {} total weight ({}bps)",
                threshold_weight, total_weight, config.threshold_bps
            );

            if config.operator_weights.is_empty() {
                blueprint_core::warn!(
                    target: "aggregation-strategy",
                    "StakeWeighted threshold requested but no operator weights provided, falling back to EqualWeight"
                );
                let num_participants = config.participant_public_keys.len();
                let threshold_percentage = (config.threshold_bps / 100) as u8;
                DynamicWeight::equal(num_participants, threshold_percentage)
            } else {
                DynamicWeight::custom(config.operator_weights.clone(), threshold_weight)
            }
        }
    };

    // Create and run the protocol
    let mut protocol = SignatureAggregationProtocol::new(
        protocol_config,
        weight_scheme,
        config.participant_public_keys.clone(),
    );

    let result = protocol
        .run(&message_hash)
        .await
        .map_err(|e| StrategyError::P2PProtocol(format!("{:?}", e)))?;

    blueprint_core::info!(
        target: "aggregation-strategy",
        "P2P aggregation complete: {} contributors",
        result.contributors.len()
    );

    // Convert the result to our format
    // Note: The P2P protocol returns an AggregatedSignature, we need to serialize it
    let sig_bytes = result.signature.to_bytes();

    // Build signer bitmap from contributors
    let mut signer_bitmap = alloy_primitives::U256::ZERO;
    let mut non_signer_indices = Vec::new();

    // We need to map PeerIds back to operator indices
    // For now, we'll use a simple approach based on sorted order
    let sorted_peers: Vec<_> = config.participant_public_keys.keys().cloned().collect();
    for (idx, peer_id) in sorted_peers.iter().enumerate() {
        if result.contributors.contains(peer_id) {
            signer_bitmap |= alloy_primitives::U256::from(1u64) << idx;
        } else {
            non_signer_indices.push(idx as u32);
        }
    }

    // Aggregate public keys of signers
    let signer_pubkeys: Vec<_> = sorted_peers
        .iter()
        .filter(|p| result.contributors.contains(p))
        .filter_map(|p| config.participant_public_keys.get(p).cloned())
        .collect();

    let aggregated_pubkey = if signer_pubkeys.len() == 1 {
        signer_pubkeys[0].to_bytes()
    } else {
        // Aggregate the public keys
        use blueprint_crypto::aggregation::AggregatableSignature;
        let dummy_sigs: Vec<_> = (0..signer_pubkeys.len())
            .map(|_| result.signature.clone())
            .collect();
        let (_, agg_pk) = ArkBlsBn254::aggregate(&dummy_sigs, &signer_pubkeys)
            .map_err(|e| StrategyError::Bls(format!("Failed to aggregate pubkeys: {:?}", e)))?;
        agg_pk.to_bytes()
    };

    Ok(AggregatedSignatureResult {
        service_id,
        call_id,
        output,
        aggregated_signature: sig_bytes,
        aggregated_pubkey,
        signer_bitmap,
        non_signer_indices,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aggregated_signature_result_debug() {
        let result = AggregatedSignatureResult {
            service_id: 1,
            call_id: 42,
            output: Bytes::from(vec![1, 2, 3]),
            aggregated_signature: vec![4, 5, 6],
            aggregated_pubkey: vec![7, 8, 9],
            signer_bitmap: alloy_primitives::U256::from(7u64),
            non_signer_indices: vec![3],
        };

        // Should be Debug-able
        let debug_str = format!("{:?}", result);
        assert!(debug_str.contains("service_id: 1"));
        assert!(debug_str.contains("call_id: 42"));
    }
}
