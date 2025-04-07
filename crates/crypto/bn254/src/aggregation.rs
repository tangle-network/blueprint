use crate::{ArkBlsBn254, ArkBlsBn254Public, ArkBlsBn254Signature, error::Bn254Error};
use ark_bn254::{G1Affine, G2Affine};
use ark_ec::AffineRepr;
use blueprint_crypto_core::{KeyType, aggregation::AggregatableSignature};
use blueprint_std::string::ToString;

impl AggregatableSignature for ArkBlsBn254 {
    type AggregatedSignature = ArkBlsBn254Signature;
    type AggregatedPublic = ArkBlsBn254Public;

    fn aggregate(
        signatures: &[ArkBlsBn254Signature],
        public_keys: &[ArkBlsBn254Public],
    ) -> Result<(ArkBlsBn254Signature, ArkBlsBn254Public), Bn254Error> {
        if signatures.is_empty() || public_keys.is_empty() {
            return Err(Bn254Error::InvalidInput(
                "No signatures or public keys provided".to_string(),
            ));
        }

        if signatures.len() != public_keys.len() {
            return Err(Bn254Error::InvalidInput(
                "Mismatched number of signatures and public keys".to_string(),
            ));
        }

        let mut aggregated_public_key: G2Affine = G2Affine::zero();
        let mut aggregated_signature: G1Affine = G1Affine::zero();
        for (i, key) in public_keys.iter().enumerate() {
            // Aggregate public keys
            let value = aggregated_public_key + key.0;
            aggregated_public_key = value.into();

            // Aggregate signatures
            let value = aggregated_signature + signatures[i].0;
            aggregated_signature = value.into();
        }

        Ok((
            ArkBlsBn254Signature(aggregated_signature),
            ArkBlsBn254Public(aggregated_public_key),
        ))
    }

    fn verify_aggregate(
        message: &[u8],
        signature: &ArkBlsBn254Signature,
        public_key: &ArkBlsBn254Public,
    ) -> Result<bool, Bn254Error> {
        Ok(ArkBlsBn254::verify(public_key, message, signature))
    }
}
