//! Incredible Squaring Blueprint Tests
//!
//! These tests verify the EVM-based job processing for the incredible squaring blueprint.
//! They test ABI encoding/decoding and the job function logic directly.

use alloy_sol_types::SolValue;
use blueprint_sdk::IntoJobResult;
use blueprint_sdk::testing::utils::setup_log;
use color_eyre::Result;
use incredible_squaring_blueprint_lib::{
    CONSENSUS_XSQUARE_JOB_ID, VERIFIED_XSQUARE_JOB_ID, XSQUARE_JOB_ID, consensus_square, square,
    verified_square,
};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment (color_eyre, logging) - safe to call multiple times
fn init_test() {
    INIT.call_once(|| {
        let _ = color_eyre::install();
        setup_log();
    });
}

/// Test that the job IDs are correctly set
#[test]
fn test_job_ids() {
    assert_eq!(XSQUARE_JOB_ID, 0, "Square job should have index 0");
    assert_eq!(
        VERIFIED_XSQUARE_JOB_ID, 1,
        "Verified square job should have index 1"
    );
    assert_eq!(
        CONSENSUS_XSQUARE_JOB_ID, 2,
        "Consensus square job should have index 2"
    );
}

/// Test that all job IDs are unique
#[test]
fn test_job_ids_unique() {
    assert_ne!(XSQUARE_JOB_ID, VERIFIED_XSQUARE_JOB_ID);
    assert_ne!(XSQUARE_JOB_ID, CONSENSUS_XSQUARE_JOB_ID);
    assert_ne!(VERIFIED_XSQUARE_JOB_ID, CONSENSUS_XSQUARE_JOB_ID);
}

/// Test ABI encoding/decoding round-trip for u64
#[test]
fn test_abi_roundtrip_u64() {
    // Test encoding a u64 value
    let input: u64 = 12345;
    let encoded = input.abi_encode();

    // Verify it's 32 bytes (ABI-encoded u64 is padded to 32 bytes)
    assert_eq!(encoded.len(), 32, "ABI-encoded u64 should be 32 bytes");

    // Decode it back
    let decoded = u64::abi_decode(&encoded).expect("Should decode successfully");
    assert_eq!(decoded, input, "Decoded value should match original");
}

/// Test the square function directly
#[tokio::test]
async fn test_square_function() -> Result<()> {
    init_test();

    // Test various input values
    let test_cases: Vec<(u64, u64)> = vec![
        (0, 0),
        (1, 1),
        (2, 4),
        (5, 25),
        (10, 100),
        (100, 10000),
        (1000, 1000000),
    ];

    for (input, expected) in test_cases {
        // Create a mock job call with ABI-encoded input
        let encoded_input = input.abi_encode();

        // Simulate what the TangleProducer does: create a JobCall with the encoded input
        let job_call =
            blueprint_sdk::JobCall::new(XSQUARE_JOB_ID, bytes::Bytes::from(encoded_input));

        // The extractor would normally extract from the job call
        // For direct testing, we can decode the body manually
        let (_, body) = job_call.into_parts();
        let decoded_input = u64::abi_decode(&body).expect("Should decode input");
        assert_eq!(decoded_input, input, "Decoded input should match");

        // Call the square function with the extracted argument
        use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
        let result: TangleResult<u64> = square(TangleArg(input)).await;

        // Verify the result
        assert_eq!(
            *result, expected,
            "Square of {} should be {}",
            input, expected
        );

        // Convert to JobResult and verify ABI encoding
        let job_result = result.into_job_result();
        assert!(job_result.is_some(), "Should produce a job result");

        if let Some(blueprint_sdk::JobResult::Ok { body, .. }) = job_result {
            let decoded_output = u64::abi_decode(&body).expect("Should decode output");
            assert_eq!(
                decoded_output, expected,
                "Decoded output should match expected"
            );
        } else {
            panic!("Expected Ok job result");
        }
    }

    Ok(())
}

/// Test edge cases for squaring
#[tokio::test]
async fn test_square_edge_cases() -> Result<()> {
    init_test();

    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // Test maximum safe value that won't overflow
    // sqrt(u64::MAX) ≈ 4294967295.99, so max safe input is 4294967295
    let max_safe: u64 = 4294967295; // 2^32 - 1
    let result: TangleResult<u64> = square(TangleArg(max_safe)).await;
    assert_eq!(*result, max_safe * max_safe);

    // Test powers of 2
    for power in 0..16 {
        let input: u64 = 1 << power;
        let result: TangleResult<u64> = square(TangleArg(input)).await;
        let expected = input * input;
        assert_eq!(*result, expected, "Square of 2^{} should be correct", power);
    }

    Ok(())
}

/// Test the TangleArg extractor with a real JobCall
#[tokio::test]
async fn test_tangle_arg_extractor() -> Result<()> {
    use blueprint_sdk::FromJobCall;
    use blueprint_sdk::tangle::extract::TangleArg;

    // Create a job call with ABI-encoded u64
    let input: u64 = 42;
    let encoded = input.abi_encode();
    let job_call = blueprint_sdk::JobCall::new(XSQUARE_JOB_ID, bytes::Bytes::from(encoded));

    // Extract using the TangleArg extractor
    let extracted: TangleArg<u64> = TangleArg::from_job_call(job_call, &()).await?;

    assert_eq!(*extracted, 42);

    Ok(())
}

/// Test that TangleResult properly ABI-encodes the output
#[test]
fn test_tangle_result_encoding() {
    use blueprint_sdk::tangle::extract::TangleResult;

    let result: TangleResult<u64> = TangleResult(12345);
    let job_result = result.into_job_result();

    assert!(job_result.is_some());

    if let Some(blueprint_sdk::JobResult::Ok { body, .. }) = job_result {
        // Verify it's valid ABI encoding
        let decoded = u64::abi_decode(&body).expect("Should decode");
        assert_eq!(decoded, 12345);

        // Verify the encoding is 32 bytes
        assert_eq!(body.len(), 32);
    } else {
        panic!("Expected Ok result");
    }
}

/// Test the full job execution flow (encode input -> call function -> encode output)
#[tokio::test]
async fn test_full_job_flow() -> Result<()> {
    use blueprint_sdk::FromJobCall;
    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // Simulate what happens in a real job execution:
    // 1. Input is ABI-encoded by the caller (user/contract)
    let user_input: u64 = 7;
    let abi_encoded_input = user_input.abi_encode();

    // 2. TangleProducer creates a JobCall with this input
    let job_call =
        blueprint_sdk::JobCall::new(XSQUARE_JOB_ID, bytes::Bytes::from(abi_encoded_input));

    // 3. The router dispatches to the job function, which extracts the arg
    let TangleArg(x): TangleArg<u64> = TangleArg::from_job_call(job_call, &()).await?;
    assert_eq!(x, 7);

    // 4. The job function computes the result
    let result: TangleResult<u64> = square(TangleArg(x)).await;
    assert_eq!(*result, 49);

    // 5. The result is converted to a JobResult with ABI-encoded body
    let job_result = result.into_job_result().unwrap();

    // 6. TangleConsumer would submit this to the contract
    if let blueprint_sdk::JobResult::Ok { body, .. } = job_result {
        // The contract would decode this
        let contract_received = u64::abi_decode(&body)?;
        assert_eq!(contract_received, 49);
    }

    Ok(())
}

/// Test that the background service starts correctly
#[tokio::test]
async fn test_background_service() -> Result<()> {
    use blueprint_sdk::runner::BackgroundService;
    use incredible_squaring_blueprint_lib::FooBackgroundService;

    let service = FooBackgroundService;
    let rx = service.start().await?;

    // The service should complete successfully
    let result = rx.await?;
    assert!(result.is_ok());

    Ok(())
}

// ═══════════════════════════════════════════════════════════════════════════
// AGGREGATION JOB TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test the verified_square function (requires 2 operator results for aggregation)
#[tokio::test]
async fn test_verified_square_function() -> Result<()> {
    init_test();

    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // Test various input values
    let test_cases: Vec<(u64, u64)> = vec![(0, 0), (1, 1), (5, 25), (100, 10000)];

    for (input, expected) in test_cases {
        let result: TangleResult<u64> = verified_square(TangleArg(input)).await;
        assert_eq!(
            *result, expected,
            "Verified square of {} should be {}",
            input, expected
        );

        // Convert to JobResult and verify ABI encoding
        let job_result = result.into_job_result();
        assert!(job_result.is_some(), "Should produce a job result");

        if let Some(blueprint_sdk::JobResult::Ok { body, .. }) = job_result {
            let decoded_output = u64::abi_decode(&body).expect("Should decode output");
            assert_eq!(
                decoded_output, expected,
                "Decoded output should match expected"
            );
        } else {
            panic!("Expected Ok job result");
        }
    }

    Ok(())
}

/// Test the consensus_square function (requires 3 operator results for aggregation)
#[tokio::test]
async fn test_consensus_square_function() -> Result<()> {
    init_test();

    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // Test various input values
    let test_cases: Vec<(u64, u64)> = vec![(0, 0), (1, 1), (7, 49), (1000, 1000000)];

    for (input, expected) in test_cases {
        let result: TangleResult<u64> = consensus_square(TangleArg(input)).await;
        assert_eq!(
            *result, expected,
            "Consensus square of {} should be {}",
            input, expected
        );

        // Convert to JobResult and verify ABI encoding
        let job_result = result.into_job_result();
        assert!(job_result.is_some(), "Should produce a job result");

        if let Some(blueprint_sdk::JobResult::Ok { body, .. }) = job_result {
            let decoded_output = u64::abi_decode(&body).expect("Should decode output");
            assert_eq!(
                decoded_output, expected,
                "Decoded output should match expected"
            );
        } else {
            panic!("Expected Ok job result");
        }
    }

    Ok(())
}

/// Test the full job flow for verified_square (2-operator aggregation)
#[tokio::test]
async fn test_verified_square_job_flow() -> Result<()> {
    use blueprint_sdk::FromJobCall;
    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // Simulate operator 1 processing
    let user_input: u64 = 8;
    let abi_encoded_input = user_input.abi_encode();
    let job_call = blueprint_sdk::JobCall::new(
        VERIFIED_XSQUARE_JOB_ID,
        bytes::Bytes::from(abi_encoded_input.clone()),
    );

    let TangleArg(x): TangleArg<u64> = TangleArg::from_job_call(job_call, &()).await?;
    let result_op1: TangleResult<u64> = verified_square(TangleArg(x)).await;
    assert_eq!(*result_op1, 64);

    // Simulate operator 2 processing (same input, same expected output)
    let job_call_op2 = blueprint_sdk::JobCall::new(
        VERIFIED_XSQUARE_JOB_ID,
        bytes::Bytes::from(abi_encoded_input),
    );

    let TangleArg(x2): TangleArg<u64> = TangleArg::from_job_call(job_call_op2, &()).await?;
    let result_op2: TangleResult<u64> = verified_square(TangleArg(x2)).await;
    assert_eq!(*result_op2, 64);

    // Both operators should produce identical results
    assert_eq!(
        *result_op1, *result_op2,
        "Both operators should produce the same result"
    );

    // In the real system, the BSM's getRequiredResultCount returns 2 for this job,
    // so both results would need to be submitted before the job is considered complete.

    Ok(())
}

/// Test the full job flow for consensus_square (3-operator aggregation)
#[tokio::test]
async fn test_consensus_square_job_flow() -> Result<()> {
    use blueprint_sdk::FromJobCall;
    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // Simulate 3 operators processing the same job
    let user_input: u64 = 9;
    let abi_encoded_input = user_input.abi_encode();

    // Operator 1
    let job_call_1 = blueprint_sdk::JobCall::new(
        CONSENSUS_XSQUARE_JOB_ID,
        bytes::Bytes::from(abi_encoded_input.clone()),
    );
    let TangleArg(x1): TangleArg<u64> = TangleArg::from_job_call(job_call_1, &()).await?;
    let result_op1: TangleResult<u64> = consensus_square(TangleArg(x1)).await;

    // Operator 2
    let job_call_2 = blueprint_sdk::JobCall::new(
        CONSENSUS_XSQUARE_JOB_ID,
        bytes::Bytes::from(abi_encoded_input.clone()),
    );
    let TangleArg(x2): TangleArg<u64> = TangleArg::from_job_call(job_call_2, &()).await?;
    let result_op2: TangleResult<u64> = consensus_square(TangleArg(x2)).await;

    // Operator 3
    let job_call_3 = blueprint_sdk::JobCall::new(
        CONSENSUS_XSQUARE_JOB_ID,
        bytes::Bytes::from(abi_encoded_input),
    );
    let TangleArg(x3): TangleArg<u64> = TangleArg::from_job_call(job_call_3, &()).await?;
    let result_op3: TangleResult<u64> = consensus_square(TangleArg(x3)).await;

    // All three operators should produce identical results
    assert_eq!(*result_op1, 81);
    assert_eq!(*result_op2, 81);
    assert_eq!(*result_op3, 81);
    assert_eq!(*result_op1, *result_op2);
    assert_eq!(*result_op2, *result_op3);

    // In the real system, the BSM's getRequiredResultCount returns 3 for this job,
    // so all three results would need to be submitted before the job is considered complete.
    // This provides Byzantine fault tolerance - even if one operator is malicious,
    // the honest majority (2/3) will produce the correct result.

    Ok(())
}

/// Test that all job functions produce consistent results for the same input
#[tokio::test]
async fn test_all_jobs_consistent() -> Result<()> {
    init_test();

    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    // All three job types should compute the same mathematical result
    let inputs = [0u64, 1, 5, 10, 100, 1000];

    for input in inputs {
        let result_basic: TangleResult<u64> = square(TangleArg(input)).await;
        let result_verified: TangleResult<u64> = verified_square(TangleArg(input)).await;
        let result_consensus: TangleResult<u64> = consensus_square(TangleArg(input)).await;

        let expected = input * input;

        assert_eq!(
            *result_basic, expected,
            "Basic square failed for input {}",
            input
        );
        assert_eq!(
            *result_verified, expected,
            "Verified square failed for input {}",
            input
        );
        assert_eq!(
            *result_consensus, expected,
            "Consensus square failed for input {}",
            input
        );

        // All should be equal
        assert_eq!(*result_basic, *result_verified);
        assert_eq!(*result_verified, *result_consensus);
    }

    Ok(())
}

/// Test aggregation scenario: simulating multiple operators submitting results
///
/// This test demonstrates the conceptual flow of job aggregation:
/// - Job 0 (square): Only 1 operator result needed
/// - Job 1 (verified_square): 2 operator results needed
/// - Job 2 (consensus_square): 3 operator results needed
///
/// The actual aggregation logic is handled by the BSM contract's
/// getRequiredResultCount() function and the Tangle runtime.
#[tokio::test]
async fn test_aggregation_scenario() -> Result<()> {
    init_test();

    use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};

    let input: u64 = 6;
    let expected_result: u64 = 36;

    // Job 0: Basic square - completes with 1 operator
    let result: TangleResult<u64> = square(TangleArg(input)).await;
    assert_eq!(*result, expected_result);
    // Job complete after 1 result ✓

    // Job 1: Verified square - needs 2 operators
    let result_v1: TangleResult<u64> = verified_square(TangleArg(input)).await;
    let result_v2: TangleResult<u64> = verified_square(TangleArg(input)).await;
    assert_eq!(*result_v1, expected_result);
    assert_eq!(*result_v2, expected_result);
    // Job complete after 2 matching results ✓

    // Job 2: Consensus square - needs 3 operators
    let result_c1: TangleResult<u64> = consensus_square(TangleArg(input)).await;
    let result_c2: TangleResult<u64> = consensus_square(TangleArg(input)).await;
    let result_c3: TangleResult<u64> = consensus_square(TangleArg(input)).await;
    assert_eq!(*result_c1, expected_result);
    assert_eq!(*result_c2, expected_result);
    assert_eq!(*result_c3, expected_result);
    // Job complete after 3 matching results ✓

    Ok(())
}
