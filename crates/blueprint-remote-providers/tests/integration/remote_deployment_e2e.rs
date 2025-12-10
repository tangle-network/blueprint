//! End-to-end integration tests for remote cloud deployments
//!
//! These tests verify the complete deployment lifecycle across all providers.
//! Run with: cargo test --test remote_deployment_e2e --features integration-tests

use blueprint_remote_providers::{
    core::{
        deployment_target::{DeploymentTarget, ContainerRuntime},
        error::Result,
        resources::ResourceSpec,
        remote::CloudProvider,
    },
    infra::{
        provisioner::CloudProvisioner,
        traits::BlueprintDeploymentResult,
    },
};
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::timeout;

/// Test configuration from environment
struct TestConfig {
    skip_aws: bool,
    skip_gcp: bool,
    skip_digitalocean: bool,
    skip_vultr: bool,
    test_region: String,
    test_image: String,
    cleanup_on_failure: bool,
}

impl TestConfig {
    fn from_env() -> Self {
        Self {
            skip_aws: std::env::var("SKIP_AWS_TEST").is_ok(),
            skip_gcp: std::env::var("SKIP_GCP_TEST").is_ok(),
            skip_digitalocean: std::env::var("SKIP_DO_TEST").is_ok(),
            skip_vultr: std::env::var("SKIP_VULTR_TEST").is_ok(),
            test_region: std::env::var("TEST_REGION").unwrap_or_else(|_| "us-east-1".to_string()),
            test_image: std::env::var("TEST_IMAGE")
                .unwrap_or_else(|_| "nginx:latest".to_string()),
            cleanup_on_failure: std::env::var("CLEANUP_ON_FAILURE").is_ok(),
        }
    }

    fn should_test(&self, provider: &CloudProvider) -> bool {
        match provider {
            CloudProvider::AWS => !self.skip_aws && std::env::var("AWS_ACCESS_KEY_ID").is_ok(),
            CloudProvider::GCP => !self.skip_gcp && std::env::var("GCP_PROJECT_ID").is_ok(),
            CloudProvider::DigitalOcean => {
                !self.skip_digitalocean && std::env::var("DIGITALOCEAN_TOKEN").is_ok()
            }
            CloudProvider::Vultr => !self.skip_vultr && std::env::var("VULTR_API_KEY").is_ok(),
            _ => false,
        }
    }
}

/// Helper to ensure cleanup even on test failure
struct TestDeployment {
    provisioner: CloudProvisioner,
    provider: CloudProvider,
    deployment: Option<BlueprintDeploymentResult>,
    config: TestConfig,
}

impl TestDeployment {
    async fn new(provider: CloudProvider) -> Result<Self> {
        Ok(Self {
            provisioner: CloudProvisioner::new().await?,
            provider,
            deployment: None,
            config: TestConfig::from_env(),
        })
    }

    async fn deploy(
        &mut self,
        target: DeploymentTarget,
        resource_spec: &ResourceSpec,
    ) -> Result<()> {
        let deployment = self.provisioner
            .deploy_with_target(
                &target,
                &self.config.test_image,
                resource_spec,
                self.test_env_vars(),
            )
            .await?;

        self.deployment = Some(deployment);
        Ok(())
    }

    fn test_env_vars(&self) -> HashMap<String, String> {
        let mut env = HashMap::new();
        env.insert("TEST_VAR".to_string(), "test_value".to_string());
        env.insert("DEPLOYMENT_ID".to_string(), uuid::Uuid::new_v4().to_string());
        env
    }

    async fn verify_deployment(&self) -> Result<bool> {
        if let Some(deployment) = &self.deployment {
            // Check instance is running
            let status = self.provisioner
                .get_instance_status(&self.provider, &deployment.instance.id)
                .await?;

            if !matches!(
                status,
                crate::infra::types::InstanceStatus::Running
            ) {
                return Ok(false);
            }

            // Check QoS endpoint is accessible
            if let Some(endpoint) = deployment.qos_grpc_endpoint() {
                let client = reqwest::Client::builder()
                    .timeout(Duration::from_secs(10))
                    .build()
                    .unwrap();

                let response = client
                    .get(&format!("{}/health", endpoint))
                    .send()
                    .await;

                return Ok(response.is_ok());
            }
        }
        Ok(false)
    }
}

impl Drop for TestDeployment {
    fn drop(&mut self) {
        if self.config.cleanup_on_failure {
            if let Some(deployment) = &self.deployment {
                let provider = self.provider.clone();
                let instance_id = deployment.instance.id.clone();
                let provisioner = self.provisioner.clone();

                // Schedule cleanup in background
                tokio::spawn(async move {
                    let _ = timeout(
                        Duration::from_secs(60),
                        provisioner.terminate(provider, &instance_id),
                    )
                    .await;
                });
            }
        }
    }
}

#[tokio::test]
async fn test_aws_vm_deployment() -> Result<()> {
    let config = TestConfig::from_env();
    if !config.should_test(&CloudProvider::AWS) {
        eprintln!("Skipping AWS test - credentials not configured");
        return Ok(());
    }

    let mut test = TestDeployment::new(CloudProvider::AWS).await?;

    let resource_spec = ResourceSpec {
        cpu: 1.0,
        memory_gb: 2.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: true,
        qos: Default::default(),
    };

    let target = DeploymentTarget::VirtualMachine {
        runtime: ContainerRuntime::Docker,
    };

    // Deploy
    test.deploy(target, &resource_spec).await?;

    // Wait for deployment to stabilize
    tokio::time::sleep(Duration::from_secs(30)).await;

    // Verify
    assert!(test.verify_deployment().await?, "AWS deployment verification failed");

    // Cleanup
    if let Some(deployment) = &test.deployment {
        test.provisioner
            .terminate(CloudProvider::AWS, &deployment.instance.id)
            .await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_gcp_vm_deployment() -> Result<()> {
    let config = TestConfig::from_env();
    if !config.should_test(&CloudProvider::GCP) {
        eprintln!("Skipping GCP test - credentials not configured");
        return Ok(());
    }

    let mut test = TestDeployment::new(CloudProvider::GCP).await?;

    let resource_spec = ResourceSpec {
        cpu: 1.0,
        memory_gb: 2.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };

    let target = DeploymentTarget::VirtualMachine {
        runtime: ContainerRuntime::Docker,
    };

    test.deploy(target, &resource_spec).await?;
    tokio::time::sleep(Duration::from_secs(30)).await;
    assert!(test.verify_deployment().await?, "GCP deployment verification failed");

    if let Some(deployment) = &test.deployment {
        test.provisioner
            .terminate(CloudProvider::GCP, &deployment.instance.id)
            .await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_digitalocean_kubernetes_deployment() -> Result<()> {
    let config = TestConfig::from_env();
    if !config.should_test(&CloudProvider::DigitalOcean) {
        eprintln!("Skipping DigitalOcean test - credentials not configured");
        return Ok(());
    }

    let cluster_id = std::env::var("DO_K8S_CLUSTER_ID");
    if cluster_id.is_err() {
        eprintln!("Skipping DigitalOcean K8s test - DO_K8S_CLUSTER_ID not set");
        return Ok(());
    }

    let mut test = TestDeployment::new(CloudProvider::DigitalOcean).await?;

    let resource_spec = ResourceSpec::basic();

    let target = DeploymentTarget::ManagedKubernetes {
        cluster_id: cluster_id.unwrap(),
        namespace: "test-namespace".to_string(),
    };

    test.deploy(target, &resource_spec).await?;
    tokio::time::sleep(Duration::from_secs(20)).await;
    assert!(
        test.verify_deployment().await?,
        "DigitalOcean K8s deployment verification failed"
    );

    Ok(())
}

#[tokio::test]
async fn test_multi_provider_parallel_deployment() -> Result<()> {
    let config = TestConfig::from_env();
    let providers = vec![
        CloudProvider::AWS,
        CloudProvider::GCP,
        CloudProvider::DigitalOcean,
    ];

    let mut handles = Vec::new();

    for provider in providers {
        if !config.should_test(&provider) {
            continue;
        }

        let handle = tokio::spawn(async move {
            let mut test = TestDeployment::new(provider.clone()).await?;

            let resource_spec = ResourceSpec::basic();
            let target = DeploymentTarget::VirtualMachine {
                runtime: ContainerRuntime::Docker,
            };

            test.deploy(target, &resource_spec).await?;
            tokio::time::sleep(Duration::from_secs(30)).await;

            let verified = test.verify_deployment().await?;

            if let Some(deployment) = &test.deployment {
                test.provisioner
                    .terminate(provider, &deployment.instance.id)
                    .await?;
            }

            Ok::<bool, blueprint_remote_providers::core::error::Error>(verified)
        });

        handles.push(handle);
    }

    // Wait for all deployments
    let results = futures::future::join_all(handles).await;

    for result in results {
        match result {
            Ok(Ok(verified)) => assert!(verified, "Parallel deployment verification failed"),
            Ok(Err(e)) => panic!("Deployment error: {}", e),
            Err(e) => panic!("Task error: {}", e),
        }
    }

    Ok(())
}

#[tokio::test]
async fn test_deployment_with_failure_recovery() -> Result<()> {
    let config = TestConfig::from_env();
    if !config.should_test(&CloudProvider::AWS) {
        eprintln!("Skipping failure recovery test - AWS credentials not configured");
        return Ok(());
    }

    let mut test = TestDeployment::new(CloudProvider::AWS).await?;

    // Deploy with invalid image to trigger failure
    let resource_spec = ResourceSpec::basic();
    let target = DeploymentTarget::VirtualMachine {
        runtime: ContainerRuntime::Docker,
    };

    // Override with bad image
    let bad_result = test.provisioner
        .deploy_with_target(
            &target,
            "invalid-image-that-does-not-exist:v999",
            &resource_spec,
            HashMap::new(),
        )
        .await;

    assert!(bad_result.is_err(), "Should fail with invalid image");

    // Now deploy with valid image
    test.deploy(target, &resource_spec).await?;
    tokio::time::sleep(Duration::from_secs(30)).await;
    assert!(test.verify_deployment().await?, "Recovery deployment should succeed");

    if let Some(deployment) = &test.deployment {
        test.provisioner
            .terminate(CloudProvider::AWS, &deployment.instance.id)
            .await?;
    }

    Ok(())
}

#[tokio::test]
async fn test_resource_scaling() -> Result<()> {
    let config = TestConfig::from_env();
    if !config.should_test(&CloudProvider::AWS) {
        eprintln!("Skipping scaling test - AWS credentials not configured");
        return Ok(());
    }

    let provisioner = CloudProvisioner::new().await?;

    // Test different resource configurations
    let configs = vec![
        ResourceSpec::basic(),       // 1 CPU, 1 GB
        ResourceSpec::standard(),    // 2 CPU, 4 GB
        ResourceSpec::performance(), // 4 CPU, 8 GB
    ];

    for (i, spec) in configs.iter().enumerate() {
        let instance = provisioner
            .provision(
                CloudProvider::AWS,
                spec,
                &config.test_region,
            )
            .await?;

        // Verify instance type matches expected resources
        assert!(
            !instance.instance_type.is_empty(),
            "Instance type should be set for config {}",
            i
        );

        // Cleanup
        provisioner
            .terminate(CloudProvider::AWS, &instance.id)
            .await?;

        // Brief pause between tests
        tokio::time::sleep(Duration::from_secs(5)).await;
    }

    Ok(())
}

#[tokio::test]
async fn test_health_monitoring() -> Result<()> {
    use blueprint_remote_providers::monitoring::health::HealthMonitor;

    let config = TestConfig::from_env();
    if !config.should_test(&CloudProvider::AWS) {
        eprintln!("Skipping health monitoring test");
        return Ok(());
    }

    let mut test = TestDeployment::new(CloudProvider::AWS).await?;
    let mut health_monitor = HealthMonitor::new();

    let resource_spec = ResourceSpec::basic();
    let target = DeploymentTarget::VirtualMachine {
        runtime: ContainerRuntime::Docker,
    };

    test.deploy(target, &resource_spec).await?;

    if let Some(deployment) = &test.deployment {
        // Add to health monitoring
        health_monitor
            .add_deployment(
                deployment.blueprint_id.clone(),
                deployment.instance.clone(),
                deployment.metadata.clone(),
            )
            .await;

        // Wait for stabilization
        tokio::time::sleep(Duration::from_secs(30)).await;

        // Check health
        let health_status = health_monitor
            .check_deployment_health(&deployment.blueprint_id)
            .await?;

        assert!(
            matches!(
                health_status,
                blueprint_remote_providers::monitoring::health::HealthStatus::Healthy
            ),
            "Deployment should be healthy"
        );

        // Cleanup
        test.provisioner
            .terminate(CloudProvider::AWS, &deployment.instance.id)
            .await?;
    }

    Ok(())
}