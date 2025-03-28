//! Tests for pricing model functionality
//!
//! These tests verify that pricing models can correctly calculate prices
//! based on resource requirements.

use crate::{
    models::{PricingModel, PricingModelType, ResourcePricing},
    types::{Price, ResourceRequirement, ResourceUnit, TimePeriod},
};

/// Test basic fixed-price model calculations
#[test]
fn test_fixed_price_model() {
    // Create a simple fixed-price model
    let model = PricingModel {
        model_type: PricingModelType::Fixed,
        name: "Test Model".to_string(),
        description: Some("Test Description".to_string()),
        blueprint_id: "test.model".to_string(),
        base_price: Some(Price {
            value: 1_000_000_000_000_000_000, // 1 TNT
            token: "TNT".to_string(),
        }),
        resource_pricing: Vec::new(), // No resource-specific pricing
        billing_period: Some(TimePeriod::Hour),
    };

    // Create some resource requirements
    let requirements = vec![
        ResourceRequirement {
            unit: ResourceUnit::CPU,
            quantity: 2,
        },
        ResourceRequirement {
            unit: ResourceUnit::MemoryMB,
            quantity: 4096,
        },
    ];

    // Calculate price
    let price = model.calculate_price(&requirements);

    // With a fixed-price model with no resource pricing, should just return the base price
    assert!(price.is_ok());
    if let Ok(price) = price {
        assert_eq!(price.value, 1_000_000_000_000_000_000);
        assert_eq!(price.token, "TNT");
    }
}

/// Test resource-based pricing calculations
#[test]
fn test_resource_based_pricing() {
    // Create a model with resource-specific pricing
    let mut model = PricingModel {
        model_type: PricingModelType::Usage,
        name: "Resource Model".to_string(),
        description: Some("Resource-based pricing".to_string()),
        blueprint_id: "test.resource.model".to_string(),
        base_price: Some(Price {
            value: 500_000_000_000_000_000, // 0.5 TNT base
            token: "TNT".to_string(),
        }),
        resource_pricing: Vec::new(), // Will add below
        billing_period: Some(TimePeriod::Hour),
    };

    // Add resource pricing
    model.resource_pricing = vec![
        ResourcePricing::new(
            ResourceUnit::CPU,
            Price {
                value: 250_000_000_000_000_000, // 0.25 TNT per CPU core
                token: "TNT".to_string(),
            },
        ),
        ResourcePricing::new(
            ResourceUnit::MemoryMB,
            Price {
                value: 1_000_000_000_000_000, // 0.001 TNT per GB of memory
                token: "TNT".to_string(),
            },
        ),
    ];

    // Create resource requirements
    let requirements = vec![
        ResourceRequirement {
            unit: ResourceUnit::CPU,
            quantity: 4, // 4 cores
        },
        ResourceRequirement {
            unit: ResourceUnit::MemoryMB,
            quantity: 8192, // 8 GB (in MB)
        },
    ];

    // Calculate price
    let price = model.calculate_price(&requirements);

    // Expected calculation:
    // Base: 0.5 TNT
    // CPU: 4 cores * 0.25 TNT = 1.0 TNT
    // Memory: 8 GB * 0.001 TNT = 0.008 TNT
    // Total: 1.508 TNT = 1_508_000_000_000_000_000 wei
    assert!(price.is_ok());
    if let Ok(price) = price {
        assert_eq!(price.value, 1_508_000_000_000_000_000);
        assert_eq!(price.token, "TNT");
    }
}

/// Test no price available for incompatible requirements
#[test]
fn test_incompatible_requirements() {
    // Create a model that only prices CPU
    let mut model = PricingModel {
        model_type: PricingModelType::Usage,
        name: "CPU Only Model".to_string(),
        description: Some("Only prices CPU resources".to_string()),
        blueprint_id: "test.cpu.model".to_string(),
        base_price: None,             // No base price
        resource_pricing: Vec::new(), // Will add below
        billing_period: Some(TimePeriod::Hour),
    };

    // Add resource pricing only for CPU
    model.resource_pricing = vec![ResourcePricing::new(
        ResourceUnit::CPU,
        Price {
            value: 250_000_000_000_000_000, // 0.25 TNT per CPU core
            token: "TNT".to_string(),
        },
    )];

    // Create resource requirements that need GPU (which this model doesn't price)
    let requirements = vec![ResourceRequirement {
        unit: ResourceUnit::GPU,
        quantity: 1,
    }];

    // Calculate price - should still return a result with just the base price (which is None)
    // or 0 TNT if there's no exact match for the resource
    let price = model.calculate_price(&requirements);

    // Since we're ignoring resources without pricing, we should still get a result
    // but the price should be 0 or the base price (if any)
    assert!(price.is_ok());
    if let Ok(price) = price {
        // Since there's no base price and no GPU pricing, the price should be 0 TNT
        assert_eq!(price.value, 0);
        assert_eq!(price.token, "TNT");
    }
}

/// Test matching models by blueprint ID
#[test]
fn test_model_blueprint_matching() {
    // Create several models with different blueprint IDs
    let models = vec![
        PricingModel {
            model_type: PricingModelType::Fixed,
            name: "Compute Model".to_string(),
            description: None,
            blueprint_id: "compute.basic".to_string(),
            base_price: Some(Price {
                value: 1_000_000_000_000_000_000,
                token: "TNT".to_string(),
            }),
            resource_pricing: Vec::new(),
            billing_period: Some(TimePeriod::Hour),
        },
        PricingModel {
            model_type: PricingModelType::Fixed,
            name: "Storage Model".to_string(),
            description: None,
            blueprint_id: "storage.basic".to_string(),
            base_price: Some(Price {
                value: 2_000_000_000_000_000_000,
                token: "TNT".to_string(),
            }),
            resource_pricing: Vec::new(),
            billing_period: Some(TimePeriod::Month),
        },
    ];

    // Find model by blueprint ID
    let compute_model = models.iter().find(|m| m.blueprint_id == "compute.basic");

    assert!(compute_model.is_some());
    if let Some(model) = compute_model {
        assert_eq!(model.name, "Compute Model");
        assert_eq!(model.billing_period, Some(TimePeriod::Hour));
    }

    // Find another model
    let storage_model = models.iter().find(|m| m.blueprint_id == "storage.basic");

    assert!(storage_model.is_some());
    if let Some(model) = storage_model {
        assert_eq!(model.name, "Storage Model");
        assert_eq!(model.billing_period, Some(TimePeriod::Month));
    }

    // Try to find non-existent model
    let non_existent = models.iter().find(|m| m.blueprint_id == "non.existent");

    assert!(non_existent.is_none());
}
