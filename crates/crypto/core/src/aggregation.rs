use crate::KeyType;

/// Trait defining the requirements for an aggregatable signature scheme
pub trait AggregatableSignature: KeyType {
    /// Generate the aggregate public key from a list of public keys
    fn aggregate_public_keys(public_keys: &[Self::Public]) -> Self::Public;

    /// Verifies the signature against multiple public keys (for aggregated signatures)
    fn verify_aggregate(
        message: &[u8],
        signature: &Self::Signature,
        public_keys: &[Self::Public],
    ) -> bool;

    /// Aggregates this signature with another signature
    fn aggregate(signatures: &[Self::Signature]) -> Result<Self::Signature, Self::Error>;
}

pub trait WeightedAggregatableSignature: AggregatableSignature {
    fn verify_aggregate(
        message: &[u8],
        signature: &Self::Signature,
        public_keys_and_weights: &[(Self::Public, u64)],
        threshold: u64,
    ) -> bool;
}
