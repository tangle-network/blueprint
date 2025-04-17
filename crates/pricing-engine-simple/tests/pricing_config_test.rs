use blueprint_pricing_engine_simple_lib::{
    error::Result,
    pricing::{PriceModel, BLOCK_TIME, calculate_resource_price, load_pricing_from_toml},
    types::ResourceUnit,
};
use chrono::Utc;
use std::path::Path;
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

    // Debug: Print all loaded pricing data
    println!("Loaded pricing data from default configuration:");
    for (key, resources) in &pricing_data {
        match key {
            Some(id) => println!("  Blueprint ID: {}", id),
            None => println!("  Default pricing"),
        }

        for resource in resources {
            println!(
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

    // Calculate total price per second for default pricing
    let mut expected_total: f64 = 0.0;
    for resource in default_resources {
        expected_total += resource.price_per_unit_rate * resource.count as f64;
    }

    // Create a price model from the resources
    let price_model = PriceModel {
        resources: default_resources.clone(),
        price_per_second_rate: expected_total,
        benchmark_profile: None,
    };

    // Verify the total cost calculation for different time periods
    let hour = 3600; // 1 hour in seconds
    let day = 86400; // 1 day in seconds
    let month = 2592000; // 30 days in seconds

    let hourly_cost = price_model.calculate_total_cost(hour);
    let daily_cost = price_model.calculate_total_cost(day);
    let monthly_cost = price_model.calculate_total_cost(month);

    println!("Pricing verification successful");
    println!("  Total price per second: ${:.6} USD", expected_total);
    println!("  Hourly cost: ${:.6} USD", hourly_cost);
    println!("  Daily cost: ${:.6} USD", daily_cost);
    println!("  Monthly cost: ${:.6} USD", monthly_cost);

    Ok(())
}

#[tokio::test]
async fn test_resource_price_calculation() -> Result<()> {
    // Test parameters
    let count = 4u64; // 4 units of a resource
    let price_per_unit = 0.001; // $0.001 per unit
    let ttl_seconds = 3600u64; // 1 hour
    
    // Test without security requirements
    let price_no_security = calculate_resource_price(
        count,
        price_per_unit,
        ttl_seconds,
        None,
    );
    
    // Expected calculation:
    // calculate_base_resource_cost(0.001 * 4) * calculate_ttl_price_adjustment(3600 * 6.0) * calculate_security_rate_adjustment(None)
    // = 0.004 * 21600 * 1.0
    // = 86.4
    let expected_price_no_security = 0.004 * (3600.0 * BLOCK_TIME) * 1.0;
    
    assert_eq!(
        price_no_security, 
        expected_price_no_security,
        "Price calculation without security requirements failed"
    );
    
    println!("Resource price calculation (no security):");
    println!("  Count: {}", count);
    println!("  Price per unit: ${:.6}", price_per_unit);
    println!("  TTL: {} seconds", ttl_seconds);
    println!("  Block time: {} seconds", BLOCK_TIME);
    println!("  Calculated price: ${:.6}", price_no_security);
    
    // Test with security requirements
    let security_requirements = AssetSecurityRequirement {
        asset: Asset::Custom(0),
            min_exposure_percent: Percent(50),
            max_exposure_percent: Percent(80),
    };
    
    let price_with_security = calculate_resource_price(
        count,
        price_per_unit,
        ttl_seconds,
        Some(security_requirements),
    );
    
    // Since calculate_security_rate_adjustment currently returns 1.0 regardless of input, the result should be the same
    let expected_price_with_security = expected_price_no_security;
    
    assert_eq!(
        price_with_security, 
        expected_price_with_security,
        "Price calculation with security requirements failed"
    );
    
    println!("Resource price calculation (with security):");
    println!("  Count: {}", count);
    println!("  Price per unit: ${:.6}", price_per_unit);
    println!("  TTL: {} seconds", ttl_seconds);
    println!("  Block time: {} seconds", BLOCK_TIME);
    println!("  Calculated price: ${:.6}", price_with_security);
    
    Ok(())
}
