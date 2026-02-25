use crate::benchmark::BenchmarkProfile;
use crate::error::{PricingError, Result};
use crate::pricing_engine::AssetSecurityRequirements;
use crate::types::ResourceUnit;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
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

/// Subscription pricing configuration for a blueprint.
/// Used when `PricingModelHint::SUBSCRIPTION` — returns a flat rate per interval.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SubscriptionPricing {
    /// Cost per billing interval (same scale as resource pricing, pre-10^9 scaling)
    pub subscription_rate: Decimal,
    /// Billing interval in seconds (e.g. 86400 = daily, 604800 = weekly)
    pub subscription_interval: u64,
    /// Per-event charge (for EVENT_DRIVEN pricing)
    pub event_rate: Decimal,
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
    _security_requirements: Option<&AssetSecurityRequirements>,
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
    security_requirements: Option<&AssetSecurityRequirements>,
) -> Decimal {
    let adjusted_base_cost = calculate_base_resource_cost(count, price_per_unit_rate);
    let adjusted_time_cost = calculate_ttl_price_adjustment(ttl_blocks);
    let security_factor = calculate_security_rate_adjustment(security_requirements);

    adjusted_base_cost * adjusted_time_cost * security_factor
}

/// Calculate the price for a subscription-based blueprint.
/// Returns a flat rate per billing interval, ignoring resource usage and TTL.
pub fn calculate_subscription_price(
    config: &SubscriptionPricing,
    security_requirements: Option<&AssetSecurityRequirements>,
) -> PriceModel {
    let security_factor = calculate_security_rate_adjustment(security_requirements);
    PriceModel {
        resources: vec![],
        total_cost: config.subscription_rate * security_factor,
        benchmark_profile: None,
    }
}

/// Calculate the price for an event-driven blueprint.
/// Returns the per-event rate from the subscription config.
pub fn calculate_event_price(
    config: &SubscriptionPricing,
    security_requirements: Option<&AssetSecurityRequirements>,
) -> PriceModel {
    let security_factor = calculate_security_rate_adjustment(security_requirements);
    let rate = config.event_rate;
    PriceModel {
        resources: vec![],
        total_cost: rate * security_factor,
        benchmark_profile: None,
    }
}

/// Calculates a price based on benchmark results and configuration.
pub fn calculate_price(
    profile: BenchmarkProfile,
    pricing_config: &HashMap<Option<u64>, Vec<ResourcePricing>>,
    blueprint_id: Option<u64>,
    ttl_blocks: u64,
    security_requirements: Option<&AssetSecurityRequirements>,
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

    // Helper: price a resource if it has a configured rate, skip otherwise.
    // Unconfigured resources are omitted rather than priced at $0.
    let mut price_resource = |kind: ResourceUnit, count: u64| {
        if count == 0 {
            return;
        }
        if let Some(&price_per_unit) = resource_price_map.get(&kind) {
            let adjusted_price =
                calculate_resource_price(count, price_per_unit, ttl_blocks, security_requirements);
            resources.push(ResourcePricing {
                kind,
                count,
                price_per_unit_rate: price_per_unit,
            });
            total_cost += adjusted_price;
        }
    };

    if let Some(cpu) = &profile.cpu_details {
        price_resource(ResourceUnit::CPU, cpu.avg_cores_used.ceil() as u64);
    }
    if let Some(mem) = &profile.memory_details {
        price_resource(ResourceUnit::MemoryMB, mem.avg_memory_mb.ceil() as u64);
    }
    if let Some(storage) = &profile.storage_details {
        price_resource(
            ResourceUnit::StorageMB,
            (storage.storage_available_gb * 1024.0).ceil() as u64,
        );
    }
    if let Some(net) = &profile.network_details {
        price_resource(
            ResourceUnit::NetworkEgressMB,
            net.network_tx_mb.ceil() as u64,
        );
        price_resource(
            ResourceUnit::NetworkIngressMB,
            net.network_rx_mb.ceil() as u64,
        );
    }
    if let Some(gpu) = &profile.gpu_details {
        if gpu.gpu_available {
            price_resource(ResourceUnit::GPU, 1);
        }
    }

    // Return the price model
    Ok(PriceModel {
        resources,
        total_cost,
        benchmark_profile: Some(profile),
    })
}

/// Load per-job pricing from a TOML file.
///
/// Format: each section key is a service ID, each key within is a job index,
/// values are prices in wei (as strings to support large U256 values).
///
/// ```toml
/// [1]
/// 0 = "1000000000000000"
/// 6 = "20000000000000000"
/// ```
pub fn load_job_pricing_from_toml(
    content: &str,
) -> Result<HashMap<(u64, u32), alloy_primitives::U256>> {
    let parsed: toml::Value = toml::from_str(content)?;
    let mut config = HashMap::new();

    let table = parsed.as_table().ok_or_else(|| {
        crate::error::PricingError::Config("job pricing TOML must be a table".to_string())
    })?;

    for (service_key, jobs) in table {
        let service_id: u64 = service_key.parse().map_err(|_| {
            crate::error::PricingError::Config(format!(
                "invalid service ID in job pricing: {service_key}"
            ))
        })?;

        let jobs_table = jobs.as_table().ok_or_else(|| {
            crate::error::PricingError::Config(format!(
                "service {service_id}: expected a table of job_index = \"price_wei\""
            ))
        })?;

        for (job_key, price_val) in jobs_table {
            let job_index: u32 = job_key.parse().map_err(|_| {
                crate::error::PricingError::Config(format!(
                    "service {service_id}: invalid job index: {job_key}"
                ))
            })?;

            let price_str = price_val.as_str().ok_or_else(|| {
                crate::error::PricingError::Config(format!(
                    "service {service_id} job {job_index}: price must be a string (wei value)"
                ))
            })?;

            let price = alloy_primitives::U256::from_str_radix(price_str, 10).map_err(|_| {
                crate::error::PricingError::Config(format!(
                    "service {service_id} job {job_index}: invalid wei value: {price_str}"
                ))
            })?;

            config.insert((service_id, job_index), price);
        }
    }

    Ok(config)
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

                    // Extract count (safe i64 → u64)
                    let count = resource_table
                        .get("count")
                        .and_then(|c| c.as_integer())
                        .unwrap_or(1);
                    let count = u64::try_from(count).map_err(|_| {
                        PricingError::Config(format!(
                            "Negative count {count} for resource {kind:?}"
                        ))
                    })?;

                    // Extract price per unit rate as Decimal (required)
                    let price_per_unit_rate = resource_table
                        .get("price_per_unit_rate")
                        .and_then(|p| {
                            p.as_float()
                                .and_then(|f| Decimal::try_from(f).ok())
                                .or_else(|| p.as_integer().map(Decimal::from))
                        })
                        .ok_or_else(|| {
                            PricingError::Config(format!(
                                "Missing or invalid price_per_unit_rate for resource {kind:?}"
                            ))
                        })?;

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

                        // Extract count (safe i64 → u64)
                        let count = resource_table
                            .get("count")
                            .and_then(|c| c.as_integer())
                            .unwrap_or(1);
                        let count = u64::try_from(count).map_err(|_| {
                            PricingError::Config(format!(
                                "Negative count {count} for resource {kind:?} in blueprint [{blueprint_id}]"
                            ))
                        })?;

                        // Extract price per unit rate as Decimal (required)
                        let price_per_unit_rate = resource_table
                            .get("price_per_unit_rate")
                            .and_then(|p| {
                                p.as_float()
                                    .and_then(|f| Decimal::try_from(f).ok())
                                    .or_else(|| p.as_integer().map(Decimal::from))
                            })
                            .ok_or_else(|| {
                                PricingError::Config(format!(
                                    "Missing or invalid price_per_unit_rate for resource {kind:?} in blueprint [{blueprint_id}]"
                                ))
                            })?;

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

/// Load subscription pricing from a pricing.toml file.
///
/// Sections with `pricing_model = "subscription"` are parsed for subscription rates.
/// Supports `[default]` and `[<blueprint_id>]` sections.
///
/// ```toml
/// [default]
/// pricing_model = "subscription"
/// subscription_rate = 0.001
/// subscription_interval = 86400
/// event_rate = 0.0001
///
/// [5]
/// pricing_model = "subscription"
/// subscription_rate = 0.005
/// subscription_interval = 604800
/// ```
pub fn load_subscription_pricing_from_toml(
    content: &str,
) -> Result<HashMap<Option<u64>, SubscriptionPricing>> {
    let parsed: toml::Value = toml::from_str(content)?;
    let mut config = HashMap::new();

    let table = match parsed.as_table() {
        Some(t) => t,
        None => return Ok(config),
    };

    for (key, value) in table {
        let section = match value.as_table() {
            Some(t) => t,
            None => continue,
        };

        // Only parse sections with pricing_model = "subscription" or "event_driven"
        let model = section
            .get("pricing_model")
            .and_then(|v| v.as_str())
            .unwrap_or("");
        if model != "subscription" && model != "event_driven" {
            continue;
        }

        let parse_decimal_field = |field: &str| -> Result<Option<Decimal>> {
            let Some(v) = section.get(field) else {
                return Ok(None);
            };
            let dec = v
                .as_float()
                .and_then(|f| Decimal::try_from(f).ok())
                .or_else(|| v.as_integer().map(Decimal::from))
                .ok_or_else(|| {
                    PricingError::Config(format!("Missing or invalid {field} in section [{key}]"))
                })?;
            if dec.is_sign_negative() {
                return Err(PricingError::Config(format!(
                    "{field} cannot be negative in section [{key}]"
                )));
            }
            Ok(Some(dec))
        };

        let parse_u64_field = |field: &str| -> Result<Option<u64>> {
            let Some(v) = section.get(field) else {
                return Ok(None);
            };
            let parsed = v
                .as_integer()
                .and_then(|i| u64::try_from(i).ok())
                .ok_or_else(|| {
                    PricingError::Config(format!("Missing or invalid {field} in section [{key}]"))
                })?;
            Ok(Some(parsed))
        };

        let (subscription_rate, subscription_interval, event_rate) = match model {
            "subscription" => {
                let subscription_rate =
                    parse_decimal_field("subscription_rate")?.ok_or_else(|| {
                        PricingError::Config(format!(
                            "Missing subscription_rate in section [{key}] for pricing_model=subscription"
                        ))
                    })?;
                if subscription_rate <= Decimal::ZERO {
                    return Err(PricingError::Config(format!(
                        "subscription_rate must be > 0 in section [{key}]"
                    )));
                }

                let subscription_interval = parse_u64_field("subscription_interval")?.ok_or_else(|| {
                    PricingError::Config(format!(
                        "Missing subscription_interval in section [{key}] for pricing_model=subscription"
                    ))
                })?;
                if subscription_interval == 0 {
                    return Err(PricingError::Config(format!(
                        "subscription_interval must be > 0 in section [{key}]"
                    )));
                }

                let event_rate = parse_decimal_field("event_rate")?.unwrap_or(Decimal::ZERO);
                (subscription_rate, subscription_interval, event_rate)
            }
            "event_driven" => {
                let event_rate = parse_decimal_field("event_rate")?.ok_or_else(|| {
                    PricingError::Config(format!(
                        "Missing event_rate in section [{key}] for pricing_model=event_driven"
                    ))
                })?;
                if event_rate <= Decimal::ZERO {
                    return Err(PricingError::Config(format!(
                        "event_rate must be > 0 in section [{key}]"
                    )));
                }

                let subscription_rate =
                    parse_decimal_field("subscription_rate")?.unwrap_or(Decimal::ZERO);
                let subscription_interval = parse_u64_field("subscription_interval")?.unwrap_or(0);
                (subscription_rate, subscription_interval, event_rate)
            }
            _ => unreachable!("guarded above"),
        };

        let pricing = SubscriptionPricing {
            subscription_rate,
            subscription_interval,
            event_rate,
        };

        let bp_key = if key == "default" {
            None
        } else {
            match key.parse::<u64>() {
                Ok(id) => Some(id),
                Err(_) => continue,
            }
        };

        config.insert(bp_key, pricing);
    }

    Ok(config)
}
