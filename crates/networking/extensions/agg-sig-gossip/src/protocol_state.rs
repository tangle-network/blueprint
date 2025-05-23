use std::collections::HashSet;

use crate::AggregationResult;
use blueprint_crypto::aggregation::AggregatableSignature;
use blueprint_std::{
    collections::HashMap,
    fmt::{Display, Formatter, Result as FmtResult},
};
use libp2p::PeerId;

/// Protocol rounds for the signature aggregation protocol
/// This makes the protocol flow more explicit and easier to debug
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProtocolRound {
    /// Initial setup phase
    Initialization,

    /// Actively collecting signatures, with round number
    SignatureCollection,

    /// Finalizing aggregation and verifying threshold
    Completion,
}

impl Display for ProtocolRound {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            ProtocolRound::Initialization => write!(f, "Initialization"),
            ProtocolRound::SignatureCollection => {
                write!(f, "Signature Collection")
            }
            ProtocolRound::Completion => write!(f, "Completion"),
        }
    }
}

/// State of the aggregation protocol for a single round
#[derive(Clone)]
pub struct AggregationState<S: AggregatableSignature> {
    /// Signatures received from participants, keyed by message and participant ID
    /// Map from message to map of participant IDs to signatures
    pub signatures_by_message: HashMap<Vec<u8>, HashSet<PeerId>>,

    /// Set of participants and their signature and message
    pub seen_signatures: HashMap<PeerId, (S::Signature, Vec<u8>)>,

    /// Our own message we're signing (to differentiate from other messages we see)
    pub local_message: Vec<u8>,

    /// Set of participants identified as malicious
    pub malicious: HashSet<PeerId>,

    /// Set of participants we've sent ACKs to
    pub sent_acks: HashSet<PeerId>,

    /// Current protocol round
    pub round: ProtocolRound,

    /// Verified aggregate result from a completion message
    pub verified_completion: Option<AggregationResult<S>>,

    /// Threshold weight
    pub threshold_weight: u64,
}

impl<S: AggregatableSignature> AggregationState<S> {
    /// Create a new aggregation state
    #[must_use]
    pub fn new(threshold_weight: u64) -> Self {
        Self {
            signatures_by_message: HashMap::new(),
            local_message: Vec::new(),
            malicious: HashSet::new(),
            seen_signatures: HashMap::new(),
            sent_acks: HashSet::new(),
            round: ProtocolRound::Initialization,
            verified_completion: None,
            threshold_weight,
        }
    }
}
