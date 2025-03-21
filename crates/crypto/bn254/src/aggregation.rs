use crate::{ArkBlsBn254, ArkBlsBn254Public, ArkBlsBn254Signature, error::Bn254Error};
use ark_bn254::{G1Affine, G2Affine};
use ark_ec::AffineRepr;
use gadget_crypto_core::{KeyType, aggregation::AggregatableSignature};

impl AggregatableSignature for ArkBlsBn254 {
    fn aggregate_public_keys(public_keys: &[ArkBlsBn254Public]) -> ArkBlsBn254Public {
        if public_keys.is_empty() {
            return ArkBlsBn254Public(G2Affine::zero());
        }

        let mut aggregated_public_key = G2Affine::zero();
        for key in public_keys {
            let value = aggregated_public_key + key.0;
            aggregated_public_key = value.into();
        }

        ArkBlsBn254Public(aggregated_public_key)
    }

    fn verify_aggregate(
        message: &[u8],
        signature: &ArkBlsBn254Signature,
        public_keys: &[ArkBlsBn254Public],
    ) -> bool {
        let mut aggregated_public_key: G2Affine = G2Affine::zero();

        for key in public_keys {
            let value = aggregated_public_key + key.0;
            aggregated_public_key = value.into();
        }

        ArkBlsBn254::verify(
            &ArkBlsBn254Public(aggregated_public_key),
            message,
            signature,
        )
    }

    fn aggregate(signatures: &[ArkBlsBn254Signature]) -> Result<ArkBlsBn254Signature, Bn254Error> {
        if signatures.is_empty() {
            return Err(Bn254Error::InvalidInput("Empty signatures".to_string()));
        }

        let mut aggregated_signature = G1Affine::zero();
        for signature in signatures.iter() {
            let value = aggregated_signature + signature.0;
            aggregated_signature = value.into();
        }

        Ok(ArkBlsBn254Signature(aggregated_signature))
    }
}
