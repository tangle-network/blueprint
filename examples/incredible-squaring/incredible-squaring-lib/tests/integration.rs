//! Integration Tests for Incredible Squaring Blueprint with Aggregation
//!
//! These tests verify the full aggregation flow:
//! 1. Create aggregation service
//! 2. Create BLS keypairs for operators
//! 3. Initialize aggregation task
//! 4. Each operator computes result, signs, and submits
//! 5. Fetch aggregated result
//! 6. Verify correctness
//!
//! Note: These tests use the aggregation service directly (no HTTP).
//! For full E2E tests with contracts, see the Solidity tests.

use alloy_sol_types::SolValue;
use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_ff::UniformRand;
use ark_serialize::CanonicalSerialize;
use blueprint_crypto_bn254;
use blueprint_sdk::testing::utils::setup_log;
use blueprint_tangle_aggregation_svc::{
    AggregationService, ServiceConfig, SubmitSignatureRequest, create_signing_message,
};
use color_eyre::Result;
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment
fn init_test() {
    INIT.call_once(|| {
        let _ = color_eyre::install();
        setup_log();
    });
}

/// BLS keypair for testing
struct BlsKeypair {
    secret: Fr,
    public: G2Affine,
}

/// Generate a BLS keypair for testing
fn generate_bls_keypair() -> BlsKeypair {
    let mut rng = rand::thread_rng();
    let secret = Fr::rand(&mut rng);
    let public: G2Affine = (G2Affine::generator() * secret).into();
    BlsKeypair { secret, public }
}

/// Sign a message with BLS
fn bls_sign(secret: Fr, message: &[u8]) -> G1Affine {
    blueprint_crypto_bn254::sign(secret, message).expect("signing should succeed")
}

/// Serialize a BLS signature (G1 point)
fn serialize_signature(sig: &G1Affine) -> Vec<u8> {
    let mut bytes = Vec::new();
    sig.serialize_compressed(&mut bytes).expect("serialize signature");
    bytes
}

/// Serialize a BLS public key (G2 point)
fn serialize_public_key(pk: &G2Affine) -> Vec<u8> {
    let mut bytes = Vec::new();
    pk.serialize_compressed(&mut bytes).expect("serialize public key");
    bytes
}

// ═══════════════════════════════════════════════════════════════════════════
// INTEGRATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test single operator job (square - requires 1 result)
#[tokio::test]
async fn test_single_operator_aggregation() -> Result<()> {
    init_test();

    let service = AggregationService::new(ServiceConfig::default());

    // Generate operator keypair
    let keypair = generate_bls_keypair();

    // Compute job result: square(5) = 25
    let input: u64 = 5;
    let output: u64 = input * input;
    let output_bytes = output.abi_encode();

    let service_id = 1u64;
    let call_id = 100u64;

    // Initialize aggregation task (1 operator, threshold 1)
    service.init_task(service_id, call_id, output_bytes.clone(), 1, 1)?;

    // Create signing message and sign
    let message = create_signing_message(service_id, call_id, &output_bytes);
    let signature = bls_sign(keypair.secret, &message);

    // Submit signature
    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&signature),
        public_key: serialize_public_key(&keypair.public),
    };
    let response = service.submit_signature(submit_req)?;

    assert!(response.accepted, "Signature should be accepted");
    assert!(response.threshold_met, "Threshold should be met with 1/1 signatures");

    // Get aggregated result
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Should have aggregated result");

    println!("Single operator aggregation test passed!");
    Ok(())
}

/// Test verified square job (requires 2 operator results)
#[tokio::test]
async fn test_two_operator_aggregation() -> Result<()> {
    init_test();

    let service = AggregationService::new(ServiceConfig::default());

    // Generate 2 operator keypairs
    let operators: Vec<_> = (0..2).map(|_| generate_bls_keypair()).collect();

    // Compute job result: verified_square(7) = 49
    let input: u64 = 7;
    let output: u64 = input * input;
    let output_bytes = output.abi_encode();

    let service_id = 2u64;
    let call_id = 200u64;

    // Initialize aggregation task (2 operators, threshold 2)
    service.init_task(service_id, call_id, output_bytes.clone(), 2, 2)?;

    // Create signing message
    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Operator 0 submits
    let sig0 = bls_sign(operators[0].secret, &message);
    let submit_req0 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig0),
        public_key: serialize_public_key(&operators[0].public),
    };
    let response0 = service.submit_signature(submit_req0)?;
    assert!(response0.accepted, "Operator 0 signature should be accepted");
    assert!(!response0.threshold_met, "Threshold should NOT be met with 1/2 signatures");

    // Check status
    let status = service.get_status(service_id, call_id);
    assert_eq!(status.signatures_collected, 1, "Should have 1 signature collected");

    // Operator 1 submits
    let sig1 = bls_sign(operators[1].secret, &message);
    let submit_req1 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 1,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig1),
        public_key: serialize_public_key(&operators[1].public),
    };
    let response1 = service.submit_signature(submit_req1)?;
    assert!(response1.accepted, "Operator 1 signature should be accepted");
    assert!(response1.threshold_met, "Threshold SHOULD be met with 2/2 signatures");

    // Get aggregated result
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Should have aggregated result");

    let result = result.unwrap();
    assert_eq!(result.service_id, service_id);
    assert_eq!(result.call_id, call_id);

    println!("Two operator (verified_square) aggregation test passed!");
    Ok(())
}

/// Test consensus square job (requires 3 operator results)
#[tokio::test]
async fn test_three_operator_consensus_aggregation() -> Result<()> {
    init_test();

    let service = AggregationService::new(ServiceConfig::default());

    // Generate 3 operator keypairs
    let operators: Vec<_> = (0..3).map(|_| generate_bls_keypair()).collect();

    // Compute job result: consensus_square(6) = 36
    let input: u64 = 6;
    let output: u64 = input * input;
    let output_bytes = output.abi_encode();

    let service_id = 3u64;
    let call_id = 300u64;

    // Initialize aggregation task (3 operators, threshold 3)
    service.init_task(service_id, call_id, output_bytes.clone(), 3, 3)?;

    // Create signing message
    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Submit signatures from all 3 operators
    for (i, keypair) in operators.iter().enumerate() {
        let sig = bls_sign(keypair.secret, &message);
        let submit_req = SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: i as u32,
            output: output_bytes.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_public_key(&keypair.public),
        };
        let response = service.submit_signature(submit_req)?;

        assert!(response.accepted, "Operator {} signature should be accepted", i);

        if i < 2 {
            assert!(!response.threshold_met, "Threshold should NOT be met with {}/3 signatures", i + 1);
        } else {
            assert!(response.threshold_met, "Threshold SHOULD be met with 3/3 signatures");
        }

        // Check status
        let status = service.get_status(service_id, call_id);
        assert_eq!(status.signatures_collected, i + 1, "Should have {} signatures collected", i + 1);

        println!(
            "Operator {} submitted: {}/{} signatures, threshold_met={}",
            i, status.signatures_collected, 3, status.threshold_met
        );
    }

    // Get aggregated result
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Should have aggregated result");

    let result = result.unwrap();

    // Verify the output is correct
    let decoded_value = u64::abi_decode(&result.output, true)?;
    assert_eq!(decoded_value, 36, "Decoded output should be 36");

    // Verify signer bitmap has all 3 operators
    assert!(result.signer_bitmap.bit(0), "Operator 0 should be in bitmap");
    assert!(result.signer_bitmap.bit(1), "Operator 1 should be in bitmap");
    assert!(result.signer_bitmap.bit(2), "Operator 2 should be in bitmap");

    println!("Three operator (consensus_square) aggregation test passed!");
    println!("  - Output: {} (expected: 36)", decoded_value);
    println!("  - Signer bitmap: {:b}", result.signer_bitmap);

    Ok(())
}

/// Test that aggregation fails before threshold is met
#[tokio::test]
async fn test_aggregation_not_available_before_threshold() -> Result<()> {
    init_test();

    let service = AggregationService::new(ServiceConfig::default());

    // Generate 3 operator keypairs
    let operators: Vec<_> = (0..3).map(|_| generate_bls_keypair()).collect();

    let output_bytes = 100u64.abi_encode();
    let service_id = 4u64;
    let call_id = 400u64;

    // Initialize aggregation task (3 operators, threshold 3)
    service.init_task(service_id, call_id, output_bytes.clone(), 3, 3)?;

    // Check that aggregated result is not available yet
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_none(), "Aggregated result should NOT be available before any signatures");

    // Submit only 2 signatures
    let message = create_signing_message(service_id, call_id, &output_bytes);

    for i in 0..2 {
        let sig = bls_sign(operators[i].secret, &message);
        let submit_req = SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: i as u32,
            output: output_bytes.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_public_key(&operators[i].public),
        };
        service.submit_signature(submit_req)?;
    }

    // Check that aggregated result is still not available
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_none(), "Aggregated result should NOT be available with only 2/3 signatures");

    // Now submit the 3rd signature
    let sig2 = bls_sign(operators[2].secret, &message);
    let submit_req2 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 2,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig2),
        public_key: serialize_public_key(&operators[2].public),
    };
    let response = service.submit_signature(submit_req2)?;
    assert!(response.threshold_met, "Threshold should be met after 3rd signature");

    // Now aggregated result should be available
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Aggregated result SHOULD be available after threshold met");

    println!("Threshold enforcement test passed!");
    Ok(())
}

/// Full end-to-end test simulating the incredible squaring workflow
#[tokio::test]
async fn test_full_incredible_squaring_aggregation_flow() -> Result<()> {
    init_test();

    use blueprint_sdk::tangle_evm::extract::TangleEvmResult;

    let service = AggregationService::new(ServiceConfig::default());

    println!("\n=== Full Incredible Squaring Aggregation Flow ===\n");

    // Generate 3 operators
    let operators: Vec<_> = (0..3).map(|_| generate_bls_keypair()).collect();

    let service_id = 10u64;
    let call_id = 1000u64;
    let input: u64 = 9;

    println!("Job: consensus_square({}) - requires 3 operator results", input);
    println!("Service ID: {}, Call ID: {}", service_id, call_id);
    println!();

    // Step 1: Each operator computes the result locally
    println!("Step 1: Operators compute results...");
    let results: Vec<TangleEvmResult<u64>> = (0..3)
        .map(|i| {
            println!("  Operator {}: computed {} * {} = {}", i, input, input, input * input);
            TangleEvmResult(input * input)
        })
        .collect();

    // All results should be the same
    let expected_output = input * input;
    for (i, result) in results.iter().enumerate() {
        assert_eq!(**result, expected_output, "Operator {} result mismatch", i);
    }

    // Step 2: Initialize aggregation task
    println!("\nStep 2: Initialize aggregation task...");
    let output_bytes = expected_output.abi_encode();
    service.init_task(service_id, call_id, output_bytes.clone(), 3, 3)?;
    println!("  Task initialized: threshold=3, operator_count=3");

    // Step 3: Each operator signs and submits
    println!("\nStep 3: Operators sign and submit...");
    let message = create_signing_message(service_id, call_id, &output_bytes);

    for (i, keypair) in operators.iter().enumerate() {
        let sig = bls_sign(keypair.secret, &message);
        let submit_req = SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: i as u32,
            output: output_bytes.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_public_key(&keypair.public),
        };
        let response = service.submit_signature(submit_req)?;
        assert!(response.accepted);

        let status = service.get_status(service_id, call_id);
        println!(
            "  Operator {}: signature accepted, {}/3 collected, threshold_met={}",
            i, status.signatures_collected, status.threshold_met
        );
    }

    // Step 4: Fetch aggregated result
    println!("\nStep 4: Fetch aggregated result...");
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some());

    let result = result.unwrap();
    let final_output = u64::abi_decode(&result.output, true)?;

    println!("  Aggregated result:");
    println!("    - Output: {} (expected: {})", final_output, expected_output);
    println!("    - Has aggregated signature: {} bytes", result.aggregated_signature.len());
    println!("    - Has aggregated pubkey: {} bytes", result.aggregated_pubkey.len());
    println!("    - Signer bitmap: {:b}", result.signer_bitmap);

    assert_eq!(final_output, expected_output);

    println!("\n=== Test Passed! ===");
    println!("The incredible squaring aggregation flow works correctly:");
    println!("  1. 3 operators computed square({}) = {}", input, expected_output);
    println!("  2. Each operator signed and submitted to aggregation service");
    println!("  3. After 3/3 threshold met, aggregated BLS signature available");
    println!("  4. Result ready for on-chain submission");

    Ok(())
}

/// Test that the signer bitmap correctly tracks which operators have signed
#[tokio::test]
async fn test_signer_bitmap_tracking() -> Result<()> {
    init_test();

    let service = AggregationService::new(ServiceConfig::default());

    // Generate 5 operator keypairs
    let operators: Vec<_> = (0..5).map(|_| generate_bls_keypair()).collect();

    let output_bytes = 42u64.abi_encode();
    let service_id = 5u64;
    let call_id = 500u64;

    // Initialize with threshold of 3 out of 5
    service.init_task(service_id, call_id, output_bytes.clone(), 5, 3)?;

    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Submit from operators 0, 2, and 4 (skipping 1 and 3)
    for &i in &[0usize, 2, 4] {
        let sig = bls_sign(operators[i].secret, &message);
        let submit_req = SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: i as u32,
            output: output_bytes.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_public_key(&operators[i].public),
        };
        service.submit_signature(submit_req)?;
    }

    // Get status and check bitmap
    let status = service.get_status(service_id, call_id);
    assert!(status.threshold_met, "Should have 3/3 threshold met");

    // Check bitmap
    assert!(status.signer_bitmap.bit(0), "Operator 0 should be in bitmap");
    assert!(!status.signer_bitmap.bit(1), "Operator 1 should NOT be in bitmap");
    assert!(status.signer_bitmap.bit(2), "Operator 2 should be in bitmap");
    assert!(!status.signer_bitmap.bit(3), "Operator 3 should NOT be in bitmap");
    assert!(status.signer_bitmap.bit(4), "Operator 4 should be in bitmap");

    // Get aggregated result and verify bitmap
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some());

    let result = result.unwrap();
    assert_eq!(result.signer_bitmap, status.signer_bitmap);

    println!("Signer bitmap tracking test passed!");
    println!("  Signed: operators 0, 2, 4");
    println!("  Not signed: operators 1, 3");
    println!("  Bitmap: {:b}", result.signer_bitmap);

    Ok(())
}
