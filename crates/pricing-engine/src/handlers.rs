use crate::benchmark::run_benchmark_suite;
use crate::benchmark_cache::{BenchmarkCache, BlueprintId};
use crate::config::OperatorConfig;
use crate::error::Result;
use log::{info, warn};
use std::sync::Arc;
use std::time::Duration;

/// Handles updates for a blueprint (registration or price target change).
/// Runs benchmarking, calculates pricing, and stores it in the cache.
pub async fn handle_blueprint_update(
    blueprint_id: BlueprintId,
    cache: Arc<BenchmarkCache>,
    config: Arc<OperatorConfig>,
    // TODO: Need a way to determine *what* to benchmark for this blueprint_id.
    // This might involve looking up details from the blockchain event data,
    // or from a local configuration mapping IDs to benchmarkable artifacts (e.g., docker images, commands).
    // For now, we use the generic benchmark command from config.
) -> Result<()> {
    info!("Handling update for blueprint ID: {blueprint_id}");

    // Configure Benchmark
    let benchmark_duration = config.benchmark_duration;
    let benchmark_interval = config.benchmark_interval;

    // Run Benchmark (Potentially long-running, ensure it doesn't block critical paths)
    let benchmark_result = run_benchmark_suite(
        blueprint_id.to_string(),
        "native".to_string(),
        Duration::from_secs(benchmark_duration),
        Duration::from_secs(benchmark_interval),
        true,  // run_cpu_test
        true,  // run_memory_test
        true,  // run_io_test
        false, // run_network_test - disable by default (can be noisy)
        true,  // run_gpu_test
    )?;

    if !benchmark_result.success {
        warn!("Benchmark command failed for blueprint {blueprint_id}. Skipping profile update.",);
        return Ok(()); // Or return an error depending on desired behavior
    }

    // Store benchmark profile in cache
    cache.store_profile(blueprint_id, &benchmark_result)?;

    info!("Successfully updated benchmark profile for blueprint ID: {blueprint_id}",);
    Ok(())
}
