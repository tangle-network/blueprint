use crate::{bls::*, error::SpCoreError};
use gadget_crypto_bls::bls377::{W3fBls377Public, W3fBls377Signature};
use gadget_crypto_bls::bls381::{W3fBls381Public, W3fBls381Signature};
use gadget_crypto_core::aggregation::AggregatableSignature;
use gadget_std::Zero;
use tnt_bls::{
    EngineBLS, Message, PublicKey, SerializableToBytes, Signature, TinyBLS377, TinyBLS381,
};

macro_rules! impl_aggregatable_sp_bls {
    ($variant:ident) => {
        paste::paste! {
            impl AggregatableSignature for [<Sp $variant>] {
                type AggregatedSignature = [<W3f $variant Signature>];
                type AggregatedPublic = [<W3f $variant Public>];

                fn aggregate(
                    signatures: &[[<Sp $variant Signature>]],
                    public_keys: &[[<Sp $variant Public>]],
                ) -> Result<(Self::AggregatedSignature, Self::AggregatedPublic), SpCoreError> {
                    if signatures.is_empty() || public_keys.is_empty() {
                        return Err(SpCoreError::InvalidInput(
                            "No signatures or public keys provided".into(),
                        ));
                    }

                    if signatures.len() != public_keys.len() {
                        return Err(SpCoreError::InvalidInput(
                            "Mismatched number of signatures and public keys".into(),
                        ));
                    }

                    let public_key_group_elts = public_keys.iter().map(|pk| pk.0.0).collect::<Vec<_>>();
                    let mut aggregated_public_key_g2 = <<[<Tiny $variant:upper>] as EngineBLS>::PublicKeyGroup>::zero();

                    for pk_bytes in public_key_group_elts {
                        let pk = tnt_bls::DoublePublicKey::<[<Tiny $variant:upper>]>::from_bytes(&pk_bytes).unwrap();
                        aggregated_public_key_g2 += pk.1;
                    }

                    let mut aggregated_signature = <<[<Tiny $variant:upper>] as EngineBLS>::SignatureGroup>::zero();

                    for sig in signatures {
                        let signature = tnt_bls::DoubleSignature::<[<Tiny $variant:upper>]>::from_bytes(&sig.0).unwrap();
                        aggregated_signature += signature.0;
                    }

                    Ok((
                        [<W3f $variant Signature>](Signature::<[<Tiny $variant:upper>]>(aggregated_signature)),
                        [<W3f $variant Public>](PublicKey::<[<Tiny $variant:upper>]>(aggregated_public_key_g2)),
                    ))
                }

                fn verify_aggregate(
                    message: &[u8],
                    signature: &[<W3f $variant Signature>],
                    public_key: &[<W3f $variant Public>],
                ) -> Result<bool, SpCoreError> {
                    Ok(Signature::<[<Tiny $variant:upper>]>::verify(
                        &signature.0,
                        &Message::new(b"", message),
                        &public_key.0,
                    ))
                }
            }
        }
    };
}

impl_aggregatable_sp_bls!(Bls377);
impl_aggregatable_sp_bls!(Bls381);
