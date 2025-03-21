use crate::participants::ParticipantMap;
use crate::{
    messages::{AggregationResult, MaliciousEvidence},
    participants::ParticipantSet,
};
use gadget_crypto::aggregation::AggregatableSignature;
use gadget_networking::types::ParticipantId;
use gadget_std::{
    collections::{HashMap, HashSet},
    fmt::{Display, Formatter, Result as FmtResult},
};

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
    /// Messages being signed, with their corresponding aggregates
    /// Map from message to (aggregate signature, contributors)
    pub messages: HashMap<Vec<u8>, (S::Signature, ParticipantSet)>,

    /// Signatures received from participants, keyed by message and participant ID
    /// Map from message to map of participant IDs to signatures
    pub signatures_by_message: HashMap<Vec<u8>, ParticipantMap<S::Signature>>,

    /// Set of participants and which messages they've signed
    /// Map from participant ID to set of message hashes they've signed
    pub participant_messages: HashMap<ParticipantId, HashSet<Vec<u8>>>,

    /// Our own message we're signing (to differentiate from other messages we see)
    pub local_message: Vec<u8>,

    /// Set of participants identified as malicious
    pub malicious: ParticipantSet,

    /// Set of participants we've seen signatures from (across all messages)
    pub seen_signatures: ParticipantSet,

    /// Set of participants we've sent ACKs to
    pub sent_acks: ParticipantSet,

    /// Current protocol round
    pub round: ProtocolRound,

    /// Verified aggregate signature from a completion message
    pub verified_completion: Option<(S::Signature, ParticipantSet)>,

    /// Maximum number of participants
    pub max_participants: u16,
}

impl<S: AggregatableSignature> AggregationState<S> {
    /// Create a new aggregation state
    pub fn new(max_participants: u16, threshold_weight: u64) -> Self {
        Self {
            messages: HashMap::new(),
            signatures_by_message: HashMap::new(),
            participant_messages: HashMap::new(),
            local_message: Vec::new(),
            malicious: ParticipantSet::new(max_participants),
            seen_signatures: ParticipantSet::new(max_participants),
            sent_acks: ParticipantSet::new(max_participants),
            round: ProtocolRound::Initialization,
            verified_completion: None,
            max_participants,
        }
    }

    /// Check if we've seen a signature from a participant
    pub fn has_seen_signature(&self, participant_id: ParticipantId) -> bool {
        self.seen_signatures.contains(participant_id)
    }

    /// Get missing participants (we haven't seen signatures from)
    pub fn get_missing_participants(&self) -> Vec<ParticipantId> {
        let mut missing = Vec::new();

        for id in 0..self.max_participants {
            let participant_id = ParticipantId(id);

            // Skip if malicious or already seen
            if self.malicious.contains(participant_id)
                || self.seen_signatures.contains(participant_id)
            {
                continue;
            }

            missing.push(participant_id);
        }

        missing
    }
}
