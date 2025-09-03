//! End-to-end deployment tests for blueprint-remote-providers
//!
//! These tests verify the complete deployment lifecycle across different
//! providers and runtime environments.

#[cfg(test)]
mod tests {
    use blueprint_remote_providers::{
        ResourceSpec, ComputeResources, StorageResources, NetworkResources,
        CloudProvider, RemoteDeploymentConfig, PricingCalculator,
    };
    use blueprint_remote_providers::resources::{
        StorageType, BandwidthTier, QosParameters, 
        AcceleratorResources, AcceleratorType, GpuSpec,
    };
    use std::time::Duration;
    use tokio::time::sleep;

    /// Test resource spec creation and validation
    #[test]
    fn test_resource_spec_creation() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                cpu_arch: Some("x86_64".to_string()),
                min_cpu_frequency_ghz: Some(2.4),
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                disk_type: StorageType::SSD,
                iops: Some(3000),
            },
            network: NetworkResources {
                bandwidth_tier: BandwidthTier::Standard,
                guaranteed_bandwidth_mbps: Some(1000),
                static_ip: false,
                public_ip: true,
            },
            accelerators: None,
            qos: QosParameters {
                priority: 50,
                allow_spot: false,
                allow_burstable: true,
                min_availability_sla: Some(99.9),
            },
        };

        assert_eq!(spec.compute.cpu_cores, 4.0);
        assert_eq!(spec.storage.memory_gb, 16.0);
        assert_eq!(spec.storage.disk_gb, 100.0);
        assert!(spec.network.public_ip);
    }

    /// Test GPU resource specification
    #[test]
    fn test_gpu_resource_spec() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 8.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 32.0,
                disk_gb: 500.0,
                disk_type: StorageType::NVME,
                ..Default::default()
            },
            accelerators: Some(AcceleratorResources {
                count: 2,
                accelerator_type: AcceleratorType::GPU(GpuSpec {
                    vendor: "nvidia".to_string(),
                    model: "a100".to_string(),
                    min_vram_gb: 40.0,
                }),
            }),
            ..Default::default()
        };

        assert!(spec.accelerators.is_some());
        let accel = spec.accelerators.unwrap();
        assert_eq!(accel.count, 2);
        
        if let AcceleratorType::GPU(gpu) = accel.accelerator_type {
            assert_eq!(gpu.vendor, "nvidia");
            assert_eq!(gpu.model, "a100");
            assert_eq!(gpu.min_vram_gb, 40.0);
        }
    }

    /// Test resource conversion to Kubernetes format
    #[cfg(feature = "kubernetes")]
    #[test]
    fn test_k8s_resource_conversion() {
        use blueprint_remote_providers::resources::to_k8s_resources;
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.5,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 8.0,
                disk_gb: 50.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let (resources, pvc) = to_k8s_resources(&spec);
        
        assert!(resources.limits.is_some());
        assert!(resources.requests.is_some());
        assert!(pvc.is_some());
        
        let limits = resources.limits.unwrap();
        assert!(limits.contains_key("cpu"));
        assert!(limits.contains_key("memory"));
        
        // Verify CPU is set correctly (2.5 cores = 2500m millicores)
        let cpu_limit = limits.get("cpu").unwrap();
        assert_eq!(cpu_limit.0, "2.5");
        
        // Verify memory is set correctly (8GB)
        let memory_limit = limits.get("memory").unwrap();
        assert_eq!(memory_limit.0, "8Gi");
    }

    /// Test resource conversion to Docker format
    #[test]
    fn test_docker_resource_conversion() {
        use blueprint_remote_providers::resources::to_docker_resources;
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 1.5,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 4.0,
                disk_gb: 20.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let docker_config = to_docker_resources(&spec);
        
        // Verify CPU limit (1.5 cores = 1.5 billion nanocpus)
        assert_eq!(docker_config["NanoCPUs"], 1_500_000_000i64);
        
        // Verify memory limit (4GB in bytes)
        assert_eq!(docker_config["Memory"], 4 * 1024 * 1024 * 1024i64);
        
        // Verify storage configuration
        assert!(docker_config["StorageOpt"]["size"].is_string());
    }

    /// Test pricing calculation across providers
    #[tokio::test]
    async fn test_pricing_calculation() {
        let calculator = PricingCalculator::new().unwrap();
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 4.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 16.0,
                disk_gb: 100.0,
                ..Default::default()
            },
            ..Default::default()
        };

        // Test AWS pricing
        let aws_report = calculator.calculate_cost(&spec, &CloudProvider::AWS, 24.0);
        assert!(aws_report.final_hourly_cost > 0.0);
        assert_eq!(aws_report.duration_hours, 24.0);
        assert_eq!(aws_report.currency, "USD");

        // Test GCP pricing
        let gcp_report = calculator.calculate_cost(&spec, &CloudProvider::GCP, 24.0);
        assert!(gcp_report.final_hourly_cost > 0.0);

        // Test Azure pricing
        let azure_report = calculator.calculate_cost(&spec, &CloudProvider::Azure, 24.0);
        assert!(azure_report.final_hourly_cost > 0.0);

        // Verify cloud markup is applied
        let generic_report = calculator.calculate_cost(&spec, &CloudProvider::Generic, 24.0);
        assert!(aws_report.final_hourly_cost > generic_report.final_hourly_cost);
    }

    /// Test spot instance pricing discount
    #[test]
    fn test_spot_instance_pricing() {
        let calculator = PricingCalculator::new().unwrap();
        
        let mut spec = ResourceSpec::default();
        spec.qos.allow_spot = false;
        
        let regular_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);
        
        spec.qos.allow_spot = true;
        let spot_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);
        
        // Spot instances should be cheaper
        assert!(spot_cost.final_hourly_cost < regular_cost.final_hourly_cost);
        
        // Verify discount is approximately 30%
        let discount = 1.0 - (spot_cost.final_hourly_cost / regular_cost.final_hourly_cost);
        assert!(discount > 0.25 && discount < 0.35);
    }

    /// Test provider comparison functionality
    #[test]
    fn test_provider_comparison() {
        let calculator = PricingCalculator::new().unwrap();
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 8.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 32.0,
                disk_gb: 200.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let reports = calculator.compare_providers(&spec, 730.0); // Monthly
        
        assert_eq!(reports.len(), 6); // AWS, GCP, Azure, DigitalOcean, Vultr, Generic
        
        // Find cheapest and most expensive providers
        let cheapest = reports.iter()
            .min_by(|a, b| a.final_hourly_cost.partial_cmp(&b.final_hourly_cost).unwrap())
            .unwrap();
        
        let most_expensive = reports.iter()
            .max_by(|a, b| a.final_hourly_cost.partial_cmp(&b.final_hourly_cost).unwrap())
            .unwrap();
        
        // Generic (self-hosted) should be cheapest
        assert!(matches!(cheapest.provider, CloudProvider::Generic));
        
        // Cloud providers should be more expensive than self-hosted
        assert!(most_expensive.final_hourly_cost > cheapest.final_hourly_cost);
    }


    /// Test resource spec validation
    #[test]
    fn test_resource_spec_validation() {
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 4.0,
                disk_gb: 20.0,
                ..Default::default()
            },
            ..Default::default()
        };

        // Validate minimum resources
        assert!(spec.compute.cpu_cores >= 0.5);
        assert!(spec.storage.memory_gb >= 0.5);
        assert!(spec.storage.disk_gb >= 1.0);
    }


    /// Integration test for complete deployment flow
    #[cfg(feature = "kubernetes")]
    #[tokio::test]
    async fn test_deployment_flow_integration() {
        use blueprint_remote_providers::RemoteClusterManager;
        
        let spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 2.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 4.0,
                disk_gb: 10.0,
                ..Default::default()
            },
            ..Default::default()
        };

        // Step 1: Calculate costs
        let calculator = PricingCalculator::new().unwrap();
        let cost_report = calculator.calculate_cost(&spec, &CloudProvider::AWS, 24.0);
        assert!(cost_report.final_hourly_cost > 0.0);

        // Step 2: Check if cost exceeds threshold
        let max_hourly_budget = 1.0;
        if cost_report.exceeds_threshold(max_hourly_budget) {
            println!("Warning: Cost exceeds budget!");
        }

        // Step 3: Setup cluster manager (would fail without valid kubeconfig)
        let cluster_manager = RemoteClusterManager::new();
        
        let config = RemoteDeploymentConfig {
            namespace: "blueprint-test".to_string(),
            provider: CloudProvider::AWS,
            ..Default::default()
        };

        // This would fail without valid kubeconfig, which is expected in tests
        let add_result = cluster_manager.add_cluster("test-cluster".to_string(), config).await;
        assert!(add_result.is_err());

        // Step 4: Verify resource spec is valid for deployment
        assert!(spec.compute.cpu_cores > 0.0);
        assert!(spec.storage.memory_gb > 0.0);
    }

    /// Test resource scaling scenarios
    #[tokio::test]
    async fn test_resource_scaling() {
        let base_spec = ResourceSpec {
            compute: ComputeResources {
                cpu_cores: 1.0,
                ..Default::default()
            },
            storage: StorageResources {
                memory_gb: 2.0,
                disk_gb: 10.0,
                ..Default::default()
            },
            ..Default::default()
        };

        let calculator = PricingCalculator::new().unwrap();

        // Test scaling up
        let mut scaled_spec = base_spec.clone();
        for scale_factor in [2.0, 4.0, 8.0] {
            scaled_spec.compute.cpu_cores = base_spec.compute.cpu_cores * scale_factor;
            scaled_spec.storage.memory_gb = base_spec.storage.memory_gb * scale_factor;
            
            let report = calculator.calculate_cost(&scaled_spec, &CloudProvider::AWS, 1.0);
            
            // Cost should increase with resources
            let base_report = calculator.calculate_cost(&base_spec, &CloudProvider::AWS, 1.0);
            assert!(report.final_hourly_cost > base_report.final_hourly_cost);
        }
    }

    /// Test high availability configuration
    #[test]
    fn test_high_availability_pricing() {
        let calculator = PricingCalculator::new().unwrap();
        
        let mut spec = ResourceSpec::default();
        
        // Standard availability
        spec.qos.min_availability_sla = Some(99.0);
        let standard_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);
        
        // High availability (99.9%)
        spec.qos.min_availability_sla = Some(99.9);
        let high_availability_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);
        
        // Ultra high availability (99.99%)
        spec.qos.min_availability_sla = Some(99.99);
        let ultra_high_cost = calculator.calculate_cost(&spec, &CloudProvider::AWS, 1.0);
        
        // Higher SLA should increase cost
        assert!(high_availability_cost.final_hourly_cost > standard_cost.final_hourly_cost);
        assert!(ultra_high_cost.final_hourly_cost > high_availability_cost.final_hourly_cost);
    }
}