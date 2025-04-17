// src/pricing.rs
use tangle_subxt::tangle_testnet_runtime::api::assets::events::created::AssetId;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityRequirement;
use crate::benchmark::BenchmarkProfile;
use crate::error::Result;
use crate::types::ResourceUnit;
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

    // Set a default TTL of 1 second for per-second pricing
    let ttl_seconds = 1;
    
    // CPU pricing
    if let Some(cpu_details) = &profile.cpu_details {
        // Round up to nearest integer for CPU cores
        let cpu_count = cpu_details.avg_cores_used.ceil() as u64;
        if cpu_count > 0 {
            // Base price per CPU core
            let price_per_unit = rate_multiplier;
            
            // Calculate CPU price using the standard formula
            let cpu_price = calculate_resource_price(
                cpu_count,
                price_per_unit,
                ttl_seconds,
                None,
            );
            
            // Add to total price (divide by TTL to get per-second rate)
            let cpu_price_per_second = cpu_price / (ttl_seconds as f64 * BLOCK_TIME);
            total_price_per_second += cpu_price_per_second;
            
            // Add CPU resource to the resources list
            resources.push(ResourcePricing {
                kind: ResourceUnit::CPU,
                count: cpu_count,
                price_per_unit_rate: price_per_unit,
            });
        }
    }

    // Memory pricing
    if let Some(memory_details) = &profile.memory_details {
        // Round up to nearest integer for memory MB
        let memory_mb = memory_details.avg_memory_mb.ceil() as u64;
        if memory_mb > 0 {
            // Memory is typically cheaper than CPU
            let price_per_unit = rate_multiplier * 0.05;
            
            // Calculate memory price using the standard formula
            let memory_price = calculate_resource_price(
                memory_mb,
                price_per_unit,
                ttl_seconds,
                None,
            );
            
            // Add to total price (divide by TTL to get per-second rate)
            let memory_price_per_second = memory_price / (ttl_seconds as f64 * BLOCK_TIME);
            total_price_per_second += memory_price_per_second;
            
            // Add memory resource to the resources list
            resources.push(ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: memory_mb,
                price_per_unit_rate: price_per_unit,
            });
        }
    }

    // Storage pricing
    if let Some(storage_details) = &profile.storage_details {
        // Convert GB to MB and round up
        let storage_mb = (storage_details.storage_available_gb * 1024.0).ceil() as u64;
        if storage_mb > 0 {
            // Storage is typically cheaper than memory
            let price_per_unit = rate_multiplier * 0.02;
            
            // Calculate storage price using the standard formula
            let storage_price = calculate_resource_price(
                storage_mb,
                price_per_unit,
                ttl_seconds,
                None,
            );
            
            // Add to total price (divide by TTL to get per-second rate)
            let storage_price_per_second = storage_price / (ttl_seconds as f64 * BLOCK_TIME);
            total_price_per_second += storage_price_per_second;
            
            // Add storage resource to the resources list
            resources.push(ResourcePricing {
                kind: ResourceUnit::StorageMB,
                count: storage_mb,
                price_per_unit_rate: price_per_unit,
            });
        }
    }

    // Network pricing
    if let Some(network_details) = &profile.network_details {
        // Network egress (outbound traffic)
        let egress_mb = network_details.network_tx_mb.ceil() as u64;
        if egress_mb > 0 {
            // Egress is typically more expensive than ingress
            let price_per_unit = rate_multiplier * 0.05;
            
            // Calculate egress price using the standard formula
            let egress_price = calculate_resource_price(
                egress_mb,
                price_per_unit,
                ttl_seconds,
                None,
            );
            
            // Add to total price (divide by TTL to get per-second rate)
            let egress_price_per_second = egress_price / (ttl_seconds as f64 * BLOCK_TIME);
            total_price_per_second += egress_price_per_second;
            
            // Add egress resource to the resources list
            resources.push(ResourcePricing {
                kind: ResourceUnit::NetworkEgressMB,
                count: egress_mb,
                price_per_unit_rate: price_per_unit,
            });
        }
        
        // Network ingress (inbound traffic)
        let ingress_mb = network_details.network_rx_mb.ceil() as u64;
        if ingress_mb > 0 {
            // Ingress is typically cheaper than egress
            let price_per_unit = rate_multiplier * 0.02;
            
            // Calculate ingress price using the standard formula
            let ingress_price = calculate_resource_price(
                ingress_mb,
                price_per_unit,
                ttl_seconds,
                None,
            );
            
            // Add to total price (divide by TTL to get per-second rate)
            let ingress_price_per_second = ingress_price / (ttl_seconds as f64 * BLOCK_TIME);
            total_price_per_second += ingress_price_per_second;
            
            // Add ingress resource to the resources list
            resources.push(ResourcePricing {
                kind: ResourceUnit::NetworkIngressMB,
                count: ingress_mb,
                price_per_unit_rate: price_per_unit,
            });
        }
    }

    // GPU pricing
    if let Some(gpu_details) = &profile.gpu_details {
        if gpu_details.gpu_available {
            // GPUs are typically much more expensive than CPUs
            let price_per_unit = rate_multiplier * 5.0;
            
            // Calculate GPU price using the standard formula
            let gpu_price = calculate_resource_price(
                1, // Assuming 1 GPU
                price_per_unit,
                ttl_seconds,
                None,
            );
            
            // Add to total price (divide by TTL to get per-second rate)
            let gpu_price_per_second = gpu_price / (ttl_seconds as f64 * BLOCK_TIME);
            total_price_per_second += gpu_price_per_second;
            
            // Add GPU resource to the resources list
            resources.push(ResourcePricing {
                kind: ResourceUnit::GPU,
                count: 1,
                price_per_unit_rate: price_per_unit,
            });
        }
    }

    // Create and return the price model
    Ok(PriceModel {
        resources,
        price_per_second_rate: total_price_per_second,
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
