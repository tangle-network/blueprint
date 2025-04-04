// src/handlers.rs
use crate::benchmark::{BenchmarkRunConfig, run_benchmark};
use crate::cache::{BlueprintHash, PriceCache};
use crate::config::OperatorConfig;
use crate::error::{PricingError, Result};
use crate::pricing::calculate_price;
use log::{error, info, warn};
use std::sync::Arc;

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

    // 1. Determine Benchmark Configuration
    // FIXME: This needs actual logic based on the blueprint_hash.
    // Using generic config command for now. A real system needs a lookup.
    let benchmark_config = BenchmarkRunConfig {
        command: config.benchmark_command.clone(),
        args: config.benchmark_args.clone(), // Adjust args based on blueprint if needed
        job_id: blueprint_hash.clone(),      // Use blueprint hash as job ID
        mode: "native".to_string(),          // Assuming native for now
        max_duration: config.benchmark_duration,
        sample_interval: config.benchmark_interval,
    };

    // 2. Run Benchmark (Potentially long-running, ensure it doesn't block critical paths)
    // Consider running this in a blocking task if it's CPU-intensive itself
    let benchmark_profile = tokio::task::spawn_blocking(move || run_benchmark(benchmark_config))
        .await
        .map_err(|e| PricingError::Benchmark(format!("Benchmark task failed: {}", e)))??; // Double ?? for JoinError and inner Result

    if !benchmark_profile.success {
        warn!(
            "Benchmark command failed for blueprint {}. Skipping price update.",
            blueprint_hash
        );
        // Optionally store a marker indicating failure or remove old price?
        // cache.remove_price(&blueprint_hash)?;
        return Ok(()); // Or return an error depending on desired behavior
    }

    // 3. Calculate Price
    let price_model = calculate_price(benchmark_profile, config.price_scaling_factor)?;
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
