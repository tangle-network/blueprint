//! Remote deployment integration with Blueprint Manager

use blueprint_remote_providers::{
    deployment::tracker::DeploymentType,
    deployment::manager_integration::{RemoteDeploymentConfig, RemoteDeploymentExtensions},
    core::remote::CloudProvider,
    core::resources::ResourceSpec,
};
use std::path::PathBuf;
use std::sync::Arc;

/// Example modification to the Blueprint Manager's event handler
/// This shows where to add hooks for remote deployments
async fn enhanced_event_handler(
    // ... existing parameters ...
    remote_extensions: Arc<RemoteDeploymentExtensions>,
    blueprint_id: u64,
    service_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Example: When a ServiceInitiated event is received
    // Check if this should be a remote deployment based on configuration

    let should_deploy_remote = check_deployment_requirements(&blueprint_id);

    if should_deploy_remote {
        // Create remote deployment configuration
        let config = RemoteDeploymentConfig {
            deployment_type: DeploymentType::AwsEc2,
            provider: Some(CloudProvider::AWS),
            region: Some("us-east-1".to_string()),
            instance_id: format!("blueprint-{}-service-{}", blueprint_id, service_id),
            resource_spec: ResourceSpec::recommended(),
            ttl_seconds: Some(3600), // 1 hour TTL
            deployed_at: chrono::Utc::now(),
        };

        // Register the remote deployment
        remote_extensions
            .event_handler
            .on_service_initiated(blueprint_id, service_id, Some(config))
            .await?;

        println!(
            "Registered remote deployment for blueprint {} service {}",
            blueprint_id, service_id
        );
    }

    Ok(())
}

/// Example modification to service cleanup logic
async fn enhanced_service_cleanup(
    remote_extensions: Arc<RemoteDeploymentExtensions>,
    blueprint_id: u64,
    service_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(
        "Cleaning up service {} from blueprint {}",
        service_id, blueprint_id
    );

    // Call the remote cleanup hook
    // This will handle remote deployment termination if applicable
    remote_extensions
        .on_service_removed(blueprint_id, service_id)
        .await?;

    // ... continue with existing cleanup logic ...

    Ok(())
}

/// Example Blueprint Manager initialization with remote extensions
async fn initialize_enhanced_blueprint_manager(
    state_dir: PathBuf,
) -> Result<Arc<RemoteDeploymentExtensions>, Box<dyn std::error::Error>> {
    // Create a mock provisioner for this example
    use blueprint_remote_providers::infrastructure::InfrastructureProvisioner;

    struct MockProvisioner;

    #[async_trait::async_trait]
    impl InfrastructureProvisioner for MockProvisioner {
        async fn provision(
            &self,
            _resource_spec: &ResourceSpec,
        ) -> blueprint_remote_providers::core::error::Result<String> {
            Ok("mock-instance-id".to_string())
        }

        async fn terminate(
            &self,
            instance_id: &str,
        ) -> blueprint_remote_providers::core::error::Result<()> {
            println!("Mock terminating instance: {}", instance_id);
            Ok(())
        }

        async fn get_status(
            &self,
            _instance_id: &str,
        ) -> blueprint_remote_providers::core::error::Result<
            blueprint_remote_providers::core::infrastructure::InstanceStatus,
        > {
            Ok(blueprint_remote_providers::core::infrastructure::InstanceStatus::Running)
        }
    }

    let provisioner = Arc::new(MockProvisioner) as Arc<dyn InfrastructureProvisioner>;

    // Initialize remote deployment extensions
    let remote_extensions = RemoteDeploymentExtensions::initialize(
        &state_dir,
        true, // Enable TTL management
        provisioner,
    )
    .await?;

    println!("Initialized remote deployment extensions");

    Ok(Arc::new(remote_extensions))
}

/// Example of using remote deployment for a specific source type
async fn spawn_remote_service(
    remote_extensions: Arc<RemoteDeploymentExtensions>,
    blueprint_id: u64,
    service_id: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Use the source extension to spawn a remote deployment
    let config = remote_extensions
        .source_extension
        .spawn_remote(
            blueprint_id,
            service_id,
            ResourceSpec::performance(),
            CloudProvider::AWS,
            "us-west-2".to_string(),
            Some(7200), // 2 hour TTL
        )
        .await?;

    println!("Spawned remote deployment: {:?}", config);

    Ok(())
}

fn check_deployment_requirements(_blueprint_id: &u64) -> bool {
    // Logic to determine if this blueprint requires remote deployment
    // This could check blueprint metadata, resource requirements, etc.
    true // For example purposes
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    // tracing_subscriber::fmt::init(); // Optional: add tracing-subscriber to Cargo.toml

    let temp_dir = tempfile::TempDir::new()?;
    let state_dir = temp_dir.path().to_path_buf();

    // Initialize enhanced Blueprint Manager with remote extensions
    let remote_extensions = initialize_enhanced_blueprint_manager(state_dir).await?;

    // Example: Handle a service initialization event
    enhanced_event_handler(
        remote_extensions.clone(),
        100, // blueprint_id
        1,   // service_id
    )
    .await?;

    // Example: Spawn a remote service directly
    spawn_remote_service(
        remote_extensions.clone(),
        200, // blueprint_id
        2,   // service_id
    )
    .await?;

    // Simulate some time passing
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Example: Clean up a service
    enhanced_service_cleanup(
        remote_extensions.clone(),
        100, // blueprint_id
        1,   // service_id
    )
    .await?;

    println!("Example completed successfully");

    Ok(())
}
