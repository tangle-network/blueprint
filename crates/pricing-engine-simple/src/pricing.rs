// src/pricing.rs
use tangle_subxt::tangle_testnet_runtime::api::assets::events::created::AssetId;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityRequirement;
use crate::benchmark::BenchmarkProfile;
use crate::error::Result;
use crate::types::ResourceUnit;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

/// The average block time in seconds
pub const BLOCK_TIME: f64 = 6.0;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourcePricing {
    /// Resource kind (CPU, Memory, GPU, etc.)
    pub kind: ResourceUnit,
    /// Quantity of the resource
    pub count: u64,
    /// Price per unit in USD with decimal precision (e.g., 0.00005 USD per MB)
    pub price_per_unit_rate: f64,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceModel {
    /// Pricing for different resource types
    pub resources: Vec<ResourcePricing>,
    /// Total price rate per second in USD with decimal precision
    pub price_per_second_rate: f64,
    /// Timestamp when this price was calculated/cached.
    pub generated_at: DateTime<Utc>,
    /// Optional: Include benchmark details used for pricing.
    pub benchmark_profile: Option<BenchmarkProfile>,
}

impl PriceModel {
    /// Calculate the total cost for a given TTL
    pub fn calculate_total_cost(&self, ttl_seconds: u64) -> f64 {
        self.price_per_second_rate * ttl_seconds as f64
    }
}

/// Function that applies pricing adjustments based on the base cost (price * count)
fn calculate_base_resource_cost(resource_count: u64, resource_price_rate: f64) -> f64 {
    // We multiply the resource count by the price rate
    resource_count as f64 * resource_price_rate
}

/// Function that applies time-based adjustments to the cost
fn calculate_ttl_price_adjustment(time_blocks: u64) -> f64 {
    // We multiply the input TTL by BLOCK_TIME
    time_blocks as f64 * BLOCK_TIME
}

/// Function that applies security requirement adjustments to the cost
fn calculate_security_rate_adjustment(_security_requirements: &Option<AssetSecurityRequirement<AssetId>>) -> f64 {
    // TODO: Implement security requirement adjustments
    1.0
}

/// Calculate the price for a specific resource based on count, rate, TTL, and security requirements
/// Following the formula: calculate_base_resource_cost(cost * count) * calculate_ttl_price_adjustment(ttl * BLOCK_TIME) * calculate_security_rate_adjustment(security requirements * adjustment rate)
pub fn calculate_resource_price(
    count: u64,
    price_per_unit_rate: f64,
    ttl_seconds: u64,
    security_requirements: Option<AssetSecurityRequirement<AssetId>>,
) -> f64 {   
    let adjusted_base_cost = calculate_base_resource_cost(count, price_per_unit_rate);

    let adjusted_time_cost = calculate_ttl_price_adjustment(ttl_seconds);
    
    let security_factor = calculate_security_rate_adjustment(&security_requirements);
    
    adjusted_base_cost * adjusted_time_cost * security_factor
}

/// Calculates a price based on benchmark results and configuration.
pub fn calculate_price(profile: BenchmarkProfile, rate_multiplier: f64) -> Result<PriceModel> {
    let mut resources = Vec::new();
    let mut total_price_per_second = 0.0;

    // CPU pricing
    let avg_cpu_cores = profile
        .cpu_details
        .as_ref()
        .map(|cpu| cpu.avg_cores_used)
        .unwrap_or(0.0);

    if avg_cpu_cores > 0.0 {
        // Convert f32 to f64 for calculations
        let avg_cpu_cores_f64 = avg_cpu_cores as f64;
        // Ensure non-negative price by using max(0.0, value)
        let cpu_price = (avg_cpu_cores_f64 * rate_multiplier).max(0.0);
        total_price_per_second += cpu_price;

        resources.push(ResourcePricing {
            kind: ResourceUnit::CPU,
            count: avg_cpu_cores.ceil() as u64,
            // Price per unit is the total price divided by the number of units
            // Ensure non-negative price per unit
            price_per_unit_rate: if avg_cpu_cores_f64 > 0.0 { (cpu_price / avg_cpu_cores_f64).max(0.0) } else { 0.0 },
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
pub fn load_pricing_from_toml(path: &str) -> Result<HashMap<Option<u64>, Vec<ResourcePricing>>> {
    use std::str::FromStr;

    // Parse the TOML file
    let toml_content = fs::read_to_string(path)?;
    let parsed_toml: toml::Value = toml::from_str(&toml_content)?;

    let mut pricing = HashMap::new();

    // Process default pricing if present
    if let Some(default_table) = parsed_toml.get("default") {
        if let Some(resources) = default_table.get("resources").and_then(|r| r.as_array()) {
            let mut default_resources = Vec::new();

            for resource in resources {
                if let Some(resource_table) = resource.as_table() {
                    // Extract resource kind
                    let kind = if let Some(kind_str) =
                        resource_table.get("kind").and_then(|k| k.as_str())
                    {
                        ResourceUnit::from_str(kind_str)?
                    } else {
                        continue; // Skip if kind is missing
                    };

                    // Extract count
                    let count = resource_table
                        .get("count")
                        .and_then(|c| c.as_integer())
                        .unwrap_or(1) as u64;

                    // Extract price per unit rate as float
                    let price_per_unit_rate = resource_table
                        .get("price_per_unit_rate")
                        .and_then(|p| {
                            p.as_float()
                                .or_else(|| p.as_integer().map(|int_val| int_val as f64))
                        })
                        .unwrap_or(0.0);

                    default_resources.push(ResourcePricing {
                        kind,
                        count,
                        price_per_unit_rate,
                    });
                }
            }

            pricing.insert(None, default_resources);
        }
    }

    // Process blueprint-specific pricing
    for (key, value) in parsed_toml.as_table().unwrap_or(&toml::value::Table::new()) {
        // Skip the default section as it's already processed
        if key == "default" {
            continue;
        }

        // Try to parse the key as a blueprint ID
        if let Ok(blueprint_id) = key.parse::<u64>() {
            if let Some(resources) = value.get("resources").and_then(|r| r.as_array()) {
                let mut blueprint_resources = Vec::new();

                for resource in resources {
                    if let Some(resource_table) = resource.as_table() {
                        // Extract resource kind
                        let kind = if let Some(kind_str) =
                            resource_table.get("kind").and_then(|k| k.as_str())
                        {
                            ResourceUnit::from_str(kind_str)?
                        } else {
                            continue; // Skip if kind is missing
                        };

                        // Extract count
                        let count = resource_table
                            .get("count")
                            .and_then(|c| c.as_integer())
                            .unwrap_or(1) as u64;

                        // Extract price per unit rate as float
                        let price_per_unit_rate = resource_table
                            .get("price_per_unit_rate")
                            .and_then(|p| {
                                p.as_float()
                                    .or_else(|| p.as_integer().map(|int_val| int_val as f64))
                            })
                            .unwrap_or(0.0);

                        blueprint_resources.push(ResourcePricing {
                            kind,
                            count,
                            price_per_unit_rate,
                        });
                    }
                }

                pricing.insert(Some(blueprint_id), blueprint_resources);
            }
        }
    }

    Ok(pricing)
}
