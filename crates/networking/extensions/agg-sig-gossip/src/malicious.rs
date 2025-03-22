use std::collections::HashMap;

use crate::{AggregationError, SignatureAggregationProtocol, SignatureWeight};
use gadget_crypto::{aggregation::AggregatableSignature, hashing::blake3_256};
use gadget_networking::types::ParticipantId;
use serde::{Deserialize, Serialize};

/// Evidence of malicious behavior
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(bound = "S: AggregatableSignature")]
pub enum MaliciousEvidence<S: AggregatableSignature> {
    /// Invalid signature that doesn't verify
    InvalidSignature {
        /// The signer's ID
        signer_id: ParticipantId,
        /// Signature
        signature: S::Signature,
        /// Message being signed
        message: Vec<u8>,
    },
    /// Conflicting valid signatures for different messages in the same round
    Equivocation {
        /// First signature
        signature1: S::Signature,
        /// Second signature
        signature2: S::Signature,
        /// First message being signed
        message1: Vec<u8>,
        /// Second message being signed
        message2: Vec<u8>,
    },
}

impl<S: AggregatableSignature, W: SignatureWeight> SignatureAggregationProtocol<S, W> {
    /// Handle a malicious report message
    pub fn handle_malicious_report(
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
        }

        Ok(())
    }

    /// Verify evidence of malicious behavior
    fn verify_malicious_evidence(
        &self,
        operator: ParticipantId,
        evidence: &MaliciousEvidence<S>,
        public_keys: &HashMap<ParticipantId, S::Public>,
    ) -> Result<bool, AggregationError> {
        match evidence {
            MaliciousEvidence::InvalidSignature {
                signature,
                message,
                signer_id,
            } => {
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

    /// Check if a participant has equivocated (signed conflicting messages)
    /// Returns evidence if equivocation is detected
    pub fn check_for_equivocation(
        &self,
        participant_id: ParticipantId,
        new_message: &[u8],
        new_signature: &S::Signature,
    ) -> Option<MaliciousEvidence<S>> {
        if let Some((signature, message)) = self.state.seen_signatures.get(&participant_id) {
            if signature == new_signature && message != new_message {
                return Some(MaliciousEvidence::Equivocation {
                    signature1: signature.clone(),
                    signature2: new_signature.clone(),
                    message1: message.clone(),
                    message2: new_message.to_vec(),
                });
            }
        }

        None
    }
}
