use super::*;
use crate::types::{ContainerImage, DeploymentSpec, PortMapping, ResourceLimits};

#[tokio::test]
async fn test_ssh_config_defaults() {
    let config = SshConfig::default();
    assert_eq!(config.user, "root");
    assert_eq!(config.port, 22);
    assert!(matches!(config.runtime, SshRuntime::Docker));
    assert!(config.hosts.is_empty());
}

#[tokio::test]
async fn test_cost_estimation_minimal() {
    let config = SshConfig {
        hosts: vec!["test.example.com".to_string()],
        ..Default::default()
    };
    
    // Create provider without actual SSH validation
    let provider = SshProvider {
        name: "test-ssh".to_string(),
        config,
        instances: RwLock::new(HashMap::new()),
        next_host_index: RwLock::new(0),
    };
    
    let mut spec = DeploymentSpec::default();
    spec.resources = ResourceLimits {
        cpu: Some("2".to_string()),
        memory: Some("4Gi".to_string()),
        storage: None,
    };
    spec.replicas = 1;
    
    let cost = provider.estimate_cost(&spec).await.unwrap();
    
    // SSH costs should be minimal
    assert!(cost.estimated_hourly < 0.01);
    assert!(cost.estimated_monthly < 10.0);
    assert_eq!(cost.currency, "USD");
    assert_eq!(cost.breakdown.get("network"), Some(&0.0));
}

#[tokio::test]
async fn test_host_rotation() {
    let config = SshConfig {
        hosts: vec![
            "host1.example.com".to_string(),
            "host2.example.com".to_string(),
            "host3.example.com".to_string(),
        ],
        ..Default::default()
    };
    
    let provider = SshProvider {
        name: "test-ssh".to_string(),
        config,
        instances: RwLock::new(HashMap::new()),
        next_host_index: RwLock::new(0),
    };
    
    // Get hosts in sequence
    let host1 = provider.get_next_host().await.unwrap();
    let host2 = provider.get_next_host().await.unwrap();
    let host3 = provider.get_next_host().await.unwrap();
    let host4 = provider.get_next_host().await.unwrap(); // Should wrap around
    
    assert_eq!(host1, "host1.example.com");
    assert_eq!(host2, "host2.example.com");
    assert_eq!(host3, "host3.example.com");
    assert_eq!(host4, "host1.example.com");
}

#[tokio::test]
async fn test_no_hosts_error() {
    let config = SshConfig::default(); // No hosts configured
    
    let provider = SshProvider {
        name: "test-ssh".to_string(),
        config,
        instances: RwLock::new(HashMap::new()),
        next_host_index: RwLock::new(0),
    };
    
    let result = provider.get_next_host().await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::ConfigurationError(_)));
}

#[tokio::test]
async fn test_instance_tracking() {
    let config = SshConfig {
        hosts: vec!["test.example.com".to_string()],
        ..Default::default()
    };
    
    let provider = SshProvider {
        name: "test-ssh".to_string(),
        config,
        instances: RwLock::new(HashMap::new()),
        next_host_index: RwLock::new(0),
    };
    
    // Add instance
    let instance_id = InstanceId::new("test-123");
    let ssh_instance = SshInstance {
        id: instance_id.clone(),
        name: "test-app".to_string(),
        host: "test.example.com".to_string(),
        process_id: Some("docker-abc".to_string()),
        port: Some(8080),
        status: InstanceStatus::Running,
    };
    
    provider.instances.write().await.insert(instance_id.clone(), ssh_instance);
    
    // Get endpoint
    let endpoint = provider.get_instance_endpoint(&instance_id).await.unwrap();
    assert!(endpoint.is_some());
    
    let endpoint = endpoint.unwrap();
    assert_eq!(endpoint.host, "test.example.com");
    assert_eq!(endpoint.port, 8080);
    assert!(!endpoint.tunnel_required);
    
    // List instances
    let instances = provider.list_instances().await.unwrap();
    assert_eq!(instances.len(), 1);
    assert_eq!(instances[0].id, instance_id);
    
    // Terminate
    provider.terminate_instance(&instance_id).await.unwrap();
    let instances = provider.list_instances().await.unwrap();
    assert_eq!(instances.len(), 0);
}

#[tokio::test]
async fn test_scale_limitation() {
    let config = SshConfig::default();
    
    let provider = SshProvider {
        name: "test-ssh".to_string(),
        config,
        instances: RwLock::new(HashMap::new()),
        next_host_index: RwLock::new(0),
    };
    
    let instance_id = InstanceId::new("test");
    
    // Scaling to 1 should work
    let result = provider.scale_instance(&instance_id, 1).await;
    assert!(result.is_ok());
    
    // Scaling beyond 1 should fail
    let result = provider.scale_instance(&instance_id, 2).await;
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), Error::ConfigurationError(_)));
}