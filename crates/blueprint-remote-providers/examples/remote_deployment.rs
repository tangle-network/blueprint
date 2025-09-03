// Example: Deploying Blueprint instances to remote clouds

use blueprint_remote_providers::{
    docker::{DockerConfig, DockerProvider},
    kubernetes::{KubernetesConfig, KubernetesProvider},
    provider::{ProviderRegistry, RemoteInfrastructureProvider},
    types::{ContainerImage, DeploymentSpec, PortMapping, Protocol, ResourceLimits},
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the provider registry
    let registry = ProviderRegistry::new();
    
    // Example 1: Local Docker deployment
    println!("Setting up Docker provider...");
    let docker_config = DockerConfig::default();
    let docker_provider = Arc::new(DockerProvider::new("local-docker", docker_config).await?);
    registry.register("docker", docker_provider.clone()).await;
    
    // Example 2: Remote Kubernetes deployment (requires kubeconfig)
    if std::env::var("KUBECONFIG").is_ok() {
        println!("Setting up Kubernetes provider...");
        let k8s_config = KubernetesConfig {
            namespace: "blueprint-demo".to_string(),
            ..Default::default()
        };
        let k8s_provider = Arc::new(KubernetesProvider::new("k8s-cluster", k8s_config).await?);
        registry.register("kubernetes", k8s_provider.clone()).await;
    }
    
    // Create a deployment specification
    let mut spec = DeploymentSpec::default();
    spec.name = "blueprint-example".to_string();
    spec.image = ContainerImage {
        repository: "nginx".to_string(),
        tag: "alpine".to_string(),
        pull_policy: blueprint_remote_providers::types::PullPolicy::IfNotPresent,
    };
    spec.resources = ResourceLimits {
        cpu: Some("500m".to_string()),
        memory: Some("256Mi".to_string()),
        storage: None,
    };
    spec.ports = vec![PortMapping {
        name: "http".to_string(),
        container_port: 80,
        host_port: Some(8080),
        protocol: Protocol::TCP,
    }];
    spec.environment.insert("BLUEPRINT_ENV".to_string(), "production".to_string());
    
    // Deploy to available providers
    println!("\nAvailable providers:");
    for provider_name in registry.list().await {
        println!("  - {}", provider_name);
    }
    
    // Deploy to Docker if available
    if let Some(docker) = registry.get("docker").await {
        println!("\n=== Deploying to Docker ===");
        
        // Estimate costs
        let cost = docker.estimate_cost(&spec).await?;
        println!("Estimated cost: ${:.2}/hour, ${:.2}/month", 
            cost.estimated_hourly, cost.estimated_monthly);
        
        // Deploy the instance
        let instance = docker.deploy_instance(spec.clone()).await?;
        println!("Deployed instance: {}", instance.id);
        println!("Status: {}", instance.status);
        
        if let Some(endpoint) = &instance.endpoint {
            println!("Endpoint: {}:{}", endpoint.host, endpoint.port);
        }
        
        // Check resources
        let resources = docker.get_available_resources().await?;
        println!("Available resources:");
        println!("  CPU: {}", resources.available_cpu);
        println!("  Memory: {}", resources.available_memory);
        println!("  Current instances: {}", resources.current_instances);
        
        // Wait a bit
        println!("\nInstance running... (press Ctrl+C to terminate)");
        tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        
        // Cleanup
        println!("Terminating instance...");
        docker.terminate_instance(&instance.id).await?;
        println!("Instance terminated");
    }
    
    // Deploy to Kubernetes if available
    if let Some(k8s) = registry.get("kubernetes").await {
        println!("\n=== Deploying to Kubernetes ===");
        
        // Deploy the instance
        let instance = k8s.deploy_instance(spec.clone()).await?;
        println!("Deployed deployment: {}", instance.id);
        println!("Status: {}", instance.status);
        
        // List all instances
        let instances = k8s.list_instances().await?;
        println!("Total deployments in namespace: {}", instances.len());
        
        // Scale the deployment
        println!("Scaling to 3 replicas...");
        k8s.scale_instance(&instance.id, 3).await?;
        
        // Wait a bit
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        
        // Check status
        let status = k8s.get_instance_status(&instance.id).await?;
        println!("Updated status: {}", status);
        
        // Cleanup
        println!("Terminating deployment...");
        k8s.terminate_instance(&instance.id).await?;
        println!("Deployment terminated");
    }
    
    println!("\n✅ Remote deployment example completed!");
    
    Ok(())
}

// Example output:
// Setting up Docker provider...
// 
// Available providers:
//   - docker
// 
// === Deploying to Docker ===
// Estimated cost: $0.01/hour, $7.30/month
// Deployed instance: abc123def456
// Status: Running
// Endpoint: 127.0.0.1:8080
// Available resources:
//   CPU: 8
//   Memory: 16Gi
//   Current instances: 1
// 
// Instance running... (press Ctrl+C to terminate)
// Terminating instance...
// Instance terminated
// 
// ✅ Remote deployment example completed!