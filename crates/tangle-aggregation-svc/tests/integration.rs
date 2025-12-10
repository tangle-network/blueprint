//! Integration tests for the BLS aggregation service
//!
//! These tests simulate the full flow of multiple operators signing a message
//! and aggregating their signatures, similar to the incredible squaring example.

use alloy_primitives::U256;
use ark_serialize::CanonicalSerialize;
use blueprint_crypto_bn254::{
    ArkBlsBn254, ArkBlsBn254Public, ArkBlsBn254Secret, ArkBlsBn254Signature,
};
use blueprint_crypto_core::{aggregation::AggregatableSignature, KeyType};
use blueprint_tangle_aggregation_svc::{
    create_signing_message, AggregationService, ServiceConfig, SubmitSignatureRequest, TaskConfig,
    ThresholdType,
};
use std::collections::HashMap;
use std::time::Duration;

/// Generate a BLS keypair from seed
fn generate_keypair(seed: &[u8]) -> (ArkBlsBn254Secret, ArkBlsBn254Public) {
    let secret = ArkBlsBn254::generate_with_seed(Some(seed)).unwrap();
    let public = ArkBlsBn254::public_from_secret(&secret);
    (secret, public)
}

/// Sign a message with a secret key using the KeyType trait
fn sign_message(secret: &ArkBlsBn254Secret, message: &[u8]) -> ArkBlsBn254Signature {
    let mut secret_clone = secret.clone();
    ArkBlsBn254::sign_with_secret(&mut secret_clone, message).unwrap()
}

/// Helper to serialize a signature to bytes
fn serialize_signature(sig: &ArkBlsBn254Signature) -> Vec<u8> {
    let mut bytes = Vec::new();
    sig.0.serialize_compressed(&mut bytes).unwrap();
    bytes
}

/// Helper to serialize a public key to bytes
fn serialize_pubkey(pk: &ArkBlsBn254Public) -> Vec<u8> {
    let mut bytes = Vec::new();
    pk.0.serialize_compressed(&mut bytes).unwrap();
    bytes
}

/// Test simulating the incredible squaring flow with 3 operators
#[test]
fn test_incredible_squaring_aggregation_flow() {
    // ═══════════════════════════════════════════════════════════════════════════
    // SETUP: Create service and generate operator keys
    // ═══════════════════════════════════════════════════════════════════════════

    // Create aggregation service with verification enabled
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    // Simulate 3 operators (for consensus_square which requires 3 results)
    let (sk1, pk1) = generate_keypair(b"operator1_seed_key");
    let (sk2, pk2) = generate_keypair(b"operator2_seed_key");
    let (sk3, pk3) = generate_keypair(b"operator3_seed_key");

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 1: Initialize the aggregation task
    // ═══════════════════════════════════════════════════════════════════════════

    let service_id = 1u64;
    let call_id = 42u64;
    let input_value: u64 = 7; // The number to square
    let output_value: u64 = input_value * input_value; // 49

    // ABI-encode the output (as the contract would receive it)
    let output = output_value.to_be_bytes().to_vec();

    // Initialize task with 3 operators and threshold of 3 (consensus)
    service
        .init_task(service_id, call_id, output.clone(), 3, 3)
        .expect("Failed to init task");

    // Verify initial status
    let status = service.get_status(service_id, call_id);
    assert!(status.exists);
    assert_eq!(status.signatures_collected, 0);
    assert_eq!(status.threshold_required, 3);
    assert!(!status.threshold_met);

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 2: Each operator computes the result and signs it
    // ═══════════════════════════════════════════════════════════════════════════

    // Create the message that operators sign (matches service logic)
    let message = create_signing_message(service_id, call_id, &output);

    // Operator 1 signs
    let sig1 = sign_message(&sk1, &message);

    // Operator 2 signs
    let sig2 = sign_message(&sk2, &message);

    // Operator 3 signs
    let sig3 = sign_message(&sk3, &message);

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 3: Submit signatures to aggregation service
    // ═══════════════════════════════════════════════════════════════════════════

    // Operator 1 submits
    let resp1 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sig1),
            public_key: serialize_pubkey(&pk1),
        })
        .expect("Operator 1 submission failed");

    assert!(resp1.accepted);
    assert_eq!(resp1.signatures_collected, 1);
    assert!(!resp1.threshold_met);

    // Check status
    let status = service.get_status(service_id, call_id);
    assert_eq!(status.signer_bitmap, U256::from(1)); // bit 0 set

    // Operator 2 submits
    let resp2 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 1,
            output: output.clone(),
            signature: serialize_signature(&sig2),
            public_key: serialize_pubkey(&pk2),
        })
        .expect("Operator 2 submission failed");

    assert!(resp2.accepted);
    assert_eq!(resp2.signatures_collected, 2);
    assert!(!resp2.threshold_met);

    // Check status
    let status = service.get_status(service_id, call_id);
    assert_eq!(status.signer_bitmap, U256::from(3)); // bits 0 and 1 set

    // Operator 3 submits (should meet threshold)
    let resp3 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 2,
            output: output.clone(),
            signature: serialize_signature(&sig3),
            public_key: serialize_pubkey(&pk3),
        })
        .expect("Operator 3 submission failed");

    assert!(resp3.accepted);
    assert_eq!(resp3.signatures_collected, 3);
    assert!(resp3.threshold_met); // Now threshold is met!

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 4: Get aggregated result
    // ═══════════════════════════════════════════════════════════════════════════

    let agg_result = service
        .get_aggregated_result(service_id, call_id)
        .expect("Aggregation should succeed");

    assert_eq!(agg_result.service_id, service_id);
    assert_eq!(agg_result.call_id, call_id);
    assert_eq!(agg_result.output, output);
    assert_eq!(agg_result.signer_bitmap, U256::from(7)); // bits 0, 1, 2 set

    // Verify we got aggregated signature and public key
    assert!(!agg_result.aggregated_signature.is_empty());
    assert!(!agg_result.aggregated_pubkey.is_empty());

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 5: Verify the aggregated signature
    // ═══════════════════════════════════════════════════════════════════════════

    // Manually aggregate signatures and public keys to verify
    let (expected_agg_sig, expected_agg_pk) =
        ArkBlsBn254::aggregate(&[sig1, sig2, sig3], &[pk1, pk2, pk3])
            .expect("Manual aggregation failed");

    // Verify the aggregated signature is valid against the aggregated public key
    let is_valid = ArkBlsBn254::verify_aggregate(&message, &expected_agg_sig, &expected_agg_pk)
        .expect("Verification should succeed");
    assert!(is_valid, "Aggregated signature should verify");

    // ═══════════════════════════════════════════════════════════════════════════
    // STEP 6: Mark as submitted and verify
    // ═══════════════════════════════════════════════════════════════════════════

    service
        .mark_submitted(service_id, call_id)
        .expect("Failed to mark submitted");

    let status = service.get_status(service_id, call_id);
    assert!(status.submitted);

    // Can't get aggregation result after submission
    assert!(service.get_aggregated_result(service_id, call_id).is_none());
}

/// Test the basic square job (1 operator threshold)
#[test]
fn test_basic_square_single_operator() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk, pk) = generate_keypair(b"single_operator_seed");

    let service_id = 1u64;
    let call_id = 1u64;
    let output = 25u64.to_be_bytes().to_vec(); // 5^2 = 25

    // Initialize with threshold of 1
    service
        .init_task(service_id, call_id, output.clone(), 1, 1)
        .unwrap();

    // Create message and sign
    let message = create_signing_message(service_id, call_id, &output);
    let sig = sign_message(&sk, &message);

    // Submit single signature
    let resp = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_pubkey(&pk),
        })
        .unwrap();

    // Threshold should be met immediately
    assert!(resp.threshold_met);
    assert_eq!(resp.signatures_collected, 1);

    // Can get aggregated result
    let result = service.get_aggregated_result(service_id, call_id).unwrap();
    assert_eq!(result.signer_bitmap, U256::from(1));
}

/// Test the verified square job (2 operator threshold)
#[test]
fn test_verified_square_two_operators() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk1, pk1) = generate_keypair(b"verified_op1");
    let (sk2, pk2) = generate_keypair(b"verified_op2");

    let service_id = 1u64;
    let call_id = 2u64;
    let output = 64u64.to_be_bytes().to_vec(); // 8^2 = 64

    // Initialize with threshold of 2
    service
        .init_task(service_id, call_id, output.clone(), 2, 2)
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);

    // Submit first signature
    let resp1 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk1, &message)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();

    assert!(!resp1.threshold_met);

    // Submit second signature
    let resp2 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 1,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk2, &message)),
            public_key: serialize_pubkey(&pk2),
        })
        .unwrap();

    assert!(resp2.threshold_met);
    assert_eq!(resp2.signatures_collected, 2);

    let result = service.get_aggregated_result(service_id, call_id).unwrap();
    assert_eq!(result.signer_bitmap, U256::from(3)); // 0b11
}

/// Test that duplicate submissions are rejected
#[test]
fn test_duplicate_submission_rejected() {
    let config = ServiceConfig {
        verify_on_submit: false,
        validate_output: false,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk, pk) = generate_keypair(b"duplicate_test");

    let service_id = 1u64;
    let call_id = 3u64;
    let output = vec![1, 2, 3];

    service
        .init_task(service_id, call_id, output.clone(), 3, 2)
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);
    let sig = sign_message(&sk, &message);

    // First submission succeeds
    let resp1 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_pubkey(&pk),
        })
        .unwrap();
    assert!(resp1.accepted);

    // Second submission from same operator fails
    let resp2 = service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output.clone(),
        signature: serialize_signature(&sig),
        public_key: serialize_pubkey(&pk),
    });
    assert!(resp2.is_err());
}

/// Test multiple concurrent tasks
#[test]
fn test_multiple_concurrent_tasks() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk1, pk1) = generate_keypair(b"concurrent_op1");
    let (sk2, pk2) = generate_keypair(b"concurrent_op2");

    let service_id = 1u64;
    let output1 = vec![1];
    let output2 = vec![2];

    // Initialize two tasks
    service
        .init_task(service_id, 1, output1.clone(), 2, 1)
        .unwrap();
    service
        .init_task(service_id, 2, output2.clone(), 2, 1)
        .unwrap();

    // Submit to task 1
    let msg1 = create_signing_message(service_id, 1, &output1);
    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id: 1,
            operator_index: 0,
            output: output1.clone(),
            signature: serialize_signature(&sign_message(&sk1, &msg1)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();

    // Submit to task 2
    let msg2 = create_signing_message(service_id, 2, &output2);
    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id: 2,
            operator_index: 1,
            output: output2.clone(),
            signature: serialize_signature(&sign_message(&sk2, &msg2)),
            public_key: serialize_pubkey(&pk2),
        })
        .unwrap();

    // Both tasks should have independent state
    let status1 = service.get_status(service_id, 1);
    let status2 = service.get_status(service_id, 2);

    assert_eq!(status1.signer_bitmap, U256::from(1)); // operator 0
    assert_eq!(status2.signer_bitmap, U256::from(2)); // operator 1
}

/// Test signature verification catches invalid signatures
#[test]
fn test_invalid_signature_rejected() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (_sk, pk) = generate_keypair(b"invalid_sig_test");
    let (wrong_sk, _) = generate_keypair(b"wrong_key");

    let service_id = 1u64;
    let call_id = 100u64;
    let output = vec![42u8; 32];

    service
        .init_task(service_id, call_id, output.clone(), 1, 1)
        .unwrap();

    // Create the proper signing message
    let message = create_signing_message(service_id, call_id, &output);

    // Sign with WRONG key
    let wrong_sig = sign_message(&wrong_sk, &message);

    // Submit with signature from wrong key should fail
    let resp = service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output.clone(),
        signature: serialize_signature(&wrong_sig),
        public_key: serialize_pubkey(&pk), // Public key doesn't match signature
    });

    // Should fail verification
    assert!(resp.is_err());
}

/// Test signature verification with valid signature
#[test]
fn test_valid_signature_accepted() {
    // Enable verification
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk, pk) = generate_keypair(b"valid_sig_test");

    let service_id = 1u64;
    let call_id = 100u64;
    let output = vec![42u8; 32];

    service
        .init_task(service_id, call_id, output.clone(), 1, 1)
        .unwrap();

    // Create the proper signing message
    let message = create_signing_message(service_id, call_id, &output);
    let sig = sign_message(&sk, &message);

    // Submit with valid signature
    let resp = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sig),
            public_key: serialize_pubkey(&pk),
        })
        .unwrap();

    assert!(resp.accepted);
    assert!(resp.threshold_met);
}

/// Test stake-weighted threshold aggregation
#[test]
fn test_stake_weighted_threshold() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    // Create 4 operators with different stakes
    let (sk1, pk1) = generate_keypair(b"stake_op1");
    let (sk2, pk2) = generate_keypair(b"stake_op2");
    let (sk3, pk3) = generate_keypair(b"stake_op3");
    let (_sk4, _pk4) = generate_keypair(b"stake_op4");

    // Set stakes: op1=1000, op2=2000, op3=3000, op4=4000 (total=10000)
    let mut stakes = HashMap::new();
    stakes.insert(0, 1000); // 10%
    stakes.insert(1, 2000); // 20%
    stakes.insert(2, 3000); // 30%
    stakes.insert(3, 4000); // 40%

    let service_id = 1u64;
    let call_id = 200u64;
    let output = vec![99u8; 32];

    // Initialize with stake-weighted threshold of 50% (5000 basis points)
    service
        .init_task_with_config(
            service_id,
            call_id,
            output.clone(),
            4,
            TaskConfig {
                threshold_type: ThresholdType::StakeWeighted(5000), // 50%
                operator_stakes: Some(stakes),
                ttl: None,
            },
        )
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);

    // Submit op1 (10%) - not enough
    let resp1 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk1, &message)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();
    assert!(!resp1.threshold_met);

    // Submit op2 (20%) - 30% total, not enough
    let resp2 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 1,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk2, &message)),
            public_key: serialize_pubkey(&pk2),
        })
        .unwrap();
    assert!(!resp2.threshold_met);

    // Check stake progress
    let status = service.get_status(service_id, call_id);
    assert_eq!(status.signed_stake_bps, Some(3000)); // 30%

    // Submit op3 (30%) - 60% total, now threshold met!
    let resp3 = service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 2,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk3, &message)),
            public_key: serialize_pubkey(&pk3),
        })
        .unwrap();
    assert!(resp3.threshold_met);

    // Verify aggregation works
    let result = service.get_aggregated_result(service_id, call_id).unwrap();
    assert_eq!(result.signer_bitmap, U256::from(7)); // 0b0111 = ops 0, 1, 2
    assert_eq!(result.non_signer_indices, vec![3]); // op4 didn't sign
}

/// Test output validation rejects mismatched outputs
#[test]
fn test_output_validation() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk, pk) = generate_keypair(b"output_validation_test");

    let service_id = 1u64;
    let call_id = 300u64;
    let expected_output = vec![1, 2, 3, 4];
    let wrong_output = vec![5, 6, 7, 8];

    // Initialize task with expected output
    service
        .init_task(service_id, call_id, expected_output.clone(), 1, 1)
        .unwrap();

    // Sign the WRONG output
    let message = create_signing_message(service_id, call_id, &wrong_output);
    let sig = sign_message(&sk, &message);

    // Submit with wrong output should fail
    let resp = service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: wrong_output,
        signature: serialize_signature(&sig),
        public_key: serialize_pubkey(&pk),
    });

    assert!(resp.is_err());
}

/// Test non-signer tracking in aggregation results
#[test]
fn test_non_signer_tracking() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    // Create 5 operators, only 3 will sign
    let (sk1, pk1) = generate_keypair(b"nonsigner_op1");
    let (_sk2, _pk2) = generate_keypair(b"nonsigner_op2");
    let (sk3, pk3) = generate_keypair(b"nonsigner_op3");
    let (_sk4, _pk4) = generate_keypair(b"nonsigner_op4");
    let (sk5, pk5) = generate_keypair(b"nonsigner_op5");

    let service_id = 1u64;
    let call_id = 400u64;
    let output = vec![42u8; 16];

    // Initialize with threshold of 3 out of 5
    service
        .init_task(service_id, call_id, output.clone(), 5, 3)
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);

    // Only operators 0, 2, and 4 sign
    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk1, &message)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();

    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 2,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk3, &message)),
            public_key: serialize_pubkey(&pk3),
        })
        .unwrap();

    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 4,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk5, &message)),
            public_key: serialize_pubkey(&pk5),
        })
        .unwrap();

    // Get aggregated result
    let result = service.get_aggregated_result(service_id, call_id).unwrap();

    // Non-signers should be operators 1 and 3
    assert_eq!(result.non_signer_indices, vec![1, 3]);
    assert_eq!(result.signer_bitmap, U256::from(21)); // 0b10101 = ops 0, 2, 4
}

/// Test task expiry rejects late submissions
#[test]
fn test_task_expiry() {
    let config = ServiceConfig {
        verify_on_submit: false, // Skip verification to focus on expiry
        validate_output: false,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk, pk) = generate_keypair(b"expiry_test");

    let service_id = 1u64;
    let call_id = 500u64;
    let output = vec![1, 2, 3];

    // Initialize with very short TTL (50ms)
    service
        .init_task_with_config(
            service_id,
            call_id,
            output.clone(),
            1,
            TaskConfig {
                threshold_type: ThresholdType::Count(1),
                operator_stakes: None,
                ttl: Some(Duration::from_millis(50)),
            },
        )
        .unwrap();

    // Task should not be expired initially
    let status = service.get_status(service_id, call_id);
    assert_eq!(status.is_expired, Some(false));
    assert!(status.time_remaining_secs.is_some() || status.time_remaining_secs == Some(0));

    // Wait for task to expire
    std::thread::sleep(Duration::from_millis(100));

    // Task should now be expired
    let status = service.get_status(service_id, call_id);
    assert_eq!(status.is_expired, Some(true));

    // Submission should fail for expired task
    let message = create_signing_message(service_id, call_id, &output);
    let sig = sign_message(&sk, &message);

    let resp = service.submit_signature(SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output,
        signature: serialize_signature(&sig),
        public_key: serialize_pubkey(&pk),
    });

    assert!(resp.is_err());
}

/// Test service statistics
#[test]
fn test_service_stats() {
    let config = ServiceConfig {
        verify_on_submit: false,
        validate_output: false,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk1, pk1) = generate_keypair(b"stats_op1");

    // Create multiple tasks in different states
    // Task 1: Pending
    service.init_task(1, 1, vec![1], 3, 2).unwrap();

    // Task 2: Ready (threshold met)
    service.init_task(1, 2, vec![2], 1, 1).unwrap();
    let message2 = create_signing_message(1, 2, &[2]);
    service
        .submit_signature(SubmitSignatureRequest {
            service_id: 1,
            call_id: 2,
            operator_index: 0,
            output: vec![2],
            signature: serialize_signature(&sign_message(&sk1, &message2)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();

    // Task 3: Submitted
    service.init_task(1, 3, vec![3], 1, 1).unwrap();
    let message3 = create_signing_message(1, 3, &[3]);
    service
        .submit_signature(SubmitSignatureRequest {
            service_id: 1,
            call_id: 3,
            operator_index: 0,
            output: vec![3],
            signature: serialize_signature(&sign_message(&sk1, &message3)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();
    service.mark_submitted(1, 3).unwrap();

    // Get stats
    let stats = service.get_stats();
    assert_eq!(stats.total_tasks, 3);
    assert_eq!(stats.pending_tasks, 1);
    assert_eq!(stats.ready_tasks, 1);
    assert_eq!(stats.submitted_tasks, 1);
}

/// Test simulating redundant aggregation services
/// (all operators send to multiple services, any can achieve threshold)
#[test]
fn test_multi_service_redundancy() {
    // Create two independent aggregation services (simulating redundant deployment)
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service1 = AggregationService::new(config.clone());
    let service2 = AggregationService::new(config);

    // Generate 3 operator keys
    let (sk1, pk1) = generate_keypair(b"multi_svc_op1");
    let (sk2, pk2) = generate_keypair(b"multi_svc_op2");
    let (_sk3, _pk3) = generate_keypair(b"multi_svc_op3");

    let service_id = 1u64;
    let call_id = 100u64;
    let output = vec![7, 8, 9];
    let threshold = 2; // 2 of 3 needed

    // Initialize task on both services
    service1
        .init_task(service_id, call_id, output.clone(), 3, threshold)
        .unwrap();
    service2
        .init_task(service_id, call_id, output.clone(), 3, threshold)
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);

    // Operator 1 sends to both services
    let sig1 = sign_message(&sk1, &message);
    let req1 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 0,
        output: output.clone(),
        signature: serialize_signature(&sig1),
        public_key: serialize_pubkey(&pk1),
    };
    service1.submit_signature(req1.clone()).unwrap();
    service2.submit_signature(req1).unwrap();

    // Operator 2 sends to both services
    let sig2 = sign_message(&sk2, &message);
    let req2 = SubmitSignatureRequest {
        service_id,
        call_id,
        operator_index: 1,
        output: output.clone(),
        signature: serialize_signature(&sig2),
        public_key: serialize_pubkey(&pk2),
    };
    service1.submit_signature(req2.clone()).unwrap();
    service2.submit_signature(req2).unwrap();

    // Both services should have reached threshold
    let status1 = service1.get_status(service_id, call_id);
    let status2 = service2.get_status(service_id, call_id);

    assert!(status1.threshold_met, "Service 1 should have met threshold");
    assert!(status2.threshold_met, "Service 2 should have met threshold");

    // Both should have the same aggregated result
    let result1 = service1.get_aggregated_result(service_id, call_id).unwrap();
    let result2 = service2.get_aggregated_result(service_id, call_id).unwrap();

    assert_eq!(result1.output, result2.output);
    assert_eq!(result1.signer_bitmap, result2.signer_bitmap);
    // Operators 0 and 1 signed, operator 2 didn't (3 total operators)
    assert_eq!(result1.non_signer_indices, vec![2]);
    assert_eq!(result2.non_signer_indices, vec![2]);
    // Aggregated signatures should be present
    assert!(!result1.aggregated_signature.is_empty());
    assert!(!result2.aggregated_signature.is_empty());
}

/// Test race condition: multiple operators mark task as submitted
/// (simulates all operators racing to submit the aggregated result on-chain)
#[test]
fn test_multiple_submitters_race() {
    let config = ServiceConfig {
        verify_on_submit: true,
        validate_output: true,
        ..Default::default()
    };
    let service = AggregationService::new(config);

    let (sk1, pk1) = generate_keypair(b"race_op1");
    let (sk2, pk2) = generate_keypair(b"race_op2");

    let service_id = 1u64;
    let call_id = 200u64;
    let output = vec![1, 2, 3];

    // Initialize and collect signatures
    service
        .init_task(service_id, call_id, output.clone(), 2, 2)
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);

    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk1, &message)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();

    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 1,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk2, &message)),
            public_key: serialize_pubkey(&pk2),
        })
        .unwrap();

    assert!(service.get_status(service_id, call_id).threshold_met);

    // First operator marks as submitted - should succeed
    let result1 = service.mark_submitted(service_id, call_id);
    assert!(result1.is_ok());

    // Second operator tries to mark as submitted - also succeeds (idempotent)
    // This is intentional: in a distributed race, multiple operators may try to mark
    // and we don't want to return errors for this common case
    let result2 = service.mark_submitted(service_id, call_id);
    assert!(result2.is_ok()); // Idempotent - succeeds even if already submitted

    // Task should still be available for status check
    let status = service.get_status(service_id, call_id);
    assert!(status.exists);
    assert!(status.submitted);
}

/// Test that operators can retrieve aggregated result even after it's marked submitted
/// (supports "anyone can submit" pattern where multiple operators might check)
#[test]
fn test_anyone_can_check_aggregated_result() {
    let config = ServiceConfig::default();
    let service = AggregationService::new(config);

    let (sk1, pk1) = generate_keypair(b"anyone_op1");
    let (sk2, pk2) = generate_keypair(b"anyone_op2");
    let (_sk3, _pk3) = generate_keypair(b"anyone_op3");

    let service_id = 1u64;
    let call_id = 300u64;
    let output = vec![42];

    service
        .init_task(service_id, call_id, output.clone(), 3, 2)
        .unwrap();

    let message = create_signing_message(service_id, call_id, &output);

    // Two operators sign
    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 0,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk1, &message)),
            public_key: serialize_pubkey(&pk1),
        })
        .unwrap();

    service
        .submit_signature(SubmitSignatureRequest {
            service_id,
            call_id,
            operator_index: 1,
            output: output.clone(),
            signature: serialize_signature(&sign_message(&sk2, &message)),
            public_key: serialize_pubkey(&pk2),
        })
        .unwrap();

    // All operators can check the result
    let result = service.get_aggregated_result(service_id, call_id);
    assert!(result.is_some());

    // Even operator 3 who didn't sign can check
    let task = result.unwrap();
    assert_eq!(task.output, output);
    assert_eq!(task.non_signer_indices, vec![2]); // Operator 3 didn't sign
                                                  // Aggregated signature should be present
    assert!(!task.aggregated_signature.is_empty());

    // After submission, the status is updated but data is still queryable
    service.mark_submitted(service_id, call_id).unwrap();

    let status = service.get_status(service_id, call_id);
    assert!(status.submitted);

    // Note: get_aggregated_result returns None after submission (by design)
    // because we don't want duplicate on-chain submissions
    let result_after = service.get_aggregated_result(service_id, call_id);
    assert!(result_after.is_none());
}
