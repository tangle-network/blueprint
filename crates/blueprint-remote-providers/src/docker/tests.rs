use super::*;
use crate::types::{ContainerImage, DeploymentSpec, PortMapping, Protocol, ResourceLimits};

#[tokio::test]
#[ignore] // Requires Docker daemon
async fn test_docker_provider_lifecycle() {
    let config = DockerConfig::default();
    let provider = DockerProvider::new("test-docker", config).await.unwrap();
    
    let mut spec = DeploymentSpec::default();
    spec.name = format!("test-container-{}", chrono::Utc::now().timestamp());
    spec.image = ContainerImage {
        repository: "alpine".to_string(),
        tag: "latest".to_string(),
        pull_policy: crate::types::PullPolicy::IfNotPresent,
    };
    spec.resources = ResourceLimits {
        cpu: Some("0.5".to_string()),
        memory: Some("128Mi".to_string()),
        storage: None,
    };
    spec.environment.insert("TEST_ENV".to_string(), "test_value".to_string());
    spec.ports = vec![PortMapping {
        name: "http".to_string(),
        container_port: 8080,
        host_port: None,
        protocol: Protocol::TCP,
    }];
    
    // Deploy instance
    let instance = provider.deploy_instance(spec.clone()).await.unwrap();
    assert!(!instance.id.as_str().is_empty());
    assert_eq!(instance.name, spec.name);
    
    // Check status
    let status = provider.get_instance_status(&instance.id).await.unwrap();
    assert!(matches!(status, InstanceStatus::Running));
    
    // List instances
    let instances = provider.list_instances().await.unwrap();
    assert!(instances.iter().any(|i| i.id == instance.id));
    
    // Get resources
    let resources = provider.get_available_resources().await.unwrap();
    assert!(resources.current_instances > 0);
    
    // Estimate costs
    let cost = provider.estimate_cost(&spec).await.unwrap();
    assert!(cost.estimated_hourly > 0.0);
    
    // Scale to 0 (stop)
    provider.scale_instance(&instance.id, 0).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    
    let status = provider.get_instance_status(&instance.id).await.unwrap();
    assert!(matches!(status, InstanceStatus::Stopped | InstanceStatus::Unknown));
    
    // Scale back to 1 (start)
    provider.scale_instance(&instance.id, 1).await.unwrap();
    
    // Cleanup
    provider.terminate_instance(&instance.id).await.unwrap();
    
    // Verify termination
    let instances = provider.list_instances().await.unwrap();
    assert!(!instances.iter().any(|i| i.id == instance.id));
}

#[tokio::test]
async fn test_docker_config_defaults() {
    let config = DockerConfig::default();
    assert_eq!(config.network, Some("bridge".to_string()));
    assert!(config.endpoint.is_none());
    assert!(config.tls_cert_path.is_none());
}

#[tokio::test]
async fn test_memory_parsing() {
    assert_eq!(parse_memory_string("1Gi"), Some(1024 * 1024 * 1024));
    assert_eq!(parse_memory_string("512Mi"), Some(512 * 1024 * 1024));
    assert_eq!(parse_memory_string("1024Ki"), Some(1024 * 1024));
    assert_eq!(parse_memory_string("1073741824"), Some(1073741824));
    assert_eq!(parse_memory_string("invalid"), None);
}

#[tokio::test]
#[ignore] // Requires Docker daemon
async fn test_docker_provider_with_port_mapping() {
    let config = DockerConfig::default();
    let provider = DockerProvider::new("test-docker", config).await.unwrap();
    
    let mut spec = DeploymentSpec::default();
    spec.name = format!("test-port-{}", chrono::Utc::now().timestamp());
    spec.image = ContainerImage {
        repository: "nginx".to_string(),
        tag: "alpine".to_string(),
        pull_policy: crate::types::PullPolicy::IfNotPresent,
    };
    spec.ports = vec![PortMapping {
        name: "http".to_string(),
        container_port: 80,
        host_port: Some(8888),
        protocol: Protocol::TCP,
    }];
    
    let instance = provider.deploy_instance(spec).await.unwrap();
    
    // Check endpoint
    let endpoint = provider.get_instance_endpoint(&instance.id).await.unwrap();
    assert!(endpoint.is_some());
    
    let endpoint = endpoint.unwrap();
    assert_eq!(endpoint.port, 8888);
    assert!(!endpoint.tunnel_required);
    
    // Cleanup
    provider.terminate_instance(&instance.id).await.unwrap();
}