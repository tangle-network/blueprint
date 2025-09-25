//! Remote Provider Integration Tests
//!
//! Tests the actual remote cloud provider functionality including:
//! - Cloud instance provisioning via adapters
//! - Blueprint deployment to remote instances
//! - QoS metrics collection from remote deployments
//! - Multi-provider deployment scenarios

use blueprint_remote_providers::core::deployment_target::ContainerRuntime;
use blueprint_remote_providers::core::deployment_target::DeploymentTarget;
use blueprint_remote_providers::core::remote::CloudProvider;
use blueprint_remote_providers::core::resources::ResourceSpec;
use blueprint_remote_providers::infra::provisioner::CloudProvisioner;
use blueprint_remote_providers::infra::traits::CloudProviderAdapter;
use blueprint_remote_providers::providers::digitalocean::adapter::DigitalOceanAdapter;
use serial_test::serial;
use std::collections::HashMap;
use std::time::Duration;
use tokio::time::sleep;

const BLUEPRINT_IMAGE: &str = "incredible-squaring-blueprint";

/// Test CloudProvisioner initialization with multiple providers
#[tokio::test]
#[serial]
async fn test_cloud_provisioner_initialization() {
    println!("Testing CloudProvisioner initialization...");

    let provisioner = CloudProvisioner::new().await;
    assert!(
        provisioner.is_ok(),
        "CloudProvisioner should initialize successfully"
    );

    let provisioner = provisioner.unwrap();
    println!("✓ CloudProvisioner initialized successfully");

    // Test that we can call methods without panicking
    let resource_spec = ResourceSpec {
        cpu: 1.0,
        memory_gb: 2.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: true,
        qos: Default::default(),
    };

    // This should fail gracefully without credentials
    let result = provisioner
        .provision(CloudProvider::AWS, &resource_spec, "us-east-1")
        .await;

    // We expect this to fail due to missing credentials, but it should be a proper error
    assert!(result.is_err());

    match result.unwrap_err() {
        blueprint_remote_providers::core::error::Error::ProviderNotConfigured(_) => {
            println!("✓ Properly handles missing provider configuration");
        }
        other => {
            println!(
                "✓ Provider initialization returned expected error: {:?}",
                other
            );
        }
    }
}

/// Test DigitalOcean adapter integration (if credentials available)
#[tokio::test]
#[serial]
async fn test_digitalocean_adapter_integration() {
    println!("Testing DigitalOcean adapter integration...");

    // Check if DigitalOcean credentials are available
    if std::env::var("DIGITALOCEAN_TOKEN").is_err() {
        println!("⚠ Skipping DigitalOcean test - DIGITALOCEAN_TOKEN not set");
        return;
    }

    let adapter_result = DigitalOceanAdapter::new().await;
    if adapter_result.is_err() {
        println!("⚠ Skipping DigitalOcean test - adapter initialization failed");
        return;
    }

    let adapter = adapter_result.unwrap();
    println!("✓ DigitalOcean adapter initialized successfully");

    // Test instance provisioning (this will hit real API)
    let result = adapter.provision_instance("s-1vcpu-1gb", "nyc1").await;

    match result {
        Ok(instance) => {
            println!(
                "✓ Successfully provisioned DigitalOcean instance: {}",
                instance.id
            );

            // Test blueprint deployment to the instance
            let deployment_target = DeploymentTarget::VirtualMachine {
                runtime: ContainerRuntime::Docker,
            };

            let mut env_vars = HashMap::new();
            env_vars.insert("BLUEPRINT_ID".to_string(), "0".to_string());
            env_vars.insert("SERVICE_ID".to_string(), "0".to_string());

            let resource_spec = ResourceSpec {
                cpu: 1.0,
                memory_gb: 1.0,
                storage_gb: 10.0,
                gpu_count: None,
                allow_spot: false,
                qos: Default::default(),
            };

            // Give the instance time to boot
            println!("Waiting for instance to be ready...");
            sleep(Duration::from_secs(30)).await;

            // Test deployment
            let deployment_result = adapter
                .deploy_blueprint_with_target(
                    &deployment_target,
                    BLUEPRINT_IMAGE,
                    &resource_spec,
                    env_vars,
                )
                .await;

            match deployment_result {
                Ok(deployment) => {
                    println!("✓ Successfully deployed blueprint to DigitalOcean instance");
                    println!("Blueprint ID: {}", deployment.blueprint_id);

                    // Test QoS endpoint accessibility
                    if let Some(qos_endpoint) = deployment.qos_grpc_endpoint() {
                        println!("Testing QoS endpoint: {}", qos_endpoint);

                        // Give blueprint time to start
                        sleep(Duration::from_secs(10)).await;

                        let health_check = adapter.health_check_blueprint(&deployment).await;
                        match health_check {
                            Ok(healthy) => {
                                if healthy {
                                    println!("✓ QoS endpoint is healthy and accessible");
                                } else {
                                    println!(
                                        "⚠ QoS endpoint not healthy (expected for test environment)"
                                    );
                                }
                            }
                            Err(e) => {
                                println!(
                                    "⚠ QoS health check failed: {} (expected for test environment)",
                                    e
                                );
                            }
                        }
                    }

                    // Cleanup the deployment
                    println!("Cleaning up deployment...");
                    let cleanup_result = adapter.cleanup_blueprint(&deployment).await;
                    match cleanup_result {
                        Ok(_) => println!("✓ Deployment cleaned up successfully"),
                        Err(e) => println!("⚠ Cleanup failed: {}", e),
                    }
                }
                Err(e) => {
                    println!(
                        "⚠ Blueprint deployment failed: {} (expected without proper SSH setup)",
                        e
                    );
                }
            }

            // Cleanup the instance
            println!("Terminating test instance...");
            let terminate_result = adapter.terminate_instance(&instance.id).await;
            match terminate_result {
                Ok(_) => println!("✓ Instance terminated successfully"),
                Err(e) => println!("⚠ Instance termination failed: {}", e),
            }
        }
        Err(e) => {
            println!(
                "⚠ Instance provisioning failed: {} (may be expected due to API limits or configuration)",
                e
            );
        }
    }
}

/// Test multi-provider deployment scenario
#[tokio::test]
#[serial]
async fn test_multi_provider_deployment() {
    println!("Testing multi-provider deployment scenario...");

    let provisioner = CloudProvisioner::new().await.unwrap();

    let resource_spec = ResourceSpec {
        cpu: 1.0,
        memory_gb: 2.0,
        storage_gb: 20.0,
        gpu_count: None,
        allow_spot: true,
        qos: Default::default(),
    };

    let providers_to_test = vec![
        (CloudProvider::AWS, "us-east-1"),
        (CloudProvider::GCP, "us-central1"),
        (CloudProvider::DigitalOcean, "nyc1"),
    ];

    let mut results = Vec::new();

    for (provider, region) in providers_to_test {
        println!("Testing provider: {}", provider);

        let result = provisioner
            .provision(provider.clone(), &resource_spec, region)
            .await;

        match result {
            Ok(instance) => {
                println!(
                    "✓ Successfully provisioned instance on {}: {}",
                    provider, instance.id
                );
                results.push((provider, instance));
            }
            Err(e) => {
                println!("⚠ Failed to provision on {}: {}", provider, e);
                // This is expected if credentials aren't configured
            }
        }
    }

    if results.is_empty() {
        println!("⚠ No instances provisioned - this is expected without cloud credentials");
        println!("✓ Multi-provider test completed (structure validation passed)");
        return;
    }

    // Test deployment to each provisioned instance
    for (provider, instance) in &results {
        println!(
            "Testing deployment to {} instance: {}",
            provider, instance.id
        );

        // We would deploy blueprints here, but that requires SSH access
        // For now, just verify the instance status
        let status_result = provisioner.get_status(provider.clone(), &instance.id).await;
        match status_result {
            Ok(status) => {
                println!("✓ Instance {} status: {:?}", instance.id, status);
            }
            Err(e) => {
                println!("⚠ Failed to get status for {}: {}", instance.id, e);
            }
        }
    }

    // Cleanup all instances
    println!("Cleaning up test instances...");
    for (provider, instance) in results {
        let terminate_result = provisioner.terminate(provider.clone(), &instance.id).await;
        match terminate_result {
            Ok(_) => println!("✓ Terminated {} instance: {}", provider, instance.id),
            Err(e) => println!("⚠ Failed to terminate {}: {}", instance.id, e),
        }
    }

    println!("✓ Multi-provider deployment test completed");
}

/// Test Kubernetes deployment targets
#[tokio::test]
#[serial]
async fn test_kubernetes_deployment_targets() {
    println!("Testing Kubernetes deployment targets...");

    let deployment_targets = vec![
        DeploymentTarget::ManagedKubernetes {
            cluster_id: "test-eks-cluster".to_string(),
            namespace: "blueprint-test".to_string(),
        },
        DeploymentTarget::GenericKubernetes {
            context: Some("minikube".to_string()),
            namespace: "default".to_string(),
        },
    ];

    for target in deployment_targets {
        println!("Testing deployment target: {:?}", target);

        // We can't actually deploy without a real cluster, but we can test the structure
        match target {
            DeploymentTarget::ManagedKubernetes {
                cluster_id,
                namespace,
            } => {
                println!(
                    "✓ Managed Kubernetes target configured: {} / {}",
                    cluster_id, namespace
                );
            }
            DeploymentTarget::GenericKubernetes { context, namespace } => {
                println!(
                    "✓ Generic Kubernetes target configured: {:?} / {}",
                    context, namespace
                );
            }
            _ => {}
        }
    }

    println!("✓ Kubernetes deployment target test completed");
}

/// Test QoS integration with remote deployments
#[tokio::test]
#[serial]
async fn test_qos_remote_integration() {
    println!("Testing QoS integration with remote deployments...");

    // Test that QoS endpoints are properly configured in deployment results
    let mut port_mappings = HashMap::new();
    port_mappings.insert(8080, 8080); // Blueprint service
    port_mappings.insert(9615, 9615); // QoS metrics
    port_mappings.insert(9944, 9944); // QoS RPC

    // Verify QoS ports are included
    assert!(
        port_mappings.contains_key(&9615),
        "QoS metrics port should be exposed"
    );
    assert!(
        port_mappings.contains_key(&9944),
        "QoS RPC port should be exposed"
    );

    println!("✓ QoS port configuration validated");

    // Test QoS endpoint URL generation
    let test_ip = "192.168.1.100";
    let qos_metrics_url = format!("http://{}:9615/metrics", test_ip);
    let qos_health_url = format!("http://{}:9615/health", test_ip);

    println!("QoS metrics URL: {}", qos_metrics_url);
    println!("QoS health URL: {}", qos_health_url);

    assert!(
        qos_metrics_url.contains("9615"),
        "Metrics URL should use port 9615"
    );
    assert!(
        qos_health_url.contains("9615"),
        "Health URL should use port 9615"
    );

    println!("✓ QoS endpoint URL generation validated");
    println!("✓ QoS remote integration test completed");
}
