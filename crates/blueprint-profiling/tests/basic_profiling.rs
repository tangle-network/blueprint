//! Basic profiling test to verify cross-platform functionality

use blueprint_profiling::{
    profile_job, InputGenerator, FaasProvider, is_faas_compatible,
};

/// Simple input generator for testing
struct SimpleInputGenerator;

impl InputGenerator for SimpleInputGenerator {
    fn generate_inputs(&self, count: usize) -> Vec<Vec<u8>> {
        (0..count)
            .map(|i| {
                let value = (i as u64) * 10;
                value.to_le_bytes().to_vec()
            })
            .collect()
    }
}

#[tokio::test]
async fn test_basic_profiling() {
    let generator = SimpleInputGenerator;

    // Profile a simple computation
    let profile = profile_job(
        0,
        |input| {
            Box::pin(async move {
                let x = u64::from_le_bytes(input[..8].try_into().unwrap());
                let result = x * x;
                result.to_le_bytes().to_vec()
            })
        },
        &generator,
        5,
    )
    .await;

    // Verify profiling captured data
    println!("Profile results:");
    println!("  Min duration: {}ms", profile.min_duration_ms);
    println!("  Avg duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);
    println!("  Max duration: {}ms", profile.max_duration_ms);
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    println!("  Sample size: {}", profile.sample_size);

    // Assert basic properties
    assert_eq!(profile.sample_size, 5);
    assert!(profile.avg_duration_ms >= profile.min_duration_ms);
    assert!(profile.p95_duration_ms >= profile.avg_duration_ms);
    assert!(profile.max_duration_ms >= profile.p95_duration_ms);

    // Should be very fast (< 10ms typically)
    assert!(
        profile.avg_duration_ms < 100,
        "Simple job took {}ms on average, expected < 100ms",
        profile.avg_duration_ms
    );

    // Check FaaS compatibility
    let aws_compatible = is_faas_compatible(&profile, FaasProvider::AwsLambda);
    println!("\nFaaS Compatibility:");
    println!("  AWS Lambda: {}", aws_compatible);

    // Simple computation should be compatible
    assert!(
        aws_compatible,
        "Simple job should be compatible with AWS Lambda"
    );
}

#[tokio::test]
async fn test_slow_job_detection() {
    let generator = SimpleInputGenerator;

    // Profile a slow job
    let profile = profile_job(
        1,
        |input| {
            Box::pin(async move {
                // Simulate slow processing
                tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

                let x = u64::from_le_bytes(input[..8].try_into().unwrap());
                let result = x * x;
                result.to_le_bytes().to_vec()
            })
        },
        &generator,
        3,
    )
    .await;

    println!("\nSlow job profile:");
    println!("  Avg duration: {}ms", profile.avg_duration_ms);

    // Should capture the 50ms delay
    assert!(
        profile.avg_duration_ms >= 50,
        "Expected avg >= 50ms, got {}ms",
        profile.avg_duration_ms
    );
}
