//! Incredible Squaring Blueprint Library
//!
//! A simple example blueprint that demonstrates job processing with Tangle EVM (v2).
//! The blueprint provides two jobs:
//! - `square`: Single operator result required (simple case)
//! - `verified_square`: Multiple operator results required (aggregation/consensus case)

use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle_evm::extract::{TangleEvmArg, TangleEvmResult};
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

// ═══════════════════════════════════════════════════════════════════════════
// JOB INDICES
// ═══════════════════════════════════════════════════════════════════════════

/// The job index for the basic square operation
/// Requires only 1 operator result to complete
pub const XSQUARE_JOB_ID: u8 = 0;

/// The job index for the verified square operation
/// Requires 2 operator results to complete (demonstrates aggregation)
pub const VERIFIED_XSQUARE_JOB_ID: u8 = 1;

/// The job index for the consensus square operation
/// Requires 3 operator results to complete (demonstrates quorum)
pub const CONSENSUS_XSQUARE_JOB_ID: u8 = 2;

// ═══════════════════════════════════════════════════════════════════════════
// JOB FUNCTIONS
// ═══════════════════════════════════════════════════════════════════════════

/// Square a number (single operator)
///
/// This job function receives a u64 value via ABI-encoded input and returns its square.
/// The input and output are automatically ABI-encoded/decoded by the extractors.
///
/// **Aggregation:** Requires 1 operator result (default behavior)
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
        assert_ne!(XSQUARE_JOB_ID, VERIFIED_XSQUARE_JOB_ID);
        assert_ne!(XSQUARE_JOB_ID, CONSENSUS_XSQUARE_JOB_ID);
        assert_ne!(VERIFIED_XSQUARE_JOB_ID, CONSENSUS_XSQUARE_JOB_ID);
    }

    #[test]
    fn test_job_indices_are_sequential() {
        assert_eq!(XSQUARE_JOB_ID, 0);
        assert_eq!(VERIFIED_XSQUARE_JOB_ID, 1);
        assert_eq!(CONSENSUS_XSQUARE_JOB_ID, 2);
    }
}
