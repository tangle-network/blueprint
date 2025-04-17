use std::fs;

use blueprint_pricing_engine_simple_lib::{
    error::Result,
    pricing::{PriceModel, load_pricing_from_toml},
};
use chrono::Utc;
use log::info;
use tempfile::tempdir;

#[tokio::test]
async fn test_load_pricing_from_toml() -> Result<()> {
    // Create a temporary directory for the test
    let temp_dir = tempdir()?;
    info!("Created temp directory: {:?}", temp_dir.path());

    // Create a comprehensive TOML file with all resource types and smaller pricing rates
    let toml_content = r#"
# Default pricing configuration with all resource types
[default]
resources = [
  # CPU is priced higher as it's a primary resource
  { kind = "CPU", count = 1, price_per_unit_rate = 100 },
  
  # Memory is priced lower per MB
  { kind = "MemoryMB", count = 1024, price_per_unit_rate = 1 },
  
  # Storage is priced similar to memory but slightly cheaper
  { kind = "StorageMB", count = 1024, price_per_unit_rate = 1 },
  
  # Network has different rates for ingress and egress
  { kind = "NetworkEgressMB", count = 1024, price_per_unit_rate = 2 },
  { kind = "NetworkIngressMB", count = 1024, price_per_unit_rate = 1 },
  
  # GPU is a premium resource
  { kind = "GPU", count = 1, price_per_unit_rate = 500 },
  
  # Request-based pricing
  { kind = "Request", count = 1000, price_per_unit_rate = 5 },
  
  # Function invocation pricing
  { kind = "Invocation", count = 1000, price_per_unit_rate = 10 },
  
  # Execution time pricing
  { kind = "ExecutionTimeMS", count = 1000, price_per_unit_rate = 1 }
]
"#;

    // Write the TOML content to a file
    let file_path = temp_dir.path().join("pricing.toml");
    fs::write(&file_path, toml_content)?;

    println!("TOML content:\n{}", toml_content);

    // Load pricing data from TOML
    let pricing_data = load_pricing_from_toml(file_path.to_str().unwrap())?;

    // Debug: Print all loaded pricing data
    println!("Loaded pricing data:");
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

    // Test default pricing
    let default_resources = pricing_data
        .get(&None)
        .expect("Default pricing should be available");

    // Define expected resource types and rates (all integer values)
    let expected_resources = [
        ("CPU", 1, 100),
        ("MemoryMB", 1024, 1),
        ("StorageMB", 1024, 1),
        ("NetworkEgressMB", 1024, 2),
        ("NetworkIngressMB", 1024, 1),
        ("GPU", 1, 500),
        ("Request", 1000, 5),
        ("Invocation", 1000, 10),
        ("ExecutionTimeMS", 1000, 1),
    ];

    // Verify each resource type exists with correct pricing
    for (resource_name, expected_count, expected_rate) in expected_resources.iter() {
        let resource = default_resources
            .iter()
            .find(|r| r.kind.to_string() == *resource_name);
        assert!(
            resource.is_some(),
            "Resource {} should be present",
            resource_name
        );

        let resource = resource.unwrap();
        assert_eq!(
            resource.count, *expected_count as u64,
            "{} count doesn't match expected value",
            resource_name
        );

        assert_eq!(
            resource.price_per_unit_rate, *expected_rate as u128,
            "{} price doesn't match expected value",
            resource_name
        );

        println!(
            "Verified {}: Count = {}, Rate = {}",
            resource_name, resource.count, resource.price_per_unit_rate
        );
    }

    // Calculate expected total price per second based on all resources
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

    // Verify the total cost calculation
    let ttl_seconds = 3600; // 1 hour
    let expected_cost = expected_total.saturating_mul(ttl_seconds as u128);
    assert_eq!(price_model.calculate_total_cost(ttl_seconds), expected_cost);

    println!("Pricing verified successfully");
    println!("  Total price per second: {} rate units", expected_total);
    println!(
        "  Total cost for {} seconds: {} rate units",
        ttl_seconds, expected_cost
    );

    Ok(())
}
