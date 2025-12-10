//! Main aggregation service logic

use crate::state::{AggregationState, TaskConfig, ThresholdType};
use crate::types::*;
use alloy_primitives::U256;
use ark_serialize::CanonicalDeserialize;
use blueprint_crypto_bn254::{ArkBlsBn254, ArkBlsBn254Public, ArkBlsBn254Signature};
use blueprint_crypto_core::{aggregation::AggregatableSignature, KeyType};
use std::sync::Arc;
use std::time::Duration;
use thiserror::Error;
use tokio::sync::watch;
use tracing::{debug, info, warn};

/// Errors from the aggregation service
#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("Task not found")]
    TaskNotFound,
    #[error("Task already exists")]
    TaskAlreadyExists,
    #[error("Task has expired")]
    TaskExpired,
    #[error("Invalid signature format")]
    InvalidSignature,
    #[error("Invalid public key format")]
    InvalidPublicKey,
    #[error("Signature verification failed")]
    VerificationFailed,
    #[error("Output mismatch: submitted output does not match task output")]
    OutputMismatch,
    #[error("Aggregation failed: {0}")]
    AggregationFailed(String),
    #[error("{0}")]
    Other(String),
}

/// Configuration for the aggregation service
#[derive(Debug, Clone)]
pub struct ServiceConfig {
    /// Whether to verify signatures on submission
    pub verify_on_submit: bool,
    /// Whether to validate that submitted output matches task output
    pub validate_output: bool,
    /// Default TTL for tasks (None = no expiry)
    pub default_task_ttl: Option<Duration>,
    /// Cleanup interval for expired/submitted tasks
    pub cleanup_interval: Option<Duration>,
    /// Whether to auto-cleanup submitted tasks
    pub auto_cleanup_submitted: bool,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            verify_on_submit: true,
            validate_output: true,
            default_task_ttl: Some(Duration::from_secs(3600)), // 1 hour default
            cleanup_interval: Some(Duration::from_secs(60)),   // Cleanup every minute
            auto_cleanup_submitted: true,
        }
    }
}

impl ServiceConfig {
    /// Create a minimal config (no verification, no cleanup)
    pub fn minimal() -> Self {
        Self {
            verify_on_submit: false,
            validate_output: false,
            default_task_ttl: None,
            cleanup_interval: None,
            auto_cleanup_submitted: false,
        }
    }
}

/// The main aggregation service
#[derive(Debug)]
pub struct AggregationService {
    state: AggregationState,
    config: ServiceConfig,
}

impl AggregationService {
    /// Create a new aggregation service
    pub fn new(config: ServiceConfig) -> Self {
        Self {
            state: AggregationState::new(),
            config,
        }
    }

    /// Create a new aggregation service wrapped in Arc
    pub fn new_shared(config: ServiceConfig) -> Arc<Self> {
        Arc::new(Self::new(config))
    }

    /// Start the background cleanup worker
    /// Returns a handle that can be used to stop the worker
    pub fn start_cleanup_worker(self: &Arc<Self>) -> Option<CleanupWorkerHandle> {
        let interval = self.config.cleanup_interval?;
        let (shutdown_tx, mut shutdown_rx) = watch::channel(false);

        let service = Arc::clone(self);

        let handle = tokio::spawn(async move {
            let mut interval_timer = tokio::time::interval(interval);

            loop {
                tokio::select! {
                    _ = interval_timer.tick() => {
                        let removed = if service.config.auto_cleanup_submitted {
                            service.state.cleanup()
                        } else {
                            service.state.cleanup_expired()
                        };
                        if removed > 0 {
                            debug!(removed, "Cleaned up tasks");
                        }
                    }
                    _ = shutdown_rx.changed() => {
                        if *shutdown_rx.borrow() {
                            info!("Cleanup worker shutting down");
                            break;
                        }
                    }
                }
            }
        });

        Some(CleanupWorkerHandle {
            shutdown_tx,
            handle,
        })
    }

    /// Initialize a new aggregation task
    pub fn init_task(
        &self,
        service_id: u64,
        call_id: u64,
        output: Vec<u8>,
        operator_count: u32,
        threshold: u32,
    ) -> Result<(), ServiceError> {
        self.init_task_with_config(
            service_id,
            call_id,
            output,
            operator_count,
            TaskConfig {
                threshold_type: ThresholdType::Count(threshold),
                ttl: self.config.default_task_ttl,
                ..Default::default()
            },
        )
    }

    /// Initialize a new aggregation task with full configuration
    pub fn init_task_with_config(
        &self,
        service_id: u64,
        call_id: u64,
        output: Vec<u8>,
        operator_count: u32,
        config: TaskConfig,
    ) -> Result<(), ServiceError> {
        info!(
            service_id,
            call_id,
            operator_count,
            ?config.threshold_type,
            "Initializing aggregation task"
        );

        self.state
            .init_task_with_config(service_id, call_id, output, operator_count, config)
            .map_err(|e| ServiceError::Other(e.to_string()))
    }

    /// Submit a signature for aggregation
    pub fn submit_signature(
        &self,
        req: SubmitSignatureRequest,
    ) -> Result<SubmitSignatureResponse, ServiceError> {
        debug!(
            service_id = req.service_id,
            call_id = req.call_id,
            operator_index = req.operator_index,
            "Received signature submission"
        );

        // Validate output matches task output (if enabled)
        if self.config.validate_output {
            let expected_output = self
                .state
                .get_task_output(req.service_id, req.call_id)
                .ok_or(ServiceError::TaskNotFound)?;

            if req.output != expected_output {
                warn!(
                    service_id = req.service_id,
                    call_id = req.call_id,
                    operator_index = req.operator_index,
                    "Output mismatch"
                );
                return Err(ServiceError::OutputMismatch);
            }
        }

        // Parse signature (G1 point)
        let signature: ArkBlsBn254Signature = ArkBlsBn254Signature(
            ark_bn254::G1Affine::deserialize_compressed(&req.signature[..])
                .map_err(|_| ServiceError::InvalidSignature)?,
        );

        // Parse public key (G2 point)
        let public_key: ArkBlsBn254Public = ArkBlsBn254Public(
            ark_bn254::G2Affine::deserialize_compressed(&req.public_key[..])
                .map_err(|_| ServiceError::InvalidPublicKey)?,
        );

        // Optionally verify the signature
        if self.config.verify_on_submit {
            // Create the message that should have been signed
            let message = create_signing_message(req.service_id, req.call_id, &req.output);

            if !ArkBlsBn254::verify(&public_key, &message, &signature) {
                warn!(
                    service_id = req.service_id,
                    call_id = req.call_id,
                    operator_index = req.operator_index,
                    "Signature verification failed"
                );
                return Err(ServiceError::VerificationFailed);
            }
        }

        // Get task status for response
        let status = self
            .state
            .get_status(req.service_id, req.call_id)
            .ok_or(ServiceError::TaskNotFound)?;

        if status.is_expired {
            return Err(ServiceError::TaskExpired);
        }

        // Submit to state
        let (count, threshold_met) = self
            .state
            .submit_signature(
                req.service_id,
                req.call_id,
                req.operator_index,
                signature,
                public_key,
            )
            .map_err(|e| ServiceError::Other(e.to_string()))?;

        info!(
            service_id = req.service_id,
            call_id = req.call_id,
            operator_index = req.operator_index,
            signatures_collected = count,
            threshold_met,
            "Signature accepted"
        );

        Ok(SubmitSignatureResponse {
            accepted: true,
            signatures_collected: count,
            threshold_required: status.threshold_required,
            threshold_met,
            error: None,
        })
    }

    /// Get status of an aggregation task
    pub fn get_status(&self, service_id: u64, call_id: u64) -> GetStatusResponse {
        match self.state.get_status(service_id, call_id) {
            Some(status) => GetStatusResponse {
                exists: true,
                signatures_collected: status.signatures_collected,
                threshold_required: status.threshold_required,
                threshold_met: status.threshold_met,
                signer_bitmap: status.signer_bitmap,
                signed_stake_bps: Some(status.signed_stake_bps),
                submitted: status.submitted,
                is_expired: Some(status.is_expired),
                time_remaining_secs: status.time_remaining_secs,
            },
            None => GetStatusResponse {
                exists: false,
                signatures_collected: 0,
                threshold_required: 0,
                threshold_met: false,
                signer_bitmap: U256::ZERO,
                signed_stake_bps: None,
                submitted: false,
                is_expired: None,
                time_remaining_secs: None,
            },
        }
    }

    /// Get aggregated result if threshold is met
    pub fn get_aggregated_result(
        &self,
        service_id: u64,
        call_id: u64,
    ) -> Option<AggregatedResultResponse> {
        let task = self.state.get_for_aggregation(service_id, call_id)?;

        // Aggregate signatures and public keys
        let (agg_sig, agg_pk) = ArkBlsBn254::aggregate(&task.signatures, &task.public_keys)
            .map_err(|e| {
                warn!(
                    service_id,
                    call_id,
                    error = %e,
                    "Aggregation failed"
                );
                e
            })
            .ok()?;

        // Serialize for response
        let mut sig_bytes = Vec::new();
        ark_serialize::CanonicalSerialize::serialize_compressed(&agg_sig.0, &mut sig_bytes).ok()?;

        let mut pk_bytes = Vec::new();
        ark_serialize::CanonicalSerialize::serialize_compressed(&agg_pk.0, &mut pk_bytes).ok()?;

        info!(
            service_id,
            call_id,
            signers = task.signatures.len(),
            non_signers = task.non_signer_indices.len(),
            "Returning aggregated result"
        );

        Some(AggregatedResultResponse {
            service_id: task.service_id,
            call_id: task.call_id,
            output: task.output,
            signer_bitmap: task.signer_bitmap,
            non_signer_indices: task.non_signer_indices,
            aggregated_signature: sig_bytes,
            aggregated_pubkey: pk_bytes,
        })
    }

    /// Mark a task as submitted to chain
    pub fn mark_submitted(&self, service_id: u64, call_id: u64) -> Result<(), ServiceError> {
        self.state
            .mark_submitted(service_id, call_id)
            .map_err(|e| ServiceError::Other(e.to_string()))
    }

    /// Remove a task
    pub fn remove_task(&self, service_id: u64, call_id: u64) -> bool {
        self.state.remove_task(service_id, call_id)
    }

    /// Get task statistics
    pub fn get_stats(&self) -> ServiceStats {
        let counts = self.state.task_counts();
        ServiceStats {
            total_tasks: counts.total,
            pending_tasks: counts.pending,
            ready_tasks: counts.ready,
            submitted_tasks: counts.submitted,
            expired_tasks: counts.expired,
        }
    }

    /// Manually trigger cleanup
    pub fn cleanup(&self) -> usize {
        self.state.cleanup()
    }

    /// Cleanup only expired tasks
    pub fn cleanup_expired(&self) -> usize {
        self.state.cleanup_expired()
    }

    /// Cleanup only submitted tasks
    pub fn cleanup_submitted(&self) -> usize {
        self.state.cleanup_submitted()
    }
}

/// Create the message that operators sign
///
/// Format: serviceId (8 bytes BE) || callId (8 bytes BE) || keccak256(output)
pub fn create_signing_message(service_id: u64, call_id: u64, output: &[u8]) -> Vec<u8> {
    use alloy_primitives::keccak256;

    let output_hash = keccak256(output);
    let mut message = Vec::with_capacity(8 + 8 + 32);
    message.extend_from_slice(&service_id.to_be_bytes());
    message.extend_from_slice(&call_id.to_be_bytes());
    message.extend_from_slice(output_hash.as_slice());
    message
}

impl Default for AggregationService {
    fn default() -> Self {
        Self::new(ServiceConfig::default())
    }
}

/// Handle for the cleanup worker
pub struct CleanupWorkerHandle {
    shutdown_tx: watch::Sender<bool>,
    handle: tokio::task::JoinHandle<()>,
}

impl CleanupWorkerHandle {
    /// Stop the cleanup worker
    pub async fn stop(self) {
        let _ = self.shutdown_tx.send(true);
        let _ = self.handle.await;
    }
}

/// Service statistics
#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize)]
pub struct ServiceStats {
    pub total_tasks: usize,
    pub pending_tasks: usize,
    pub ready_tasks: usize,
    pub submitted_tasks: usize,
    pub expired_tasks: usize,
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::keccak256;

    #[test]
    fn test_create_signing_message() {
        let service_id = 1u64;
        let call_id = 100u64;
        let output = vec![1, 2, 3, 4];

        let message = create_signing_message(service_id, call_id, &output);

        // Message should be 48 bytes: 8 + 8 + 32
        assert_eq!(message.len(), 48);

        // Check service_id encoding (big-endian)
        assert_eq!(&message[0..8], &service_id.to_be_bytes());

        // Check call_id encoding (big-endian)
        assert_eq!(&message[8..16], &call_id.to_be_bytes());

        // Check output hash
        let expected_hash = keccak256(&output);
        assert_eq!(&message[16..48], expected_hash.as_slice());
    }

    #[test]
    fn test_create_signing_message_deterministic() {
        let msg1 = create_signing_message(1, 100, &[1, 2, 3]);
        let msg2 = create_signing_message(1, 100, &[1, 2, 3]);
        assert_eq!(msg1, msg2);
    }

    #[test]
    fn test_create_signing_message_different_inputs() {
        let msg1 = create_signing_message(1, 100, &[1, 2, 3]);
        let msg2 = create_signing_message(2, 100, &[1, 2, 3]);
        let msg3 = create_signing_message(1, 101, &[1, 2, 3]);
        let msg4 = create_signing_message(1, 100, &[1, 2, 4]);

        assert_ne!(msg1, msg2);
        assert_ne!(msg1, msg3);
        assert_ne!(msg1, msg4);
    }

    #[test]
    fn test_service_config_default() {
        let config = ServiceConfig::default();
        assert!(config.verify_on_submit);
        assert!(config.validate_output);
        assert!(config.default_task_ttl.is_some());
        assert!(config.cleanup_interval.is_some());
        assert!(config.auto_cleanup_submitted);
    }

    #[test]
    fn test_service_config_minimal() {
        let config = ServiceConfig::minimal();
        assert!(!config.verify_on_submit);
        assert!(!config.validate_output);
        assert!(config.default_task_ttl.is_none());
        assert!(config.cleanup_interval.is_none());
        assert!(!config.auto_cleanup_submitted);
    }

    #[test]
    fn test_aggregation_service_init_task() {
        let service = AggregationService::new(ServiceConfig::minimal());

        assert!(service.init_task(1, 100, vec![1, 2, 3], 5, 3).is_ok());

        // Duplicate should fail
        let result = service.init_task(1, 100, vec![1, 2, 3], 5, 3);
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregation_service_get_status_nonexistent() {
        let service = AggregationService::default();

        let status = service.get_status(1, 100);
        assert!(!status.exists);
        assert_eq!(status.signatures_collected, 0);
    }

    #[test]
    fn test_aggregation_service_get_status_exists() {
        let service = AggregationService::new(ServiceConfig::minimal());
        service.init_task(1, 100, vec![], 5, 3).unwrap();

        let status = service.get_status(1, 100);
        assert!(status.exists);
        assert_eq!(status.signatures_collected, 0);
        assert_eq!(status.threshold_required, 3);
        assert!(!status.threshold_met);
        assert!(!status.submitted);
    }

    #[test]
    fn test_aggregation_service_mark_submitted() {
        let service = AggregationService::new(ServiceConfig::minimal());
        service.init_task(1, 100, vec![], 5, 3).unwrap();

        assert!(service.mark_submitted(1, 100).is_ok());

        let status = service.get_status(1, 100);
        assert!(status.submitted);
    }

    #[test]
    fn test_aggregation_service_mark_submitted_nonexistent() {
        let service = AggregationService::default();

        let result = service.mark_submitted(1, 100);
        assert!(result.is_err());
    }

    #[test]
    fn test_aggregation_service_get_aggregated_result_nonexistent() {
        let service = AggregationService::default();

        let result = service.get_aggregated_result(1, 100);
        assert!(result.is_none());
    }

    #[test]
    fn test_aggregation_service_get_aggregated_result_threshold_not_met() {
        let service = AggregationService::new(ServiceConfig::minimal());
        service.init_task(1, 100, vec![], 5, 3).unwrap();

        // No signatures submitted
        let result = service.get_aggregated_result(1, 100);
        assert!(result.is_none());
    }

    #[test]
    fn test_aggregation_service_stats() {
        let service = AggregationService::new(ServiceConfig::minimal());

        service.init_task(1, 100, vec![], 5, 3).unwrap();
        service.init_task(1, 101, vec![], 5, 3).unwrap();

        let stats = service.get_stats();
        assert_eq!(stats.total_tasks, 2);
        assert_eq!(stats.pending_tasks, 2);
        assert_eq!(stats.ready_tasks, 0);
        assert_eq!(stats.submitted_tasks, 0);
        assert_eq!(stats.expired_tasks, 0);
    }

    #[test]
    fn test_aggregation_service_remove_task() {
        let service = AggregationService::new(ServiceConfig::minimal());
        service.init_task(1, 100, vec![], 5, 3).unwrap();

        assert!(service.get_status(1, 100).exists);
        assert!(service.remove_task(1, 100));
        assert!(!service.get_status(1, 100).exists);
    }
}
