//! Basic profiling example showing real-world usage
//!
//! Run with: cargo run --example basic_usage

use blueprint_profiling::{JobProfile, ProfileConfig, ProfileRunner};
use std::time::Duration;

/// Simulates a computational job (e.g., hash computation, cryptographic operation)
async fn computational_job(input: u64) -> Result<u64, Box<dyn std::error::Error + Send + Sync>> {
    // Simulate some computation
    let mut result = input;
    for _ in 0..1000 {
        result = result.wrapping_mul(2).wrapping_add(1);
    }
    Ok(result)
}

/// Simulates a job that would NOT be suitable for FaaS (too slow)
async fn slow_job() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tokio::time::sleep(Duration::from_secs(2)).await;
    Ok(())
}

/// Simulates a memory-intensive job
async fn memory_intensive_job() -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    // Allocate 10MB
    let data = vec![0u8; 10 * 1024 * 1024];
    tokio::time::sleep(Duration::from_millis(50)).await;
    Ok(data)
}

fn analyze_faas_compatibility(profile: &JobProfile) {
    println!("\n=== FaaS Compatibility Analysis ===");

    // AWS Lambda limits
    const AWS_LAMBDA_TIMEOUT_MS: u64 = 900_000; // 15 minutes
    const AWS_LAMBDA_MEMORY_MB: u32 = 10_240; // 10GB

    // GCP Cloud Functions limits
    const GCP_TIMEOUT_MS: u64 = 540_000; // 9 minutes
    const GCP_MEMORY_MB: u32 = 32_768; // 32GB

    let aws_compatible = profile.p95_duration_ms < AWS_LAMBDA_TIMEOUT_MS
        && profile.peak_memory_mb < AWS_LAMBDA_MEMORY_MB
        && !profile.stateful
        && !profile.persistent_connections;

    let gcp_compatible = profile.p95_duration_ms < GCP_TIMEOUT_MS
        && profile.peak_memory_mb < GCP_MEMORY_MB
        && !profile.stateful
        && !profile.persistent_connections;

    println!("AWS Lambda: {}", if aws_compatible { "✓ Compatible" } else { "✗ Not Compatible" });
    println!("  - Duration: {}ms / {}ms limit", profile.p95_duration_ms, AWS_LAMBDA_TIMEOUT_MS);
    println!("  - Memory: {}MB / {}MB limit", profile.peak_memory_mb, AWS_LAMBDA_MEMORY_MB);

    println!("\nGCP Functions: {}", if gcp_compatible { "✓ Compatible" } else { "✗ Not Compatible" });
    println!("  - Duration: {}ms / {}ms limit", profile.p95_duration_ms, GCP_TIMEOUT_MS);
    println!("  - Memory: {}MB / {}MB limit", profile.peak_memory_mb, GCP_MEMORY_MB);

    if profile.stateful {
        println!("\n⚠ Job is stateful - not recommended for FaaS");
    }
    if profile.persistent_connections {
        println!("⚠ Job maintains persistent connections - not recommended for FaaS");
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Blueprint Profiling Example ===\n");

    // Example 1: Fast computational job (FaaS-suitable)
    println!("1. Profiling fast computational job...");
    let config = ProfileConfig {
        sample_size: 10,
        warmup_runs: 2,
        max_execution_time: Duration::from_secs(30),
    };

    let profile = ProfileRunner::profile_job(
        || async { computational_job(12345).await.map(|_| ()) },
        config.clone(),
    )
    .await?;

    println!("Results:");
    println!("  Avg: {}ms, P95: {}ms, P99: {}ms",
        profile.avg_duration_ms,
        profile.p95_duration_ms,
        profile.p99_duration_ms
    );
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    analyze_faas_compatibility(&profile);

    // Example 2: Slow job (NOT FaaS-suitable)
    println!("\n\n2. Profiling slow job...");
    let slow_config = ProfileConfig {
        sample_size: 3,
        warmup_runs: 0,
        max_execution_time: Duration::from_secs(10),
    };

    let profile = ProfileRunner::profile_job(slow_job, slow_config).await?;

    println!("Results:");
    println!("  Avg: {}ms, P95: {}ms, P99: {}ms",
        profile.avg_duration_ms,
        profile.p95_duration_ms,
        profile.p99_duration_ms
    );
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    analyze_faas_compatibility(&profile);

    // Example 3: Memory-intensive job
    println!("\n\n3. Profiling memory-intensive job...");
    let profile = ProfileRunner::profile_job(
        || async { memory_intensive_job().await.map(|_| ()) },
        config,
    )
    .await?;

    println!("Results:");
    println!("  Avg: {}ms, P95: {}ms, P99: {}ms",
        profile.avg_duration_ms,
        profile.p95_duration_ms,
        profile.p99_duration_ms
    );
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    analyze_faas_compatibility(&profile);

    println!("\n=== Profiling Complete ===");
    Ok(())
}
