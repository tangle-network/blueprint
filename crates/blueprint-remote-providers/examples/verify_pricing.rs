//! Verify real-time pricing APIs are working
//! Run with: cargo run -p blueprint-remote-providers --example verify_pricing

use blueprint_remote_providers::{
    pricing::fetcher::PricingFetcher,
    core::remote::CloudProvider,
};

#[tokio::main]
async fn main() {
    println!("🔍 Verifying Real-Time Pricing APIs\n");
    
    // Test direct API access
    println!("1️⃣  Testing direct API access:");
    test_direct_apis().await;
    
    // Test PricingFetcher
    println!("\n2️⃣  Testing PricingFetcher:");
    test_pricing_fetcher().await;
}

async fn test_direct_apis() {
    let client = reqwest::Client::new();
    
    // AWS Vantage
    print!("   AWS (instances.vantage.sh): ");
    match client.get("https://instances.vantage.sh/instances.json")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await 
    {
        Ok(resp) if resp.status().is_success() => {
            println!("✅ Working");
        }
        Ok(resp) => {
            println!("❌ HTTP {}", resp.status());
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }
    
    // Azure Vantage
    print!("   Azure (instances.vantage.sh/azure): ");
    match client.get("https://instances.vantage.sh/azure/instances.json")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await 
    {
        Ok(resp) if resp.status().is_success() => {
            println!("✅ Working");
        }
        Ok(resp) => {
            println!("❌ HTTP {}", resp.status());
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }
    
    // GCP uses embedded pricing data
    print!("   GCP (cloud.google.com/compute/all-pricing): ");
    println!("Uses simplified pricing data");
    
    // DigitalOcean via web scraping
    print!("   DigitalOcean (digitalocean.com/pricing): ");
    match client.get("https://www.digitalocean.com/pricing/droplets")
        .timeout(std::time::Duration::from_secs(5))
        .send()
        .await 
    {
        Ok(resp) if resp.status().is_success() => {
            println!("✅ Working");
        }
        Ok(resp) => {
            println!("❌ HTTP {}", resp.status());
        }
        Err(e) => {
            println!("❌ Failed: {}", e);
        }
    }
}

async fn test_pricing_fetcher() {
    let mut fetcher = PricingFetcher::new();
    
    for (provider, region) in [
        (CloudProvider::AWS, "us-east-1"),      // AWS via Vantage
        (CloudProvider::Azure, "us-east"),       // Azure via Vantage
        (CloudProvider::GCP, "us-central1"),     // GCP with hardcoded data
        (CloudProvider::DigitalOcean, "nyc1"),   // DO via web scraping
    ] {
        print!("   {:?}: ", provider);
        
        match fetcher.find_best_instance(
            provider,
            region,
            2.0,  // 2 vCPUs
            4.0,  // 4 GB RAM
            0.10, // $0.10/hr budget
        ).await {
            Ok(instance) => {
                println!("✅ {} @ ${:.4}/hr", instance.name, instance.hourly_price);
            }
            Err(e) => {
                println!("❌ {}", e);
            }
        }
    }
}