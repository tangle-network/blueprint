use crate::benchmark::BenchmarkProfile;
use crate::error::{PricingError, Result};
use crate::pricing_engine::AssetSecurityRequirements;
use crate::types::ResourceUnit;
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use toml;

/// The average block time in seconds
pub fn block_time() -> Decimal {
    Decimal::new(6, 0)
}

/// TTL-based pricing curve: multipliers at evenly-spaced duration points.
///
/// Operators define a vector of multipliers. Element 0 corresponds to TTL=0,
/// the last element corresponds to `max_duration_secs`. Values between points
/// are linearly interpolated. TTLs beyond `max_duration_secs` clamp to the
/// last multiplier.
///
/// ```toml
/// [ttl_curve]
/// max_duration_secs = 31536000  # 1 year
/// multipliers = [1.2, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.45, 0.4, 0.38, 0.36, 0.35]
/// # index:        0    1    2    3    4    5    6    7     8    9     10    11
/// # ~months:      0    1    2    3    4    5    6    7     8    9     10    11
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TtlPricingCurve {
    /// Multiplier values at evenly-spaced TTL points. Must have at least 2 elements.
    multipliers: Vec<f64>,
    /// The TTL duration (in seconds) that the last element corresponds to.
    max_duration_secs: u64,
}

impl Default for TtlPricingCurve {
    fn default() -> Self {
        Self {
            // Linear: single multiplier of 1.0 at both endpoints = pure linear pricing
            multipliers: vec![1.0, 1.0],
            max_duration_secs: 31_536_000, // 1 year
        }
    }
}

impl TtlPricingCurve {
    /// Construct a validated TTL pricing curve.
    pub fn new(multipliers: Vec<f64>, max_duration_secs: u64) -> std::result::Result<Self, String> {
        let curve = Self {
            multipliers,
            max_duration_secs,
        };
        curve.validate()?;
        Ok(curve)
    }

    /// Read-only access to the multiplier values.
    pub fn multipliers(&self) -> &[f64] {
        &self.multipliers
    }

    /// Read-only access to the max duration in seconds.
    pub fn max_duration_secs(&self) -> u64 {
        self.max_duration_secs
    }

    /// Evaluate the curve at a given TTL duration in seconds.
    /// Returns the interpolated multiplier.
    pub fn evaluate(&self, ttl_secs: u64) -> Decimal {
        if self.multipliers.is_empty() {
            return Decimal::ONE;
        }
        if self.multipliers.len() == 1 || self.max_duration_secs == 0 {
            return Decimal::try_from(self.multipliers[0]).unwrap_or(Decimal::ONE);
        }

        let t = (ttl_secs as f64 / self.max_duration_secs as f64).clamp(0.0, 1.0);
        let n = self.multipliers.len() - 1;
        let idx = t * n as f64;
        let lo = (idx.floor() as usize).min(n);
        let hi = (lo + 1).min(n);
        let frac = idx - lo as f64;

        let value = self.multipliers[lo] * (1.0 - frac) + self.multipliers[hi] * frac;
        // Clamp to non-negative
        Decimal::try_from(value.max(0.0)).unwrap_or(Decimal::ZERO)
    }

    /// Validate the curve configuration.
    pub fn validate(&self) -> std::result::Result<(), String> {
        if self.multipliers.len() < 2 {
            return Err("ttl_curve.multipliers must have at least 2 elements".into());
        }
        if self.multipliers.len() > 100 {
            return Err("ttl_curve.multipliers must have at most 100 elements".into());
        }
        if self.max_duration_secs == 0 {
            return Err("ttl_curve.max_duration_secs must be > 0".into());
        }
        for (i, &m) in self.multipliers.iter().enumerate() {
            if m.is_nan() || m.is_infinite() {
                return Err(format!("ttl_curve.multipliers[{i}] is NaN or infinite"));
            }
            if m < 0.0 {
                return Err(format!("ttl_curve.multipliers[{i}] is negative: {m}"));
            }
        }
        Ok(())
    }
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

/// Calculate the time-based price adjustment factor based on TTL in blocks.
///
/// When a `TtlPricingCurve` is provided, the TTL is converted to seconds and
/// the curve multiplier is applied: `ttl_seconds * curve_multiplier`.
/// When no curve is provided, falls back to pure linear: `ttl_blocks * BLOCK_TIME`.
fn calculate_ttl_price_adjustment(
    time_blocks: u64,
    ttl_curve: Option<&TtlPricingCurve>,
) -> Decimal {
    let ttl_seconds = Decimal::from(time_blocks) * block_time();
    match ttl_curve {
        Some(curve) => {
            let block_time_secs = block_time().to_u64().unwrap_or(6);
            let ttl_secs_u64 =
                (time_blocks as u128 * block_time_secs as u128).min(u64::MAX as u128) as u64;
            ttl_seconds * curve.evaluate(ttl_secs_u64)
        }
        None => ttl_seconds,
    }
}

/// Function that applies security requirement adjustments to the cost
fn calculate_security_rate_adjustment(
    _security_requirements: Option<&AssetSecurityRequirements>,
) -> Decimal {
    // TODO: Implement security requirement adjustments
    Decimal::ONE
}

/// Calculate the price for a specific resource based on count, rate, TTL, and security requirements.
///
/// Formula: `base_cost(count * rate) * ttl_adjustment(blocks, curve) * security_factor`
pub fn calculate_resource_price(
    count: u64,
    price_per_unit_rate: Decimal,
    ttl_blocks: u64,
    security_requirements: Option<&AssetSecurityRequirements>,
) -> Decimal {
    calculate_resource_price_with_curve(
        count,
        price_per_unit_rate,
        ttl_blocks,
        security_requirements,
        None,
    )
}

/// Calculate resource price with an optional TTL pricing curve.
pub fn calculate_resource_price_with_curve(
    count: u64,
    price_per_unit_rate: Decimal,
    ttl_blocks: u64,
    security_requirements: Option<&AssetSecurityRequirements>,
    ttl_curve: Option<&TtlPricingCurve>,
) -> Decimal {
    let adjusted_base_cost = calculate_base_resource_cost(count, price_per_unit_rate);
    let adjusted_time_cost = calculate_ttl_price_adjustment(ttl_blocks, ttl_curve);
    let security_factor = calculate_security_rate_adjustment(security_requirements);

    adjusted_base_cost * adjusted_time_cost * security_factor
}

/// TEE pricing configuration.
///
/// Loaded from the `[tee]` section of pricing.toml:
/// ```toml
/// [tee]
/// available = true
/// multiplier = 1.5
/// provider = "aws_nitro"
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TeePricing {
    /// Whether this operator can provide TEE execution.
    pub available: bool,
    /// Price multiplier when TEE is requested (e.g. 1.5 = 50% premium).
    pub multiplier: Decimal,
    /// TEE provider name (e.g. "aws_nitro", "intel_tdx").
    pub provider: String,
}

impl Default for TeePricing {
    fn default() -> Self {
        Self {
            available: false,
            multiplier: Decimal::ONE,
            provider: String::new(),
        }
    }
}

/// Apply TEE pricing adjustment to a base cost.
///
/// If `require_tee` is true and TEE is available, multiplies by the configured
/// TEE premium. If `require_tee` is true but TEE is unavailable, returns an error.
pub fn apply_tee_pricing(
    base_cost: Decimal,
    require_tee: bool,
    tee_config: &TeePricing,
) -> Result<Decimal> {
    if !require_tee {
        return Ok(base_cost);
    }
    if !tee_config.available {
        return Err(PricingError::TeeNotAvailable);
    }
    Ok(base_cost * tee_config.multiplier)
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
    calculate_price_with_curve(
        profile,
        pricing_config,
        blueprint_id,
        ttl_blocks,
        security_requirements,
        None,
    )
}

/// Calculates a price with an optional TTL pricing curve for non-linear duration pricing.
pub fn calculate_price_with_curve(
    profile: BenchmarkProfile,
    pricing_config: &HashMap<Option<u64>, Vec<ResourcePricing>>,
    blueprint_id: Option<u64>,
    ttl_blocks: u64,
    security_requirements: Option<&AssetSecurityRequirements>,
    ttl_curve: Option<&TtlPricingCurve>,
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
            let adjusted_price = calculate_resource_price_with_curve(
                count,
                price_per_unit,
                ttl_blocks,
                security_requirements,
                ttl_curve,
            );
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

/// Load TEE pricing from a pricing.toml file.
///
/// Reads the `[tee]` section. Returns `TeePricing::default()` (unavailable) if absent.
///
/// ```toml
/// [tee]
/// available = true
/// multiplier = 1.5
/// provider = "aws_nitro"
/// ```
pub fn load_tee_pricing_from_toml(content: &str) -> Result<TeePricing> {
    let parsed: toml::Value = toml::from_str(content)?;

    let Some(tee_section) = parsed.get("tee").and_then(|v| v.as_table()) else {
        return Ok(TeePricing::default());
    };

    let available = tee_section
        .get("available")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    let multiplier = tee_section
        .get("multiplier")
        .and_then(|v| {
            v.as_float()
                .and_then(|f| Decimal::try_from(f).ok())
                .or_else(|| v.as_integer().map(Decimal::from))
        })
        .unwrap_or(Decimal::ONE);

    if multiplier <= Decimal::ZERO {
        return Err(PricingError::Config(
            "TEE multiplier must be positive".to_string(),
        ));
    }

    let provider = tee_section
        .get("provider")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    Ok(TeePricing {
        available,
        multiplier,
        provider,
    })
}

/// Load TTL pricing curve from a pricing.toml file.
///
/// Reads the `[ttl_curve]` section. Returns `None` if absent (pure linear pricing).
///
/// ```toml
/// [ttl_curve]
/// max_duration_secs = 31536000
/// multipliers = [1.2, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.45, 0.4, 0.38, 0.36, 0.35]
/// ```
pub fn load_ttl_curve_from_toml(content: &str) -> Result<Option<TtlPricingCurve>> {
    let parsed: toml::Value = toml::from_str(content)?;

    let Some(section) = parsed.get("ttl_curve").and_then(|v| v.as_table()) else {
        return Ok(None);
    };

    let max_duration_secs = section
        .get("max_duration_secs")
        .and_then(|v| v.as_integer())
        .map(|v| v as u64)
        .unwrap_or(31_536_000);

    let multipliers = section
        .get("multipliers")
        .and_then(|v| v.as_array())
        .ok_or_else(|| {
            PricingError::Config("ttl_curve requires a 'multipliers' array".to_string())
        })?
        .iter()
        .enumerate()
        .map(|(i, v)| {
            v.as_float()
                .or_else(|| v.as_integer().map(|n| n as f64))
                .ok_or_else(|| {
                    PricingError::Config(format!("ttl_curve.multipliers[{i}] must be a number"))
                })
        })
        .collect::<Result<Vec<f64>>>()?;

    let curve = TtlPricingCurve::new(multipliers, max_duration_secs)
        .map_err(|e| PricingError::Config(format!("ttl_curve: {e}")))?;

    Ok(Some(curve))
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── TtlPricingCurve evaluation ──────────────────────────────────────

    #[test]
    fn linear_curve_matches_no_curve() {
        let curve = TtlPricingCurve::default(); // [1.0, 1.0]
        // At any point, multiplier should be 1.0
        assert_eq!(curve.evaluate(0), Decimal::ONE);
        assert_eq!(curve.evaluate(15_768_000), Decimal::ONE); // 6 months
        assert_eq!(curve.evaluate(31_536_000), Decimal::ONE); // 1 year
    }

    #[test]
    fn discount_curve_interpolates() {
        let curve = TtlPricingCurve::new(vec![1.0, 0.5], 100).unwrap();
        // At t=0: 1.0, at t=50: 0.75, at t=100: 0.5
        assert_eq!(curve.evaluate(0), Decimal::ONE);
        assert_eq!(curve.evaluate(100), Decimal::try_from(0.5).unwrap());
        // Midpoint
        let mid = curve.evaluate(50);
        assert_eq!(mid, Decimal::try_from(0.75).unwrap());
    }

    #[test]
    fn multi_point_curve() {
        // [1.2, 1.0, 0.8, 0.5, 0.35] over 4 months
        let curve = TtlPricingCurve::new(
            vec![1.2, 1.0, 0.8, 0.5, 0.35],
            4 * 30 * 86400, // ~120 days
        )
        .unwrap();
        // At t=0: 1.2 (spot premium)
        assert_eq!(curve.evaluate(0), Decimal::try_from(1.2).unwrap());
        // At max: 0.35
        assert_eq!(
            curve.evaluate(4 * 30 * 86400),
            Decimal::try_from(0.35).unwrap()
        );
        // Quarter way (between index 0 and 1): 1.2 → 1.0
        let quarter = curve.evaluate(30 * 86400);
        assert_eq!(quarter, Decimal::ONE);
    }

    #[test]
    fn clamp_beyond_max_duration() {
        let curve = TtlPricingCurve::new(vec![1.0, 0.5], 100).unwrap();
        // Beyond max should clamp to last value
        assert_eq!(curve.evaluate(200), Decimal::try_from(0.5).unwrap());
        assert_eq!(curve.evaluate(u64::MAX), Decimal::try_from(0.5).unwrap());
    }

    #[test]
    fn single_multiplier_rejected_by_constructor() {
        assert!(TtlPricingCurve::new(vec![0.8], 1000).is_err());
    }

    #[test]
    fn empty_multipliers_rejected_by_constructor() {
        assert!(TtlPricingCurve::new(vec![], 1000).is_err());
    }

    // ── Validation ──────────────────────────────────────────────────────

    #[test]
    fn new_rejects_too_few() {
        assert!(TtlPricingCurve::new(vec![1.0], 100).is_err());
    }

    #[test]
    fn new_rejects_too_many() {
        assert!(TtlPricingCurve::new(vec![1.0; 101], 100).is_err());
    }

    #[test]
    fn new_rejects_negative() {
        assert!(TtlPricingCurve::new(vec![1.0, -0.5], 100).is_err());
    }

    #[test]
    fn new_rejects_nan() {
        assert!(TtlPricingCurve::new(vec![1.0, f64::NAN], 100).is_err());
    }

    #[test]
    fn new_accepts_100_elements() {
        let multipliers: Vec<f64> = (0..100).map(|i| 1.0 - i as f64 * 0.005).collect();
        assert!(TtlPricingCurve::new(multipliers, 31_536_000).is_ok());
    }

    // ── TOML loading ────────────────────────────────────────────────────

    #[test]
    fn load_ttl_curve_from_toml_absent() {
        let toml = "[tee]\navailable = false\n";
        assert!(load_ttl_curve_from_toml(toml).unwrap().is_none());
    }

    #[test]
    fn load_ttl_curve_from_toml_present() {
        let toml = r#"
[ttl_curve]
max_duration_secs = 31536000
multipliers = [1.2, 1.0, 0.9, 0.8, 0.7, 0.6, 0.5, 0.45, 0.4, 0.38, 0.36, 0.35]
"#;
        let curve = load_ttl_curve_from_toml(toml).unwrap().unwrap();
        assert_eq!(curve.multipliers().len(), 12);
        assert_eq!(curve.max_duration_secs(), 31_536_000);
        assert_eq!(curve.evaluate(0), Decimal::try_from(1.2).unwrap());
    }

    #[test]
    fn load_ttl_curve_rejects_invalid() {
        let toml = r#"
[ttl_curve]
max_duration_secs = 100
multipliers = [1.0]
"#;
        assert!(load_ttl_curve_from_toml(toml).is_err());
    }

    // ── Integration: curve affects resource pricing ─────────────────────

    #[test]
    fn resource_price_with_discount_curve() {
        let curve = TtlPricingCurve::new(
            vec![1.0, 0.5], // 50% discount at max duration
            600,            // 600 seconds = 100 blocks at 6s
        )
        .unwrap();

        let linear_price = calculate_resource_price(1, Decimal::ONE, 100, None);
        let curved_price =
            calculate_resource_price_with_curve(1, Decimal::ONE, 100, None, Some(&curve));

        // At max duration (100 blocks = 600 seconds), multiplier = 0.5
        assert_eq!(curved_price, linear_price * Decimal::try_from(0.5).unwrap());
    }

    #[test]
    fn resource_price_no_curve_is_linear() {
        let price_a = calculate_resource_price(1, Decimal::ONE, 100, None);
        let price_b = calculate_resource_price_with_curve(1, Decimal::ONE, 100, None, None);
        assert_eq!(price_a, price_b);
    }
}
