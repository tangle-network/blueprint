//! Price calculation logic for the pricing engine
//!
//! This module implements the core price calculation algorithms for various pricing models.

use crate::error::{PricingError, Result};
use crate::models::{PricingModel, PricingModelType};
use crate::types::{Price, ResourceRequirement};

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
