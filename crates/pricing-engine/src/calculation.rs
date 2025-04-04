//! Price calculation logic for the pricing engine
//!
//! This module implements the core price calculation algorithms for various pricing models.

use crate::error::{PricingError, Result};
use crate::models::{PricingModel, PricingModelType};
use crate::types::{DynamicResourcePricing, Price, ResourceRequirement};

/// Calculate service price based on resource requirements and pricing model
pub fn calculate_service_price(
    requirements: &[ResourceRequirement],
    model: &PricingModel,
) -> Result<Price> {
    match model.model_type {
        PricingModelType::Fixed => {
            // For fixed pricing, simply return the base price
            if let Some(base_price) = &model.base_price {
                Ok(base_price.clone())
            } else {
                Err(PricingError::InvalidModelConfiguration(
                    "Fixed price model has no base price".to_string(),
                )
                .into())
            }
        }
        PricingModelType::Usage => {
            // For usage-based pricing, we need to calculate based on each resource
            let mut total_price = if let Some(base_price) = &model.base_price {
                base_price.clone()
            } else {
                Price::new(0, "TNT")
            };

            // Process each resource requirement
            let mut found_match = false;

            for req in requirements {
                // Find matching pricing for this resource
                for resource_pricing in &model.resource_pricing {
                    if resource_pricing.unit == req.unit {
                        // Calculate price based on the quantity
                        match resource_pricing.calculate_price(req.quantity) {
                            Ok(price) => {
                                total_price = total_price
                                    .add(&price)
                                    .map_err(|e| PricingError::CalculationError(e.to_string()))?;
                                found_match = true;
                            }
                            Err(e) => return Err(e.into()),
                        };
                    }
                }
            }

            if !found_match && requirements.len() > 0 {
                return Err(PricingError::NoPricingModelForResource.into());
            }

            Ok(total_price)
        }
        PricingModelType::Tiered => {
            // Similar to usage, but with tier awareness
            let mut total_price = if let Some(base_price) = &model.base_price {
                base_price.clone()
            } else {
                Price::new(0, "TNT")
            };

            // Process each resource requirement
            let mut found_match = false;

            for req in requirements {
                // Find matching pricing for this resource
                for resource_pricing in &model.resource_pricing {
                    if resource_pricing.unit == req.unit {
                        // Calculate price based on the quantity and tiers
                        match resource_pricing.calculate_price(req.quantity) {
                            Ok(price) => {
                                total_price = total_price
                                    .add(&price)
                                    .map_err(|e| PricingError::CalculationError(e.to_string()))?;
                                found_match = true;
                            }
                            Err(e) => return Err(e.into()),
                        };
                    }
                }
            }

            if !found_match && requirements.len() > 0 {
                return Err(PricingError::NoPricingModelForResource.into());
            }

            Ok(total_price)
        }
    }
}

fn calculate_dynamic_price(
    pricing: &[DynamicResourcePricing],
    requirements: &[ResourceRequirement],
    token: String,
) -> Price {
    let mut total = 0;
    for req in requirements {
        if let Some(model) = pricing.iter().find(|p| p.unit == req.unit) {
            let units = req.quantity;
            let contribution = match &model.function {
                PriceFunction::Linear { per_unit } => per_unit * units,
                PriceFunction::Exponential { base, exponent } => {
                    let f_units = units as f64;
                    (*base as f64 * f_units.powf(*exponent as f64)) as u128
                }
                PriceFunction::Tiered(ranges) => {
                    // find which range the units fall into
                    let mut tier_price = 0;
                    for (min, max, per_unit) in ranges {
                        if units >= *min && units <= *max {
                            tier_price = per_unit * units;
                            break;
                        }
                    }
                    tier_price
                }
            };
            total += contribution;
        }
    }

    // TODO: Apply oracle price call for token

    Price {
        value: total,
        token,
    }
}
