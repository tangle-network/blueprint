use core::fmt::Debug;
use serde::{Serialize, de::DeserializeOwned};

use crate::KeyType;

/// Trait defining the requirements for an aggregatable signature scheme
pub trait AggregatableSignature: KeyType {
    type AggregatedSignature: Clone + Debug + Serialize + DeserializeOwned + Send + Sync;
    type AggregatedPublic: Clone + Debug + Serialize + DeserializeOwned + Send + Sync;

    /// Aggregates signatures and public keys
    fn aggregate(
        signatures: &[Self::Signature],
        public_keys: &[Self::Public],
    ) -> Result<(Self::AggregatedSignature, Self::AggregatedPublic), Self::Error>;

    /// Verifies them
    fn verify_aggregate(
        message: &[u8],
        signature: &Self::AggregatedSignature,
        public_key: &Self::AggregatedPublic,
    ) -> Result<bool, Self::Error>;
}

pub trait WeightedAggregatableSignature: AggregatableSignature {
    fn verify_weighted_aggregate(
        message: &[u8],
        signatures: &[Self::Signature],
        public_keys_and_weights: &[(Self::Public, u64)],
        threshold: u64,
    ) -> bool {
        let public_keys = public_keys_and_weights
            .iter()
            .map(|(pk, _)| pk.clone())
            .collect::<Vec<_>>();
        let weight_sum = public_keys_and_weights.iter().map(|(_, w)| *w).sum::<u64>();

        match Self::aggregate(signatures, &public_keys) {
            Ok((aggregated_signature, aggregated_public)) => {
                match Self::verify_aggregate(message, &aggregated_signature, &aggregated_public) {
                    Ok(is_valid) => {
                        if !is_valid {
                            return false;
                        }
                    }
                    Err(_) => return false,
                }

                if weight_sum < threshold {
                    return false;
                }

                true
            }
            Err(_) => false,
        }
    }
}
