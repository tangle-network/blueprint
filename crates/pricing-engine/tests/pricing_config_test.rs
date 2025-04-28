use std::collections::HashMap;
use std::path::Path;

use blueprint_core::info;
use blueprint_pricing_engine_lib::{
    error::Result,
    pricing::{BLOCK_TIME, PriceModel, calculate_resource_price, load_pricing_from_toml},
    types::ResourceUnit,
};
use tangle_subxt::tangle_testnet_runtime::api::runtime_types::{
    sp_arithmetic::per_things::Percent,
    tangle_primitives::services::types::{Asset, AssetSecurityRequirement},
};

#[tokio::test]
async fn test_default_pricing_config() -> Result<()> {
    // Path to our default pricing configuration
    let config_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("config")
        .join("default_pricing.toml");

    // Ensure the config file exists
    assert!(
        config_path.exists(),
        "Default pricing config file not found"
    );

    // Load pricing data from TOML
    let pricing_data = load_pricing_from_toml(config_path.to_str().unwrap())?;

    // Print all loaded pricing data
    info!("Loaded pricing data from default configuration:");
    for (key, resources) in &pricing_data {
        match key {
            Some(id) => info!("  Blueprint ID: {}", id),
            None => info!("  Default pricing"),
        }

        for resource in resources {
            info!(
                "    Resource: {}, Count: {}, Price: ${:.6} per unit",
                resource.kind, resource.count, resource.price_per_unit_rate
            );
        }
    }

    // Verify default pricing exists
    let default_resources = pricing_data
        .get(&None)
        .expect("Default pricing should be available");
    assert!(
        !default_resources.is_empty(),
        "Default pricing should not be empty"
    );

    // Verify all resource types are present in default pricing
    let expected_resource_types = [
        ResourceUnit::CPU,
        ResourceUnit::MemoryMB,
        ResourceUnit::StorageMB,
        ResourceUnit::NetworkEgressMB,
        ResourceUnit::NetworkIngressMB,
        ResourceUnit::GPU,
        ResourceUnit::Request,
        ResourceUnit::Invocation,
        ResourceUnit::ExecutionTimeMS,
    ];

    for expected_type in &expected_resource_types {
        let resource = default_resources.iter().find(|r| {
            matches!(
                (&r.kind, expected_type),
                (ResourceUnit::CPU, ResourceUnit::CPU)
                    | (ResourceUnit::MemoryMB, ResourceUnit::MemoryMB)
                    | (ResourceUnit::StorageMB, ResourceUnit::StorageMB)
                    | (ResourceUnit::NetworkEgressMB, ResourceUnit::NetworkEgressMB)
                    | (
                        ResourceUnit::NetworkIngressMB,
                        ResourceUnit::NetworkIngressMB
                    )
                    | (ResourceUnit::GPU, ResourceUnit::GPU)
                    | (ResourceUnit::Request, ResourceUnit::Request)
                    | (ResourceUnit::Invocation, ResourceUnit::Invocation)
                    | (ResourceUnit::ExecutionTimeMS, ResourceUnit::ExecutionTimeMS)
            )
        });

        assert!(
            resource.is_some(),
            "Resource type {:?} should be present",
            expected_type
        );
    }

    // Verify blueprint-specific pricing
    assert!(
        pricing_data.contains_key(&Some(1)),
        "Blueprint 1 pricing should be available"
    );
    assert!(
        pricing_data.contains_key(&Some(2)),
        "Blueprint 2 pricing should be available"
    );

    // Calculate total cost for default pricing
    let mut total_cost: f64 = 0.0;
    for resource in default_resources {
        total_cost += resource.price_per_unit_rate * resource.count as f64;
    }

    // Create a price model from the resources
    let price_model = PriceModel {
        resources: default_resources.clone(),
        total_cost,
        benchmark_profile: None,
    };

    // Create a pricing configuration for testing
    let mut pricing_config = HashMap::new();
    pricing_config.insert(None::<u64>, default_resources.clone());

    info!("Pricing verification successful");
    info!("  Total cost: ${:.6} USD", price_model.total_cost);

    Ok(())
}

#[tokio::test]
async fn test_resource_price_calculation() -> Result<()> {
    // Test parameters
    let count = 4u64; // 4 units of a resource
    let price_per_unit = 0.001; // $0.001 per unit
    let ttl_blocks = 600u64; // 600 blocks (equivalent to 1 hour with 6-second blocks)

    // Test without security requirements
    let price_no_security = calculate_resource_price(count, price_per_unit, ttl_blocks, None);

    // Expected calculation:
    // calculate_base_resource_cost(0.001 * 4) * calculate_ttl_price_adjustment(600) * calculate_security_rate_adjustment(None)
    // = 0.004 * (600 * 6.0) * 1.0
    // = 0.004 * 3600 * 1.0
    // = 14.4
    let expected_price_no_security = 0.004 * (ttl_blocks as f64 * BLOCK_TIME) * 1.0;

    assert_eq!(
        price_no_security, expected_price_no_security,
        "Price calculation without security requirements failed"
    );

    info!("Resource price calculation (no security):");
    info!("  Count: {}", count);
    info!("  Price per unit: ${:.6}", price_per_unit);
    info!(
        "  TTL: {} blocks ({} seconds)",
        ttl_blocks,
        ttl_blocks as f64 * BLOCK_TIME
    );
    info!("  Calculated price: ${:.6}", price_no_security);

    // Test with security requirements
    let security_requirements = AssetSecurityRequirement {
        asset: Asset::Custom(0),
        min_exposure_percent: Percent(50),
        max_exposure_percent: Percent(80),
    };

    let price_with_security = calculate_resource_price(
        count,
        price_per_unit,
        ttl_blocks,
        Some(security_requirements),
    );

    // With security requirements, the security factor is still 1.0 (default)
    // So the price should be the same as without security requirements
    let expected_price_with_security = expected_price_no_security;

    assert_eq!(
        price_with_security, expected_price_with_security,
        "Price calculation with security requirements failed"
    );

    info!("Resource price calculation (with security):");
    info!("  Count: {}", count);
    info!("  Price per unit: ${:.6}", price_per_unit);
    info!(
        "  TTL: {} blocks ({} seconds)",
        ttl_blocks,
        ttl_blocks as f64 * BLOCK_TIME
    );
    info!("  Calculated price: ${:.6}", price_with_security);

    Ok(())
}
