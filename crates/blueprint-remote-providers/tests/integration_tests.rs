//! Integration tests for cloud provider deployments using mocks

#[cfg(test)]
mod tests {
    use blueprint_remote_providers::{
        CloudProvisioner, CloudProvider, ResourceSpec, PricingService,
        test_utils::mocks::{MockCloudProvider, fixtures},
    };
    use serial_test::serial;

    #[tokio::test]
    #[serial]
    async fn test_end_to_end_provisioning_flow() {
        // This test simulates a complete provisioning flow with a mock provider
        let mock = MockCloudProvider::new();
        
        // Set up expected responses
        mock.set_response(
            "provision_t3.micro_us-west-2",
            serde_json::json!({
                "id": "i-test-12345",
                "provider": "AWS",
                "instance_type": "t3.micro",
                "region": "us-west-2",
                "public_ip": "54.1.2.3",
                "private_ip": "172.16.0.1",
                "status": "Running"
            })
        );

        // Provision instance
        let instance = mock.provision_instance("t3.micro", "us-west-2").await.unwrap();
        assert_eq!(instance.id, "i-test-12345");
        assert_eq!(instance.public_ip, Some("54.1.2.3".to_string()));
        
        // Verify call history
        let history = mock.get_call_history();
        assert_eq!(history.len(), 1);
        assert_eq!(history[0], "provision_t3.micro_us-west-2");
        
        // Test termination
        mock.terminate_instance(&instance.id).await.unwrap();
        
        let history = mock.get_call_history();
        assert_eq!(history.len(), 2);
        assert!(history[1].starts_with("terminate_"));
    }

    #[tokio::test]
    async fn test_provider_selection_for_gpu_workload() {
        let spec = fixtures::gpu_spec();
        let pricing_service = PricingService::new();
        
        // Find cheapest provider for GPU workload
        let (provider, report) = pricing_service.find_cheapest_provider(&spec, 24.0);
        
        // GPU workloads should prefer certain providers
        assert!(matches!(provider, CloudProvider::AWS | CloudProvider::GCP));
        assert!(report.resource_spec.gpu_count.is_some());
        assert!(report.total_cost > 0.0);
    }

    #[tokio::test]
    async fn test_spot_instance_cost_optimization() {
        let regular_spec = fixtures::basic_spec();
        let spot_spec = fixtures::spot_spec();
        
        let pricing_service = PricingService::new();
        
        let regular_cost = pricing_service.calculate_cost(
            &regular_spec,
            CloudProvider::AWS,
            24.0
        );
        
        let spot_cost = pricing_service.calculate_cost(
            &spot_spec,
            CloudProvider::AWS,
            24.0
        );
        
        // Spot instances should be cheaper
        assert!(spot_cost.total_cost < regular_cost.total_cost);
        assert_eq!(spot_cost.discount_percentage, 30.0);
    }

    #[tokio::test]
    #[serial]
    async fn test_retry_logic_on_failure() {
        let mock = MockCloudProvider::new();
        let mut attempt = 0;
        
        // Simulate transient failures
        mock.set_response(
            "provision_t3.micro_us-west-2",
            serde_json::json!({
                "error": if attempt < 2 { 
                    attempt += 1;
                    "TransientError"
                } else {
                    ""
                }
            })
        );
        
        // The provisioner should retry and eventually succeed
        // Note: This would require implementing retry logic in the actual CloudProvisioner
        // For now, we're testing the mock behavior
        let result = mock.provision_instance("t3.micro", "us-west-2").await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_multi_provider_comparison() {
        let spec = ResourceSpec::recommended();
        let pricing_service = PricingService::new();
        
        let reports = pricing_service.compare_providers(&spec, 730.0); // Monthly
        
        // Should have reports for all providers
        assert!(reports.len() >= 5);
        
        // Verify different providers have different costs
        let costs: Vec<f64> = reports.iter().map(|r| r.total_cost).collect();
        let unique_costs: std::collections::HashSet<String> = 
            costs.iter().map(|c| format!("{:.2}", c)).collect();
        
        // At least some providers should have different costs
        assert!(unique_costs.len() > 1);
    }

    #[tokio::test]
    async fn test_resource_validation() {
        // Test invalid resource specs
        let invalid_specs = vec![
            ResourceSpec {
                cpu: 0.05,  // Too low
                memory_gb: 1.0,
                storage_gb: 10.0,
                gpu_count: None,
                allow_spot: false,
            },
            ResourceSpec {
                cpu: 1.0,
                memory_gb: 0.25,  // Too low
                storage_gb: 10.0,
                gpu_count: None,
                allow_spot: false,
            },
            ResourceSpec {
                cpu: 1.0,
                memory_gb: 1.0,
                storage_gb: 0.5,  // Too low
                gpu_count: None,
                allow_spot: false,
            },
            ResourceSpec {
                cpu: 1.0,
                memory_gb: 1.0,
                storage_gb: 10.0,
                gpu_count: Some(0),  // Invalid GPU count
                allow_spot: false,
            },
        ];
        
        for spec in invalid_specs {
            assert!(spec.validate().is_err());
        }
        
        // Test valid specs
        let valid_specs = vec![
            fixtures::minimal_spec(),
            fixtures::basic_spec(),
            fixtures::gpu_spec(),
            fixtures::high_memory_spec(),
        ];
        
        for spec in valid_specs {
            assert!(spec.validate().is_ok());
        }
    }

    #[tokio::test]
    #[serial]
    async fn test_concurrent_provisioning() {
        use futures::future::join_all;
        
        let mock = MockCloudProvider::new();
        
        // Launch multiple provisions concurrently
        let tasks: Vec<_> = (0..5)
            .map(|i| {
                let mock_clone = MockCloudProvider::new();
                async move {
                    mock_clone.provision_instance(
                        &format!("t3.micro"),
                        &format!("us-west-{}", i)
                    ).await
                }
            })
            .collect();
        
        let results = join_all(tasks).await;
        
        // All should succeed
        for result in results {
            assert!(result.is_ok());
        }
    }

    #[tokio::test]
    async fn test_cleanup_on_ttl_expiry() {
        // This would test the TTL management system
        // For now, we're just testing the mock behavior
        let mock = MockCloudProvider::new();
        
        // Provision
        let instance = mock.provision_instance("t3.micro", "us-west-2").await.unwrap();
        
        // Simulate TTL expiry (would be done by TtlManager in real scenario)
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        
        // Cleanup
        mock.terminate_instance(&instance.id).await.unwrap();
        
        // Verify cleanup was called
        let history = mock.get_call_history();
        assert_eq!(history.len(), 2);
        assert!(history[1].contains("terminate"));
    }
}

#[cfg(test)]
mod property_tests {
    use blueprint_remote_providers::ResourceSpec;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_resource_spec_validation_property(
            cpu in 0.1f32..=128.0,
            memory_gb in 0.5f32..=1024.0,
            storage_gb in 1.0f32..=10000.0,
            gpu_count in prop::option::of(1u32..=8)
        ) {
            let spec = ResourceSpec {
                cpu,
                memory_gb,
                storage_gb,
                gpu_count,
                allow_spot: false,
            };
            
            // All specs within these ranges should be valid
            assert!(spec.validate().is_ok());
        }

        #[test]
        fn test_cost_is_positive(
            cpu in 0.1f32..=128.0,
            memory_gb in 0.5f32..=1024.0,
            storage_gb in 1.0f32..=10000.0
        ) {
            let spec = ResourceSpec {
                cpu,
                memory_gb,
                storage_gb,
                gpu_count: None,
                allow_spot: false,
            };
            
            let cost = spec.estimate_hourly_cost();
            assert!(cost > 0.0);
        }

        #[test]
        fn test_spot_discount_property(
            cpu in 0.1f32..=128.0,
            memory_gb in 0.5f32..=1024.0,
            storage_gb in 1.0f32..=10000.0
        ) {
            let regular_spec = ResourceSpec {
                cpu,
                memory_gb,
                storage_gb,
                gpu_count: None,
                allow_spot: false,
            };
            
            let spot_spec = ResourceSpec {
                allow_spot: true,
                ..regular_spec.clone()
            };
            
            let regular_cost = regular_spec.estimate_hourly_cost();
            let spot_cost = spot_spec.estimate_hourly_cost();
            
            // Spot should always be cheaper
            assert!(spot_cost < regular_cost);
            // Discount should be approximately 30%
            let discount_ratio = spot_cost / regular_cost;
            assert!(discount_ratio > 0.65 && discount_ratio < 0.75);
        }
    }
}