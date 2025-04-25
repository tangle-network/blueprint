use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use crate::pricing_engine;
use blueprint_crypto::{BytesEncoding, KeyType};
use parity_scale_codec::Encode;
use tangle_subxt::subxt::utils::AccountId32;

pub type BlueprintId = u64;
pub type OperatorId = AccountId32;

#[derive(Debug, Clone)]
pub struct SignedQuote<K: KeyType> {
    pub quote_details: pricing_engine::QuoteDetails,
    pub signature: K::Signature,
    pub operator_id: OperatorId,
    pub proof_of_work: Vec<u8>,
}

pub struct OperatorSigner<K: KeyType> {
    keypair: K::Secret,
    operator_id: OperatorId,
}

impl<K: KeyType> OperatorSigner<K> {
    /// Creates a new Operator Signer
    pub fn new(
        _config: &OperatorConfig,
        keypair: K::Secret,
        operator_id: OperatorId,
    ) -> Result<Self> {
        Ok(OperatorSigner {
            keypair,
            operator_id,
        })
    }

    /// Returns a signed quote,made up of the quote details, signature, operator ID, and proof of work
    pub fn sign_quote(
        &mut self,
        quote_details: pricing_engine::QuoteDetails,
        proof_of_work: Vec<u8>,
    ) -> Result<SignedQuote<K>> {
        // Hash the quote details
        let hash = hash_quote_details(&quote_details)?;
        blueprint_core::info!("Hash: {:?}", hash);

        // Sign the hash
        let signature = K::sign_with_secret(&mut self.keypair, &hash)
            .map_err(|e| PricingError::Signing(format!("Error {:?} signing quote hash", e)))?;
        // let signature_bytes = signature.clone().to_bytes();

        // let public_key = K::public_from_secret(&self.keypair);

        // let signature_bytes = if signature_bytes.len() == 65 {
        //     signature_bytes[..65].try_into().unwrap()
        // } else if signature_bytes.len() == 64 {
        //     let mut sig_array = [0u8; 65];
        //     sig_array[0..64].copy_from_slice(&signature_bytes[..64]);
        //     sig_array[64] = 0;
        //     sig_array
        // } else {
        //     panic!("Unexpected signature length: {}", signature_bytes.len());
        // };
        // blueprint_core::info!("Signature: {:?}", signature_bytes);

        // let public_key_bytes = public_key.to_bytes();
        // let public_key_bytes = if public_key_bytes.len() == 33 {
        //     public_key_bytes[..33].try_into().unwrap()
        // } else if public_key_bytes.len() == 32 {
        //     let mut pub_array = [0u8; 33];
        //     pub_array[0..32].copy_from_slice(&public_key_bytes);
        //     pub_array[32] = 0;
        //     pub_array
        // } else {
        //     panic!("Unexpected public key length: {}", public_key_bytes.len());
        // };
        // blueprint_core::info!("Public key: {:?}", public_key_bytes);
        // assert!(sp_io::crypto::ecdsa_verify(&signature_bytes.into(), &hash, &public_key_bytes.into()));

        // // assert!(sp_io::crypto::ecdsa_verify_prehashed(&signature_bytes.into(), &hash, &public_key_bytes.into()));

        // // assert!(sp_io::crypto::ecdsa_verify(&signature.0, &hash, &public_key.0));

        // assert!(K::verify(&public_key, &hash, &signature));

        Ok(SignedQuote {
            quote_details,
            signature,
            operator_id: self.operator_id.clone(),
            proof_of_work,
        })
    }

    /// Returns the public key associated with the signer.
    pub fn public_key(&self) -> K::Public {
        K::public_from_secret(&self.keypair)
    }

    /// Returns the operator ID
    pub fn operator_id(&self) -> OperatorId {
        self.operator_id.clone()
    }
}

/// Creates a hash of the quote details for on-chain verification
pub fn hash_quote_details(quote_details: &pricing_engine::QuoteDetails) -> Result<[u8; 32]> {
    let on_chain_quote = crate::utils::create_on_chain_quote_type(quote_details);
    let serialized = on_chain_quote.encode();
    let keccak_hash = sp_core::keccak_256(&serialized);
    Ok(keccak_hash)
}

/// Verify a quote signature by checking the signature against the hash of the quote details.
pub fn verify_quote<K: KeyType>(quote: &SignedQuote<K>, public_key: &K::Public) -> Result<bool> {
    let hash = hash_quote_details(&quote.quote_details)?;
    Ok(K::verify(public_key, &hash, &quote.signature))
}
