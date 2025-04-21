mod protocol;
pub use protocol::{AggregationError, ProtocolConfig, SignatureAggregationProtocol};

// State management
mod protocol_state;
pub use protocol_state::{AggregationState, ProtocolRound};

// Aggregator selection
mod aggregator_selection;
pub use aggregator_selection::AggregatorSelector;

// Malicious detection
mod malicious;
pub use malicious::MaliciousEvidence;

// Message types
mod messages;
pub use messages::{AggSigMessage, AggregationResult};

// Signature weighting schemes
mod signature_weight;
pub use signature_weight::{CustomWeight, EqualWeight, SignatureWeight};

#[cfg(test)]
mod tests;
