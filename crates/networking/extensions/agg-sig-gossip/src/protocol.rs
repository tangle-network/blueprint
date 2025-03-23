use crate::{
    MaliciousEvidence,
    aggregator_selection::AggregatorSelector,
    messages::{AggSigMessage, AggregationResult},
    participants::ParticipantSet,
    protocol_state::{AggregationState, ProtocolRound},
    signature_weight::SignatureWeight,
};
use blueprint_core::{error, info, warn};
use blueprint_crypto::{aggregation::AggregatableSignature, hashing::blake3_256};
use blueprint_networking::{
    service_handle::NetworkServiceHandle,
    types::{MessageRouting, ParticipantId, ParticipantInfo, ProtocolMessage},
};
use blueprint_std::{
    collections::{HashMap, HashSet},
    fmt::Debug,
    time::{Duration, Instant},
};
use thiserror::Error;
use tracing::debug;

/// Error types for the aggregation protocol
#[derive(Debug, Error)]
pub enum AggregationError {
    #[error("Threshold not met: got {0}, need {1}")]
    ThresholdNotMet(usize, usize),

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

    #[error("Signing error: {0}")]
    SigningError(String),

    #[error("Missing data")]
    MissingData,
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
    pub config: ProtocolConfig,

    /// Protocol state
    pub state: AggregationState<S>,

    /// Weight scheme for determining threshold
    pub weight_scheme: W,

    /// Aggregator selector
    pub aggregator_selector: AggregatorSelector,

    /// Map of public keys for all participants
    pub participant_public_keys: HashMap<ParticipantId, S::Public>,

    /// Set of messages we've re-gossiped
    pub messages_re_gossiped: HashSet<[u8; 32]>,
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
            messages_re_gossiped: HashSet::new(),
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
            .map(|map| map.contains(&participant_id))
            .unwrap_or(false)
    }

    /// Handle a signature share from another participant
    async fn handle_signature_share(
        &mut self,
        sender_id: ParticipantId,
        signer_id: ParticipantId,
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

        // Check if we've already seen this signature from this participant for this message (using the consistent key)
        if self.has_seen_signature_for_message(&signer_id, &message) {
            debug!(
                "Node {} already has signature from {} for this message",
                self.config.local_id.0, signer_id.0
            );
            return Ok(());
        }

        // Verify the signature
        if !self.verify_signature(signer_id, &signature, &message, public_keys) {
            debug!(
                "Node {} received invalid signature from {}",
                self.config.local_id.0, signer_id.0
            );

            // If the signature is invalid, mark sender as malicious
            // The sender is the one who is gossiping the signature.
            // Either (1) they are sending an invalid signature from themselves
            // or (2) they are relaying an invalid signature from another participant.
            // In both cases, we should mark them as malicious.
            self.mark_participant_malicious(
                sender_id,
                MaliciousEvidence::InvalidSignature {
                    message: message.clone(),
                    signature,
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

        // Check for equivocation (signing a new message with the same key)
        self.check_for_equivocation(signer_id, &message, &signature);

        // Add the signature to our collection using the consistent message instance
        self.add_signature(sender_id, signature.clone(), message.clone());

        // Re-gossip the signature to ensure network propagation
        let msg_hash = blake3_256(message.as_slice());
        if !self.messages_re_gossiped.contains(&msg_hash) {
            debug!(
                "Node {} re-gossiping signature from {}",
                self.config.local_id.0, sender_id.0
            );
            let share_msg = AggSigMessage::SignatureShare {
                signer_id,
                signature,
                message,
            };
            self.send_message(share_msg, None, network_handle).await?;
            self.messages_re_gossiped.insert(msg_hash);
        }

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
                signer_id,
                signature,
                message,
            } => {
                self.handle_signature_share(
                    sender_id,
                    signer_id,
                    signature,
                    message,
                    public_keys,
                    network_handle,
                )
                .await
            }
            AggSigMessage::MaliciousReport { operator, evidence } => {
                self.handle_malicious_report(operator, evidence, public_keys)
            }
            AggSigMessage::ProtocolComplete(result) => self.handle_protocol_complete(result),
        }
    }

    /// Handle a protocol completion message
    fn handle_protocol_complete(
        &mut self,
        result: AggregationResult<S>,
    ) -> Result<(), AggregationError> {
        // Skip if protocol is already finalized
        if self.state.round == ProtocolRound::Completion {
            return Ok(());
        }

        self.verify_result(&result)?;

        // All checks passed, mark protocol as completed
        self.state.round = ProtocolRound::Completion;
        self.state.verified_completion = Some(result);

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
                            if !self.is_aggregator() {
                                continue;
                            }

                            // Find the message with the highest total weight
                            let (highest_weight_message, highest_weight) = self.get_highest_weight_message();
                            debug!(
                                "Node {} highest weight message: {:?} (weight {})",
                                self.config.local_id.0,
                                highest_weight_message.clone(),
                                highest_weight
                            );

                            // Check if the highest weight message meets threshold
                            match self.build_result(&highest_weight_message, current_round) {
                                Ok(Some(result)) => {
                                    // Send completion message
                                    if let Err(e) = self.send_completion_message(
                                        result.clone(),
                                        network_handle
                                    ).await {
                                        warn!("Failed to send completion message: {:?}", e);
                                    }

                                    debug!("Protocol completed successfully with highest weight message");
                                    return Ok(result);
                                }
                                Ok(None) => {
                                    debug!(
                                        "Highest weight message doesn't meet threshold yet"
                                    );
                                }
                                Err(e) => {
                                    warn!("Error building result for highest weight message: {:?}", e);
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

        // Create signature map for this message if it doesn't exist
        let sig_map = self
            .state
            .signatures_by_message
            .entry(message.clone())
            .or_insert_with(|| ParticipantSet::new(self.config.max_participants));

        // Add the signature
        sig_map.add(participant_id);

        // Add the signature to the seen signatures map
        self.state
            .seen_signatures
            .insert(participant_id, (signature.clone(), message.to_vec()));
        debug!(
            "Node {} added signature from participant {} to signatures map",
            self.config.local_id.0, participant_id.0
        );
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
            signer_id: id,
            signature,
            message: message.to_vec(),
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
        aggregation_result: AggregationResult<S>,
        network_handle: &NetworkServiceHandle<S>,
    ) -> Result<(), AggregationError> {
        // Create completion message
        let complete_msg = AggSigMessage::ProtocolComplete(aggregation_result);

        // Broadcast to all nodes
        self.send_message(complete_msg, None, network_handle).await
    }

    fn get_highest_weight_message(&self) -> (Vec<u8>, u64) {
        let mut highest_weight = 0;
        let mut highest_weight_message = Vec::new();

        for (message, sig_map) in self.state.signatures_by_message.iter() {
            let weight = self.weight_scheme.calculate_weight(sig_map);
            if weight > highest_weight {
                highest_weight = weight;
                highest_weight_message = message.clone();
            }
        }

        (highest_weight_message, highest_weight)
    }
}
