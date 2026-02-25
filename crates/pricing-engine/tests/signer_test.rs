use alloy_primitives::U256;
use blueprint_crypto::{KeyType, k256::K256Ecdsa};
use blueprint_pricing_engine_lib::{
    OperatorSigner,
    error::{PricingError, Result},
    signer::{QuoteSigningDomain, SignableQuote, quote_digest_eip712, verify_quote},
};
use rust_decimal::prelude::FromPrimitive;

mod utils;

#[tokio::test]
async fn test_sign_and_verify_quote() -> Result<()> {
    // Create a test config
    let config = utils::create_test_config();

    // Initialize an operator signer with a new keypair
    let secret = K256Ecdsa::generate_with_seed(None)
        .map_err(|e| PricingError::Other(format!("Failed to generate keypair: {e}")))?;

    let domain = QuoteSigningDomain {
        chain_id: 1,
        verifying_contract: alloy_primitives::Address::ZERO,
    };

    let mut signer = OperatorSigner::new(&config, secret, domain)?;

    // Create a deterministic QuoteDetails message
    let quote_details = utils::create_test_quote_details();
    let signable_quote = SignableQuote::new(
        quote_details.clone(),
        rust_decimal::Decimal::from_f64(quote_details.total_cost_rate)
            .ok_or_else(|| PricingError::Other("invalid decimal".to_string()))?,
    )?;

    // Create proof of work
    let proof_of_work = vec![1, 2, 3, 4];

    // Sign the quote
    let signed_quote = signer.sign_quote(signable_quote, proof_of_work)?;

    // Verify the signature
    let public_key = signer.verifying_key();
    let is_valid = verify_quote(&signed_quote, &public_key, domain)?;

    // Verify that the signature is valid
    assert!(is_valid, "Signature should be valid");

    // Verify that the quote details in the signed quote match the original
    assert_eq!(
        signed_quote.quote_details.blueprint_id, quote_details.blueprint_id,
        "Blueprint ID should match"
    );
    assert_eq!(
        signed_quote.quote_details.ttl_blocks, quote_details.ttl_blocks,
        "TTL blocks should match"
    );
    assert_eq!(
        signed_quote.quote_details.total_cost_rate, quote_details.total_cost_rate,
        "Total cost rate should match"
    );

    // Tamper with the quote details and verify the signature is no longer valid
    let mut tampered_quote = signed_quote.clone();
    tampered_quote.abi_details.totalCost += U256::from(1u8);

    let is_valid_tampered = verify_quote(&tampered_quote, &public_key, domain)?;
    assert!(
        !is_valid_tampered,
        "Signature should be invalid for tampered quote"
    );

    Ok(())
}

#[tokio::test]
async fn test_service_quote_digest_changes_with_resource_commitments() -> Result<()> {
    let config = utils::create_test_config();
    let secret = K256Ecdsa::generate_with_seed(None)
        .map_err(|e| PricingError::Other(format!("Failed to generate keypair: {e}")))?;

    let domain = QuoteSigningDomain {
        chain_id: 1,
        verifying_contract: alloy_primitives::Address::ZERO,
    };

    let mut signer = OperatorSigner::new(&config, secret, domain)?;
    let quote_details = utils::create_test_quote_details();
    let signable_quote = SignableQuote::new(
        quote_details,
        rust_decimal::Decimal::from_f64(0.0001)
            .ok_or_else(|| PricingError::Other("invalid decimal".to_string()))?,
    )?;
    let signed_quote = signer.sign_quote(signable_quote, vec![1, 2, 3, 4])?;

    let mut mutated = signed_quote.abi_details.clone();
    assert!(
        !mutated.resourceCommitments.is_empty(),
        "test quote should include resource commitments"
    );
    mutated.resourceCommitments[0].count += 1;

    let original = quote_digest_eip712(&signed_quote.abi_details, domain)?;
    let changed = quote_digest_eip712(&mutated, domain)?;
    assert_ne!(
        original, changed,
        "changing resource commitments must change the EIP-712 digest"
    );

    Ok(())
}
