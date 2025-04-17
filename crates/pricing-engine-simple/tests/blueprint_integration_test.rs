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

    // Create a simple TOML file with default pricing
    let toml_content = r#"
# Default pricing configuration
[default]
resources = [
  { kind = "CPU", count = 1, price_per_unit_rate = 1000000 },
  { kind = "MemoryMB", count = 1024, price_per_unit_rate = 500000 }
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
            println!("    Resource: {}, Count: {}, Price: {}", 
                     resource.kind, resource.count, resource.price_per_unit_rate);
        }
    }
    
    // Test default pricing
    let default_resources = pricing_data.get(&None).expect("Default pricing should be available");
    
    // Verify CPU pricing
    let cpu_resource = default_resources.iter().find(|r| r.kind.to_string() == "CPU");
    assert!(cpu_resource.is_some(), "CPU resource should be present");
    assert_eq!(cpu_resource.unwrap().price_per_unit_rate, 1_000_000, "CPU price mismatch");
    
    // Verify Memory pricing
    let memory_resource = default_resources.iter().find(|r| r.kind.to_string() == "MemoryMB");
    assert!(memory_resource.is_some(), "Memory resource should be present");
    assert_eq!(memory_resource.unwrap().price_per_unit_rate, 500_000, "Memory price mismatch");
    
    // Calculate expected total price per second
    let expected_total = 1_000_000 + (500_000 * 1024);
    
    // Create a price model from the resources
    let price_model = PriceModel {
        resources: default_resources.clone(),
        price_per_second_rate: expected_total,
        generated_at: Utc::now(),
        benchmark_profile: None,
    };
    
    // Verify the total cost calculation
    let ttl_seconds = 3600; // 1 hour
    let expected_cost = expected_total * ttl_seconds as u128;
    assert_eq!(price_model.calculate_total_cost(ttl_seconds), expected_cost);
    
    println!("Pricing verified successfully");
    println!("  CPU price: 1000000 rate units");
    println!("  Memory price: 500000 rate units per MB");
    println!("  Total price per second: {} rate units", expected_total);
    println!("  Total cost for {} seconds: {} rate units", ttl_seconds, expected_cost);
    
    Ok(())
}
