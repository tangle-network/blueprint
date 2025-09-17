//! Tests for cloud provider API integration
//!
//! These tests verify that we can fetch machine types and pricing
//! from provider APIs to avoid hardcoded data.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::monitoring::discovery::{CloudCredentials, MachineTypeDiscovery};
    use crate::remote::CloudProvider;
    use std::env;

    /// Test fetching actual DigitalOcean droplet sizes via API
    #[tokio::test]
    #[ignore] // Run with --ignored and DO_API_TOKEN env var
    async fn test_digitalocean_api_sizes() {
        let api_token = match env::var("DO_API_TOKEN") {
            Ok(token) => token,
            Err(_) => {
                println!("Skipping DigitalOcean API test - set DO_API_TOKEN env var to run");
                return;
            }
        };

        let mut discovery = MachineTypeDiscovery::new();
        let credentials = CloudCredentials {
            api_token: Some(api_token),
            ..Default::default()
        };

        #[cfg(feature = "api-clients")]
        {
            let machines = discovery
                .discover_machine_types(&CloudProvider::DigitalOcean, "nyc3", &credentials)
                .await
                .expect("Failed to fetch DigitalOcean sizes");

            assert!(
                !machines.is_empty(),
                "Should discover at least one machine type"
            );

            // Verify we got API data, not fallback
            let has_real_data = machines.iter().any(|m| m.hourly_price.is_some());
            assert!(has_real_data, "Should have pricing data from API");

            // Verify expected droplet sizes exist
            let size_names: Vec<&str> = machines.iter().map(|m| m.name.as_str()).collect();
            assert!(
                size_names.contains(&"s-1vcpu-1gb"),
                "Should include basic droplet size"
            );

            println!(
                "✅ Fetched {} DigitalOcean droplet sizes from API",
                machines.len()
            );
            for machine in machines.iter().take(5) {
                println!(
                    "  - {}: {} vCPUs, {:.1}GB RAM, ${:.3}/hour",
                    machine.name,
                    machine.vcpus,
                    machine.memory_gb,
                    machine.hourly_price.unwrap_or(0.0)
                );
            }
        }

        #[cfg(not(feature = "api-clients"))]
        {
            println!("⚠️ Skipping API test - api-clients feature not enabled");
        }
    }

    /// Test fetching actual Vultr plans via API
    #[tokio::test]
    #[ignore] // Run with --ignored and VULTR_API_KEY env var
    async fn test_vultr_api_plans() {
        let api_key = match env::var("VULTR_API_KEY") {
            Ok(key) => key,
            Err(_) => {
                println!("Skipping Vultr API test - set VULTR_API_KEY env var to run");
                return;
            }
        };

        let mut discovery = MachineTypeDiscovery::new();
        let credentials = CloudCredentials {
            api_key: Some(api_key),
            ..Default::default()
        };

        #[cfg(feature = "api-clients")]
        {
            let machines = discovery
                .discover_machine_types(&CloudProvider::Vultr, "ewr", &credentials)
                .await
                .expect("Failed to fetch Vultr plans");

            assert!(
                !machines.is_empty(),
                "Should discover at least one machine type"
            );

            // Verify we got API data, not fallback
            let has_real_data = machines.iter().any(|m| m.hourly_price.is_some());
            assert!(has_real_data, "Should have pricing data from API");

            println!("✅ Fetched {} Vultr plans from API", machines.len());
            for machine in machines.iter().take(5) {
                println!(
                    "  - {}: {} vCPUs, {:.1}GB RAM, ${:.3}/hour",
                    machine.name,
                    machine.vcpus,
                    machine.memory_gb,
                    machine.hourly_price.unwrap_or(0.0)
                );
            }
        }

        #[cfg(not(feature = "api-clients"))]
        {
            println!("⚠️ Skipping API test - api-clients feature not enabled");
        }
    }

    /// Test AWS instance type discovery (requires AWS credentials)
    #[tokio::test]
    #[ignore] // Run with --ignored and proper AWS credentials
    async fn test_aws_api_instances() {
        // This would require proper AWS credentials setup
        // For now, just verify our fallback data structure
        let mut discovery = MachineTypeDiscovery::new();
        let credentials = CloudCredentials::default();

        let machines = discovery
            .discover_machine_types(&CloudProvider::AWS, "us-east-1", &credentials)
            .await
            .expect("Should at least return fallback data");

        assert!(
            !machines.is_empty(),
            "Should have fallback AWS instance types"
        );

        // Verify expected instance types exist in fallback
        let instance_names: Vec<&str> = machines.iter().map(|m| m.name.as_str()).collect();
        assert!(
            instance_names.contains(&"t3.micro"),
            "Should include t3.micro"
        );
        assert!(
            instance_names.contains(&"c5.large"),
            "Should include c5.large"
        );

        println!("✅ Verified AWS instance types (using fallback data)");
        for machine in &machines {
            println!(
                "  - {}: {} vCPUs, {:.1}GB RAM, ${:.3}/hour",
                machine.name,
                machine.vcpus,
                machine.memory_gb,
                machine.hourly_price.unwrap_or(0.0)
            );
        }
    }

    /// Test that machine type discovery finds appropriate matches
    #[tokio::test]
    async fn test_machine_type_matching() {
        let mut discovery = MachineTypeDiscovery::new();

        // Populate with test data for AWS
        let credentials = CloudCredentials::default();
        discovery
            .discover_machine_types(&CloudProvider::AWS, "us-east-1", &credentials)
            .await
            .expect("Should load fallback data");

        // Test finding a small instance
        let small_match = discovery.find_best_match(
            &CloudProvider::AWS,
            1,          // min 1 vCPU
            1.0,        // min 1GB RAM
            false,      // no GPU needed
            Some(0.02), // max $0.02/hour
        );

        assert!(
            small_match.is_some(),
            "Should find a match for small requirements"
        );
        let small = small_match.unwrap();
        assert!(small.vcpus >= 1, "Should meet vCPU requirement");
        assert!(small.memory_gb >= 1.0, "Should meet memory requirement");
        if let Some(price) = small.hourly_price {
            assert!(price <= 0.02, "Should meet price requirement");
        }

        // Test finding a GPU instance
        let gpu_match = discovery.find_best_match(
            &CloudProvider::AWS,
            4,    // min 4 vCPUs
            16.0, // min 16GB RAM
            true, // GPU needed
            None, // no price limit
        );

        assert!(gpu_match.is_some(), "Should find a GPU match");
        let gpu = gpu_match.unwrap();
        assert!(gpu.gpu_count > 0, "Should have GPU");
        assert!(gpu.vcpus >= 4, "Should meet vCPU requirement");
        assert!(gpu.memory_gb >= 16.0, "Should meet memory requirement");

        println!("✅ Machine type matching works correctly");
        println!(
            "  Small match: {} ({}vCPU, {:.1}GB)",
            small.name, small.vcpus, small.memory_gb
        );
        println!(
            "  GPU match: {} ({}vCPU, {:.1}GB, {}GPU)",
            gpu.name, gpu.vcpus, gpu.memory_gb, gpu.gpu_count
        );
    }

    /// Test cost comparison across providers
    #[tokio::test]
    async fn test_provider_cost_comparison() {
        use crate::pricing::integration::PricingCalculator;
        use crate::resources::ResourceSpec;

        let calculator = PricingCalculator::new().expect("Should create pricing calculator");

        let spec = ResourceSpec {
            cpu: 2.0,
            memory_gb: 4.0,
            storage_gb: 80.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        // Compare costs across multiple providers
        let providers = vec![
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
        ];

        let mut costs = Vec::new();
        for provider in providers {
            let report = calculator.calculate_cost(&spec, &provider, 24.0);
            println!("{}: ${:.4}/hour", provider, report.final_hourly_cost);
            costs.push((provider, report.final_hourly_cost));
        }

        // Verify cost differences make sense (should vary by provider markup)
        costs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());

        let cheapest = costs[0].1;
        let most_expensive = costs.last().unwrap().1;
        let price_range = most_expensive - cheapest;

        assert!(
            price_range > 0.0,
            "Should have price variation between providers"
        );
        assert!(
            price_range < cheapest * 0.5,
            "Price variation should be reasonable"
        );

        println!(
            "✅ Cost comparison working - price range: ${:.4}/hour",
            price_range
        );
        println!("  Cheapest: {} at ${:.4}/hour", costs[0].0, costs[0].1);
        println!(
            "  Most expensive: {} at ${:.4}/hour",
            costs.last().unwrap().0,
            costs.last().unwrap().1
        );
    }
}
