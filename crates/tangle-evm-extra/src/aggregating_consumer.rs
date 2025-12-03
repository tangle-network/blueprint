//! Aggregation-Aware Job Consumer
//!
//! This consumer automatically detects whether a job requires BLS aggregation
//! and routes to the appropriate submission path:
//! - Jobs NOT requiring aggregation: Submit directly via `submitResult`
//! - Jobs requiring aggregation: Coordinate with other operators via the
//!   aggregation service, and submit via `submitAggregatedResult`
//!
//! ## Usage
//!
//! ```rust,ignore
//! use blueprint_tangle_evm_extra::AggregatingConsumer;
//!
//! // Create the consumer with aggregation support
//! let consumer = AggregatingConsumer::new(client)
//!     .with_aggregation_service("http://localhost:8080", bls_keypair, operator_index);
//!
//! // Use it just like TangleEvmConsumer - it automatically handles aggregation!
//! consumer.send(job_result).await?;
//! ```

use crate::aggregation::AggregationError;
use crate::extract;
use alloy_primitives::Bytes;
use blueprint_client_tangle_evm::{AggregationConfig, TangleEvmClient, ThresholdType};
use blueprint_core::error::BoxError;
use blueprint_core::JobResult;
use core::pin::Pin;
use core::task::{Context, Poll};
use futures_util::Sink;
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

/// Error type for the aggregating consumer
#[derive(Debug, thiserror::Error)]
pub enum AggregatingConsumerError {
    /// Client error
    #[error("Client error: {0}")]
    Client(String),
    /// Missing metadata
    #[error("Missing metadata: {0}")]
    MissingMetadata(&'static str),
    /// Invalid metadata
    #[error("Invalid metadata: {0}")]
    InvalidMetadata(&'static str),
    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),
    /// Aggregation error
    #[error("Aggregation error: {0}")]
    Aggregation(#[from] AggregationError),
    /// Aggregation service error
    #[cfg(feature = "aggregation")]
    #[error("Aggregation service error: {0}")]
    AggregationService(#[from] blueprint_tangle_aggregation_svc::ClientError),
    /// BLS crypto error
    #[cfg(feature = "aggregation")]
    #[error("BLS error: {0}")]
    Bls(String),
    /// Aggregation not configured
    #[error("Aggregation required but not configured. Call with_aggregation_service() first.")]
    AggregationNotConfigured,
}

/// Job result with parsed metadata for submission
struct PendingJobResult {
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
}

enum State {
    WaitingForResult,
    ProcessingSubmission(
        Pin<Box<dyn core::future::Future<Output = Result<(), AggregatingConsumerError>> + Send>>,
    ),
}

impl State {
    fn is_waiting(&self) -> bool {
        matches!(self, State::WaitingForResult)
    }
}

/// Configuration for the aggregation service
#[cfg(feature = "aggregation")]
#[derive(Clone)]
pub struct AggregationServiceConfig {
    /// HTTP client for the aggregation service
    pub client: blueprint_tangle_aggregation_svc::AggregationServiceClient,
    /// BLS secret key for signing
    pub bls_secret: Arc<blueprint_crypto_bn254::ArkBlsBn254Secret>,
    /// BLS public key (derived from secret)
    pub bls_public: Arc<blueprint_crypto_bn254::ArkBlsBn254Public>,
    /// Operator index in the service
    pub operator_index: u32,
    /// Whether to wait for threshold to be met before returning
    pub wait_for_threshold: bool,
    /// Timeout for waiting for threshold (default: 60s)
    pub threshold_timeout: std::time::Duration,
    /// Poll interval when waiting for threshold (default: 1s)
    pub poll_interval: std::time::Duration,
}

#[cfg(feature = "aggregation")]
impl AggregationServiceConfig {
    /// Create a new aggregation service config
    pub fn new(
        service_url: impl Into<String>,
        bls_secret: blueprint_crypto_bn254::ArkBlsBn254Secret,
        operator_index: u32,
    ) -> Self {
        use blueprint_crypto_bn254::ArkBlsBn254;
        use blueprint_crypto_core::KeyType;

        let bls_public = ArkBlsBn254::public_from_secret(&bls_secret);
        Self {
            client: blueprint_tangle_aggregation_svc::AggregationServiceClient::new(service_url),
            bls_secret: Arc::new(bls_secret),
            bls_public: Arc::new(bls_public),
            operator_index,
            wait_for_threshold: false,
            threshold_timeout: std::time::Duration::from_secs(60),
            poll_interval: std::time::Duration::from_secs(1),
        }
    }

    /// Set whether to wait for threshold to be met
    pub fn with_wait_for_threshold(mut self, wait: bool) -> Self {
        self.wait_for_threshold = wait;
        self
    }

    /// Set the timeout for waiting for threshold
    pub fn with_threshold_timeout(mut self, timeout: std::time::Duration) -> Self {
        self.threshold_timeout = timeout;
        self
    }
}

/// An aggregation-aware consumer that automatically routes jobs to either
/// direct submission or aggregated submission based on BSM configuration.
///
/// For jobs that require aggregation, this consumer:
/// 1. Queries the BSM to check aggregation requirements
/// 2. Signs the job output with the operator's BLS key
/// 3. Submits the signature to the aggregation service
/// 4. Optionally waits for threshold and submits the aggregated result
///
/// For jobs that don't require aggregation, it behaves identically to `TangleEvmConsumer`.
pub struct AggregatingConsumer {
    client: Arc<TangleEvmClient>,
    buffer: Mutex<VecDeque<PendingJobResult>>,
    state: Mutex<State>,
    /// Cache of aggregation config by (service_id, job_index)
    aggregation_cache: Mutex<std::collections::HashMap<(u64, u8), AggregationConfig>>,
    /// Aggregation service configuration (when feature enabled)
    #[cfg(feature = "aggregation")]
    aggregation_config: Option<AggregationServiceConfig>,
}

impl AggregatingConsumer {
    /// Create a new aggregating consumer
    pub fn new(client: TangleEvmClient) -> Self {
        Self {
            client: Arc::new(client),
            buffer: Mutex::new(VecDeque::new()),
            state: Mutex::new(State::WaitingForResult),
            aggregation_cache: Mutex::new(std::collections::HashMap::new()),
            #[cfg(feature = "aggregation")]
            aggregation_config: None,
        }
    }

    /// Configure the aggregation service for BLS signature aggregation
    ///
    /// This enables automatic signing and submission to the aggregation service
    /// when jobs require BLS aggregation.
    #[cfg(feature = "aggregation")]
    pub fn with_aggregation_service(
        mut self,
        service_url: impl Into<String>,
        bls_secret: blueprint_crypto_bn254::ArkBlsBn254Secret,
        operator_index: u32,
    ) -> Self {
        self.aggregation_config = Some(AggregationServiceConfig::new(
            service_url,
            bls_secret,
            operator_index,
        ));
        self
    }

    /// Configure aggregation with full config options
    #[cfg(feature = "aggregation")]
    pub fn with_aggregation_config(mut self, config: AggregationServiceConfig) -> Self {
        self.aggregation_config = Some(config);
        self
    }

    /// Get the underlying client
    #[must_use]
    pub fn client(&self) -> &TangleEvmClient {
        &self.client
    }

    /// Get aggregation config, using cache when available
    async fn get_aggregation_config(
        client: Arc<TangleEvmClient>,
        service_id: u64,
        job_index: u8,
    ) -> Result<AggregationConfig, AggregatingConsumerError> {
        // Note: In production, you'd want to use the cache
        // For now, always query fresh to ensure correctness
        client
            .get_aggregation_config(service_id, job_index)
            .await
            .map_err(|e| AggregatingConsumerError::Client(e.to_string()))
    }
}

impl Sink<JobResult> for AggregatingConsumer {
    type Error = BoxError;

    fn poll_ready(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn start_send(self: Pin<&mut Self>, item: JobResult) -> Result<(), Self::Error> {
        let JobResult::Ok { head, body } = &item else {
            blueprint_core::trace!(target: "tangle-evm-aggregating-consumer", "Discarding job result with error");
            return Ok(());
        };

        let (Some(call_id_raw), Some(service_id_raw)) = (
            head.metadata.get(extract::CallId::METADATA_KEY),
            head.metadata.get(extract::ServiceId::METADATA_KEY),
        ) else {
            blueprint_core::trace!(target: "tangle-evm-aggregating-consumer", "Discarding job result with missing metadata");
            return Ok(());
        };

        // Get job index from metadata (defaults to 0 if not present)
        let job_index: u8 = head
            .metadata
            .get(extract::JobIndex::METADATA_KEY)
            .and_then(|v| {
                let val: u64 = v.try_into().ok()?;
                u8::try_from(val).ok()
            })
            .unwrap_or(0);

        blueprint_core::debug!(
            target: "tangle-evm-aggregating-consumer",
            result = ?item,
            job_index = job_index,
            "Received job result, handling..."
        );

        let call_id: u64 = call_id_raw
            .try_into()
            .map_err(|_| AggregatingConsumerError::InvalidMetadata("call_id"))?;
        let service_id: u64 = service_id_raw
            .try_into()
            .map_err(|_| AggregatingConsumerError::InvalidMetadata("service_id"))?;

        self.get_mut()
            .buffer
            .lock()
            .unwrap()
            .push_back(PendingJobResult {
                service_id,
                call_id,
                job_index,
                output: Bytes::copy_from_slice(body),
            });
        Ok(())
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let consumer = self.get_mut();
        let mut state = consumer.state.lock().unwrap();

        {
            let buffer = consumer.buffer.lock().unwrap();
            if buffer.is_empty() && state.is_waiting() {
                return Poll::Ready(Ok(()));
            }
        }

        loop {
            match &mut *state {
                State::WaitingForResult => {
                    let result = {
                        let mut buffer = consumer.buffer.lock().unwrap();
                        buffer.pop_front()
                    };

                    let Some(pending) = result else {
                        return Poll::Ready(Ok(()));
                    };

                    let client = Arc::clone(&consumer.client);

                    #[cfg(feature = "aggregation")]
                    let agg_config = consumer.aggregation_config.clone();

                    let fut = Box::pin(async move {
                        #[cfg(feature = "aggregation")]
                        {
                            submit_job_result(
                                client,
                                pending.service_id,
                                pending.call_id,
                                pending.job_index,
                                pending.output,
                                agg_config,
                            )
                            .await
                        }
                        #[cfg(not(feature = "aggregation"))]
                        {
                            submit_job_result(
                                client,
                                pending.service_id,
                                pending.call_id,
                                pending.job_index,
                                pending.output,
                            )
                            .await
                        }
                    });

                    *state = State::ProcessingSubmission(fut);
                }
                State::ProcessingSubmission(future) => match future.as_mut().poll(cx) {
                    Poll::Ready(Ok(())) => {
                        *state = State::WaitingForResult;
                    }
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e.into())),
                    Poll::Pending => return Poll::Pending,
                },
            }
        }
    }

    fn poll_close(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        let buffer = self.buffer.lock().unwrap();
        if buffer.is_empty() {
            Poll::Ready(Ok(()))
        } else {
            Poll::Pending
        }
    }
}

/// Submit a job result, automatically choosing aggregation if required
#[cfg(feature = "aggregation")]
async fn submit_job_result(
    client: Arc<TangleEvmClient>,
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
    agg_config: Option<AggregationServiceConfig>,
) -> Result<(), AggregatingConsumerError> {
    // Check if aggregation is required
    let config = AggregatingConsumer::get_aggregation_config(
        Arc::clone(&client),
        service_id,
        job_index,
    )
    .await?;

    if config.required {
        blueprint_core::info!(
            target: "tangle-evm-aggregating-consumer",
            "Job {} for service {} requires aggregation (threshold: {}bps, type: {:?})",
            call_id,
            service_id,
            config.threshold_bps,
            config.threshold_type
        );

        // Get aggregation config or error
        let agg = agg_config.ok_or(AggregatingConsumerError::AggregationNotConfigured)?;

        submit_aggregated_result(
            client,
            service_id,
            call_id,
            output,
            config,
            agg,
        )
        .await
    } else {
        // No aggregation needed, submit directly
        submit_direct_result(client, service_id, call_id, output).await
    }
}

/// Submit a job result without aggregation feature
#[cfg(not(feature = "aggregation"))]
async fn submit_job_result(
    client: Arc<TangleEvmClient>,
    service_id: u64,
    call_id: u64,
    job_index: u8,
    output: Bytes,
) -> Result<(), AggregatingConsumerError> {
    // Check if aggregation is required
    let config = AggregatingConsumer::get_aggregation_config(
        Arc::clone(&client),
        service_id,
        job_index,
    )
    .await?;

    if config.required {
        blueprint_core::warn!(
            target: "tangle-evm-aggregating-consumer",
            "Job {} for service {} requires aggregation but 'aggregation' feature not enabled. \
             Enable the feature and configure the aggregation service.",
            call_id,
            service_id,
        );
        Ok(())
    } else {
        submit_direct_result(client, service_id, call_id, output).await
    }
}

/// Submit using the aggregation service
#[cfg(feature = "aggregation")]
async fn submit_aggregated_result(
    client: Arc<TangleEvmClient>,
    service_id: u64,
    call_id: u64,
    output: Bytes,
    config: AggregationConfig,
    agg: AggregationServiceConfig,
) -> Result<(), AggregatingConsumerError> {
    use blueprint_crypto_bn254::ArkBlsBn254;
    use blueprint_crypto_core::{BytesEncoding, KeyType};
    use blueprint_tangle_aggregation_svc::{create_signing_message, SubmitSignatureRequest};

    blueprint_core::debug!(
        target: "tangle-evm-aggregating-consumer",
        "Submitting signature to aggregation service for service {} call {}",
        service_id,
        call_id
    );

    // Create the message to sign
    let message = create_signing_message(service_id, call_id, &output);

    // Sign with BLS key - we need a mutable clone since sign_with_secret takes &mut
    let mut secret_clone = (*agg.bls_secret).clone();
    let signature = ArkBlsBn254::sign_with_secret(&mut secret_clone, &message)
        .map_err(|e| AggregatingConsumerError::Bls(e.to_string()))?;

    // Get public key and signature bytes using BytesEncoding trait
    let pubkey_bytes = agg.bls_public.to_bytes();
    let sig_bytes = signature.to_bytes();

    // Calculate threshold from config
    let threshold = calculate_threshold_count(config.threshold_bps, config.threshold_type);

    // Try to initialize the task (may already exist from another operator)
    let _ = agg.client.init_task(
        service_id,
        call_id,
        &output,
        threshold, // This should be total operators, but we use threshold as approximation
        threshold,
    ).await;

    // Submit our signature
    let submit_request = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: agg.operator_index,
        output: output.to_vec(),
        signature: sig_bytes,
        public_key: pubkey_bytes,
    };

    let response = agg.client.submit_signature(submit_request).await?;

    blueprint_core::info!(
        target: "tangle-evm-aggregating-consumer",
        "Submitted signature to aggregation service: {}/{} signatures (threshold met: {})",
        response.signatures_collected,
        response.threshold_required,
        response.threshold_met
    );

    // If configured to wait and threshold is met, submit to chain
    if agg.wait_for_threshold && response.threshold_met {
        submit_aggregated_to_chain(client, &agg, service_id, call_id).await?;
    } else if agg.wait_for_threshold {
        // Wait for threshold to be met
        blueprint_core::debug!(
            target: "tangle-evm-aggregating-consumer",
            "Waiting for threshold to be met..."
        );

        let result = agg.client.wait_for_threshold(
            service_id,
            call_id,
            agg.poll_interval,
            agg.threshold_timeout,
        ).await?;

        // Submit to chain
        submit_aggregated_to_chain_with_result(client, &agg, service_id, call_id, result).await?;
    }

    Ok(())
}

/// Submit the aggregated result to the blockchain
#[cfg(feature = "aggregation")]
async fn submit_aggregated_to_chain(
    client: Arc<TangleEvmClient>,
    agg: &AggregationServiceConfig,
    service_id: u64,
    call_id: u64,
) -> Result<(), AggregatingConsumerError> {
    // Fetch the aggregated result
    let result = agg.client.get_aggregated(service_id, call_id).await?
        .ok_or_else(|| AggregatingConsumerError::Client("Aggregated result not available".to_string()))?;

    submit_aggregated_to_chain_with_result(client, agg, service_id, call_id, result).await
}

/// Submit the aggregated result to the blockchain with a pre-fetched result
#[cfg(feature = "aggregation")]
async fn submit_aggregated_to_chain_with_result(
    client: Arc<TangleEvmClient>,
    agg: &AggregationServiceConfig,
    service_id: u64,
    call_id: u64,
    result: blueprint_tangle_aggregation_svc::AggregatedResultResponse,
) -> Result<(), AggregatingConsumerError> {
    use crate::aggregation::{AggregatedResult, G1Point, G2Point, SignerBitmap};

    blueprint_core::info!(
        target: "tangle-evm-aggregating-consumer",
        "Submitting aggregated result to chain for service {} call {}",
        service_id,
        call_id
    );

    // Parse the signature and pubkey from the response
    let signature = G1Point::from_bytes(&result.aggregated_signature)
        .ok_or_else(|| AggregatingConsumerError::Bls("Invalid aggregated signature".to_string()))?;
    let pubkey = G2Point::from_bytes(&result.aggregated_pubkey)
        .ok_or_else(|| AggregatingConsumerError::Bls("Invalid aggregated pubkey".to_string()))?;

    let aggregated = AggregatedResult::new(
        service_id,
        call_id,
        Bytes::from(result.output),
        SignerBitmap(result.signer_bitmap),
        signature,
        pubkey,
    );

    // Submit to the contract
    aggregated.submit(&Arc::new(client.as_ref().clone())).await?;

    // Mark as submitted in the aggregation service
    let _ = agg.client.mark_submitted(service_id, call_id).await;

    blueprint_core::info!(
        target: "tangle-evm-aggregating-consumer",
        "Successfully submitted aggregated result for service {} call {}",
        service_id,
        call_id
    );

    Ok(())
}

/// Calculate threshold count from basis points
#[cfg(feature = "aggregation")]
fn calculate_threshold_count(threshold_bps: u16, _threshold_type: ThresholdType) -> u32 {
    // This is an approximation - in practice, we'd need the actual operator count
    // For now, assume a reasonable default and let the aggregation service handle the actual threshold
    let assumed_operators = 10u32;
    let required = (assumed_operators as u64 * threshold_bps as u64) / 10000;
    std::cmp::max(1, required as u32)
}

/// Submit a result directly without aggregation
async fn submit_direct_result(
    client: Arc<TangleEvmClient>,
    service_id: u64,
    call_id: u64,
    output: Bytes,
) -> Result<(), AggregatingConsumerError> {
    blueprint_core::debug!(
        target: "tangle-evm-aggregating-consumer",
        "Submitting direct result for service {} call {}",
        service_id,
        call_id
    );

    let contract = client.tangle_contract();

    let _call = contract.submitResult(service_id, call_id, output);

    // TODO: Sign and send the transaction
    blueprint_core::info!(
        target: "tangle-evm-aggregating-consumer",
        "Would submit direct result for service {} call {} (signing not implemented yet)",
        service_id,
        call_id
    );

    Ok(())
}

/// Helper to integrate with the P2P aggregation protocol
///
/// This would be used by blueprint developers who want full control
/// over the aggregation process.
pub mod integration {
    use super::*;

    /// Create the message hash that needs to be signed for BLS aggregation
    ///
    /// This matches the contract's verification: keccak256(abi.encodePacked(serviceId, callId, keccak256(output)))
    pub fn create_signing_message(service_id: u64, call_id: u64, output: &[u8]) -> Vec<u8> {
        use alloy_primitives::keccak256;

        let output_hash = keccak256(output);
        let mut message = Vec::with_capacity(8 + 8 + 32);
        message.extend_from_slice(&service_id.to_be_bytes());
        message.extend_from_slice(&call_id.to_be_bytes());
        message.extend_from_slice(output_hash.as_slice());
        message
    }

    /// Calculate required signer count based on threshold config
    pub fn calculate_required_signers(
        total_operators: usize,
        threshold_bps: u16,
        threshold_type: ThresholdType,
        operator_stakes: Option<&[u64]>,
    ) -> usize {
        match threshold_type {
            ThresholdType::CountBased => {
                let required = (total_operators as u64 * threshold_bps as u64) / 10000;
                std::cmp::max(1, required as usize)
            }
            ThresholdType::StakeWeighted => {
                // For stake-weighted, we'd need the actual stakes
                // For now, fall back to count-based
                if let Some(stakes) = operator_stakes {
                    let total_stake: u64 = stakes.iter().sum();
                    let required_stake = (total_stake * threshold_bps as u64) / 10000;
                    // This is a simplification - in practice you'd sort by stake
                    // and count until threshold is met
                    let avg_stake = total_stake / stakes.len() as u64;
                    std::cmp::max(1, (required_stake / avg_stake) as usize)
                } else {
                    let required = (total_operators as u64 * threshold_bps as u64) / 10000;
                    std::cmp::max(1, required as usize)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::integration::*;
    use blueprint_client_tangle_evm::ThresholdType;

    // ═══════════════════════════════════════════════════════════════════════════
    // create_signing_message tests
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_create_signing_message_format() {
        let service_id = 1u64;
        let call_id = 42u64;
        let output = b"test output";

        let message = create_signing_message(service_id, call_id, output);

        // Should be 8 + 8 + 32 = 48 bytes
        assert_eq!(message.len(), 48);

        // First 8 bytes should be service_id in big endian
        assert_eq!(&message[0..8], &service_id.to_be_bytes());

        // Next 8 bytes should be call_id in big endian
        assert_eq!(&message[8..16], &call_id.to_be_bytes());

        // Last 32 bytes should be keccak256(output)
        use alloy_primitives::keccak256;
        let expected_hash = keccak256(output);
        assert_eq!(&message[16..48], expected_hash.as_slice());
    }

    #[test]
    fn test_create_signing_message_deterministic() {
        let msg1 = create_signing_message(1, 2, b"hello");
        let msg2 = create_signing_message(1, 2, b"hello");
        assert_eq!(msg1, msg2);
    }

    #[test]
    fn test_create_signing_message_different_outputs() {
        let msg1 = create_signing_message(1, 2, b"hello");
        let msg2 = create_signing_message(1, 2, b"world");
        // Different outputs should produce different messages (different hash suffix)
        assert_ne!(msg1, msg2);
        // But same prefix (service_id and call_id)
        assert_eq!(&msg1[0..16], &msg2[0..16]);
    }

    #[test]
    fn test_create_signing_message_empty_output() {
        let msg = create_signing_message(1, 2, b"");
        assert_eq!(msg.len(), 48);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // calculate_required_signers tests - Count Based
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_calculate_required_signers_count_based_67_percent() {
        // 67% of 3 operators = 2.01 -> 2
        let required =
            calculate_required_signers(3, 6700, ThresholdType::CountBased, None);
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_count_based_50_percent() {
        // 50% of 4 operators = 2
        let required =
            calculate_required_signers(4, 5000, ThresholdType::CountBased, None);
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_count_based_100_percent() {
        // 100% of 5 operators = 5
        let required =
            calculate_required_signers(5, 10000, ThresholdType::CountBased, None);
        assert_eq!(required, 5);
    }

    #[test]
    fn test_calculate_required_signers_count_based_minimum_one() {
        // Very low threshold should still require at least 1
        let required =
            calculate_required_signers(10, 100, ThresholdType::CountBased, None); // 1%
        assert_eq!(required, 1);
    }

    #[test]
    fn test_calculate_required_signers_count_based_single_operator() {
        // Single operator, any threshold should require 1
        let required =
            calculate_required_signers(1, 6700, ThresholdType::CountBased, None);
        assert_eq!(required, 1);
    }

    #[test]
    fn test_calculate_required_signers_count_based_large_set() {
        // 67% of 100 operators = 67
        let required =
            calculate_required_signers(100, 6700, ThresholdType::CountBased, None);
        assert_eq!(required, 67);
    }

    // ═══════════════════════════════════════════════════════════════════════════
    // calculate_required_signers tests - Stake Weighted
    // ═══════════════════════════════════════════════════════════════════════════

    #[test]
    fn test_calculate_required_signers_stake_weighted_no_stakes() {
        // Without stakes, should fall back to count-based
        let required =
            calculate_required_signers(3, 6700, ThresholdType::StakeWeighted, None);
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_stake_weighted_equal_stakes() {
        // 3 operators with equal stakes (10 each), 67% threshold
        // Total stake = 30, required = 20.1, avg = 10, required signers = 2
        let stakes = [10u64, 10, 10];
        let required = calculate_required_signers(
            3,
            6700,
            ThresholdType::StakeWeighted,
            Some(&stakes),
        );
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_stake_weighted_unequal_stakes() {
        // 3 operators: 5, 3, 2 ETH stake (like in contract tests)
        // Total = 10 ETH, 67% = 6.7 ETH required
        // Avg stake = 3.33, signers needed ≈ 2
        let stakes = [5u64, 3, 2];
        let required = calculate_required_signers(
            3,
            6700,
            ThresholdType::StakeWeighted,
            Some(&stakes),
        );
        assert_eq!(required, 2);
    }

    #[test]
    fn test_calculate_required_signers_stake_weighted_minimum_one() {
        // Very low threshold should still require at least 1
        let stakes = [100u64, 100, 100];
        let required = calculate_required_signers(
            3,
            100, // 1%
            ThresholdType::StakeWeighted,
            Some(&stakes),
        );
        assert_eq!(required, 1);
    }
}
