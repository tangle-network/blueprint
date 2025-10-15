//! Simple example demonstrating blueprint profiling using the current API.

use blueprint_profiling::{ProfileConfig, ProfileRunner};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Blueprint Profiling Example\n");

    let config = ProfileConfig {
        sample_size: 20,
        warmup_runs: 2,
        ..Default::default()
    };

    // Profile a simple square computation
    let profile = ProfileRunner::profile_job(
        || async {
            // Simulate some computation work
            let mut total = 0u64;
            for value in 0..10_000 {
                total = total.wrapping_add(value * value);
            }
            if total == 0 {
                Err("unexpected zero total".into())
            } else {
                Ok(())
            }
        },
        config,
    )
    .await?;

    println!("Profiling Results (Sample size: {})", profile.sample_size);
    println!("  Average duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);
    println!("  P99 duration: {}ms", profile.p99_duration_ms);
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    println!("  Stateful: {}", profile.stateful);
    println!(
        "  Persistent connections: {}",
        profile.persistent_connections
    );
    println!();

    println!("✅ Profiling completed successfully.");
    Ok(())
}
