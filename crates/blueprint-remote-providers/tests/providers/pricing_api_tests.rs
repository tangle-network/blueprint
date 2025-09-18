//! Integration test for real-time pricing APIs
//! 
//! Run with: cargo test -p blueprint-remote-providers --test real_pricing_test -- --nocapture

use blueprint_remote_providers::{
    pricing::fetcher::{PricingFetcher, InstanceInfo},
    remote::CloudProvider,
};

#[tokio::test]
async fn test_aws_vantage_api() {
    println!("\n🔍 Testing AWS pricing from instances.vantage.sh...");
    
    let client = reqwest::Client::new();
    let url = "https://instances.vantage.sh/aws/instances.json";
    
    match client.get(url).send().await {
        Ok(response) => {
            println!("✅ Connected to instances.vantage.sh");
            
            if let Ok(text) = response.text().await {
                // Just check we got JSON
                if text.starts_with('[') || text.starts_with('{') {
                    println!("✅ Got valid JSON response");
                    
                    // Try to parse a few entries
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(array) = json.as_array() {
                            println!("📊 Found {} AWS instance types", array.len());
                            
                            // Show first 3 instances
                            for (i, instance) in array.iter().take(3).enumerate() {
                                if let Some(name) = instance.get("api_name") {
                                    if let Some(price) = instance.get("hourly_price") {
                                        println!("  {}. {} - ${}/hr", i+1, name, price);
                                    }
                                }
                            }
                        }
                    }
                } else {
                    println!("❌ Response doesn't look like JSON");
                }
            }
        }
        Err(e) => {
            println!("⚠️  Could not connect to instances.vantage.sh: {}", e);
            println!("   This might be due to network restrictions or the API being down");
        }
    }
}

#[tokio::test]
async fn test_azure_pricing_api() {
    println!("\n🔍 Testing Azure pricing API...");
    
    let client = reqwest::Client::new();
    let url = "https://prices.azure.com/api/retail/prices?api-version=2021-10-01-preview&$top=5";
    
    match client.get(url).send().await {
        Ok(response) => {
            println!("✅ Connected to prices.azure.com");
            
            if let Ok(json) = response.json::<serde_json::Value>().await {
                if let Some(items) = json.get("Items").and_then(|i| i.as_array()) {
                    println!("📊 Got {} Azure pricing items", items.len());
                    
                    for (i, item) in items.iter().take(3).enumerate() {
                        if let Some(name) = item.get("armSkuName") {
                            if let Some(price) = item.get("retailPrice") {
                                println!("  {}. {} - ${}", i+1, name, price);
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("⚠️  Could not connect to Azure pricing API: {}", e);
        }
    }
}

#[tokio::test]
async fn test_pricing_fetcher_integration() {
    println!("\n🔍 Testing PricingFetcher with real APIs...");
    
    let mut fetcher = PricingFetcher::new();
    
    // Test AWS pricing
    println!("\n📦 Testing AWS instance selection:");
    match fetcher.find_best_instance(
        CloudProvider::AWS,
        "us-west-2",
        2.0,  // 2 vCPUs
        4.0,  // 4 GB RAM
        0.10, // Max $0.10/hr
    ).await {
        Ok(instance) => {
            println!("✅ Found AWS instance: {}", instance.name);
            println!("   vCPUs: {}", instance.vcpus);
            println!("   Memory: {} GB", instance.memory_gb);
            println!("   Price: ${:.4}/hr", instance.hourly_price);
        }
        Err(e) => {
            println!("⚠️  Could not find suitable AWS instance: {}", e);
        }
    }
    
    // Test Azure pricing
    println!("\n📦 Testing Azure instance selection:");
    match fetcher.find_best_instance(
        CloudProvider::Azure,
        "eastus",
        2.0,
        4.0,
        0.10,
    ).await {
        Ok(instance) => {
            println!("✅ Found Azure instance: {}", instance.name);
            println!("   vCPUs: {}", instance.vcpus);
            println!("   Memory: {} GB", instance.memory_gb);
            println!("   Price: ${:.4}/hr", instance.hourly_price);
        }
        Err(e) => {
            println!("⚠️  Could not find suitable Azure instance: {}", e);
        }
    }
    
    // Test GCP pricing
    println!("\n📦 Testing GCP instance selection:");
    match fetcher.find_best_instance(
        CloudProvider::GCP,
        "us-central1",
        2.0,
        4.0,
        0.10,
    ).await {
        Ok(instance) => {
            println!("✅ Found GCP instance: {}", instance.name);
            println!("   vCPUs: {}", instance.vcpus);
            println!("   Memory: {} GB", instance.memory_gb);
            println!("   Price: ${:.4}/hr", instance.hourly_price);
        }
        Err(e) => {
            println!("⚠️  Could not find suitable GCP instance: {}", e);
        }
    }
}

#[tokio::test]
async fn test_cheapest_provider_selection() {
    println!("\n💰 Testing cheapest provider selection...");
    
    let mut fetcher = PricingFetcher::new();
    let mut results = Vec::new();
    
    for provider in [CloudProvider::AWS, CloudProvider::Azure, CloudProvider::GCP, CloudProvider::DigitalOcean] {
        match fetcher.find_best_instance(
            provider.clone(),
            "us-west-2",
            2.0,
            8.0,
            1.0,
        ).await {
            Ok(instance) => {
                results.push((provider, instance));
            }
            Err(_) => {}
        }
    }
    
    if !results.is_empty() {
        // Sort by price
        results.sort_by(|a, b| a.1.hourly_price.partial_cmp(&b.1.hourly_price).unwrap());
        
        println!("\n🏆 Price comparison for 2 vCPU, 8GB RAM:");
        for (i, (provider, instance)) in results.iter().enumerate() {
            let medal = match i {
                0 => "🥇",
                1 => "🥈",
                2 => "🥉",
                _ => "  ",
            };
            println!("{} {:?}: {} at ${:.4}/hr", 
                medal, provider, instance.name, instance.hourly_price);
        }
        
        if let Some((cheapest_provider, cheapest_instance)) = results.first() {
            println!("\n✨ Cheapest option: {:?} {} at ${:.4}/hr",
                cheapest_provider, cheapest_instance.name, cheapest_instance.hourly_price);
        }
    }
}