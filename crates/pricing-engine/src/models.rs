//! Pricing models for cloud services
//!
//! This module defines the pricing models for Tangle cloud services.

use core::fmt;
use parity_scale_codec::{Decode, Encode};
use scale_info::TypeInfo;

#[cfg(feature = "std")]
use serde::{Deserialize, Serialize};

use crate::error::PricingError;
use crate::types::{Price, ResourceRequirement, ResourceUnit, TimePeriod};

/// Pricing tier for tiered pricing models
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PricingTier {
    /// Start value for this tier (inclusive)
    pub start: u128,
    /// End value for this tier (exclusive)
    pub end: Option<u128>,
    /// Price per unit in this tier
    pub price_per_unit: Price,
}

/// Resource-based pricing configuration for a specific resource
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct ResourcePricing {
    /// Resource unit being priced
    pub unit: ResourceUnit,
    /// Price per unit of resource
    pub price_per_unit: Price,
    /// Optional minimum quantity for pricing
    pub min_quantity: Option<u128>,
    /// Optional maximum quantity supported
    pub max_quantity: Option<u128>,
    /// Optional time period for recurring pricing
    pub time_period: Option<TimePeriod>,
    /// Optional tiers for tiered pricing
    pub tiers: Option<Vec<PricingTier>>,
}

impl ResourcePricing {
    /// Create a new simple resource pricing with price per unit
    pub fn new(unit: ResourceUnit, price_per_unit: Price) -> Self {
        Self {
            unit,
            price_per_unit,
            min_quantity: None,
            max_quantity: None,
            time_period: None,
            tiers: None,
        }
    }

    /// Set the time period for recurring pricing
    pub fn with_time_period(mut self, period: TimePeriod) -> Self {
        self.time_period = Some(period);
        self
    }

    /// Set minimum quantity
    pub fn with_min_quantity(mut self, min: u128) -> Self {
        self.min_quantity = Some(min);
        self
    }

    /// Set maximum quantity
    pub fn with_max_quantity(mut self, max: u128) -> Self {
        self.max_quantity = Some(max);
        self
    }

    /// Set pricing tiers
    pub fn with_tiers(mut self, tiers: Vec<PricingTier>) -> Self {
        self.tiers = Some(tiers);
        self
    }

    /// Calculate price for a given quantity of this resource
    pub fn calculate_price(&self, quantity: u128) -> Result<Price, PricingError> {
        // Check quantity against min/max
        if let Some(min) = self.min_quantity {
            if quantity < min {
                return Err(PricingError::QuantityBelowMinimum(quantity));
            }
        }

        if let Some(max) = self.max_quantity {
            if quantity > max {
                return Err(PricingError::QuantityAboveMaximum(quantity));
            }
        }

        // Calculate price based on tiered or flat pricing
        if let Some(tiers) = &self.tiers {
            // Find the applicable tier(s)
            let mut applicable_tiers = Vec::new();
            for tier in tiers {
                if quantity >= tier.start && (tier.end.is_none() || quantity < tier.end.unwrap()) {
                    applicable_tiers.push(tier);
                }
            }

            if applicable_tiers.is_empty() {
                return Err(PricingError::CalculationError(
                    "No applicable pricing tier found".to_string(),
                ));
            }

            // In tiered pricing, we might need to calculate prices for different tiers
            // For simplicity, we just use the first applicable tier in this implementation
            let tier = &applicable_tiers[0];

            // Price = quantity * price_per_unit
            let price = Price {
                value: quantity.saturating_mul(tier.price_per_unit.value) / 1_000_000,
                token: tier.price_per_unit.token.clone(),
            };

            Ok(price)
        } else {
            // For flat pricing, we simply multiply quantity by price per unit
            let price = Price {
                value: quantity.saturating_mul(self.price_per_unit.value) / 1_000_000,
                token: self.price_per_unit.token.clone(),
            };

            Ok(price)
        }
    }
}

/// Pricing model types supported by the system
#[derive(Debug, Clone, PartialEq, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum PricingModelType {
    /// Fixed price regardless of usage
    Fixed,
    /// Price based on resource usage
    Usage,
    /// Tiered pricing with different rates at different usage levels
    Tiered,
}

#[cfg(feature = "std")]
impl fmt::Display for PricingModelType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PricingModelType::Fixed => write!(f, "Fixed"),
            PricingModelType::Usage => write!(f, "Usage-based"),
            PricingModelType::Tiered => write!(f, "Tiered"),
        }
    }
}

/// Main pricing model that combines different pricing strategies
#[derive(Debug, Clone, Encode, Decode, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub struct PricingModel {
    /// Type of pricing model
    pub model_type: PricingModelType,
    /// Name of the pricing model
    pub name: String,
    /// Description of the pricing model
    pub description: Option<String>,
    /// Blueprint ID this model applies to (replaces category)
    pub blueprint_id: String,
    /// Base price (for fixed pricing or minimum charge)
    pub base_price: Option<Price>,
    /// Resource-specific pricing
    pub resource_pricing: Vec<ResourcePricing>,
    /// Time period for recurring charges
    pub billing_period: Option<TimePeriod>,
}

impl PricingModel {
    /// Create a new pricing model
    pub fn new(
        model_type: PricingModelType,
        name: impl Into<String>,
        blueprint_id: impl Into<String>,
    ) -> Self {
        Self {
            model_type,
            name: name.into(),
            description: None,
            blueprint_id: blueprint_id.into(),
            base_price: None,
            resource_pricing: Vec::new(),
            billing_period: None,
        }
    }

    /// Add description
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the base price
    pub fn with_base_price(mut self, price: Price) -> Self {
        self.base_price = Some(price);
        self
    }

    /// Add resource pricing
    pub fn with_resource_pricing(mut self, pricing: ResourcePricing) -> Self {
        self.resource_pricing.push(pricing);
        self
    }

    /// Set billing period
    pub fn with_billing_period(mut self, period: TimePeriod) -> Self {
        self.billing_period = Some(period);
        self
    }
}

/// Trait for pricing strategies
pub trait PricingStrategy {
    /// Calculate the price for a service based on resource requirements
    fn calculate_price(
        &self,
        requirements: &[ResourceRequirement],
        model: &PricingModel,
    ) -> Result<Price, PricingError>;
}

/// Create a fixed price model
pub fn create_fixed_price_model(
    name: impl Into<String>,
    blueprint_id: impl Into<String>,
    price: Price,
    period: TimePeriod,
) -> PricingModel {
    PricingModel::new(PricingModelType::Fixed, name, blueprint_id)
        .with_base_price(price)
        .with_billing_period(period)
}

/// Create a usage-based pricing model
pub fn create_usage_model(
    name: impl Into<String>,
    blueprint_id: impl Into<String>,
    resources: Vec<ResourcePricing>,
    period: TimePeriod,
) -> PricingModel {
    let mut model =
        PricingModel::new(PricingModelType::Usage, name, blueprint_id).with_billing_period(period);

    for resource in resources {
        model = model.with_resource_pricing(resource);
    }

    model
}

/// Create a tiered pricing model
pub fn create_tiered_model(
    name: impl Into<String>,
    blueprint_id: impl Into<String>,
    resources: Vec<ResourcePricing>,
    period: TimePeriod,
) -> PricingModel {
    let mut model =
        PricingModel::new(PricingModelType::Tiered, name, blueprint_id).with_billing_period(period);

    for resource in resources {
        model = model.with_resource_pricing(resource);
    }

    model
}

/// Recommend a pricing model based on resource requirements
pub fn recommend_model(
    resources: &[ResourceRequirement],
    blueprint_id: impl Into<String>,
) -> PricingModel {
    // Default resource pricing for CPU and memory
    let cpu_pricing = ResourcePricing::new(
        ResourceUnit::CPU,
        Price::new(10_000_000, "TNT"), // 10 TNT per CPU core
    )
    .with_time_period(TimePeriod::Hour);

    let memory_pricing = ResourcePricing::new(
        ResourceUnit::MemoryMB,
        Price::new(10_000, "TNT"), // 0.01 TNT per MB of memory
    )
    .with_time_period(TimePeriod::Hour);

    // Check which resources are needed
    let has_cpu = resources
        .iter()
        .any(|r| matches!(r.unit, ResourceUnit::CPU));
    let has_memory = resources
        .iter()
        .any(|r| matches!(r.unit, ResourceUnit::MemoryMB));
    let has_storage = resources
        .iter()
        .any(|r| matches!(r.unit, ResourceUnit::StorageMB));

    // Create different models based on resource types
    if has_storage {
        let storage_pricing = ResourcePricing::new(
            ResourceUnit::StorageMB,
            Price::new(1_000, "TNT"), // 0.001 TNT per MB of storage
        )
        .with_time_period(TimePeriod::Hour);

        let mut model = create_usage_model(
            "Storage Optimized",
            blueprint_id,
            vec![storage_pricing],
            TimePeriod::Hour,
        );

        if has_cpu {
            model = model.with_resource_pricing(cpu_pricing);
        }
        if has_memory {
            model = model.with_resource_pricing(memory_pricing);
        }

        model
    } else {
        // For compute-focused workloads
        let mut resources = Vec::new();
        if has_cpu {
            resources.push(cpu_pricing);
        }
        if has_memory {
            resources.push(memory_pricing);
        }

        if resources.is_empty() {
            // Fallback model if no recognized resources
            create_fixed_price_model(
                "Basic Service",
                blueprint_id,
                Price::new(5_000_000, "TNT"), // 5 TNT
                TimePeriod::Hour,
            )
        } else {
            create_usage_model(
                "Compute Standard",
                blueprint_id,
                resources,
                TimePeriod::Hour,
            )
        }
    }
}
