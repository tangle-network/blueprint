use gadget_crypto::{BytesEncoding, KeyType, aggregation::AggregatableSignature};
use gadget_networking::types::ParticipantId;
use gadget_std::{
    collections::HashMap,
    collections::HashSet,
    hash::{Hash, Hasher},
};
use std::collections::hash_map::DefaultHasher;

/// Simplified mechanism for selecting aggregators in a deterministic way based on public keys.
/// This approach ensures the selection is cryptographically tamper-resistant.
#[derive(Clone, Debug)]
pub struct AggregatorSelector {
    /// Number of desired aggregators (approximate)
    target_aggregators: u16,
}

impl AggregatorSelector {
    /// Create a new aggregator selector with desired number of aggregators
    pub fn new(target_aggregators: u16) -> Self {
        Self {
            target_aggregators: target_aggregators.max(1),
        }
    }

    /// Check if a participant should be an aggregator based on their public key
    pub fn is_aggregator<S: AggregatableSignature>(
        &self,
        participant_id: ParticipantId,
        participants_with_keys: &HashMap<ParticipantId, S::Public>,
        message_context: &[u8],
    ) -> bool {
        if participants_with_keys.is_empty() {
            return false;
        }

        // We need the public key for this participant
        let Some(public_key) = participants_with_keys.get(&participant_id) else {
            return false;
        };

        // Create a deterministic hash from the public key and context
        let mut hasher = DefaultHasher::new();

        // Hash the serialized representation of the public key
        public_key.to_bytes().hash(&mut hasher);

        // Add message context to make the selection unique per protocol instance
        message_context.hash(&mut hasher);

        // Calculate the threshold based on number of participants and target aggregators
        let total_participants = participants_with_keys.len() as u16;
        let selection_threshold = if total_participants <= self.target_aggregators {
            // If we have fewer participants than desired aggregators, everyone is an aggregator
            u64::MAX
        } else {
            // Calculate a threshold that will select approximately target_aggregators nodes
            let selection_ratio = self.target_aggregators as f64 / total_participants as f64;
            (selection_ratio * u64::MAX as f64) as u64
        };

        // Node is an aggregator if its hash is below the threshold
        hasher.finish() < selection_threshold
    }

    /// Get all participants that should be aggregators
    pub fn select_aggregators<S: AggregatableSignature>(
        &self,
        participants_with_keys: &HashMap<ParticipantId, S::Public>,
        message_context: &[u8],
    ) -> HashSet<ParticipantId> {
        participants_with_keys
            .keys()
            .filter(|&id| self.is_aggregator::<S>(*id, participants_with_keys, message_context))
            .copied()
            .collect()
    }

    /// Get the target number of aggregators
    pub fn target_aggregator_count(&self) -> u16 {
        self.target_aggregators
    }
}
