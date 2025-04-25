use blueprint_crypto::KeyType;
use blueprint_crypto::k256::K256Ecdsa;
use blueprint_pricing_engine_lib::{
    OperatorSigner,
    config::OperatorConfig,
    error::{PricingError, Result},
    pricing_engine,
    signer::verify_quote,
    utils::u32_to_u128_bytes,
};
use tangle_subxt::tangle_testnet_runtime::api::runtime_apis::rewards_api::types::query_user_rewards::AccountId;

/// Test the full sign and verify flow
#[tokio::test]
async fn test_sign_and_verify_quote() -> Result<()> {
    // Create a test config
    let config = create_test_config();

    // Initialize an operator signer with a new keypair
    // Use the correct method from K256Ecdsa to generate a keypair
    let secret = K256Ecdsa::generate_with_seed(None)
        .map_err(|e| PricingError::Other(format!("Failed to generate keypair: {}", e)))?;

    let mut signer = OperatorSigner::<K256Ecdsa>::new(&config, secret, AccountId::from([0u8; 32]))?;

    // Create a deterministic QuoteDetails message
    let quote_details = create_test_quote_details();

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

/// Helper function to create a test QuoteDetails message with deterministic values
fn create_test_quote_details() -> pricing_engine::QuoteDetails {
    let resource = pricing_engine::ResourcePricing {
        kind: "CPU".to_string(),
        count: 2,
        price_per_unit_rate: 0.000001,
    };

    let security_commitment = pricing_engine::AssetSecurityCommitment {
        asset: Some(pricing_engine::Asset {
            asset_type: Some(pricing_engine::asset::AssetType::Custom(u32_to_u128_bytes(
                1234,
            ))),
        }),
        exposure_percent: 50,
    };

    pricing_engine::QuoteDetails {
        blueprint_id: 12345,
        ttl_blocks: 100,
        total_cost_rate: 0.0001,
        timestamp: 1650000000,
        expiry: 1650001000,
        resources: vec![resource],
        security_commitments: Some(security_commitment),
    }
}

/// Helper function to create a test config
fn create_test_config() -> OperatorConfig {
    OperatorConfig {
        keystore_path: "/tmp/test-keystore".to_string(),
        database_path: "./data/test_benchmark_cache".to_string(),
        rpc_port: 9000,
        rpc_bind_address: "127.0.0.1:9000".to_string(),
        benchmark_command: "echo".to_string(),
        benchmark_args: vec!["benchmark".to_string()],
        benchmark_duration: 10,
        benchmark_interval: 1,
        keypair_path: "/tmp/test-keypair".to_string(),
        rpc_timeout: 30,
        rpc_max_connections: 100,
        quote_validity_duration_secs: 300,
    }
}
