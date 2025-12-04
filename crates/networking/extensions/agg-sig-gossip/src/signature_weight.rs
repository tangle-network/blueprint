use std::collections::HashSet;

use libp2p::PeerId;

/// Trait for weighting of participants in signature aggregation
pub trait SignatureWeight {
    /// Returns the weight for a participant
    fn weight(&self, peer_id: &PeerId) -> u64;

    /// Returns the total weight of all participants
    fn total_weight(&self) -> u64;

    /// Returns the threshold weight required for a valid aggregate
    fn threshold_weight(&self) -> u64;

    /// Calculates the total weight of a set of participants
    /// Uses saturating arithmetic to prevent overflow
    fn calculate_weight(&self, participants: &HashSet<PeerId>) -> u64 {
        participants
            .iter()
            .fold(0u64, |acc, id| acc.saturating_add(self.weight(id)))
    }

    /// Checks if a set of participants meets the required threshold
    fn meets_threshold(&self, participants: &HashSet<PeerId>) -> bool {
        self.calculate_weight(participants) >= self.threshold_weight()
    }
}

/// A simple equal-weight implementation
pub struct EqualWeight {
    total_participants: usize,
    threshold_percentage: u8,
}

impl EqualWeight {
    #[must_use]
    /// Creates a new `EqualWeight` instance
    ///
    /// # Panics
    ///
    /// Panics if the threshold percentage is greater than 100
    pub fn new(total_participants: usize, threshold_percentage: u8) -> Self {
        assert!(
            threshold_percentage <= 100,
            "Threshold percentage must be <= 100"
        );
        Self {
            total_participants,
            threshold_percentage,
        }
    }
}

impl SignatureWeight for EqualWeight {
    fn weight(&self, _peer_id: &PeerId) -> u64 {
        1
    }

    fn total_weight(&self) -> u64 {
        self.total_participants as u64
    }

    fn threshold_weight(&self) -> u64 {
        // Use saturating multiplication to prevent overflow
        let product = (self.total_participants as u64).saturating_mul(u64::from(self.threshold_percentage));
        product / 100
    }
}

/// A custom weight map implementation
pub struct CustomWeight {
    weights: std::collections::HashMap<PeerId, u64>,
    threshold_weight: u64,
}

impl CustomWeight {
    #[must_use]
    pub fn new(weights: std::collections::HashMap<PeerId, u64>, threshold_weight: u64) -> Self {
        Self {
            weights,
            threshold_weight,
        }
    }
}

impl SignatureWeight for CustomWeight {
    fn weight(&self, participant_id: &PeerId) -> u64 {
        *self.weights.get(participant_id).unwrap_or(&0)
    }

    fn total_weight(&self) -> u64 {
        // Use saturating arithmetic to prevent overflow
        self.weights.values().fold(0u64, |acc, &w| acc.saturating_add(w))
    }

    fn threshold_weight(&self) -> u64 {
        self.threshold_weight
    }
}

/// A dynamic weight scheme that can be either equal-weight or custom-weight
///
/// This enum allows switching between weight schemes at runtime without
/// needing to specify the concrete type at compile time.
pub enum DynamicWeight {
    /// Equal weight for all participants
    Equal(EqualWeight),
    /// Custom weights per participant
    Custom(CustomWeight),
}

impl DynamicWeight {
    /// Create a new equal-weight scheme
    #[must_use]
    pub fn equal(total_participants: usize, threshold_percentage: u8) -> Self {
        Self::Equal(EqualWeight::new(total_participants, threshold_percentage))
    }

    /// Create a new custom-weight scheme
    #[must_use]
    pub fn custom(weights: std::collections::HashMap<PeerId, u64>, threshold_weight: u64) -> Self {
        Self::Custom(CustomWeight::new(weights, threshold_weight))
    }
}

impl SignatureWeight for DynamicWeight {
    fn weight(&self, participant_id: &PeerId) -> u64 {
        match self {
            DynamicWeight::Equal(w) => w.weight(participant_id),
            DynamicWeight::Custom(w) => w.weight(participant_id),
        }
    }

    fn total_weight(&self) -> u64 {
        match self {
            DynamicWeight::Equal(w) => w.total_weight(),
            DynamicWeight::Custom(w) => w.total_weight(),
        }
    }

    fn threshold_weight(&self) -> u64 {
        match self {
            DynamicWeight::Equal(w) => w.threshold_weight(),
            DynamicWeight::Custom(w) => w.threshold_weight(),
        }
    }
}
