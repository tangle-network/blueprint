use crate::benchmark::BenchmarkProfile;
use crate::error::Result;
use crate::types::ResourceUnit;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tangle_subxt::tangle_testnet_runtime::api::assets::events::created::AssetId;
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityRequirement;
use toml;

/// The average block time in seconds
pub fn block_time() -> Decimal {
    Decimal::new(6, 0)
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResourcePricing {
    /// Resource kind (CPU, Memory, GPU, etc.)
    pub kind: ResourceUnit,
    /// Quantity of the resource
    pub count: u64,
    /// Price per unit in USD with decimal precision (e.g., 0.00005 USD per MB)
    pub price_per_unit_rate: Decimal,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceModel {
    /// Pricing for different resource types
    pub resources: Vec<ResourcePricing>,
    /// Total cost in USD with decimal precision
    pub total_cost: Decimal,
    /// Optional: Include benchmark details used for pricing.
    pub benchmark_profile: Option<BenchmarkProfile>,
}

/// Function that applies pricing adjustments based on the base cost (price * count)
fn calculate_base_resource_cost(resource_count: u64, resource_price_rate: Decimal) -> Decimal {
    // We multiply the resource count by the price rate
    Decimal::from(resource_count) * resource_price_rate
}

/// Calculate the time-based price adjustment factor based on TTL in blocks
/// Each block represents BLOCK_TIME seconds
fn calculate_ttl_price_adjustment(time_blocks: u64) -> Decimal {
    // We multiply the input TTL by BLOCK_TIME
    Decimal::from(time_blocks) * block_time()
}

/// Function that applies security requirement adjustments to the cost
fn calculate_security_rate_adjustment(
    _security_requirements: &Option<AssetSecurityRequirement<AssetId>>,
) -> Decimal {
    // TODO: Implement security requirement adjustments
    Decimal::ONE
}

/// Calculate the price for a specific resource based on count, rate, TTL, and security requirements
/// Following the formula: calculate_base_resource_cost(cost * count) * calculate_ttl_price_adjustment(time_blocks) * calculate_security_rate_adjustment(security requirements)
pub fn calculate_resource_price(
    count: u64,
    price_per_unit_rate: Decimal,
    ttl_blocks: u64,
    security_requirements: Option<AssetSecurityRequirement<AssetId>>,
) -> Decimal {
    let adjusted_base_cost = calculate_base_resource_cost(count, price_per_unit_rate);
    let adjusted_time_cost = calculate_ttl_price_adjustment(ttl_blocks);
    let security_factor = calculate_security_rate_adjustment(&security_requirements);

    adjusted_base_cost * adjusted_time_cost * security_factor
}

/// Calculates a price based on benchmark results and configuration.
pub fn calculate_price(
    profile: BenchmarkProfile,
    pricing_config: &HashMap<Option<u64>, Vec<ResourcePricing>>,
    blueprint_id: Option<u64>,
    ttl_blocks: u64,
    security_requirements: Option<AssetSecurityRequirement<AssetId>>,
) -> Result<PriceModel> {
    let mut resources = Vec::new();
    let mut total_cost = Decimal::ZERO;

    // Get the appropriate pricing configuration based on blueprint ID or default
    let resource_pricing = pricing_config
        .get(&blueprint_id)
        .or_else(|| pricing_config.get(&None))
        .ok_or_else(|| {
            crate::error::PricingError::Config(
                "No pricing configuration found for the specified blueprint ID or default"
                    .to_string(),
            )
        })?;

    // Create a map for quick lookup of resource pricing
    let mut resource_price_map: HashMap<ResourceUnit, Decimal> = HashMap::new();
    for resource in resource_pricing {
        resource_price_map.insert(resource.kind.clone(), resource.price_per_unit_rate);
    }

    // CPU pricing
    if let Some(cpu_details) = &profile.cpu_details {
        // Round up to nearest integer for CPU cores
        let cpu_count = cpu_details.avg_cores_used.ceil() as u64;
        if cpu_count > 0 {
            // Get the price per CPU core from the configuration or use a default
            let price_per_unit = resource_price_map
                .get(&ResourceUnit::CPU)
                .cloned()
                .unwrap_or(Decimal::ZERO);

            let adjusted_price = calculate_resource_price(
                cpu_count,
                price_per_unit,
                ttl_blocks,
                security_requirements.clone(),
            );

            resources.push(ResourcePricing {
                kind: ResourceUnit::CPU,
                count: cpu_count,
                price_per_unit_rate: price_per_unit,
            });

            total_cost += adjusted_price;
        }
    }

    // Memory pricing
    if let Some(memory_details) = &profile.memory_details {
        // Round up to nearest MB
        let memory_mb = memory_details.avg_memory_mb.ceil() as u64;
        if memory_mb > 0 {
            // Get the price per MB from the configuration or use a default
            let price_per_unit = resource_price_map
                .get(&ResourceUnit::MemoryMB)
                .cloned()
                .unwrap_or(Decimal::ZERO);

            let adjusted_price = calculate_resource_price(
                memory_mb,
                price_per_unit,
                ttl_blocks,
                security_requirements.clone(),
            );

            resources.push(ResourcePricing {
                kind: ResourceUnit::MemoryMB,
                count: memory_mb,
                price_per_unit_rate: price_per_unit,
            });

            total_cost += adjusted_price;
        }
    }

    // Storage pricing
    if let Some(storage_details) = &profile.storage_details {
        // Convert GB to MB and round up
        let storage_mb = (storage_details.storage_available_gb * 1024.0).ceil() as u64;
        if storage_mb > 0 {
            // Get the price per MB from the configuration or use a default
            let price_per_unit = resource_price_map
                .get(&ResourceUnit::StorageMB)
                .cloned()
                .unwrap_or(Decimal::ZERO);

            let adjusted_price = calculate_resource_price(
                storage_mb,
                price_per_unit,
                ttl_blocks,
                security_requirements.clone(),
            );

            resources.push(ResourcePricing {
                kind: ResourceUnit::StorageMB,
                count: storage_mb,
                price_per_unit_rate: price_per_unit,
            });

            total_cost += adjusted_price;
        }
    }

    // Network egress pricing
    if let Some(network_details) = &profile.network_details {
        // Network egress (outbound traffic)
        let egress_mb = network_details.network_tx_mb.ceil() as u64;
        if egress_mb > 0 {
            // Get the price per MB from the configuration or use a default
            let price_per_unit = resource_price_map
                .get(&ResourceUnit::NetworkEgressMB)
                .cloned()
                .unwrap_or(Decimal::ZERO);

            let adjusted_price = calculate_resource_price(
                egress_mb,
                price_per_unit,
                ttl_blocks,
                security_requirements.clone(),
            );

            resources.push(ResourcePricing {
                kind: ResourceUnit::NetworkEgressMB,
                count: egress_mb,
                price_per_unit_rate: price_per_unit,
            });

            total_cost += adjusted_price;
        }
    }

    // Network ingress pricing
    if let Some(network_details) = &profile.network_details {
        // Network ingress (inbound traffic)
        let ingress_mb = network_details.network_rx_mb.ceil() as u64;
        if ingress_mb > 0 {
            // Get the price per MB from the configuration or use a default
            let price_per_unit = resource_price_map
                .get(&ResourceUnit::NetworkIngressMB)
                .cloned()
                .unwrap_or(Decimal::ZERO);

            let adjusted_price = calculate_resource_price(
                ingress_mb,
                price_per_unit,
                ttl_blocks,
                security_requirements.clone(),
            );

            resources.push(ResourcePricing {
                kind: ResourceUnit::NetworkIngressMB,
                count: ingress_mb,
                price_per_unit_rate: price_per_unit,
            });

            total_cost += adjusted_price;
        }
    }

    // GPU pricing
    if let Some(gpu_details) = &profile.gpu_details {
        if gpu_details.gpu_available {
            // Get the price per GPU unit from the configuration or use a default
            let price_per_unit = resource_price_map
                .get(&ResourceUnit::GPU)
                .cloned()
                .unwrap_or(Decimal::ZERO);

            let adjusted_price = calculate_resource_price(
                1,
                price_per_unit,
                ttl_blocks,
                security_requirements.clone(),
            );

            resources.push(ResourcePricing {
                kind: ResourceUnit::GPU,
                count: 1, // TODO: Support multiple GPUs
                price_per_unit_rate: price_per_unit,
            });
            total_cost += adjusted_price;
        }
    }

    // Return the price model
    Ok(PriceModel {
        resources,
        total_cost,
        benchmark_profile: Some(profile),
    })
}

/// Load pricing from a pricing.toml file
pub fn load_pricing_from_toml(content: &str) -> Result<HashMap<Option<u64>, Vec<ResourcePricing>>> {
    use std::str::FromStr;

    let parsed_toml: toml::Value = toml::from_str(content)?;

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

                    // Extract price per unit rate as Decimal
                    let price_per_unit_rate = resource_table
                        .get("price_per_unit_rate")
                        .and_then(|p| {
                            p.as_float()
                                .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                                .or_else(|| p.as_integer().map(Decimal::from))
                        })
                        .unwrap_or(Decimal::ZERO);

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

                        // Extract price per unit rate as Decimal
                        let price_per_unit_rate = resource_table
                            .get("price_per_unit_rate")
                            .and_then(|p| {
                                p.as_float()
                                    .map(|f| Decimal::try_from(f).unwrap_or(Decimal::ZERO))
                                    .or_else(|| p.as_integer().map(Decimal::from))
                            })
                            .unwrap_or(Decimal::ZERO);

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
