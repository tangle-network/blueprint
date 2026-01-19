//! Incredible Squaring Blueprint Library
//!
//! A simple example blueprint that demonstrates job processing with Tangle EVM (v2).
//!
//! # Job Matrix
//!
//! This blueprint demonstrates all four combinations of execution location × aggregation:
//!
//! | Execution | Aggregation | Job ID | Function |
//! |-----------|-------------|--------|----------|
//! | Local     | Single (1)  | 0      | `square` |
//! | Local     | Multi (2)   | 1      | `verified_square` |
//! | Local     | Multi (3)   | 2      | `consensus_square` |
//! | FaaS      | Single (1)  | 3      | `square_faas` |
//! | FaaS      | Multi (2)   | 4      | `verified_square_faas` |
//!
//! **Execution** determines WHERE the job runs:
//! - **Local**: On the operator's machine directly
//! - **FaaS**: On serverless infrastructure (Lambda, Cloud Functions, etc.)
//!
//! **Aggregation** determines HOW MANY operator results are needed:
//! - **Single**: 1 result completes the job
//! - **Multi**: N results required (for redundancy/consensus)

use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle_evm::TangleEvmLayer;
use blueprint_sdk::tangle_evm::extract::{TangleEvmArg, TangleEvmResult};
use blueprint_sdk::{Job, Router};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

// ═══════════════════════════════════════════════════════════════════════════
// JOB INDICES
// ═══════════════════════════════════════════════════════════════════════════
//
// 2x2 matrix: Execution Location × Aggregation
//
//                     │  Local Execution    │  FaaS Execution
// ────────────────────┼─────────────────────┼──────────────────────
//  Single Operator    │  XSQUARE (0)        │  XSQUARE_FAAS (3)
//  (1 result)         │                     │
// ────────────────────┼─────────────────────┼──────────────────────
//  Multi Operator     │  VERIFIED (1)       │  VERIFIED_FAAS (4)
//  (2+ results)       │  CONSENSUS (2)      │
// ────────────────────┴─────────────────────┴──────────────────────

/// Job 0: Local + Single operator (1 result required)
pub const XSQUARE_JOB_ID: u8 = 0;

/// Job 1: Local + Aggregated (2 results required)
pub const VERIFIED_XSQUARE_JOB_ID: u8 = 1;

/// Job 2: Local + Aggregated (3 results required - quorum)
pub const CONSENSUS_XSQUARE_JOB_ID: u8 = 2;

/// Job 3: FaaS + Single operator (1 result required)
pub const XSQUARE_FAAS_JOB_ID: u8 = 3;

/// Job 4: FaaS + Aggregated (2 results required)
pub const VERIFIED_XSQUARE_FAAS_JOB_ID: u8 = 4;

// ═══════════════════════════════════════════════════════════════════════════
// JOB FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Square a number (single operator, local execution)
///
/// This job function receives a u64 value via ABI-encoded input and returns its square.
/// The input and output are automatically ABI-encoded/decoded by the extractors.
///
/// **Aggregation:** Requires 1 operator result (default behavior)
/// **Execution:** Local - runs on blueprint operator's machine
///
/// # Arguments
///
/// * `x` - The number to square, extracted from ABI-encoded job inputs
///
/// # Returns
///
/// Returns the square of the input value as ABI-encoded output
pub async fn square(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
    let result = x * x;
    TangleEvmResult(result)
}

/// Square a number via FaaS (serverless execution)
///
/// This job is **IDENTICAL** in logic to `square` but is designed to run on
/// serverless infrastructure (AWS Lambda, GCP Cloud Functions, etc.) instead
/// of directly on the operator's machine.
///
/// This demonstrates the key property: **same job logic, different execution location**.
/// The result flows through the same consumer pipeline to reach onchain.
///
/// **Aggregation:** Requires 1 operator result
/// **Execution:** FaaS - runs on serverless infrastructure
///
/// # Arguments
///
/// * `x` - The number to square
///
/// # Returns
///
/// Returns the square of the input value
pub async fn square_faas(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
    let result = x * x;
    TangleEvmResult(result)
}

/// Square a number with verification (requires 2 operators)
///
/// This job is identical to `square` but requires 2 operator results before
/// the job is considered complete. This demonstrates the aggregation feature
/// where multiple operators must submit matching results.
///
/// **Aggregation:** Requires 2 operator results
///
/// # Use Case
///
/// Use this when you want basic redundancy - if both operators submit the same
/// result, you have higher confidence in correctness.
///
/// # Arguments
///
/// * `x` - The number to square
///
/// # Returns
///
/// Returns the square of the input value
pub async fn verified_square(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
    let result = x * x;
    TangleEvmResult(result)
}

/// Square a number with consensus (requires 3 operators)
///
/// This job requires 3 operator results, demonstrating a quorum-based approach.
/// The protocol can compare results and use the majority answer.
///
/// **Aggregation:** Requires 3 operator results
///
/// # Use Case
///
/// Use this when you need Byzantine fault tolerance. With 3 operators,
/// even if 1 is malicious or faulty, the 2 honest operators will produce
/// the correct result.
///
/// # Arguments
///
/// * `x` - The number to square
///
/// # Returns
///
/// Returns the square of the input value
pub async fn consensus_square(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
    let result = x * x;
    TangleEvmResult(result)
}

/// Square a number via FaaS with verification (requires 2 operators)
///
/// This combines **FaaS execution** with **aggregation**: the job runs on
/// serverless infrastructure, but requires 2 operator results before completion.
///
/// **Aggregation:** Requires 2 operator results
/// **Execution:** FaaS - runs on serverless infrastructure
///
/// # Use Case
///
/// Use this when you need both:
/// - Scalability/cost benefits of serverless execution
/// - Redundancy/verification from multiple operators
///
/// # Arguments
///
/// * `x` - The number to square
///
/// # Returns
///
/// Returns the square of the input value
pub async fn verified_square_faas(TangleEvmArg(x): TangleEvmArg<u64>) -> TangleEvmResult<u64> {
    let result = x * x;
    TangleEvmResult(result)
}

// ═══════════════════════════════════════════════════════════════════════════
// ROUTER
// ═══════════════════════════════════════════════════════════════════════════

/// Router wiring the squaring jobs onto the Tangle EVM layer.
///
/// Reused by the binary and the Anvil harness test so we have a single source
/// of truth for job registration and aggregation semantics.
#[must_use]
pub fn router() -> Router {
    Router::new()
        // Local execution jobs
        .route(XSQUARE_JOB_ID, square.layer(TangleEvmLayer))
        .route(
            VERIFIED_XSQUARE_JOB_ID,
            verified_square.layer(TangleEvmLayer),
        )
        .route(
            CONSENSUS_XSQUARE_JOB_ID,
            consensus_square.layer(TangleEvmLayer),
        )
        // FaaS execution jobs
        .route(XSQUARE_FAAS_JOB_ID, square_faas.layer(TangleEvmLayer))
        .route(
            VERIFIED_XSQUARE_FAAS_JOB_ID,
            verified_square_faas.layer(TangleEvmLayer),
        )
}

// ═══════════════════════════════════════════════════════════════════════════
// BACKGROUND SERVICE
// ═══════════════════════════════════════════════════════════════════════════

/// Example background service
///
/// This demonstrates how to attach background services to the blueprint runner.
/// Background services run alongside job processing and can be used for
/// tasks like periodic health checks, data synchronization, etc.
#[derive(Clone)]
pub struct FooBackgroundService;

impl BackgroundService for FooBackgroundService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            let _ = tx.send(Ok(()));
        });
        Ok(rx)
    }
}

// ═══════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_job_indices_are_unique() {
        let all_ids = [
            XSQUARE_JOB_ID,
            VERIFIED_XSQUARE_JOB_ID,
            CONSENSUS_XSQUARE_JOB_ID,
            XSQUARE_FAAS_JOB_ID,
            VERIFIED_XSQUARE_FAAS_JOB_ID,
        ];

        // Check all pairs are unique
        for i in 0..all_ids.len() {
            for j in (i + 1)..all_ids.len() {
                assert_ne!(
                    all_ids[i], all_ids[j],
                    "Job IDs at positions {} and {} are not unique",
                    i, j
                );
            }
        }
    }

    #[test]
    fn test_job_indices_are_sequential() {
        assert_eq!(XSQUARE_JOB_ID, 0);
        assert_eq!(VERIFIED_XSQUARE_JOB_ID, 1);
        assert_eq!(CONSENSUS_XSQUARE_JOB_ID, 2);
        assert_eq!(XSQUARE_FAAS_JOB_ID, 3);
        assert_eq!(VERIFIED_XSQUARE_FAAS_JOB_ID, 4);
    }

    #[test]
    fn test_job_matrix_coverage() {
        // Local + Single
        assert_eq!(XSQUARE_JOB_ID, 0);
        // Local + Multi
        assert_eq!(VERIFIED_XSQUARE_JOB_ID, 1);
        assert_eq!(CONSENSUS_XSQUARE_JOB_ID, 2);
        // FaaS + Single
        assert_eq!(XSQUARE_FAAS_JOB_ID, 3);
        // FaaS + Multi
        assert_eq!(VERIFIED_XSQUARE_FAAS_JOB_ID, 4);
    }
}
