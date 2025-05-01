use blueprint_crypto::KeyType;
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_pricing_engine_lib::{
    OperatorSigner,
    error::{PricingError, Result},
    signer::verify_quote,
};
use tangle_subxt::tangle_testnet_runtime::api::runtime_apis::rewards_api::types::query_user_rewards::AccountId;

mod utils;

#[tokio::test]
async fn test_sign_and_verify_quote() -> Result<()> {
    // Create a test config
    let config = utils::create_test_config();

    // Initialize an operator signer with a new keypair
    let secret = K256Ecdsa::generate_with_seed(None)
        .map_err(|e| PricingError::Other(format!("Failed to generate keypair: {}", e)))?;

    let mut signer = OperatorSigner::<K256Ecdsa>::new(&config, secret, AccountId::from([0u8; 32]))?;

    // Create a deterministic QuoteDetails message
    let quote_details = utils::create_test_quote_details();

    // Create proof of work
    let proof_of_work = vec![1, 2, 3, 4];

    // Sign the quote
    let signed_quote = signer.sign_quote(quote_details.clone(), proof_of_work)?;

    // Verify the signature
    let public_key = signer.public_key();
    let is_valid = verify_quote(&signed_quote, &public_key)?;

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
    tampered_quote.quote_details.total_cost_rate += 1.0;

    let is_valid_tampered = verify_quote(&tampered_quote, &public_key)?;
    assert!(
        !is_valid_tampered,
        "Signature should be invalid for tampered quote"
    );

    Ok(())
}
