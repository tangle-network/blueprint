//! Direct Submission Tests (No Aggregation)
//!
//! These tests verify the scenario where multiple operators run a job
//! independently and each submits their result directly to the chain,
//! without going through BLS signature aggregation.
//!
//! This is the "no aggregation" flow:
//! 1. Job is triggered on-chain
//! 2. All running operators receive the job event
//! 3. Each operator computes the result independently
//! 4. Each operator submits their result directly via TangleEvmConsumer
//! 5. Contract receives N individual results from N operators
//!
//! Use cases:
//! - Jobs that don't require consensus (any operator's result is valid)
//! - Jobs where multiple results are desired for redundancy/comparison
//! - Simple jobs where aggregation overhead isn't justified

use alloy_sol_types::SolValue;
use blueprint_sdk::tangle_evm::extract::{TangleEvmArg, TangleEvmResult};
use blueprint_sdk::testing::utils::setup_log;
use blueprint_sdk::{IntoJobResult, JobResult};
use color_eyre::Result;
use incredible_squaring_blueprint_lib::{
    CONSENSUS_XSQUARE_JOB_ID, VERIFIED_XSQUARE_JOB_ID, XSQUARE_JOB_ID, consensus_square, square,
    verified_square,
};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize test environment
fn init_test() {
    INIT.call_once(|| {
        let _ = color_eyre::install();
        setup_log();
    });
}

/// Simulated operator that computes a job result
struct SimulatedOperator {
    id: usize,
    #[allow(dead_code)]
    address: String,
}

impl SimulatedOperator {
    fn new(id: usize) -> Self {
        Self {
            id,
            address: format!("0x{:040x}", id + 1),
        }
    }

    /// Compute the square job result
    async fn compute_square(&self, input: u64) -> TangleEvmResult<u64> {
        println!("Operator {} computing square({})", self.id, input);
        square(TangleEvmArg(input)).await
    }

    /// Compute the verified_square job result
    async fn compute_verified_square(&self, input: u64) -> TangleEvmResult<u64> {
        println!("Operator {} computing verified_square({})", self.id, input);
        verified_square(TangleEvmArg(input)).await
    }

    /// Compute the consensus_square job result
    async fn compute_consensus_square(&self, input: u64) -> TangleEvmResult<u64> {
        println!("Operator {} computing consensus_square({})", self.id, input);
        consensus_square(TangleEvmArg(input)).await
    }
}

/// Simulated job result that would be submitted to the contract
#[allow(dead_code)]
struct SubmittedResult {
    operator_id: usize,
    service_id: u64,
    call_id: u64,
    output: Vec<u8>,
}

// ═══════════════════════════════════════════════════════════════════════════
// DIRECT SUBMISSION TESTS
// ═══════════════════════════════════════════════════════════════════════════

/// Test: Multiple operators compute and submit the same square job independently
///
/// Scenario: 3 operators are running, a square(5) job is triggered.
/// All 3 operators compute the result and submit directly.
/// Each should produce the same output (25).
#[tokio::test]
async fn test_multi_operator_direct_submission_square() -> Result<()> {
    init_test();
    println!("\n=== Multi-Operator Direct Submission: square job ===\n");

    let operators: Vec<SimulatedOperator> = (0..3).map(SimulatedOperator::new).collect();

    let service_id = 1u64;
    let call_id = 100u64;
    let input: u64 = 5;
    let expected_output: u64 = 25;

    println!("Job: square({}) = {}", input, expected_output);
    println!("Operators running: {}", operators.len());
    println!();

    // Each operator computes the result independently
    let mut submitted_results: Vec<SubmittedResult> = Vec::new();

    for operator in &operators {
        let result = operator.compute_square(input).await;

        // Verify computation is correct
        assert_eq!(
            *result, expected_output,
            "Operator {} computed wrong result",
            operator.id
        );

        // Convert to JobResult (what TangleEvmConsumer would receive)
        let job_result = result.into_job_result();
        assert!(job_result.is_some(), "Should produce a job result");

        if let Some(JobResult::Ok { body, .. }) = job_result {
            // Verify the ABI-encoded output
            let decoded = u64::abi_decode(&body)?;
            assert_eq!(decoded, expected_output);

            // Simulate what would be submitted
            submitted_results.push(SubmittedResult {
                operator_id: operator.id,
                service_id,
                call_id,
                output: body.to_vec(),
            });

            println!(
                "  Operator {} would submit: service_id={}, call_id={}, output={}",
                operator.id, service_id, call_id, decoded
            );
        }
    }

    // Verify all operators submitted
    assert_eq!(submitted_results.len(), 3, "All 3 operators should submit");

    // Verify all results are identical
    let first_output = &submitted_results[0].output;
    for result in &submitted_results {
        assert_eq!(
            &result.output, first_output,
            "All operators should produce identical output"
        );
    }

    println!("\n=== Test Passed! ===");
    println!(
        "All {} operators computed and would submit identical results",
        operators.len()
    );

    Ok(())
}

/// Test: Multiple operators submit verified_square job independently
///
/// This job requires 2 results. Without aggregation, each operator
/// submits directly and the contract tracks result count.
#[tokio::test]
async fn test_multi_operator_direct_submission_verified_square() -> Result<()> {
    init_test();
    println!("\n=== Multi-Operator Direct Submission: verified_square job ===\n");

    let operators: Vec<SimulatedOperator> = (0..3).map(SimulatedOperator::new).collect();

    let service_id = 2u64;
    let call_id = 200u64;
    let input: u64 = 7;
    let expected_output: u64 = 49;

    println!(
        "Job: verified_square({}) = {} (requires 2 results)",
        input, expected_output
    );
    println!("Operators running: {}", operators.len());
    println!();

    let mut submitted_results: Vec<SubmittedResult> = Vec::new();

    for operator in &operators {
        let result = operator.compute_verified_square(input).await;
        assert_eq!(*result, expected_output);

        let job_result = result.into_job_result();
        if let Some(JobResult::Ok { body, .. }) = job_result {
            submitted_results.push(SubmittedResult {
                operator_id: operator.id,
                service_id,
                call_id,
                output: body.to_vec(),
            });
            println!("  Operator {} submitted result", operator.id);
        }
    }

    // All 3 operators submitted, but only 2 are required
    assert_eq!(submitted_results.len(), 3);
    println!("\nContract would receive 3 results (2 required for verification)");
    println!(
        "  - Result count meets requirement: {} >= 2",
        submitted_results.len()
    );

    Ok(())
}

/// Test: Multiple operators submit consensus_square job independently
///
/// This job requires 3 results. All operators must submit.
#[tokio::test]
async fn test_multi_operator_direct_submission_consensus_square() -> Result<()> {
    init_test();
    println!("\n=== Multi-Operator Direct Submission: consensus_square job ===\n");

    let operators: Vec<SimulatedOperator> = (0..5).map(SimulatedOperator::new).collect();

    let service_id = 3u64;
    let call_id = 300u64;
    let input: u64 = 6;
    let expected_output: u64 = 36;

    println!(
        "Job: consensus_square({}) = {} (requires 3 results)",
        input, expected_output
    );
    println!("Operators running: {}", operators.len());
    println!();

    let mut submitted_results: Vec<SubmittedResult> = Vec::new();

    for operator in &operators {
        let result = operator.compute_consensus_square(input).await;
        assert_eq!(*result, expected_output);

        let job_result = result.into_job_result();
        if let Some(JobResult::Ok { body, .. }) = job_result {
            submitted_results.push(SubmittedResult {
                operator_id: operator.id,
                service_id,
                call_id,
                output: body.to_vec(),
            });
        }
    }

    assert_eq!(submitted_results.len(), 5);
    println!("Contract would receive 5 results (3 required for consensus)");
    println!(
        "  - Consensus requirement met: {} >= 3",
        submitted_results.len()
    );

    Ok(())
}

/// Test: Verify job results contain correct job ID metadata
#[tokio::test]
async fn test_direct_submission_job_ids() -> Result<()> {
    init_test();
    println!("\n=== Direct Submission: Job ID Verification ===\n");

    let operator = SimulatedOperator::new(0);
    let input: u64 = 10;

    // Test each job type produces results that could be matched to correct job
    let square_result = operator.compute_square(input).await;
    let verified_result = operator.compute_verified_square(input).await;
    let consensus_result = operator.compute_consensus_square(input).await;

    // All should compute the same value (100)
    assert_eq!(*square_result, 100);
    assert_eq!(*verified_result, 100);
    assert_eq!(*consensus_result, 100);

    // All should produce valid job results
    assert!(square_result.into_job_result().is_some());
    assert!(verified_result.into_job_result().is_some());
    assert!(consensus_result.into_job_result().is_some());

    println!("All job types produce valid results:");
    println!("  - square (job_id={}) -> {}", XSQUARE_JOB_ID, 100);
    println!(
        "  - verified_square (job_id={}) -> {}",
        VERIFIED_XSQUARE_JOB_ID, 100
    );
    println!(
        "  - consensus_square (job_id={}) -> {}",
        CONSENSUS_XSQUARE_JOB_ID, 100
    );

    Ok(())
}

/// Test: Simulate tracking which operators have submitted
///
/// This mimics what the BSM contract does with operatorSubmittedResult mapping.
#[tokio::test]
async fn test_track_operator_submissions() -> Result<()> {
    init_test();
    println!("\n=== Tracking Operator Submissions ===\n");

    let operators: Vec<SimulatedOperator> = (0..5).map(SimulatedOperator::new).collect();

    let _service_id = 4u64;
    let _call_id = 400u64;
    let input: u64 = 3;

    // Track submissions like the contract would
    let mut operator_submitted: std::collections::HashMap<usize, bool> =
        std::collections::HashMap::new();
    let mut result_count = 0usize;

    println!("Simulating partial submission (only operators 0, 2, 4 submit):\n");

    // Only some operators submit (simulating real-world scenario)
    for &idx in &[0usize, 2, 4] {
        let operator = &operators[idx];
        let result = operator.compute_square(input).await;

        if result.into_job_result().is_some() {
            operator_submitted.insert(operator.id, true);
            result_count += 1;
            println!(
                "  Operator {} submitted (total: {})",
                operator.id, result_count
            );
        }
    }

    // Verify tracking
    assert!(operator_submitted.get(&0).copied().unwrap_or(false));
    assert!(!operator_submitted.get(&1).copied().unwrap_or(false));
    assert!(operator_submitted.get(&2).copied().unwrap_or(false));
    assert!(!operator_submitted.get(&3).copied().unwrap_or(false));
    assert!(operator_submitted.get(&4).copied().unwrap_or(false));

    println!("\nSubmission tracking:");
    for i in 0..5 {
        let submitted = operator_submitted.get(&i).copied().unwrap_or(false);
        println!(
            "  Operator {}: {}",
            i,
            if submitted {
                "SUBMITTED"
            } else {
                "not submitted"
            }
        );
    }
    println!(
        "\nTotal results: {} / {} operators",
        result_count,
        operators.len()
    );

    Ok(())
}

/// Test: Compare direct submission vs aggregation scenarios
///
/// Demonstrates the difference between:
/// - Direct: Each operator submits independently (N transactions)
/// - Aggregated: Signatures collected, one aggregated submission (1 transaction)
#[tokio::test]
async fn test_direct_vs_aggregated_comparison() -> Result<()> {
    init_test();
    println!("\n=== Direct Submission vs Aggregation Comparison ===\n");

    let num_operators = 5;
    let operators: Vec<SimulatedOperator> =
        (0..num_operators).map(SimulatedOperator::new).collect();

    let input: u64 = 8;
    let expected: u64 = 64;

    println!(
        "Scenario: {} operators, square({}) = {}\n",
        num_operators, input, expected
    );

    // Direct submission: each operator submits
    println!("DIRECT SUBMISSION (no aggregation):");
    println!("  - Each operator computes result locally");
    println!("  - Each operator submits result to chain");
    println!("  - Contract receives {} individual results", num_operators);
    println!("  - Transactions required: {}", num_operators);

    let mut direct_results = Vec::new();
    for op in &operators {
        let result = op.compute_square(input).await;
        assert_eq!(*result, expected);
        direct_results.push(result);
    }

    println!("\nAGGREGATED SUBMISSION (with BLS aggregation):");
    println!("  - Each operator computes result locally");
    println!("  - Each operator signs result with BLS key");
    println!("  - Signatures aggregated by aggregation service");
    println!("  - ONE aggregated result submitted to chain");
    println!("  - Transactions required: 1");

    println!("\nTRADE-OFFS:");
    println!(
        "  Direct:     Simpler, no coordination, but {} txs",
        num_operators
    );
    println!("  Aggregated: More complex, requires coordination, but 1 tx");
    println!("  Gas savings with aggregation: ~{}x", num_operators);

    Ok(())
}

/// Test: Large number of operators submitting directly
///
/// Stress test simulating many operators in a service.
#[tokio::test]
async fn test_many_operators_direct_submission() -> Result<()> {
    init_test();
    println!("\n=== Many Operators Direct Submission ===\n");

    let num_operators = 20;
    let operators: Vec<SimulatedOperator> =
        (0..num_operators).map(SimulatedOperator::new).collect();

    let input: u64 = 12;
    let expected: u64 = 144;

    println!(
        "Simulating {} operators computing square({})",
        num_operators, input
    );

    let mut all_results_correct = true;
    let mut result_count = 0;

    for operator in &operators {
        let result = operator.compute_square(input).await;
        if *result != expected {
            all_results_correct = false;
            println!("  ERROR: Operator {} computed wrong result", operator.id);
        }

        if result.into_job_result().is_some() {
            result_count += 1;
        }
    }

    assert!(
        all_results_correct,
        "All operators should compute correct result"
    );
    assert_eq!(
        result_count, num_operators,
        "All operators should produce job results"
    );

    println!("All {} operators computed correct results", num_operators);
    println!("All {} operators would submit to chain", result_count);

    Ok(())
}
