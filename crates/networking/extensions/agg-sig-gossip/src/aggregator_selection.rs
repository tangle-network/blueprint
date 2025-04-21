use crate::{
    AggregationError, AggregationResult, ProtocolRound, SignatureAggregationProtocol,
    SignatureWeight,
};
use blueprint_core::{debug, error, warn};
use blueprint_crypto::{BytesEncoding, aggregation::AggregatableSignature};
use blueprint_std::{
    collections::HashSet,
    collections::{HashMap, hash_map::DefaultHasher},
    hash::{Hash, Hasher},
};
use libp2p::PeerId;

/// Simplified mechanism for selecting aggregators in a deterministic way based on public keys.
/// This approach ensures the selection is cryptographically tamper-resistant.
#[derive(Clone, Debug)]
pub struct AggregatorSelector {
    /// Number of desired aggregators (approximate)
    target_aggregators: u16,
}

impl AggregatorSelector {
    /// Create a new aggregator selector with desired number of aggregators
    #[must_use]
    pub fn new(target_aggregators: u16) -> Self {
        Self {
            target_aggregators: target_aggregators.max(1),
        }
    }

    #[allow(
        clippy::cast_sign_loss,
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation
    )]
    #[must_use]
    /// Check if a participant should be an aggregator based on their public key
    ///
    /// # Arguments
    ///
    /// * `participant_id` - The ID of the participant to check
    /// * `participants_with_keys` - A map of participant IDs to their public keys
    /// * `message_context` - The context of the message being signed
    ///
    /// # Returns
    ///
    /// Returns `true` if the participant should be an aggregator, `false` otherwise
    ///
    /// # Panics
    ///
    /// Panics if the number of participants is greater than `u16::MAX`
    pub fn is_aggregator<S: AggregatableSignature>(
        &self,
        peer_id: PeerId,
        participants_with_keys: &HashMap<PeerId, S::Public>,
        message_context: &[u8],
    ) -> bool {
        if participants_with_keys.is_empty() {
            return false;
        }

        // We need the public key for this participant
        let Some(public_key) = participants_with_keys.get(&peer_id) else {
            return false;
        };

        // Create a deterministic hash from the public key and context
        let mut hasher = DefaultHasher::new();

        // Hash the serialized representation of the public key
        public_key.to_bytes().hash(&mut hasher);

        // Add message context to make the selection unique per protocol instance
        message_context.hash(&mut hasher);

        // Calculate the threshold based on number of participants and target aggregators
        let total_participants = u16::try_from(participants_with_keys.len()).unwrap_or(u16::MAX);
        let selection_threshold = if total_participants <= self.target_aggregators {
            // If we have fewer participants than desired aggregators, everyone is an aggregator
            u64::MAX
        } else {
            // Calculate a threshold that will select approximately target_aggregators nodes
            let selection_ratio =
                f64::from(self.target_aggregators) / f64::from(total_participants);
            (selection_ratio * u64::MAX as f64) as u64
        };

        // Node is an aggregator if its hash is below the threshold
        hasher.finish() < selection_threshold
    }

    /// Get all participants that should be aggregators
    #[must_use]
    pub fn select_aggregators<S: AggregatableSignature>(
        &self,
        participants_with_keys: &HashMap<PeerId, S::Public>,
        message_context: &[u8],
    ) -> HashSet<PeerId> {
        participants_with_keys
            .keys()
            .filter(|&id| self.is_aggregator::<S>(*id, participants_with_keys, message_context))
            .copied()
            .collect()
    }
}

impl<S: AggregatableSignature, W: SignatureWeight> SignatureAggregationProtocol<S, W> {
    /// Check for a given message if we have enough signatures to meet the threshold
    ///
    /// # Arguments
    ///
    /// * `message` - The message to check the threshold for
    ///
    /// # Returns
    ///
    /// Returns the participants that contributed to the message if the threshold is met,
    /// otherwise returns `None`
    ///
    /// # Errors
    ///
    /// Returns an error if the threshold is not met
    ///
    /// # Panics
    ///
    /// Panics if the threshold is not met and the round is `Completion`
    pub fn check_threshold(
        &mut self,
        message: &[u8],
    ) -> Result<Option<HashSet<PeerId>>, AggregationError> {
        match self.state.signatures_by_message.get(message) {
            Some(contributors) => {
                // Filter out malicious contributors
                let mut honest_contributors = HashSet::new();
                for id in contributors.iter() {
                    if !self.state.malicious.contains(&id) {
                        honest_contributors.insert(id.clone());
                    }
                }
                let total_weight = self.weight_scheme.calculate_weight(&honest_contributors);
                let threshold_weight = self.weight_scheme.threshold_weight();
                debug!(
                    "Total weight: {}, threshold weight: {}",
                    total_weight, threshold_weight
                );
                // Check if we've met the threshold
                if total_weight < threshold_weight {
                    if matches!(self.state.round, ProtocolRound::Completion) {
                        return Err(AggregationError::ThresholdNotMet(
                            usize::try_from(total_weight).unwrap(),
                            usize::try_from(threshold_weight).unwrap(),
                        ));
                    }
                    return Ok(None);
                }
                Ok(Some(contributors.clone()))
            }
            None => Ok(None),
        }
    }

    /// Collect signatures and public keys for verification
    #[allow(clippy::type_complexity)]
    fn collect_signatures_and_public_keys(
        &self,
        contributors: &HashSet<PeerId>,
    ) -> Result<(Vec<S::Signature>, Vec<S::Public>), AggregationError> {
        // Collect signatures and public keys for verification
        let mut signatures = Vec::new();
        let mut public_keys = Vec::new();
        for id in contributors.iter() {
            let is_malicious = self.state.malicious.contains(&id);

            if is_malicious {
                continue;
            }

            if let Some((sig, _)) = self.state.seen_signatures.get(&id) {
                signatures.push(sig.clone());
            }

            if let Some(pk) = self.participant_public_keys.get(&id) {
                public_keys.push(pk.clone());
            }
        }

        if signatures.is_empty() {
            debug!("Missing data for verification");
            return Err(AggregationError::MissingData);
        }

        Ok((signatures, public_keys))
    }

    /// Aggregate and verify signatures
    ///
    /// # Errors
    ///
    /// Returns an error if the public keys or signatures are
    /// not valid and it fails to aggregate
    pub fn aggregate_and_verify(
        &mut self,
        message: &[u8],
        contributors: &HashSet<PeerId>,
        maybe_aggregated_signature: Option<S::AggregatedSignature>,
    ) -> Result<Option<AggregationResult<S>>, AggregationError> {
        let (signatures, public_keys) = self.collect_signatures_and_public_keys(contributors)?;

        // Verify with the new API (re-aggregate to get proper types)
        match S::aggregate(&signatures, &public_keys) {
            Ok((aggregated_sig, aggregated_pub)) => {
                // Verify the aggregated signature
                let sig_to_verify = maybe_aggregated_signature.unwrap_or(aggregated_sig);
                if !S::verify_aggregate(message, &sig_to_verify, &aggregated_pub).unwrap_or(false) {
                    warn!("Aggregated signature verification failed");
                    return Ok(None);
                }

                let total_weight = self.weight_scheme.calculate_weight(contributors);

                Ok(Some(AggregationResult {
                    message: message.to_vec(),
                    signature: sig_to_verify,
                    contributors: contributors.clone(),
                    total_weight: Some(total_weight),
                    malicious_participants: self.state.malicious.clone(),
                }))
            }
            Err(e) => {
                error!("Aggregation error during verification: {:?}", e);
                Err(AggregationError::Protocol(format!(
                    "Failed to aggregate: {:?}",
                    e
                )))
            }
        }
    }

    /// Build a result for the current protocol round, if possible
    ///
    /// # Errors
    ///
    /// Returns an error if the threshold is not met
    pub fn build_result(
        &mut self,
        message: &[u8],
        round: &ProtocolRound,
    ) -> Result<Option<AggregationResult<S>>, AggregationError> {
        debug!("Building result for round {:?}", round);

        let threshold_met = self.check_threshold(message)?;
        debug!(
            "Verifying threshold: message={:?}, sufficient={:?}",
            message, threshold_met
        );

        let Some(contributors) = threshold_met else {
            return Ok(None);
        };

        // Verify with the new API (re-aggregate to get proper types)
        let result = self.aggregate_and_verify(message, &contributors, None)?;
        Ok(result)
    }

    /// Verify a result received from another node
    ///
    /// # Errors
    ///
    /// Returns an error if the result is missing data or the threshold is not met
    pub fn verify_result(&mut self, result: &AggregationResult<S>) -> Result<(), AggregationError> {
        match self.aggregate_and_verify(
            &result.message,
            &result.contributors,
            Some(result.signature.clone()),
        ) {
            Ok(Some(_)) => {}
            Ok(None) => return Err(AggregationError::MissingData),
            Err(e) => return Err(e),
        }

        // Check the threshold
        self.check_threshold(&result.message)?;
        Ok(())
    }
}
