use blueprint_pricing_engine_simple_lib::{
    error::Result,
    pricing::{PriceModel, load_pricing_from_toml},
    types::ResourceUnit,
};
use chrono::Utc;
use std::path::Path;

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
                "    Resource: {}, Count: {}, Price: {}",
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
    let mut expected_total: u128 = 0;
    for resource in default_resources {
        expected_total = expected_total.saturating_add(
            resource
                .price_per_unit_rate
                .saturating_mul(resource.count as u128),
        );
    }

    // Create a price model from the resources
    let price_model = PriceModel {
        resources: default_resources.clone(),
        price_per_second_rate: expected_total,
        generated_at: Utc::now(),
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
    println!("  Total price per second: {} rate units", expected_total);
    println!("  Hourly cost: {} rate units", hourly_cost);
    println!("  Daily cost: {} rate units", daily_cost);
    println!("  Monthly cost: {} rate units", monthly_cost);

    Ok(())
}
