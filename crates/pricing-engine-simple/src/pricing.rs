// src/pricing.rs
use crate::benchmark::BenchmarkProfile;
use crate::error::Result;
use crate::types::ResourceUnit;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use toml;

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

                    // Extract price per unit rate
                    let price_per_unit_rate = resource_table
                        .get("price_per_unit_rate")
                        .and_then(|p| {
                            p.as_integer()
                                .map(|int_val| int_val as u128)
                                .or_else(|| p.as_float().map(|float_val| float_val as u128))
                        })
                        .unwrap_or(0);

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

                        // Extract price per unit rate
                        let price_per_unit_rate = resource_table
                            .get("price_per_unit_rate")
                            .and_then(|p| {
                                if let Some(int_val) = p.as_integer() {
                                    Some(int_val as u128)
                                } else {
                                    p.as_float().map(|float_val| float_val as u128)
                                }
                            })
                            .unwrap_or(0);

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
