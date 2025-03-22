use crate::{
    aggregator_selection::AggregatorSelector,
    messages::{AggSigMessage, AggregationResult, MaliciousEvidence},
    participants::{ParticipantMap, ParticipantSet},
    protocol_state::{AggregationState, ProtocolRound},
    signature_weight::SignatureWeight,
    zk_proof::{ThresholdProofGenerator, ThresholdWeightProof},
};
use blueprint_core::{error, info, warn};
use gadget_crypto::{aggregation::AggregatableSignature, hashing::keccak_256};
use gadget_networking::{
    service_handle::NetworkServiceHandle,
    types::{MessageRouting, ParticipantId, ParticipantInfo, ProtocolMessage},
};
use std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
    time::{Duration, Instant},
};
use thiserror::Error;
use tracing::debug;

/// Error types for the aggregation protocol
#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Invalid signature from participant {0}")]
    InvalidSignature(ParticipantId),

    #[error("Duplicate different signature from participant {0}")]
    DuplicateSignature(ParticipantId),

    #[error("Threshold not met: got {0}, need {1}")]
    ThresholdNotMet(usize, usize),

    #[error("Aggregation error: {0}")]
    AggregationError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] bincode::Error),

    #[error("Key not found")]
    KeyNotFound,

    #[error("Network error: {0}")]
    NetworkError(String),

    #[error("Protocol error: {0}")]
    Protocol(String),

    #[error("Timeout")]
    Timeout,

    #[error("Invalid message content")]
    InvalidMessage,

    #[error("Signing error: {0}")]
    SigningError(String),

    #[error("Protocol operation interrupted")]
    Interrupted,
}

/// Configuration for the aggregation protocol
#[derive(Clone)]
pub struct ProtocolConfig {
    /// Local participant ID
    pub local_id: ParticipantId,

    /// Maximum number of participants
    pub max_participants: u16,

    /// Number of aggregators to select
    pub num_aggregators: u16,

    /// Timeout for collecting signatures
    pub timeout: Duration,

    /// Protocol ID for message routing
    pub protocol_id: String,
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            local_id: ParticipantId(0),
            max_participants: 100,
            num_aggregators: 3,
            timeout: Duration::from_secs(5),
            protocol_id: "sig-agg".to_string(),
        }
    }
}

/// The main protocol for signature aggregation
pub struct SignatureAggregationProtocol<S, W>
where
    S: AggregatableSignature,
    W: SignatureWeight,
{
    /// Protocol configuration
    config: ProtocolConfig,

    /// Protocol state
    state: AggregationState<S>,

    /// Weight scheme for determining threshold
    weight_scheme: W,

    /// Aggregator selector
    aggregator_selector: AggregatorSelector,

    /// Map of public keys for all participants
    participant_public_keys: HashMap<ParticipantId, S::Public>,
}

impl<S, W> SignatureAggregationProtocol<S, W>
where
    S: AggregatableSignature,
    W: SignatureWeight,
{
    /// Create a new signature aggregation protocol instance
    pub fn new(config: ProtocolConfig, weight_scheme: W) -> Self {
        // Create default state with threshold weight from the weight scheme
        let threshold_weight = weight_scheme.threshold_weight();
        let state = AggregationState::new(config.max_participants, threshold_weight);

        // Create aggregator selector with target number from config
        let aggregator_selector = AggregatorSelector::new(config.num_aggregators);

        Self {
            state,
            config,
            weight_scheme,
            aggregator_selector,
            participant_public_keys: HashMap::new(),
        }
    }

    /// Get protocol ID for testing purposes
    pub fn protocol_id(&self) -> &str {
        &self.config.protocol_id
    }

    /// Check if we've already seen a signature from a participant for a specific message
    fn has_seen_signature_for_message(
        &self,
        participant_id: &ParticipantId,
        message: &[u8],
    ) -> bool {
        self.state
            .signatures_by_message
            .get(message)
            .map(|map| map.contains_key(*participant_id))
            .unwrap_or(false)
    }

    /// Handle a signature share from another participant
    async fn handle_signature_share(
        &mut self,
        sender_id: ParticipantId,
        signature: S::Signature,
        message: Vec<u8>,
        public_keys: &HashMap<ParticipantId, S::Public>,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        debug!(
            "Node {} received signature from {} for message of length {}",
            self.config.local_id.0,
            sender_id.0,
            message.len()
        );

        // Print first few bytes of the message for debugging
        if message.len() > 4 {
            debug!(
                "Node {} received message starts with: {:?}",
                self.config.local_id.0,
                &message[0..4]
            );
        }

        // Important: Check if this message matches our local message by content, not reference
        let message_matches_local =
            message.len() == self.state.local_message.len() && message == self.state.local_message;

        // Use local message instance if content matches to ensure consistent map keys
        let message_to_use = if message_matches_local {
            debug!(
                "Node {} using local message instance for processing signature",
                self.config.local_id.0
            );
            self.state.local_message.clone()
        } else {
            // Try to find an existing matching message in our maps to reuse the same key
            let mut existing_msg = None;
            for (msg, _) in &self.state.signatures_by_message {
                if msg.len() == message.len() && msg == &message {
                    debug!(
                        "Node {} found existing message instance in signatures map",
                        self.config.local_id.0
                    );
                    existing_msg = Some(msg.clone());
                    break;
                }
            }
            existing_msg.unwrap_or_else(|| message.clone())
        };

        // Check if we've already seen this signature from this participant for this message (using the consistent key)
        if self.has_seen_signature_for_message(&sender_id, &message_to_use) {
            debug!(
                "Node {} already has signature from {} for this message",
                self.config.local_id.0, sender_id.0
            );
            return Ok(());
        }

        // Verify the signature
        if !self.verify_signature(sender_id, &signature, &message_to_use, public_keys) {
            debug!(
                "Node {} received invalid signature from {}",
                self.config.local_id.0, sender_id.0
            );

            // If the signature is invalid, mark participant as malicious
            self.mark_participant_malicious(
                sender_id,
                MaliciousEvidence::InvalidSignature {
                    signature,
                    message: message_to_use.clone(),
                },
                network_handle,
            )
            .await?;
            return Ok(());
        }

        debug!(
            "Node {} verified valid signature from {}",
            self.config.local_id.0, sender_id.0
        );

        // Mark that we've seen this participant's signature
        self.state.seen_signatures.add(sender_id);

        // Add the signature to our collection using the consistent message instance
        self.add_signature(sender_id, signature.clone(), message_to_use.clone());

        // Send an acknowledgment to the sender
        self.send_ack(sender_id, &message_to_use, network_handle)
            .await?;

        // Update our local aggregate regardless of whether we're an aggregator
        // This ensures all nodes can build a result when they have enough signatures
        debug!(
            "Node {} updating local aggregate with signature from {}",
            self.config.local_id.0, sender_id.0
        );
        match self.update_aggregate_for_message(&message_to_use) {
            Ok(()) => {
                debug!(
                    "Node {} successfully updated aggregate",
                    self.config.local_id.0
                );
            }
            Err(e) => {
                warn!(
                    "Node {} failed to update aggregate: {:?}",
                    self.config.local_id.0, e
                );
            }
        }

        // Re-gossip the signature to ensure network propagation
        // Always re-gossip received signatures to ensure they reach all nodes
        debug!(
            "Node {} re-gossiping signature from {}",
            self.config.local_id.0, sender_id.0
        );
        let share_msg = AggSigMessage::SignatureShare {
            signature,
            message: message_to_use,
            weight: Some(self.weight_scheme.weight(&sender_id)),
        };
        self.send_message(share_msg, None, network_handle).await?;

        Ok(())
    }

    /// Handle an incoming protocol message
    pub async fn handle_message(
        &mut self,
        protocol_msg: ProtocolMessage<S>,
        public_keys: &HashMap<ParticipantId, S::Public>,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        let routing = protocol_msg.routing.clone();
        let sender_id = routing.sender.id;

        // Deserialize the message
        let message = bincode::deserialize::<AggSigMessage<S>>(&protocol_msg.payload)?;

        match message {
            AggSigMessage::SignatureShare {
                signature,
                message,
                weight,
            } => {
                self.handle_signature_share(
                    sender_id,
                    signature,
                    message,
                    public_keys,
                    network_handle,
                )
                .await
            }
            AggSigMessage::AckSignatures { seen_from, .. } => self.handle_ack_signatures(seen_from),
            AggSigMessage::MaliciousReport { operator, evidence } => {
                self.handle_malicious_report(operator, evidence, public_keys)
            }
            AggSigMessage::ProtocolComplete {
                aggregate_signature,
                message,
                contributors,
            } => self.handle_protocol_complete(
                sender_id,
                aggregate_signature,
                message,
                contributors,
                public_keys,
            ),
        }
    }

    /// Handle an acknowledgment message
    fn handle_ack_signatures(
        &mut self,
        seen_from: std::collections::HashSet<ParticipantId>,
    ) -> Result<(), AggregationError> {
        // Update our seen signatures set with new information
        let seen_set = ParticipantSet::from_hashset(&seen_from, self.config.max_participants);
        self.state.seen_signatures.union(&seen_set);

        Ok(())
    }

    /// Handle a malicious report message
    fn handle_malicious_report(
        &mut self,
        operator: ParticipantId,
        evidence: MaliciousEvidence<S>,
        public_keys: &HashMap<ParticipantId, S::Public>,
    ) -> Result<(), AggregationError> {
        // Verify the evidence
        let is_malicious = self.verify_malicious_evidence(operator, &evidence, public_keys)?;

        if is_malicious {
            // Add to malicious set
            self.state.malicious.add(operator);

            // Remove from current aggregate if present
            let local_message = self.state.local_message.clone();
            if !local_message.is_empty() {
                if let Some((_, contributors)) = self.state.messages.get_mut(&local_message) {
                    if contributors.contains(operator) {
                        contributors.remove(operator);

                        // We need to rebuild the aggregate signature
                        if self.is_aggregator() {
                            self.rebuild_aggregate()?;
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Handle a protocol completion message
    fn handle_protocol_complete(
        &mut self,
        sender_id: ParticipantId,
        aggregate_signature: S::Signature,
        message: Vec<u8>,
        contributors: HashSet<ParticipantId>,
        public_keys: &HashMap<ParticipantId, S::Public>,
    ) -> Result<(), AggregationError> {
        // Skip if protocol is already finalized
        if self.state.round == ProtocolRound::Completion {
            return Ok(());
        }

        // Verify message matches what we're signing
        if message != self.state.local_message {
            warn!(
                "Completion message has mismatched message from {}",
                sender_id.0
            );
            return Ok(());
        }

        // Convert contributors to our ParticipantSet format
        let contributor_set =
            ParticipantSet::from_hashset(&contributors, self.state.max_participants);

        // Check if weight meets threshold
        let total_weight = self.weight_scheme.calculate_weight(&contributor_set);
        if total_weight < self.weight_scheme.threshold_weight() {
            warn!(
                "Completion message with insufficient weight from {}",
                sender_id.0
            );
            return Ok(());
        }

        // Collect signatures and public keys for verification
        let mut signatures = Vec::new();
        let mut public_key_vec = Vec::new();
        let mut missing_data = false;

        for &id in &contributors {
            // Get the participant's signature
            if let Some(sig_map) = self.state.signatures_by_message.get(&message) {
                if let Some(sig) = sig_map.get(id) {
                    if let Some(key) = public_keys.get(&id) {
                        signatures.push(sig.clone());
                        public_key_vec.push(key.clone());
                    } else {
                        missing_data = true;
                        break;
                    }
                } else {
                    missing_data = true;
                    break;
                }
            } else {
                missing_data = true;
                break;
            }
        }

        if missing_data {
            warn!(
                "Missing signatures or public keys for verification from {}",
                sender_id.0
            );
            return Ok(());
        }

        // Aggregate and verify the signatures
        let (agg_sig, agg_pub) = match S::aggregate(&message, &signatures, &public_key_vec) {
            Ok(result) => result,
            Err(e) => {
                warn!("Aggregation error in completion message: {:?}", e);
                return Ok(());
            }
        };

        // Verify the aggregate signature with the new API
        if !S::verify_aggregate(&message, &agg_sig, &agg_pub).unwrap_or(false) {
            warn!(
                "Invalid aggregate signature in completion message from {}",
                sender_id.0
            );
            return Ok(());
        }

        // All checks passed, mark protocol as completed
        self.state.round = ProtocolRound::Completion;
        self.state.verified_completion = Some((aggregate_signature, contributor_set));

        Ok(())
    }

    /// Mark a participant as malicious and broadcast a report
    async fn mark_participant_malicious(
        &mut self,
        participant_id: ParticipantId,
        evidence: MaliciousEvidence<S>,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        self.state.malicious.add(participant_id);

        // Create malicious report
        let report_msg = AggSigMessage::MaliciousReport {
            operator: participant_id,
            evidence,
        };

        // Broadcast report
        self.send_message(report_msg, None, network_handle).await
    }

    /// Send an acknowledgment message to a participant
    async fn send_ack(
        &mut self,
        recipient: ParticipantId,
        message: &[u8],
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        let ack_msg = AggSigMessage::AckSignatures {
            message_hash: self.hash_message(message),
            seen_from: self.state.seen_signatures.to_hashset(),
        };

        // Send acknowledgment only to the sender
        self.send_message(ack_msg, Some(recipient), network_handle)
            .await?;
        self.state.sent_acks.add(recipient);

        Ok(())
    }

    /// Verify a signature is valid
    fn verify_signature(
        &self,
        sender_id: ParticipantId,
        signature: &S::Signature,
        message: &[u8],
        public_keys: &HashMap<ParticipantId, S::Public>,
    ) -> bool {
        if let Some(public_key) = public_keys.get(&sender_id) {
            S::verify(public_key, message, signature)
        } else {
            warn!("Missing public key for {}", sender_id.0);
            false
        }
    }

    /// Verify evidence of malicious behavior
    fn verify_malicious_evidence(
        &self,
        operator: ParticipantId,
        evidence: &MaliciousEvidence<S>,
        public_keys: &HashMap<ParticipantId, S::Public>,
    ) -> Result<bool, AggregationError> {
        match evidence {
            MaliciousEvidence::InvalidSignature { signature, message } => {
                let operator_key = public_keys.get(&operator).ok_or_else(|| {
                    AggregationError::Protocol(format!("Missing public key for {}", operator.0))
                })?;

                // Verify the signature is invalid - handle the Result properly
                let is_valid = S::verify(operator_key, message, signature);
                Ok(!is_valid)
            }

            MaliciousEvidence::Equivocation {
                signature1,
                signature2,
                message1,
                message2,
            } => {
                let operator_key = public_keys.get(&operator).ok_or_else(|| {
                    AggregationError::Protocol(format!("Missing public key for {}", operator.0))
                })?;

                // Messages must be different - signing the same message multiple times is allowed
                if message1 == message2 {
                    return Ok(false); // Not malicious to sign the same message multiple times
                }

                // Both signatures must be valid for their respective messages
                let is_valid1 = S::verify(operator_key, message1, signature1);
                let is_valid2 = S::verify(operator_key, message2, signature2);

                Ok(is_valid1 && is_valid2)
            }
        }
    }

    /// Update the local aggregate for a message
    fn update_aggregate_for_message(&mut self, message: &[u8]) -> Result<(), AggregationError> {
        // Get signatures for this message
        let sig_map = match self.state.signatures_by_message.get(message) {
            Some(map) => map,
            None => {
                // Try to find the message by content comparison
                for (msg, map) in &self.state.signatures_by_message {
                    if msg.len() == message.len() && msg == message {
                        return self.update_aggregate_for_message(msg);
                    }
                }
                debug!("No signatures found for this message");
                return Ok(());
            }
        };

        // Collect valid signatures from non-malicious participants
        let mut signatures = Vec::new();
        let mut contributors = ParticipantSet::new(self.config.max_participants);
        let mut public_keys = Vec::new();

        // Iterate through participants with signatures
        for id_val in 0..self.config.max_participants {
            let id = ParticipantId(id_val);
            if sig_map.contains_key(id) && !self.state.malicious.contains(id) {
                if let Some(sig) = sig_map.get(id) {
                    signatures.push(sig.clone());
                    contributors.add(id);

                    if let Some(pk) = self.participant_public_keys.get(&id) {
                        public_keys.push(pk.clone());
                    }
                }
            }
        }

        if signatures.is_empty() || public_keys.len() != signatures.len() {
            debug!("No valid signatures or missing public keys for this message");
            return Ok(());
        }

        // Aggregate the signatures with the new API
        match S::aggregate(message, &signatures, &public_keys) {
            Ok((agg_sig, _)) => {
                // Store the aggregated signature and contributors in the messages map
                self.state
                    .messages
                    .insert(message.to_vec(), (agg_sig, contributors.clone()));

                // Log threshold information
                let total_weight = self.weight_scheme.calculate_weight(&contributors);
                let threshold_weight = self.weight_scheme.threshold_weight();
                debug!(
                    "Updated signature collection: contributors={}, total_weight={}, threshold={}, sufficient={}",
                    contributors.len(),
                    total_weight,
                    threshold_weight,
                    total_weight >= threshold_weight
                );

                Ok(())
            }
            Err(e) => {
                error!("Aggregation error: {:?}", e);
                Err(AggregationError::AggregationError(format!(
                    "Failed to aggregate: {:?}",
                    e
                )))
            }
        }
    }

    /// Rebuild the aggregate after removing malicious operators
    fn rebuild_aggregate(&mut self) -> Result<(), AggregationError> {
        // If local message is empty, nothing to do
        if self.state.local_message.is_empty() {
            return Ok(());
        }

        // Get the set of valid contributors (non-malicious)
        let valid_contributors = {
            let local_message = self.state.local_message.clone();
            if let Some((_, contributors)) = self.state.messages.get(&local_message) {
                let mut valid = contributors.clone();
                // Remove all malicious participants
                for id in self.state.malicious.iter() {
                    valid.remove(id);
                }
                valid
            } else {
                // No existing message data, nothing to rebuild
                return Ok(());
            }
        };

        if valid_contributors.is_empty() {
            // No valid contributors left, remove the message entry completely
            self.state.messages.remove(&self.state.local_message);
            return Ok(());
        }

        // Collect signatures and public keys from valid contributors
        let mut signatures = Vec::new();
        let mut public_keys = Vec::new();
        let local_message = self.state.local_message.clone();

        if let Some(sig_map) = self.state.signatures_by_message.get(&local_message) {
            for id in valid_contributors.iter() {
                if let Some(sig) = sig_map.get(id) {
                    signatures.push(sig.clone());

                    if let Some(pk) = self.participant_public_keys.get(&id) {
                        public_keys.push(pk.clone());
                    } else {
                        // Missing public key, can't aggregate
                        return Err(AggregationError::Protocol(format!(
                            "Missing public key for {}",
                            id.0
                        )));
                    }
                }
            }
        }

        if signatures.is_empty() || signatures.len() != public_keys.len() {
            // No signatures or missing public keys, remove the message entry
            self.state.messages.remove(&self.state.local_message);
            return Ok(());
        }

        // Aggregate signatures with the new API
        match S::aggregate(&local_message, &signatures, &public_keys) {
            Ok((agg_sig, _)) => {
                // Store the updated aggregate
                self.state.messages.insert(
                    self.state.local_message.clone(),
                    (agg_sig, valid_contributors),
                );
                Ok(())
            }
            Err(e) => Err(AggregationError::AggregationError(format!(
                "Failed to aggregate signatures: {:?}",
                e
            ))),
        }
    }

    /// Hash a message for acknowledgments
    fn hash_message(&self, message: &[u8]) -> Vec<u8> {
        keccak_256(message).to_vec()
    }

    /// Check if this node is selected as an aggregator for the current round
    pub fn is_aggregator(&self) -> bool {
        self.aggregator_selector.is_aggregator::<S>(
            self.config.local_id,
            &self.participant_public_keys,
            &self.state.local_message,
        )
    }

    /// Run the protocol until completion or timeout
    pub async fn run(
        &mut self,
        message: Vec<u8>,
        signing_key: &mut S::Secret,
        public_keys: &HashMap<ParticipantId, S::Public>,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<AggregationResult<S>, AggregationError> {
        debug!("Starting protocol run for node {}", self.config.local_id.0);
        debug!("Protocol timeout set to {:?}", self.config.timeout);

        // Set the local message first to ensure all operations reference the correct message
        self.state.local_message = message.clone();

        // Initialize the signature collection phase - now without a round number
        self.state.round = ProtocolRound::SignatureCollection;

        // Store the public keys for aggregator selection
        self.participant_public_keys = public_keys.clone();

        // Select aggregators based on the message and public keys
        self.aggregator_selector
            .select_aggregators::<S>(&self.participant_public_keys, &self.state.local_message);

        debug!(
            "Node {} is_aggregator: {}",
            self.config.local_id.0,
            self.is_aggregator()
        );

        // Sign and broadcast our signature to all participants
        debug!(
            "Node {} signing and broadcasting message",
            self.config.local_id.0
        );
        self.sign_and_broadcast(&message, signing_key, network_handle)
            .await?;

        // Main protocol loop
        let timeout = Instant::now() + self.config.timeout;
        let mut check_interval = tokio::time::interval(Duration::from_millis(100));
        let mut message_check_interval = tokio::time::interval(Duration::from_millis(50));

        debug!(
            "Node {} entering main protocol loop",
            self.config.local_id.0
        );

        loop {
            tokio::select! {
                _ = message_check_interval.tick() => {
                    // Check for incoming messages from the network
                    let mut network_handle_mut = network_handle.clone();
                    while let Some(protocol_msg) = network_handle_mut.next_protocol_message() {
                        debug!(
                            "Node {} received network message from {}",
                            self.config.local_id.0,
                            protocol_msg.routing.sender.id.0
                        );

                        // Process the incoming message
                        if let Err(e) = self.handle_message(protocol_msg, public_keys, network_handle).await {
                            warn!(
                                "Node {} error handling message: {:?}",
                                self.config.local_id.0, e
                            );
                        }
                    }
                }

                _ = check_interval.tick() => {
                    // Check if we have enough signatures to complete
                    let current_round = self.state.round.clone();

                    match current_round {
                        ProtocolRound::SignatureCollection => {
                            // Check current signature count and total
                            let sig_count = if let Some(sig_map) = self.state.signatures_by_message.get(&self.state.local_message) {
                                sig_map.len()
                            } else {
                                0
                            };

                            debug!(
                                "Node {} collecting signatures: collected {}/{} signatures",
                                self.config.local_id.0,
                                sig_count,
                                self.config.max_participants
                            );

                            // Check if we have enough signatures to complete
                            match self.build_result(&message, current_round) {
                                Ok(Some(result)) => {
                                    // If we're an aggregator, send completion message
                                    if self.is_aggregator() {
                                        // Collect signatures and public keys for all contributors
                                        let mut signatures = Vec::new();
                                        let mut public_keys = Vec::new();
                                        for id in &result.contributors {
                                            if let Some(sig_map) = self.state.signatures_by_message.get(&self.state.local_message) {
                                                if let Some(sig) = sig_map.get(id) {
                                                    signatures.push(sig.clone());
                                                    if let Some(pk) = self.participant_public_keys.get(id) {
                                                        public_keys.push(pk.clone());
                                                    }
                                                }
                                            }
                                        }

                                        // Create a ParticipantSet from contributors
                                        let contributor_set = ParticipantSet::from_hashset(
                                            &result.contributors,
                                            self.config.max_participants
                                        );

                                        // Send completion message
                                        if let Err(e) = self.send_completion_message(
                                            &self.state.local_message,
                                            &signatures,
                                            &public_keys,
                                            &contributor_set,
                                            network_handle
                                        ).await {
                                            warn!("Failed to send completion message: {:?}", e);
                                        }
                                    }

                                    debug!("Protocol completed successfully");
                                    return Ok(result);
                                }
                                Ok(None) => {
                                    debug!(
                                        "Still waiting for signatures"
                                    );
                                }
                                Err(e) => {
                                    warn!("Error building result: {:?}", e);
                                }
                            }
                        }
                        ProtocolRound::Completion => {
                            debug!("Protocol already marked as completed");
                            // Try one more time to build the result
                            return match self.build_result(&message, current_round) {
                                Ok(Some(result)) => Ok(result),
                                Ok(None) => Err(AggregationError::ThresholdNotMet(0, 0)),
                                Err(e) => {
                                    warn!("Final error building result: {:?}", e);
                                    Err(e)
                                }
                            };
                        }
                        _ => {}
                    }
                }

                _ = tokio::time::sleep_until(tokio::time::Instant::from_std(timeout)) => {
                    debug!("Protocol timed out for node {}", self.config.local_id.0);

                    // On timeout, mark as completion round
                    let completion_round = ProtocolRound::Completion;
                    self.state.round = completion_round.clone();

                    // Try to build a result with what we have
                    return match self.build_result(&message, completion_round) {
                        Ok(Some(result)) => Ok(result),
                        _ => Err(AggregationError::Timeout)
                    };
                }
            }
        }
    }

    /// Build a result for the current protocol round, if possible
    fn build_result(
        &mut self,
        message: &[u8],
        round: ProtocolRound,
    ) -> Result<Option<AggregationResult<S>>, AggregationError> {
        debug!("Building result for round {:?}", round);

        // Find the contributors for this message
        let (agg_sig, contributors) = match self.state.messages.get(message) {
            Some((agg_sig, contributors)) => (agg_sig.clone(), contributors.clone()),
            None => {
                // Try to find by comparing contents
                let mut found_entry = None;
                for (msg, entry) in &self.state.messages {
                    if msg.len() == message.len() && msg == message {
                        found_entry = Some(entry.clone());
                        break;
                    }
                }

                match found_entry {
                    Some((agg_sig, contributors)) => (agg_sig, contributors),
                    None => {
                        debug!("No contributors found for this message");
                        return Ok(None);
                    }
                }
            }
        };

        // Check if we have enough signatures to meet threshold
        let total_weight = self.weight_scheme.calculate_weight(&contributors);
        let threshold_weight = self.weight_scheme.threshold_weight();

        debug!(
            "Verifying threshold: contributors={}, total_weight={}, threshold={}, sufficient={}",
            contributors.len(),
            total_weight,
            threshold_weight,
            total_weight >= threshold_weight
        );

        // Check if we've met the threshold
        if total_weight < threshold_weight {
            if matches!(round, ProtocolRound::Completion) {
                return Err(AggregationError::ThresholdNotMet(
                    total_weight as usize,
                    threshold_weight as usize,
                ));
            }
            return Ok(None);
        }

        // Collect signatures and public keys for verification
        let mut signatures = Vec::new();
        let mut public_keys = Vec::new();
        let mut missing_data = false;

        for id in contributors.iter() {
            if let Some(sig_map) = self.state.signatures_by_message.get(message) {
                if let Some(sig) = sig_map.get(id) {
                    signatures.push(sig.clone());

                    if let Some(pk) = self.participant_public_keys.get(&id) {
                        public_keys.push(pk.clone());
                    } else {
                        missing_data = true;
                        break;
                    }
                }
            }
        }

        if missing_data || signatures.is_empty() {
            debug!("Missing data for verification");
            return Ok(None);
        }

        // Verify with the new API (re-aggregate to get proper types)
        match S::aggregate(message, &signatures, &public_keys) {
            Ok((aggregated_sig, aggregated_pub)) => {
                // Verify the aggregated signature
                if !S::verify_aggregate(message, &aggregated_sig, &aggregated_pub).unwrap_or(false)
                {
                    warn!("Aggregated signature verification failed");
                    return Ok(None);
                }

                // Convert the ParticipantSet to a HashSet
                let contributors_hashset = contributors.to_hashset();

                // Create a map of participant weights
                let mut weight_map = HashMap::new();
                for id in &contributors_hashset {
                    weight_map.insert(*id, self.weight_scheme.weight(id));
                }

                // Create a successful result
                Ok(Some(AggregationResult {
                    signature: agg_sig,
                    contributors: contributors_hashset,
                    weights: Some(weight_map),
                    total_weight: Some(total_weight),
                    malicious_participants: self.state.malicious.to_hashset(),
                    completion_round: 0,
                    total_rounds: 0,
                }))
            }
            Err(e) => {
                error!("Aggregation error during verification: {:?}", e);
                Err(AggregationError::AggregationError(format!(
                    "Failed to aggregate: {:?}",
                    e
                )))
            }
        }
    }

    /// New helper method to add a signature to our state
    fn add_signature(
        &mut self,
        participant_id: ParticipantId,
        signature: S::Signature,
        message: Vec<u8>,
    ) {
        debug!(
            "Node {} adding signature from participant {} for message length {}",
            self.config.local_id.0,
            participant_id.0,
            message.len()
        );

        if message.len() > 4 {
            debug!(
                "Node {} signature message starts with: {:02x?}",
                self.config.local_id.0,
                &message[0..4]
            );
        }

        // Check if this is our local message - if so, use the existing instance for consistency
        let msg_to_use = if message == self.state.local_message {
            debug!(
                "Node {} using local message instance for consistency",
                self.config.local_id.0
            );
            self.state.local_message.clone()
        } else {
            // Try to find an existing matching message to reuse
            let mut existing_msg = None;
            for (msg, _) in &self.state.signatures_by_message {
                if msg.len() == message.len() && msg == &message {
                    debug!(
                        "Node {} reusing existing message instance in signatures map",
                        self.config.local_id.0
                    );
                    existing_msg = Some(msg.clone());
                    break;
                }
            }
            existing_msg.unwrap_or_else(|| message)
        };

        // Create signature map for this message if it doesn't exist
        let sig_map = self
            .state
            .signatures_by_message
            .entry(msg_to_use.clone())
            .or_insert_with(|| ParticipantMap::new(self.config.max_participants));

        // Add the signature
        sig_map.insert(participant_id, signature);
        debug!(
            "Node {} added signature from participant {} to signatures map",
            self.config.local_id.0, participant_id.0
        );

        // Update the participant_messages map to track which messages each participant has signed
        let msg_hash = self.hash_message(&msg_to_use);
        self.state
            .participant_messages
            .entry(participant_id)
            .or_insert_with(HashSet::new)
            .insert(msg_hash);

        // Try to update the aggregate right away to ensure consistent state
        if let Err(e) = self.update_aggregate_for_message(&msg_to_use) {
            warn!(
                "Node {} failed immediate aggregate update after adding signature: {:?}",
                self.config.local_id.0, e
            );
        }
    }

    /// Helper to send a protocol message
    async fn send_message(
        &self,
        message: AggSigMessage<S>,
        specific_recipient: Option<ParticipantId>,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        let payload = bincode::serialize(&message)?;

        let recipient = specific_recipient.map(|id| ParticipantInfo {
            id,
            verification_id_key: None, // This would be filled in by the network layer
        });

        let routing = MessageRouting {
            message_id: 0,
            round_id: 0,
            sender: ParticipantInfo {
                id: self.config.local_id,
                verification_id_key: None, // This would be filled in by the network layer
            },
            recipient,
        };

        // Send the message using the NetworkServiceHandle
        network_handle
            .send(routing, payload)
            .map_err(|e| AggregationError::NetworkError(format!("Failed to send message: {}", e)))
    }

    /// Count the total number of signatures we've collected
    fn count_total_signatures(&self) -> usize {
        let mut count = 0;
        for sig_map in self.state.signatures_by_message.values() {
            count += sig_map.len();
        }
        count
    }

    /// Resend our own signature if progress stalls
    async fn resend_local_signature(
        &self,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        // Only resend if we have a local message
        if self.state.local_message.is_empty() {
            return Ok(());
        }

        // Get our signature for the local message
        if let Some(sig_map) = self
            .state
            .signatures_by_message
            .get(&self.state.local_message)
        {
            if let Some(signature) = sig_map.get(self.config.local_id) {
                // Create and send message
                let msg = AggSigMessage::SignatureShare {
                    signature: signature.clone(),
                    message: self.state.local_message.clone(),
                    weight: Some(self.weight_scheme.weight(&self.config.local_id)),
                };

                self.send_message(msg, None, network_handle).await?;
            }
        }

        Ok(())
    }

    /// Check if a participant has equivocated (signed conflicting messages)
    /// Returns evidence if equivocation is detected
    fn check_for_equivocation(
        &self,
        participant_id: ParticipantId,
        new_message: &[u8],
        new_signature: &S::Signature,
    ) -> Option<MaliciousEvidence<S>> {
        let new_message_hash = self.hash_message(new_message);

        // Clone the message hashes to avoid borrow checker issues
        let prev_message_hashes =
            if let Some(hashes) = self.state.participant_messages.get(&participant_id) {
                hashes.clone()
            } else {
                return None;
            };

        // For each previously signed message hash
        for prev_hash in prev_message_hashes {
            // Skip if it's the same message
            if prev_hash == new_message_hash {
                continue;
            }

            // Find the original message and signature for this hash
            for (prev_message, sig_map) in &self.state.signatures_by_message {
                if self.hash_message(prev_message) == prev_hash
                    && sig_map.contains_key(participant_id)
                {
                    if let Some(prev_sig) = sig_map.get(participant_id) {
                        // We found a different message with a valid signature from this participant
                        return Some(MaliciousEvidence::Equivocation {
                            signature1: prev_sig.clone(),
                            signature2: new_signature.clone(),
                            message1: prev_message.clone(),
                            message2: new_message.to_vec(),
                        });
                    }
                }
            }
        }

        None
    }

    /// Sign a message and broadcast the signature
    async fn sign_and_broadcast(
        &mut self,
        message: &[u8],
        signing_key: &mut S::Secret,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        info!("Node {} attempting to sign message", self.config.local_id.0);

        // Sign the message
        let signature = match S::sign_with_secret(signing_key, message) {
            Ok(sig) => {
                info!(
                    "Node {} successfully signed message",
                    self.config.local_id.0
                );
                sig
            }
            Err(e) => {
                error!("Node {} signing error: {:?}", self.config.local_id.0, e);
                return Err(AggregationError::SigningError(format!(
                    "Failed to sign message: {:?}",
                    e
                )));
            }
        };

        // Store our signature
        let id = self.config.local_id;
        self.add_signature(id, signature.clone(), message.to_vec());
        info!(
            "Node {} stored its signature in local state",
            self.config.local_id.0
        );

        // Create and send the signature message
        let sig_msg = AggSigMessage::SignatureShare {
            signature,
            message: message.to_vec(),
            weight: Some(self.weight_scheme.weight(&id)),
        };

        info!(
            "Node {} broadcasting signature to network",
            self.config.local_id.0
        );
        self.send_message(sig_msg, None, network_handle).await?;
        debug!(
            "Node {} successfully broadcasted signature",
            self.config.local_id.0
        );

        Ok(())
    }

    /// Add a helper method to send completion message with the new API
    async fn send_completion_message(
        &self,
        message: &[u8],
        signatures: &[S::Signature],
        public_keys: &[S::Public],
        contributors: &ParticipantSet,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        // Aggregate signatures with the new API
        let (aggregate_signature, _) =
            S::aggregate(message, signatures, public_keys).map_err(|e| {
                AggregationError::AggregationError(format!(
                    "Failed to aggregate signatures: {:?}",
                    e
                ))
            })?;

        // Create completion message
        let complete_msg = AggSigMessage::ProtocolComplete {
            aggregate_signature,
            message: message.to_vec(),
            contributors: contributors.to_hashset(),
        };

        // Broadcast to all nodes
        self.send_message(complete_msg, None, network_handle).await
    }
}
