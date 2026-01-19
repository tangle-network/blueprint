//! Memory intensive profiling example showing peak memory tracking.

use blueprint_profiling::{ProfileConfig, ProfileRunner};
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::Arc;
use tokio::sync::Mutex;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Blueprint Memory Profiling Example\n");

    let config = ProfileConfig {
        sample_size: 15,
        warmup_runs: 2,
        ..Default::default()
    };

    // Use shared state to vary workload size on each invocation
    let rng = Arc::new(Mutex::new(StdRng::seed_from_u64(42)));

    let profile = ProfileRunner::profile_job(
        || {
            let rng = Arc::clone(&rng);
            async move {
                let mut rng = rng.lock().await;
                let size = rng.gen_range(10_000..200_000);

                let mut buffer = Vec::with_capacity(size);
                buffer.resize(size, 7u8);

                // Simulate CPU work
                let sum: u64 = buffer.iter().map(|&b| b as u64).sum();
                if sum == 0 {
                    Err("unexpected zero sum".into())
                } else {
                    Ok(())
                }
            }
        },
        config,
    )
    .await?;

    println!("Profiling Results (Sample size: {})", profile.sample_size);
    println!("  Average duration: {}ms", profile.avg_duration_ms);
    println!("  P95 duration: {}ms", profile.p95_duration_ms);
    println!("  Peak memory: {}MB", profile.peak_memory_mb);
    println!("  Stateful: {}", profile.stateful);
    println!(
        "  Persistent connections: {}",
        profile.persistent_connections
    );
    println!();

    println!("✅ Memory profiling completed successfully.");
    Ok(())
}
