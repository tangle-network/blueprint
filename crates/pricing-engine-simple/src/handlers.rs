// src/handlers.rs
use crate::benchmark::run_benchmark_suite;
use crate::cache::{BlueprintHash, PriceCache};
use crate::config::OperatorConfig;
use crate::error::Result;
use crate::pricing::calculate_price;
use log::{info, warn};
use std::sync::Arc;
use std::time::Duration;

/// Handles updates for a blueprint (registration or price target change).
/// Runs benchmarking, calculates pricing, and stores it in the cache.
pub async fn handle_blueprint_update(
    blueprint_hash: BlueprintHash, // Use the hash directly as the ID
    cache: Arc<PriceCache>,
    config: Arc<OperatorConfig>,
    // TODO: Need a way to determine *what* to benchmark for this blueprint_hash.
    // This might involve looking up details from the blockchain event data,
    // or from a local configuration mapping hashes to benchmarkable artifacts (e.g., docker images, commands).
    // For now, we use the generic benchmark command from config.
) -> Result<()> {
    info!("Handling update for blueprint: {}", blueprint_hash);

    // 1. Configure Benchmark
    let benchmark_duration = config.benchmark_duration;
    let benchmark_interval = config.benchmark_interval;

    // 2. Run Benchmark (Potentially long-running, ensure it doesn't block critical paths)
    let benchmark_result = run_benchmark_suite(
        blueprint_hash.clone(),
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
        warn!(
            "Benchmark command failed for blueprint {}. Skipping price update.",
            blueprint_hash
        );
        // Optionally store a marker indicating failure or remove old price?
        // cache.remove_price(&blueprint_hash)?;
        return Ok(()); // Or return an error depending on desired behavior
    }

    // 3. Calculate Price
    let price_model = calculate_price(benchmark_result, config.price_scaling_factor)?;
    info!(
        "Calculated price model for {}: {:?}",
        blueprint_hash, price_model
    );

    // 4. Store Price in Cache
    cache.store_price(&blueprint_hash, &price_model)?;

    info!(
        "Successfully updated price for blueprint: {}",
        blueprint_hash
    );
    Ok(())
}
