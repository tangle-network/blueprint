// Re-export main types
mod protocol;
pub use protocol::{AggregationError, ProtocolConfig, SignatureAggregationProtocol};

// State management
mod protocol_state;
pub use protocol_state::{AggregationState, ProtocolRound};

// Aggregator selection
mod aggregator_selection;
pub use aggregator_selection::AggregatorSelector;

// Participant data structures
mod participants;
pub use participants::{ParticipantMap, ParticipantSet};

// Message types
mod messages;
pub use messages::{AggSigMessage, AggregationResult, MaliciousEvidence};

// Signature weighting schemes
mod signature_weight;
pub use signature_weight::{CustomWeight, EqualWeight, SignatureWeight};

// ZK proof generation (optional component)
mod zk_proof;
pub use zk_proof::{ThresholdProofGenerator, ThresholdWeightProof};

#[cfg(test)]
mod tests;
