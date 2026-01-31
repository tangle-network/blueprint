//! Integration Tests for Incredible Squaring Blueprint with Aggregation
//!
//! These tests verify the full aggregation flow with MAXIMUM validation:
//! 1. Signature verification (valid and invalid)
//! 2. Output validation (matching and mismatched)
//! 3. Duplicate submission handling
//! 4. Threshold enforcement
//! 5. Cryptographic verification of aggregated signatures
//! 6. Signer bitmap accuracy
//!
//! Note: These tests use the aggregation service directly (no HTTP).
//! For full E2E tests with contracts, see the Solidity tests.

use alloy_sol_types::SolValue;
use ark_bn254::{Fr, G1Affine, G2Affine};
use ark_ec::AffineRepr;
use ark_ff::UniformRand;
use ark_serialize::{CanonicalDeserialize, CanonicalSerialize};
use blueprint_crypto_bn254;
use blueprint_sdk::testing::utils::setup_log;
use blueprint_tangle_aggregation_svc::{
    AggregationService, ServiceConfig, ServiceError, SubmitSignatureRequest, create_signing_message,
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
    sig.serialize_compressed(&mut bytes)
        .expect("serialize signature");
    bytes
}

/// Serialize a BLS public key (G2 point)
fn serialize_public_key(pk: &G2Affine) -> Vec<u8> {
    let mut bytes = Vec::new();
    pk.serialize_compressed(&mut bytes)
        .expect("serialize public key");
    bytes
}

/// Verify a BLS signature using the crypto library
fn verify_bls_signature(signature: &G1Affine, public_key: &G2Affine, message: &[u8]) -> bool {
    blueprint_crypto_bn254::verify(*public_key, message, *signature)
}

/// Verify an aggregated signature
fn verify_aggregated_signature(
    aggregated_sig: &G1Affine,
    aggregated_pk: &G2Affine,
    message: &[u8],
) -> bool {
    verify_bls_signature(aggregated_sig, aggregated_pk, message)
}

// ═══════════════════════════════════════════════════════════════════════════
// BASIC AGGREGATION TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test single operator job (square - requires 1 result)
#[tokio::test]
async fn test_single_operator_aggregation() -> Result<()> {
    init_test();
    println!("\n=== Single Operator Aggregation ===\n");

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

    // Verify our signature is valid before submission
    assert!(
        verify_bls_signature(&signature, &keypair.public, &message),
        "Signature should be valid before submission"
    );

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
    assert!(
        response.threshold_met,
        "Threshold should be met with 1/1 signatures"
    );
    assert_eq!(response.signatures_collected, 1);
    assert_eq!(response.threshold_required, 1);

    // Get aggregated result
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Should have aggregated result");

    let result = result.unwrap();

    // Verify the aggregated signature is cryptographically valid
    let agg_sig = G1Affine::deserialize_compressed(&result.aggregated_signature[..])
        .expect("deserialize aggregated signature");
    let agg_pk = G2Affine::deserialize_compressed(&result.aggregated_pubkey[..])
        .expect("deserialize aggregated pubkey");

    assert!(
        verify_aggregated_signature(&agg_sig, &agg_pk, &message),
        "Aggregated signature must be cryptographically valid"
    );

    println!("  Input: {}", input);
    println!("  Output: {} (expected: {})", output, input * input);
    println!("  Signature valid: YES");
    println!("  Aggregated signature verified: YES");
    println!("\n=== Test Passed! ===");

    Ok(())
}

/// Test verified square job (requires 2 operator results)
#[tokio::test]
async fn test_two_operator_aggregation() -> Result<()> {
    init_test();
    println!("\n=== Two Operator Aggregation ===\n");

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
    assert!(
        verify_bls_signature(&sig0, &operators[0].public, &message),
        "Operator 0 sig valid"
    );

    let submit_req0 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig0),
        public_key: serialize_public_key(&operators[0].public),
    };
    let response0 = service.submit_signature(submit_req0)?;
    assert!(
        response0.accepted,
        "Operator 0 signature should be accepted"
    );
    assert!(
        !response0.threshold_met,
        "Threshold should NOT be met with 1/2 signatures"
    );
    assert_eq!(response0.signatures_collected, 1);

    // Aggregated result should NOT be available yet
    assert!(
        service.get_aggregated_result(service_id, call_id).is_none(),
        "Aggregated result should NOT be available before threshold"
    );

    // Operator 1 submits
    let sig1 = bls_sign(operators[1].secret, &message);
    assert!(
        verify_bls_signature(&sig1, &operators[1].public, &message),
        "Operator 1 sig valid"
    );

    let submit_req1 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 1,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig1),
        public_key: serialize_public_key(&operators[1].public),
    };
    let response1 = service.submit_signature(submit_req1)?;
    assert!(
        response1.accepted,
        "Operator 1 signature should be accepted"
    );
    assert!(
        response1.threshold_met,
        "Threshold SHOULD be met with 2/2 signatures"
    );
    assert_eq!(response1.signatures_collected, 2);

    // Get and verify aggregated result
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Should have aggregated result");

    let result = result.unwrap();
    let agg_sig = G1Affine::deserialize_compressed(&result.aggregated_signature[..]).unwrap();
    let agg_pk = G2Affine::deserialize_compressed(&result.aggregated_pubkey[..]).unwrap();

    assert!(
        verify_aggregated_signature(&agg_sig, &agg_pk, &message),
        "Aggregated signature must be cryptographically valid for 2 signers"
    );

    // Verify signer bitmap
    assert!(result.signer_bitmap.bit(0), "Operator 0 in bitmap");
    assert!(result.signer_bitmap.bit(1), "Operator 1 in bitmap");

    println!("  Two operators signed and threshold met");
    println!("  Aggregated signature verified: YES");
    println!("\n=== Test Passed! ===");

    Ok(())
}

/// Test consensus square job (requires 3 operator results)
#[tokio::test]
async fn test_three_operator_consensus_aggregation() -> Result<()> {
    init_test();
    println!("\n=== Three Operator Consensus Aggregation ===\n");

    let service = AggregationService::new(ServiceConfig::default());

    // Generate 3 operator keypairs
    let operators: Vec<_> = (0..3).map(|_| generate_bls_keypair()).collect();

    let output: u64 = 36;
    let output_bytes = output.abi_encode();

    let service_id = 3u64;
    let call_id = 300u64;

    service.init_task(service_id, call_id, output_bytes.clone(), 3, 3)?;

    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Submit all 3 signatures
    for (i, keypair) in operators.iter().enumerate() {
        let sig = bls_sign(keypair.secret, &message);
        assert!(
            verify_bls_signature(&sig, &keypair.public, &message),
            "Op {} sig valid",
            i
        );

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
        if i < 2 {
            assert!(!response.threshold_met, "Threshold NOT met at {}/3", i + 1);
        } else {
            assert!(response.threshold_met, "Threshold met at 3/3");
        }
    }

    // Verify aggregated result
    let result = service.get_aggregated_result(service_id, call_id).unwrap();
    let agg_sig = G1Affine::deserialize_compressed(&result.aggregated_signature[..]).unwrap();
    let agg_pk = G2Affine::deserialize_compressed(&result.aggregated_pubkey[..]).unwrap();

    assert!(
        verify_aggregated_signature(&agg_sig, &agg_pk, &message),
        "3-of-3 aggregated signature must be valid"
    );

    // Verify all 3 in bitmap
    assert!(result.signer_bitmap.bit(0));
    assert!(result.signer_bitmap.bit(1));
    assert!(result.signer_bitmap.bit(2));

    // Verify output decodes correctly
    let decoded = u64::abi_decode(&result.output)?;
    assert_eq!(decoded, 36);

    println!("  Three operators signed, aggregated signature valid");
    println!("\n=== Test Passed! ===");

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// INVALID SIGNATURE TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test that signatures from wrong key are rejected
#[tokio::test]
async fn test_invalid_signature_wrong_key_rejected() -> Result<()> {
    init_test();
    println!("\n=== Invalid Signature (Wrong Key) Rejection ===\n");

    let service = AggregationService::new(ServiceConfig::default());

    let correct_keypair = generate_bls_keypair();
    let wrong_keypair = generate_bls_keypair();

    let output_bytes = 100u64.abi_encode();
    let service_id = 10u64;
    let call_id = 1000u64;

    service.init_task(service_id, call_id, output_bytes.clone(), 1, 1)?;

    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Sign with WRONG key
    let wrong_sig = bls_sign(wrong_keypair.secret, &message);

    // Submit with wrong signature but claim it's from correct_keypair
    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&wrong_sig),
        public_key: serialize_public_key(&correct_keypair.public), // Claiming wrong pubkey
    };

    let result = service.submit_signature(submit_req);
    assert!(result.is_err(), "Should reject signature from wrong key");

    match result.unwrap_err() {
        ServiceError::VerificationFailed => println!("  Correctly rejected: VerificationFailed"),
        e => panic!("Expected VerificationFailed, got: {:?}", e),
    }

    // Verify task is still pending (signature wasn't accepted)
    let status = service.get_status(service_id, call_id);
    assert_eq!(
        status.signatures_collected, 0,
        "No signatures should be collected"
    );

    println!("\n=== Test Passed! Invalid signature was rejected ===");
    Ok(())
}

/// Test that signatures on wrong message are rejected
#[tokio::test]
async fn test_invalid_signature_wrong_message_rejected() -> Result<()> {
    init_test();
    println!("\n=== Invalid Signature (Wrong Message) Rejection ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let keypair = generate_bls_keypair();

    let correct_output_bytes = 100u64.abi_encode();
    let wrong_output_bytes = 999u64.abi_encode(); // Different output

    let service_id = 11u64;
    let call_id = 1100u64;

    service.init_task(service_id, call_id, correct_output_bytes.clone(), 1, 1)?;

    // Sign the WRONG message
    let wrong_message = create_signing_message(service_id, call_id, &wrong_output_bytes);
    let sig = bls_sign(keypair.secret, &wrong_message);

    // Submit signature (with correct pubkey but wrong message signed)
    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: correct_output_bytes.clone(), // Claiming correct output
        signature: serialize_signature(&sig),
        public_key: serialize_public_key(&keypair.public),
    };

    let result = service.submit_signature(submit_req);
    assert!(result.is_err(), "Should reject signature on wrong message");

    match result.unwrap_err() {
        ServiceError::VerificationFailed => println!("  Correctly rejected: VerificationFailed"),
        e => panic!("Expected VerificationFailed, got: {:?}", e),
    }

    println!("\n=== Test Passed! Signature on wrong message was rejected ===");
    Ok(())
}

/// Test that malformed signatures are rejected
#[tokio::test]
async fn test_malformed_signature_rejected() -> Result<()> {
    init_test();
    println!("\n=== Malformed Signature Rejection ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let keypair = generate_bls_keypair();

    let output_bytes = 50u64.abi_encode();
    let service_id = 12u64;
    let call_id = 1200u64;

    service.init_task(service_id, call_id, output_bytes.clone(), 1, 1)?;

    // Submit with garbage signature bytes
    let garbage_sig = vec![0xFF; 48]; // Wrong format

    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: garbage_sig,
        public_key: serialize_public_key(&keypair.public),
    };

    let result = service.submit_signature(submit_req);
    assert!(result.is_err(), "Should reject malformed signature");

    match result.unwrap_err() {
        ServiceError::InvalidSignature => println!("  Correctly rejected: InvalidSignature"),
        e => panic!("Expected InvalidSignature, got: {:?}", e),
    }

    println!("\n=== Test Passed! Malformed signature was rejected ===");
    Ok(())
}

/// Test that malformed public keys are rejected
#[tokio::test]
async fn test_malformed_public_key_rejected() -> Result<()> {
    init_test();
    println!("\n=== Malformed Public Key Rejection ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let keypair = generate_bls_keypair();

    let output_bytes = 50u64.abi_encode();
    let service_id = 13u64;
    let call_id = 1300u64;

    service.init_task(service_id, call_id, output_bytes.clone(), 1, 1)?;

    let message = create_signing_message(service_id, call_id, &output_bytes);
    let sig = bls_sign(keypair.secret, &message);

    // Submit with garbage public key bytes
    let garbage_pk = vec![0xFF; 96]; // Wrong format

    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig),
        public_key: garbage_pk,
    };

    let result = service.submit_signature(submit_req);
    assert!(result.is_err(), "Should reject malformed public key");

    match result.unwrap_err() {
        ServiceError::InvalidPublicKey => println!("  Correctly rejected: InvalidPublicKey"),
        e => panic!("Expected InvalidPublicKey, got: {:?}", e),
    }

    println!("\n=== Test Passed! Malformed public key was rejected ===");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// OUTPUT MISMATCH TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test that submitting with wrong output is rejected
#[tokio::test]
async fn test_output_mismatch_rejected() -> Result<()> {
    init_test();
    println!("\n=== Output Mismatch Rejection ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let keypair = generate_bls_keypair();

    let correct_output_bytes = 100u64.abi_encode();
    let wrong_output_bytes = 999u64.abi_encode();

    let service_id = 20u64;
    let call_id = 2000u64;

    service.init_task(service_id, call_id, correct_output_bytes.clone(), 1, 1)?;

    // Sign the wrong output (malicious operator trying to submit different result)
    let wrong_message = create_signing_message(service_id, call_id, &wrong_output_bytes);
    let sig = bls_sign(keypair.secret, &wrong_message);

    // Submit claiming wrong output
    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: wrong_output_bytes, // WRONG output
        signature: serialize_signature(&sig),
        public_key: serialize_public_key(&keypair.public),
    };

    let result = service.submit_signature(submit_req);
    assert!(result.is_err(), "Should reject mismatched output");

    match result.unwrap_err() {
        ServiceError::OutputMismatch => println!("  Correctly rejected: OutputMismatch"),
        e => panic!("Expected OutputMismatch, got: {:?}", e),
    }

    println!("\n=== Test Passed! Output mismatch was rejected ===");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// DUPLICATE SUBMISSION TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test that duplicate submissions from same operator are handled
#[tokio::test]
async fn test_duplicate_submission_handling() -> Result<()> {
    init_test();
    println!("\n=== Duplicate Submission Handling ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let keypair = generate_bls_keypair();

    let output_bytes = 64u64.abi_encode();
    let service_id = 30u64;
    let call_id = 3000u64;

    service.init_task(service_id, call_id, output_bytes.clone(), 3, 2)?;

    let message = create_signing_message(service_id, call_id, &output_bytes);
    let sig = bls_sign(keypair.secret, &message);

    // First submission
    let submit_req = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig),
        public_key: serialize_public_key(&keypair.public),
    };

    let response1 = service.submit_signature(submit_req.clone())?;
    assert!(response1.accepted);
    assert_eq!(response1.signatures_collected, 1);

    // Duplicate submission from SAME operator
    let response2 = service.submit_signature(submit_req.clone());

    // Check status - should still only have 1 signature
    let status = service.get_status(service_id, call_id);

    // The service should either:
    // - Reject the duplicate (error), OR
    // - Accept but not double-count (signatures_collected still 1)
    if response2.is_ok() {
        assert_eq!(
            status.signatures_collected, 1,
            "Duplicate should not be double-counted"
        );
        println!("  Duplicate was accepted but not double-counted");
    } else {
        println!("  Duplicate was rejected: {:?}", response2.unwrap_err());
    }

    // Verify bitmap only has operator 0 once
    assert!(status.signer_bitmap.bit(0));
    assert!(!status.signer_bitmap.bit(1));

    println!("\n=== Test Passed! Duplicate handled correctly ===");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// THRESHOLD ENFORCEMENT TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test that aggregation is not available before threshold is met
#[tokio::test]
async fn test_aggregation_not_available_before_threshold() -> Result<()> {
    init_test();
    println!("\n=== Threshold Enforcement ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let operators: Vec<_> = (0..3).map(|_| generate_bls_keypair()).collect();

    let output_bytes = 100u64.abi_encode();
    let service_id = 40u64;
    let call_id = 4000u64;

    service.init_task(service_id, call_id, output_bytes.clone(), 3, 3)?;

    // No signatures - should not be available
    assert!(
        service.get_aggregated_result(service_id, call_id).is_none(),
        "No result with 0/3 signatures"
    );

    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Submit 1 signature
    let sig0 = bls_sign(operators[0].secret, &message);
    service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig0),
        public_key: serialize_public_key(&operators[0].public),
    })?;

    assert!(
        service.get_aggregated_result(service_id, call_id).is_none(),
        "No result with 1/3 signatures"
    );

    // Submit 2nd signature
    let sig1 = bls_sign(operators[1].secret, &message);
    service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 1,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig1),
        public_key: serialize_public_key(&operators[1].public),
    })?;

    assert!(
        service.get_aggregated_result(service_id, call_id).is_none(),
        "No result with 2/3 signatures"
    );

    // Submit 3rd signature
    let sig2 = bls_sign(operators[2].secret, &message);
    let response = service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 2,
        output: output_bytes.clone(),
        signature: serialize_signature(&sig2),
        public_key: serialize_public_key(&operators[2].public),
    })?;

    assert!(response.threshold_met, "Threshold should be met at 3/3");

    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some(), "Result SHOULD be available at 3/3");

    println!("  Threshold enforced correctly: 0/3 ✗, 1/3 ✗, 2/3 ✗, 3/3 ✓");
    println!("\n=== Test Passed! ===");

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// SIGNER BITMAP TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test that signer bitmap correctly tracks which operators signed
#[tokio::test]
async fn test_signer_bitmap_tracking() -> Result<()> {
    init_test();
    println!("\n=== Signer Bitmap Tracking ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let operators: Vec<_> = (0..5).map(|_| generate_bls_keypair()).collect();

    let output_bytes = 42u64.abi_encode();
    let service_id = 50u64;
    let call_id = 5000u64;

    // 5 operators, threshold 3
    service.init_task(service_id, call_id, output_bytes.clone(), 5, 3)?;

    let message = create_signing_message(service_id, call_id, &output_bytes);

    // Only operators 0, 2, 4 sign (skip 1 and 3)
    for &i in &[0usize, 2, 4] {
        let sig = bls_sign(operators[i].secret, &message);
        service.submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: i as u32,
            output: output_bytes.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_public_key(&operators[i].public),
        })?;
    }

    let status = service.get_status(service_id, call_id);
    assert!(status.threshold_met, "3/3 threshold met");

    // Verify bitmap
    assert!(status.signer_bitmap.bit(0), "Op 0 in bitmap");
    assert!(!status.signer_bitmap.bit(1), "Op 1 NOT in bitmap");
    assert!(status.signer_bitmap.bit(2), "Op 2 in bitmap");
    assert!(!status.signer_bitmap.bit(3), "Op 3 NOT in bitmap");
    assert!(status.signer_bitmap.bit(4), "Op 4 in bitmap");

    // Get aggregated result and verify
    let result = service.get_aggregated_result(service_id, call_id).unwrap();

    // Verify non-signer indices
    assert!(
        result.non_signer_indices.contains(&1),
        "Op 1 should be in non-signers"
    );
    assert!(
        result.non_signer_indices.contains(&3),
        "Op 3 should be in non-signers"
    );
    assert_eq!(
        result.non_signer_indices.len(),
        2,
        "Should have 2 non-signers"
    );

    // Verify aggregated signature is valid
    let agg_sig = G1Affine::deserialize_compressed(&result.aggregated_signature[..]).unwrap();
    let agg_pk = G2Affine::deserialize_compressed(&result.aggregated_pubkey[..]).unwrap();
    assert!(
        verify_aggregated_signature(&agg_sig, &agg_pk, &message),
        "Partial aggregated signature must be valid"
    );

    println!("  Signers: 0, 2, 4");
    println!("  Non-signers: 1, 3");
    println!("  Bitmap: {:b}", result.signer_bitmap);
    println!("  Aggregated signature valid: YES");
    println!("\n=== Test Passed! ===");

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// TASK NOT FOUND TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test submitting to non-existent task
#[tokio::test]
async fn test_submit_to_nonexistent_task() -> Result<()> {
    init_test();
    println!("\n=== Submit to Non-existent Task ===\n");

    let service = AggregationService::new(ServiceConfig::default());
    let keypair = generate_bls_keypair();

    let output_bytes = 100u64.abi_encode();
    let message = create_signing_message(999, 999, &output_bytes);
    let sig = bls_sign(keypair.secret, &message);

    let result = service.submit_signature(SubmitSignatureRequest {
        service_id: 999,
        call_id: 999,
        operator_index: 0,
        output: output_bytes,
        signature: serialize_signature(&sig),
        public_key: serialize_public_key(&keypair.public),
    });

    assert!(
        result.is_err(),
        "Should reject submission to non-existent task"
    );
    match result.unwrap_err() {
        ServiceError::TaskNotFound => println!("  Correctly rejected: TaskNotFound"),
        e => panic!("Expected TaskNotFound, got: {:?}", e),
    }

    println!("\n=== Test Passed! ===");
    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// FULL E2E FLOW TEST
// ═══════════════════════════════════════════════════════════════════════════

/// Full end-to-end test with all validations
#[tokio::test]
async fn test_full_e2e_aggregation_flow() -> Result<()> {
    init_test();
    println!("\n=== Full E2E Aggregation Flow ===\n");

    use blueprint_sdk::tangle::extract::TangleResult;

    let service = AggregationService::new(ServiceConfig::default());

    // 3 operators for consensus_square
    let operators: Vec<_> = (0..3).map(|_| generate_bls_keypair()).collect();

    let service_id = 100u64;
    let call_id = 10000u64;
    let input: u64 = 9;

    println!(
        "Job: consensus_square({}) - requires 3 operator results",
        input
    );

    // Step 1: Operators compute results
    println!("\nStep 1: Compute results...");
    let results: Vec<TangleResult<u64>> = (0..3)
        .map(|i| {
            let result = input * input;
            println!("  Operator {}: square({}) = {}", i, input, result);
            TangleResult(result)
        })
        .collect();

    let expected_output = input * input;
    let output_bytes = expected_output.abi_encode();

    // Step 2: Initialize task
    println!("\nStep 2: Initialize aggregation task...");
    service.init_task(service_id, call_id, output_bytes.clone(), 3, 3)?;

    // Step 3: Each operator signs and submits
    println!("\nStep 3: Sign and submit...");
    let message = create_signing_message(service_id, call_id, &output_bytes);

    for (i, (keypair, result)) in operators.iter().zip(results.iter()).enumerate() {
        // Verify result matches expected
        assert_eq!(**result, expected_output, "Op {} result mismatch", i);

        let sig = bls_sign(keypair.secret, &message);

        // Verify signature before submission
        assert!(
            verify_bls_signature(&sig, &keypair.public, &message),
            "Op {} signature invalid before submit",
            i
        );

        let response = service.submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: i as u32,
            output: output_bytes.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_public_key(&keypair.public),
        })?;

        assert!(response.accepted);
        println!(
            "  Op {}: accepted, {}/3 collected, threshold_met={}",
            i, response.signatures_collected, response.threshold_met
        );
    }

    // Step 4: Get and verify aggregated result
    println!("\nStep 4: Verify aggregated result...");
    let result = service.get_aggregated_result(service_id, call_id).unwrap();

    // Verify output
    let final_output = u64::abi_decode(&result.output)?;
    assert_eq!(final_output, expected_output);
    println!("  Output: {} ✓", final_output);

    // Verify signer bitmap
    assert!(result.signer_bitmap.bit(0));
    assert!(result.signer_bitmap.bit(1));
    assert!(result.signer_bitmap.bit(2));
    println!("  Signer bitmap: {:b} ✓", result.signer_bitmap);

    // Verify no non-signers
    assert!(result.non_signer_indices.is_empty());
    println!("  Non-signers: none ✓");

    // Verify aggregated signature cryptographically
    let agg_sig = G1Affine::deserialize_compressed(&result.aggregated_signature[..]).unwrap();
    let agg_pk = G2Affine::deserialize_compressed(&result.aggregated_pubkey[..]).unwrap();
    assert!(
        verify_aggregated_signature(&agg_sig, &agg_pk, &message),
        "Aggregated signature must be valid"
    );
    println!(
        "  Aggregated signature: {} bytes, VALID ✓",
        result.aggregated_signature.len()
    );
    println!(
        "  Aggregated pubkey: {} bytes ✓",
        result.aggregated_pubkey.len()
    );

    println!("\n=== Full E2E Test Passed! ===");
    println!("All cryptographic proofs verified.");

    Ok(())
}
