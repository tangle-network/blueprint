use crate::{
    bls377::{W3fBls377, W3fBls377Public, W3fBls377Signature},
    bls381::{W3fBls381, W3fBls381Public, W3fBls381Signature},
    error::BlsError,
};
use blueprint_crypto_core::{KeyType, aggregation::AggregatableSignature};
use blueprint_std::Zero;
use tnt_bls::{PublicKey, Signature, TinyBLS377, TinyBLS381};

macro_rules! impl_aggregatable_bls {
    ($variant:ident) => {
        paste::paste! {
            impl AggregatableSignature for [<W3f $variant>] {
                type AggregatedSignature = [<W3f $variant Signature>];
                type AggregatedPublic = [<W3f $variant Public>];

                fn aggregate(
                    signatures: &[Self::AggregatedSignature],
                    public_keys: &[Self::AggregatedPublic],
                ) -> Result<(Self::AggregatedSignature, Self::AggregatedPublic), BlsError> {
                    if signatures.is_empty() || public_keys.is_empty() {
                        return Err(BlsError::InvalidInput(
                            "No signatures or public keys provided".to_string(),
                        ));
                    }

                    if signatures.len() != public_keys.len() {
                        return Err(BlsError::InvalidInput(
                            "Mismatched number of signatures and public keys".to_string(),
                        ));
                    }

                    let public_key_group_elts = public_keys.iter().map(|pk| pk.0.0).collect::<Vec<_>>();
                    let mut aggregated_public_key =
                        <<[<Tiny $variant:upper>] as tnt_bls::EngineBLS>::PublicKeyGroup>::zero();

                    for pk in public_key_group_elts {
                        aggregated_public_key += pk;
                    }

                    let mut aggregated_signature =
                        <<[<Tiny $variant:upper>] as tnt_bls::EngineBLS>::SignatureGroup>::zero();

                    for sig in signatures {
                        aggregated_signature += sig.0.0;
                    }

                    Ok((
                        [<W3f $variant Signature>](Signature(aggregated_signature)),
                        [<W3f $variant Public>](PublicKey(aggregated_public_key)),
                    ))
                }

                fn verify_aggregate(
                    message: &[u8],
                    signature: &Self::AggregatedSignature,
                    public_key: &Self::AggregatedPublic,
                ) -> Result<bool, BlsError> {
                    Ok(Self::verify(public_key, message, signature))
                }
            }
        }
    };
}

impl_aggregatable_bls!(Bls377);
impl_aggregatable_bls!(Bls381);
