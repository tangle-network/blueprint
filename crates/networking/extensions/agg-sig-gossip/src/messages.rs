use crate::MaliciousEvidence;
use blueprint_crypto::aggregation::AggregatableSignature;
use blueprint_std::{collections::HashSet, fmt::Debug};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

/// Protocol message types for signature aggregation
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "S: AggregatableSignature")]
pub enum AggSigMessage<S: AggregatableSignature> {
    /// Initial signature share from a participant
    SignatureShare {
        /// The signer's ID, since we allow re-gossiping of signatures
        signer_id: PeerId,
        /// The signature
        signature: S::Signature,
        /// The message being signed
        message: Vec<u8>,
    },
    /// Report malicious behavior
    MaliciousReport {
        /// The accused operator
        operator: PeerId,
        /// Evidence of malicious behavior
        evidence: MaliciousEvidence<S>,
    },
    /// Protocol completion message
    /// Sent when a node has enough signatures to meet the threshold
    ProtocolComplete(AggregationResult<S>),
}

/// Information about aggregators for the current round
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorInfo {
    /// Set of designated aggregators for the current round
    pub aggregators: HashSet<PeerId>,
    /// Selection seed used to determine aggregators
    pub selection_seed: Vec<u8>,
}

/// Result of the aggregation protocol
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "S: AggregatableSignature")]
pub struct AggregationResult<S: AggregatableSignature> {
    /// The message being signed
    pub message: Vec<u8>,
    /// The aggregated signature
    pub signature: S::AggregatedSignature,
    /// Set of participants who contributed to the signature
    pub contributors: HashSet<PeerId>,
    /// Total weight of the aggregate signature
    pub total_weight: Option<u64>,
    /// Set of participants identified as malicious
    pub malicious_participants: HashSet<PeerId>,
}
