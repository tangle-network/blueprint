use super::*;
use crate::types::{ContainerImage, DeploymentSpec, PortMapping, Protocol, ResourceLimits};

#[tokio::test]
#[ignore] // Requires a running Kubernetes cluster
async fn test_kubernetes_provider_lifecycle() {
    let config = KubernetesConfig {
        namespace: "blueprint-test".to_string(),
        ..Default::default()
    };
    
    let provider = KubernetesProvider::new("test-k8s", config).await.unwrap();
    
    let mut spec = DeploymentSpec::default();
    spec.name = "test-deployment".to_string();
    spec.image = ContainerImage {
        repository: "nginx".to_string(),
        tag: "alpine".to_string(),
        pull_policy: crate::types::PullPolicy::IfNotPresent,
    };
    spec.resources = ResourceLimits {
        cpu: Some("100m".to_string()),
        memory: Some("128Mi".to_string()),
        storage: None,
    };
    spec.ports = vec![PortMapping {
        name: "http".to_string(),
        container_port: 80,
        host_port: None,
        protocol: Protocol::TCP,
    }];
    
    // Deploy instance
    let instance = provider.deploy_instance(spec.clone()).await.unwrap();
    assert!(!instance.id.as_str().is_empty());
    assert_eq!(instance.name, "test-deployment");
    
    // Wait for deployment to be ready
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Check status
    let status = provider.get_instance_status(&instance.id).await.unwrap();
    assert!(matches!(status, InstanceStatus::Running | InstanceStatus::Pending));
    
    // Get endpoint
    let endpoint = provider.get_instance_endpoint(&instance.id).await.unwrap();
    assert!(endpoint.is_some());
    
    // List instances
    let instances = provider.list_instances().await.unwrap();
    assert!(instances.iter().any(|i| i.id == instance.id));
    
    // Scale instance
    provider.scale_instance(&instance.id, 2).await.unwrap();
    
    // Estimate costs
    let cost = provider.estimate_cost(&spec).await.unwrap();
    assert!(cost.estimated_hourly > 0.0);
    
    // Cleanup
    provider.terminate_instance(&instance.id).await.unwrap();
}

#[tokio::test]
async fn test_kubernetes_config_defaults() {
    let config = KubernetesConfig::default();
    assert_eq!(config.namespace, "blueprint-remote");
    assert!(matches!(config.service_type, ServiceType::ClusterIP));
    assert!(config.kubeconfig_path.is_none());
    assert!(config.context.is_none());
}

#[tokio::test]
async fn test_cost_estimation() {
    let config = KubernetesConfig::default();
    
    // This test doesn't require a real cluster
    let provider = KubernetesProvider {
        name: "test".to_string(),
        config,
        client: Client::try_from(Config::new("http://localhost:8080".parse().unwrap())).unwrap(),
    };
    
    let mut spec = DeploymentSpec::default();
    spec.resources = ResourceLimits {
        cpu: Some("2".to_string()),
        memory: Some("4Gi".to_string()),
        storage: None,
    };
    spec.replicas = 3;
    
    let cost = provider.estimate_cost(&spec).await.unwrap();
    
    // 2 CPUs * $0.0464/hour + 4GB * $0.004/hour = $0.1088/hour per replica
    // 3 replicas = $0.3264/hour
    assert!((cost.estimated_hourly - 0.3264).abs() < 0.01);
    
    // Monthly = hourly * 730
    assert!((cost.estimated_monthly - 238.272).abs() < 1.0);
    
    assert_eq!(cost.currency, "USD");
    assert!(cost.breakdown.contains_key("compute"));
    assert!(cost.breakdown.contains_key("memory"));
}