// src/pricing.rs
use crate::benchmark::BenchmarkProfile;
use crate::error::Result;
use crate::types::ResourceUnit;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourcePricing {
    /// Resource kind (CPU, Memory, GPU, etc.)
    pub kind: ResourceUnit,
    /// Quantity of the resource
    pub count: u64,
    /// Price per unit in the smallest denomination of the chosen currency (e.g., wei, satoshi)
    pub price_per_unit_rate: u128,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceModel {
    /// Pricing for different resource types
    pub resources: Vec<ResourcePricing>,
    /// Total price rate per second in the smallest unit (e.g., Wei for Ethereum).
    pub price_per_second_rate: u128,
    /// Timestamp when this price was calculated/cached.
    pub generated_at: DateTime<Utc>,
    /// Optional: Include benchmark details used for pricing.
    pub benchmark_profile: Option<BenchmarkProfile>,
}

impl PriceModel {
    /// Calculate the total cost for a given TTL
    pub fn calculate_total_cost(&self, ttl_seconds: u64) -> u128 {
        self.price_per_second_rate
            .saturating_mul(ttl_seconds as u128)
    }
}

/// Calculates a price based on benchmark results and configuration.
pub fn calculate_price(profile: BenchmarkProfile, rate_multiplier: f64) -> Result<PriceModel> {
    let mut resources = Vec::new();
    let mut total_price_per_second = 0u128;

    // CPU pricing
    let avg_cpu_cores = profile
        .cpu_details
        .as_ref()
        .map(|cpu| cpu.avg_cores_used)
        .unwrap_or(0.0);

    if avg_cpu_cores > 0.0 {
        let cpu_price = (avg_cpu_cores as f64 * rate_multiplier).max(0.0).round() as u128;
        total_price_per_second = total_price_per_second.saturating_add(cpu_price);

        resources.push(ResourcePricing {
            kind: ResourceUnit::CPU,
            count: avg_cpu_cores.ceil() as u64,
            // Ensure price_per_unit is not zero if avg_cpu_cores is non-zero but rounds to 0
            // Use max(1) to avoid division by zero
            price_per_unit_rate: cpu_price / (avg_cpu_cores.ceil() as u128).max(1),
        });
    }

    // Memory pricing (example)
    // Add memory pricing if available in the profile

    // GPU pricing (example)
    // Add GPU pricing if available in the profile

    Ok(PriceModel {
        resources,
        price_per_second_rate: total_price_per_second,
        generated_at: chrono::Utc::now(),
        benchmark_profile: Some(profile),
    })
}

/// Load pricing from a pricing.toml file
pub fn load_pricing_from_toml(_path: &str) -> Result<HashMap<Option<u64>, Vec<ResourcePricing>>> {
    // This would parse a TOML file with pricing information
    // For now, return a simple example
    let mut pricing = HashMap::new();

    // Default pricing (no specific blueprint)
    pricing.insert(
        None,
        vec![
            ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 1,
                price_per_unit_rate: 1_000_000, // Example rate per CPU unit
            },
            ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 1024,
                price_per_unit_rate: 500_000, // Example rate per 1024 MB
            },
        ],
    );

    // Example specific blueprint pricing
    pricing.insert(
        Some(1),
        vec![
            ResourcePricing {
                kind: ResourceUnit::CPU,
                count: 2,
                price_per_unit_rate: 2_000_000,
            },
            ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: 2048,
                price_per_unit_rate: 1_000_000,
            },
        ],
    );

    Ok(pricing)
}
