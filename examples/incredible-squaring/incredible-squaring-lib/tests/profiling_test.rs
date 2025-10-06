//! Test profiling for the square job
//!
//! This demonstrates how profiling works for determining FaaS compatibility.

use blueprint_profiling::{JobProfile, ProfileConfig, ProfileRunner};
use incredible_squaring_blueprint_lib::square;
use blueprint_sdk::tangle::extract::TangleArg;
use std::time::Duration;

#[tokio::test]
async fn test_profile_square_job() {
    // Configure profiling with reasonable defaults
    let config = ProfileConfig {
        sample_size: 10,
        warmup_runs: 2,
        max_execution_time: Duration::from_secs(10),
    };

    // Profile the square job
    let profile = ProfileRunner::profile_job(
        || async {
            // Generate a test input
            let x = 12345u64;

            // Call the actual job
            let result = square(TangleArg(x)).await;

            // Verify correctness
            assert_eq!(result.0, x * x);

            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        },
        config,
    )
    .await;

    // Verify profiling succeeded
    assert!(profile.is_ok(), "Profiling failed: {:?}", profile.err());

    let profile: JobProfile = profile.unwrap();

    // Display results
    println!("Profile results:");
    println!("  Avg duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);
    println!("  P99 duration: {}ms", profile.p99_duration_ms);
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    println!("  Sample size: {}", profile.sample_size);

    // Assert basic properties
    assert_eq!(profile.sample_size, 10);
    assert!(profile.p95_duration_ms >= profile.avg_duration_ms);
    assert!(profile.p99_duration_ms >= profile.p95_duration_ms);

    // Square job should be VERY fast (< 100ms typically)
    assert!(
        profile.avg_duration_ms < 100,
        "Square job took {}ms on average, expected < 100ms",
        profile.avg_duration_ms
    );

    // Check FaaS compatibility (based on AWS Lambda limits)
    let aws_lambda_timeout_ms = 900_000; // 15 minutes
    let aws_lambda_memory_mb = 10_240; // 10GB max

    let faas_compatible = profile.p95_duration_ms < aws_lambda_timeout_ms
        && profile.peak_memory_mb < aws_lambda_memory_mb;

    println!("\nFaaS Compatibility (AWS Lambda limits):");
    println!("  P95 duration: {}ms < {}ms: {}",
        profile.p95_duration_ms,
        aws_lambda_timeout_ms,
        profile.p95_duration_ms < aws_lambda_timeout_ms
    );
    println!("  Peak memory: {}MB < {}MB: {}",
        profile.peak_memory_mb,
        aws_lambda_memory_mb,
        profile.peak_memory_mb < aws_lambda_memory_mb
    );
    println!("  Overall compatible: {}", faas_compatible);

    // Square should be compatible
    assert!(
        faas_compatible,
        "Square job should be compatible with AWS Lambda"
    );
}

#[tokio::test]
async fn test_profiling_detects_slow_job() {
    let config = ProfileConfig {
        sample_size: 5,
        warmup_runs: 1,
        max_execution_time: Duration::from_secs(10),
    };

    // Simulate a slow job that takes ~100ms
    let profile = ProfileRunner::profile_job(
        || async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        },
        config,
    )
    .await;

    assert!(profile.is_ok());
    let profile = profile.unwrap();

    println!("\nSlow job profile:");
    println!("  Avg duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);

    // Should capture the 100ms delay
    assert!(
        profile.avg_duration_ms >= 100,
        "Expected avg >= 100ms, got {}ms",
        profile.avg_duration_ms
    );
}

#[tokio::test]
async fn test_profiling_detects_timeout() {
    let config = ProfileConfig {
        sample_size: 3,
        warmup_runs: 0,
        max_execution_time: Duration::from_millis(50),
    };

    // Create a job that will timeout
    let profile = ProfileRunner::profile_job(
        || async {
            tokio::time::sleep(Duration::from_secs(10)).await;
            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        },
        config,
    )
    .await;

    // Should fail due to timeout
    assert!(profile.is_err());
    println!("Timeout correctly detected: {:?}", profile.err());
}

#[tokio::test]
async fn test_profiling_varying_inputs() {
    // Test with different input values to ensure profiling handles variation
    let config = ProfileConfig {
        sample_size: 10,
        warmup_runs: 1,
        max_execution_time: Duration::from_secs(10),
    };

    let mut counter = 0u64;

    let profile = ProfileRunner::profile_job(
        || async {
            // Use varying inputs across runs
            let x = match counter % 3 {
                0 => 10u64,
                1 => 1_000_000u64,
                _ => u64::MAX / 2,
            };
            counter += 1;

            let result = square(TangleArg(x)).await;
            assert_eq!(result.0, x.wrapping_mul(x));

            Ok::<(), Box<dyn std::error::Error + Send + Sync>>(())
        },
        config,
    )
    .await;

    assert!(profile.is_ok());
    let profile = profile.unwrap();

    println!("\nVarying inputs profile:");
    println!("  Avg duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);

    // Should still be fast
    assert!(profile.avg_duration_ms < 100);
}
