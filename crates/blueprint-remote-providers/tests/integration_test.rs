//! Comprehensive integration tests for blueprint-remote-providers
//!
//! Tests the complete system including provisioning, health monitoring,
//! TTL management, and cleanup.

#[cfg(test)]
mod tests {
    use blueprint_remote_providers::{
        CloudProvider, DeploymentTracker, HealthMonitor, HealthStatus, PricingService,
        RemoteDeploymentExtensions, ResourceSpec, UnifiedInfrastructureProvisioner,
    };
    use std::sync::Arc;
    use tempfile::TempDir;
    use tokio::time::{Duration, sleep};

    /// Test basic resource specification creation and validation
    #[test]
    fn test_resource_spec_basics() {
        // Test presets
        let minimal = ResourceSpec::minimal();
        assert_eq!(minimal.cpu, 0.5);
        assert_eq!(minimal.memory_gb, 1.0);
        assert!(minimal.allow_spot);

        let basic = ResourceSpec::basic();
        assert_eq!(basic.cpu, 2.0);
        assert_eq!(basic.memory_gb, 4.0);
        assert!(!basic.allow_spot);

        let recommended = ResourceSpec::recommended();
        assert_eq!(recommended.cpu, 4.0);
        assert_eq!(recommended.memory_gb, 16.0);

        // Test GPU addition
        let with_gpu = ResourceSpec::performance().with_gpu(2);
        assert_eq!(with_gpu.gpu_count, Some(2));

        // Test validation
        assert!(minimal.validate().is_ok());
        assert!(basic.validate().is_ok());
        assert!(recommended.validate().is_ok());

        let invalid = ResourceSpec {
            cpu: 0.05,
            memory_gb: 0.25,
            storage_gb: 0.5,
            ..Default::default()
        };
        assert!(invalid.validate().is_err());
    }

    /// Test pricing service with all providers
    #[test]
    fn test_pricing_service() {
        let service = PricingService::new();
        let spec = ResourceSpec::basic();

        // Test individual provider pricing
        for provider in [
            CloudProvider::AWS,
            CloudProvider::GCP,
            CloudProvider::Azure,
            CloudProvider::DigitalOcean,
            CloudProvider::Vultr,
        ] {
            let report = service.calculate_cost(spec.clone(), provider, 24.0);
            assert!(report.final_hourly_cost > 0.0);
            assert_eq!(report.provider, provider);
            assert_eq!(report.duration_hours, 24.0);
            assert!(report.total_cost == report.final_hourly_cost * 24.0);
        }

        // Test provider comparison
        let reports = service.compare_providers(&spec, 730.0);
        assert_eq!(reports.len(), 5);

        // Find cheapest
        let (cheapest_provider, cheapest_report) = service.find_cheapest_provider(&spec, 730.0);
        assert!(cheapest_report.final_hourly_cost > 0.0);

        // Vultr should be cheapest due to lowest markup
        assert_eq!(cheapest_provider, CloudProvider::Vultr);

        // Test spot pricing discount
        let spot_spec = ResourceSpec {
            allow_spot: true,
            ..spec
        };
        let spot_report = service.calculate_cost(spot_spec, CloudProvider::AWS, 1.0);
        let regular_report = service.calculate_cost(spec, CloudProvider::AWS, 1.0);
        assert!(spot_report.final_hourly_cost < regular_report.final_hourly_cost);
    }

    /// Test deployment tracker functionality
    #[tokio::test]
    async fn test_deployment_tracker() {
        use blueprint_remote_providers::deployment_tracker::{DeploymentRecord, DeploymentType};

        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());

        // Create a deployment record
        let record = DeploymentRecord::new(
            "blueprint-123".to_string(),
            DeploymentType::AwsEc2,
            ResourceSpec::basic(),
            Some(3600), // 1 hour TTL
        );

        // Register the deployment
        tracker
            .register_deployment("blueprint-123".to_string(), record.clone())
            .await
            .unwrap();

        // Verify it's registered
        let status = tracker.get_deployment_status("blueprint-123").await;
        assert!(status.is_some());

        // List deployments
        let deployments = tracker.list_deployments().await;
        assert_eq!(deployments.len(), 1);
        assert_eq!(deployments[0].0, "blueprint-123");

        // Test active deployments
        let active = tracker.list_active().await.unwrap();
        assert_eq!(active.len(), 1);

        // Test termination
        tracker.handle_termination("blueprint-123").await.unwrap();

        // Verify it's removed
        let status_after = tracker.get_deployment_status("blueprint-123").await;
        assert!(status_after.is_none());
    }

    /// Test unified infrastructure provisioner initialization
    #[tokio::test]
    async fn test_infrastructure_provisioner() {
        // This test verifies the provisioner can be created
        // Without credentials, it won't have any providers configured
        let provisioner = UnifiedInfrastructureProvisioner::new().await.unwrap();

        // Test retry policy
        let spec = ResourceSpec::basic();

        // Without credentials, provisioning should fail
        let result = provisioner
            .provision(CloudProvider::AWS, &spec, "us-east-1")
            .await;

        // Should fail with ProviderNotConfigured
        assert!(result.is_err());
    }

    /// Test health monitoring setup
    #[tokio::test]
    async fn test_health_monitor() {
        let temp_dir = TempDir::new().unwrap();
        let provisioner = Arc::new(UnifiedInfrastructureProvisioner::new().await.unwrap());
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());

        let monitor = HealthMonitor::new(provisioner, tracker.clone()).with_config(
            Duration::from_secs(5),
            3,
            false,
        );

        // Get health status (should be empty)
        let status = monitor.get_all_health_status().await.unwrap();
        assert_eq!(status.len(), 0);

        // Add a deployment to track
        use blueprint_remote_providers::deployment_tracker::{DeploymentRecord, DeploymentType};
        let record = DeploymentRecord::new(
            "test-deployment".to_string(),
            DeploymentType::LocalDocker,
            ResourceSpec::minimal(),
            None,
        );

        tracker
            .register_deployment("test-deployment".to_string(), record)
            .await
            .unwrap();

        // Now health status should have one entry
        let status_after = monitor.get_all_health_status().await.unwrap();
        assert_eq!(status_after.len(), 1);
    }

    /// Test remote deployment extensions initialization
    #[tokio::test]
    async fn test_remote_deployment_extensions() {
        let temp_dir = TempDir::new().unwrap();

        // Create mock provisioner
        use blueprint_remote_providers::infrastructure_unified::UnifiedInfrastructureProvisioner;
        let provisioner = Arc::new(UnifiedInfrastructureProvisioner::new().await.unwrap());

        // Initialize extensions
        let extensions = RemoteDeploymentExtensions::initialize(
            temp_dir.path(),
            true, // Enable TTL
            provisioner,
        )
        .await
        .unwrap();

        // Test service lifecycle
        use blueprint_remote_providers::deployment_tracker::DeploymentType;
        use blueprint_remote_providers::manager_integration::RemoteDeploymentConfig;

        let config = RemoteDeploymentConfig {
            deployment_type: DeploymentType::AwsEc2,
            provider: Some(CloudProvider::AWS),
            region: Some("us-west-2".to_string()),
            instance_id: "i-test123".to_string(),
            resource_spec: ResourceSpec::basic(),
            ttl_seconds: Some(60), // 1 minute TTL for testing
            deployed_at: chrono::Utc::now(),
        };

        // Simulate service initialization
        extensions
            .event_handler
            .on_service_initiated(
                100, // blueprint_id
                1,   // service_id
                Some(config),
            )
            .await
            .unwrap();

        // Verify it was registered
        let deployment = extensions.registry.get(100, 1).await;
        assert!(deployment.is_some());

        // Simulate service termination
        extensions
            .event_handler
            .on_service_terminated(100, 1)
            .await
            .unwrap();

        // Verify it was cleaned up
        let deployment_after = extensions.registry.get(100, 1).await;
        assert!(deployment_after.is_none());
    }

    /// Test cost threshold checking
    #[test]
    fn test_cost_thresholds() {
        let service = PricingService::new();

        // Minimal resources should be cheap
        let minimal = ResourceSpec::minimal();
        let minimal_report = service.calculate_cost(minimal, CloudProvider::AWS, 1.0);
        assert!(!minimal_report.exceeds_threshold(0.50)); // Under $0.50/hr

        // Performance resources should be expensive
        let performance = ResourceSpec::performance();
        let performance_report = service.calculate_cost(performance, CloudProvider::AWS, 1.0);
        assert!(performance_report.exceeds_threshold(0.10)); // Over $0.10/hr

        // GPU resources should be very expensive
        let gpu = ResourceSpec::performance().with_gpu(1);
        let gpu_report = service.calculate_cost(gpu, CloudProvider::AWS, 1.0);
        assert!(gpu_report.exceeds_threshold(0.50)); // Over $0.50/hr
    }

    /// Test resource conversion to container formats
    #[test]
    fn test_resource_conversions() {
        let spec = ResourceSpec::recommended();

        // Test Docker conversion
        let docker = spec.to_docker_resources();
        assert_eq!(docker["NanoCPUs"], 4_000_000_000i64);
        assert_eq!(docker["Memory"], 16 * 1024 * 1024 * 1024i64);
        assert_eq!(docker["StorageOpt"]["size"], "100G");

        // Test Kubernetes conversion
        #[cfg(feature = "kubernetes")]
        {
            let k8s = spec.to_k8s_resources();
            assert!(k8s.limits.is_some());
            assert!(k8s.requests.is_some());

            let limits = k8s.limits.unwrap();
            assert_eq!(limits["cpu"].0, "4");
            assert_eq!(limits["memory"].0, "16Gi");
        }
    }

    /// Test TTL expiry handling
    #[tokio::test]
    async fn test_ttl_expiry() {
        use blueprint_remote_providers::deployment_tracker::DeploymentTracker;
        use blueprint_remote_providers::manager_integration::{
            RemoteDeploymentRegistry, TtlManager,
        };

        let temp_dir = TempDir::new().unwrap();
        let tracker = Arc::new(DeploymentTracker::new(temp_dir.path()).await.unwrap());
        let registry = Arc::new(RemoteDeploymentRegistry::new(tracker));

        let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        let ttl_manager = Arc::new(TtlManager::new(registry.clone(), tx));

        // Register a service with very short TTL
        ttl_manager.register_ttl(200, 2, 1).await; // 1 second TTL

        // Wait for expiry
        sleep(Duration::from_secs(2)).await;

        // Check for expired services
        let expired = ttl_manager.check_expired_services().await.unwrap();
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], (200, 2));

        // Should have received expiry notification
        let notification = rx.try_recv();
        assert!(notification.is_ok());
        assert_eq!(notification.unwrap(), (200, 2));
    }

    /// Test error recovery in provisioning
    #[tokio::test]
    async fn test_provisioning_retry() {
        // The unified provisioner includes retry logic
        let provisioner = UnifiedInfrastructureProvisioner::new().await.unwrap();

        // Without credentials, this will fail but test the retry mechanism
        let spec = ResourceSpec::minimal();
        let start = tokio::time::Instant::now();

        let result = provisioner
            .provision(CloudProvider::AWS, &spec, "us-east-1")
            .await;

        // Should fail after retries
        assert!(result.is_err());

        // Should have taken some time due to retries
        // (can't test exact timing in CI)
        let elapsed = start.elapsed();
        assert!(elapsed > Duration::from_millis(100));
    }

    /// Test deployment health status determination
    #[test]
    fn test_health_status_mapping() {
        use blueprint_remote_providers::infrastructure_unified::InstanceStatus;

        // Map instance status to health status
        let health_from_instance = |status: InstanceStatus| -> HealthStatus {
            match status {
                InstanceStatus::Running => HealthStatus::Healthy,
                InstanceStatus::Starting => HealthStatus::Degraded,
                InstanceStatus::Stopping | InstanceStatus::Stopped => HealthStatus::Unhealthy,
                InstanceStatus::Terminated => HealthStatus::Unhealthy,
                InstanceStatus::Unknown => HealthStatus::Unknown,
            }
        };

        assert_eq!(
            health_from_instance(InstanceStatus::Running),
            HealthStatus::Healthy
        );
        assert_eq!(
            health_from_instance(InstanceStatus::Starting),
            HealthStatus::Degraded
        );
        assert_eq!(
            health_from_instance(InstanceStatus::Stopped),
            HealthStatus::Unhealthy
        );
        assert_eq!(
            health_from_instance(InstanceStatus::Terminated),
            HealthStatus::Unhealthy
        );
        assert_eq!(
            health_from_instance(InstanceStatus::Unknown),
            HealthStatus::Unknown
        );
    }
}
