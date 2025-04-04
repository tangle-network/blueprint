// src/pricing.rs
use crate::benchmark::BenchmarkProfile;
use crate::error::Result;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceModel {
    /// Price in the smallest unit (e.g., Wei for Ethereum) per second of execution.
    /// This is a simplified model. A real system might have separate prices
    /// for CPU, memory, IO, duration, etc.
    pub price_per_second_wei: u128,
    /// Timestamp when this price was calculated/cached.
    pub generated_at: DateTime<Utc>,
    /// Optional: Include benchmark details used for pricing.
    pub benchmark_profile: Option<BenchmarkProfile>,
}

/// Calculates a price based on benchmark results and configuration.
///
/// This is a placeholder implementation. A real system would have a more
/// sophisticated pricing model based on resource usage (CPU, memory, IO)
/// and potentially market conditions or operator policies.
pub fn calculate_price(
    profile: BenchmarkProfile,
    scaling_factor: f64, // e.g., Wei per unit of resource (like avg CPU core)
) -> Result<PriceModel> {
    // Example: Price based on average CPU cores used.
    // Ensure scaling_factor and profile values make sense to avoid overflow/underflow.
    let price_per_second = profile.avg_cpu_cores as f64 * scaling_factor;

    // Add costs for memory, IO etc. if needed
    // price += profile.avg_memory_mb as f64 * memory_scaling_factor;

    // Round and convert to integer (Wei)
    let price_per_second_wei = price_per_second.max(0.0).round() as u128;

    Ok(PriceModel {
        price_per_second_wei,
        generated_at: Utc::now(),
        benchmark_profile: Some(profile),
    })
}
