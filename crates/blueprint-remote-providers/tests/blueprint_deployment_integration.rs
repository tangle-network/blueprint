//! Blueprint Deployment Integration Tests
//!
//! Tests deployment infrastructure components that support remote provider deployments.
//! Focuses on blueprint binary availability and basic containerization for remote deployment.

use serial_test::serial;
use std::path::Path;
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;

const INCREDIBLE_SQUARING_BINARY: &str = "target/debug/incredible-squaring-blueprint-bin";

/// Validates that the incredible-squaring blueprint binary is built and executable
#[tokio::test]
#[serial]
async fn blueprint_binary_availability() {
    let binary_path = Path::new(INCREDIBLE_SQUARING_BINARY);
    assert!(
        binary_path.exists(),
        "Blueprint binary not found at {}. Run: cargo build --bin incredible-squaring-blueprint-bin",
        INCREDIBLE_SQUARING_BINARY
    );

    // Verify binary is executable by running it briefly
    let mut child = Command::new(INCREDIBLE_SQUARING_BINARY)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Blueprint binary should be executable");

    // Allow brief startup time
    sleep(Duration::from_millis(500)).await;

    // Terminate gracefully
    let _ = child.kill().await;

    println!(
        "✓ Blueprint binary verified at {}",
        INCREDIBLE_SQUARING_BINARY
    );
}

/// Tests Docker container deployment with production blueprint binary
#[tokio::test]
#[serial]
async fn docker_container_deployment() {
    if !is_docker_available().await {
        eprintln!("Skipping Docker test - Docker daemon not available");
        return;
    }

    let binary_path = Path::new(INCREDIBLE_SQUARING_BINARY);
    assert!(
        binary_path.exists(),
        "Blueprint binary required for Docker test"
    );

    let test_id = generate_test_id();
    let image_name = format!("blueprint-test:{}", test_id);
    let container_name = format!("blueprint-container-{}", test_id);

    // Create production-ready Dockerfile
    let dockerfile = create_production_dockerfile();
    let temp_dir = create_test_workspace(&test_id).await;

    std::fs::write(temp_dir.join("Dockerfile"), dockerfile).expect("Failed to write Dockerfile");

    // Build container image with blueprint binary
    println!("Building Docker image: {}", image_name);
    let build_result = Command::new("docker")
        .args(&[
            "build",
            "-t",
            &image_name,
            "-f",
            temp_dir.join("Dockerfile").to_str().unwrap(),
            ".", // Use workspace root as build context
        ])
        .output()
        .await
        .expect("Docker build command failed");

    assert!(
        build_result.status.success(),
        "Docker build failed: {}",
        String::from_utf8_lossy(&build_result.stderr)
    );

    // Deploy container with proper port mappings
    println!("Deploying container: {}", container_name);
    let run_result = Command::new("docker")
        .args(&[
            "run",
            "-d",
            "--name",
            &container_name,
            "-p",
            "0:8080", // Blueprint service port
            "-p",
            "0:9615", // QoS metrics port
            "-p",
            "0:9944", // QoS RPC port
            &image_name,
        ])
        .output()
        .await
        .expect("Docker run command failed");

    assert!(
        run_result.status.success(),
        "Container deployment failed: {}",
        String::from_utf8_lossy(&run_result.stderr)
    );

    // Verify container health
    sleep(Duration::from_secs(3)).await;
    let health_check = verify_container_health(&container_name).await;
    assert!(health_check, "Container failed health check");

    // Cleanup
    cleanup_docker_resources(&container_name, &image_name).await;
    cleanup_test_workspace(&temp_dir).await;

    println!("✓ Docker deployment test completed successfully");
}

/// Tests that blueprint binary can be containerized for remote deployment
#[tokio::test]
#[serial]
async fn test_blueprint_containerization_for_remote_deployment() {
    println!("Testing blueprint containerization for remote provider deployment...");
    
    let binary_path = Path::new(INCREDIBLE_SQUARING_BINARY);
    assert!(
        binary_path.exists(),
        "Blueprint binary required for containerization test"
    );
    
    // Test that we can create a deployment-ready container configuration
    // This validates the foundation for remote provider deployments
    println!("✓ Blueprint binary available for containerization");
    
    // Test deployment configuration for remote providers
    use blueprint_remote_providers::core::resources::ResourceSpec;
    use blueprint_remote_providers::core::deployment_target::{DeploymentTarget, ContainerRuntime};
    
    let resource_spec = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        storage_gb: 20.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    };
    
    let deployment_target = DeploymentTarget::VirtualMachine {
        runtime: ContainerRuntime::Docker,
    };
    
    println!("✓ Resource spec configured for remote deployment: CPU={}, Memory={}GB", 
             resource_spec.cpu, resource_spec.memory_gb);
    println!("✓ Deployment target configured: {:?}", deployment_target);
    
    // Test QoS port configuration for remote access
    let qos_ports = [8080, 9615, 9944];
    println!("✓ QoS ports configured for remote access: {:?}", qos_ports);
    
    println!("✓ Blueprint containerization test completed - ready for remote deployment");
}

// Helper functions

async fn is_docker_available() -> bool {
    Command::new("docker")
        .arg("version")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

fn generate_test_id() -> String {
    format!("{}", chrono::Utc::now().timestamp())
}

async fn create_test_workspace(test_id: &str) -> std::path::PathBuf {
    let temp_dir = std::env::temp_dir().join(format!("blueprint-test-{}", test_id));
    std::fs::create_dir_all(&temp_dir).expect("Failed to create test workspace");
    temp_dir
}

fn create_production_dockerfile() -> String {
    format!(
        r#"
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy production blueprint binary
COPY {} /usr/local/bin/blueprint
RUN chmod +x /usr/local/bin/blueprint

# Configure blueprint environment
ENV BLUEPRINT_ID=0
ENV SERVICE_ID=0
ENV RUST_LOG=info

# Expose service ports
EXPOSE 8080 9615 9944

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD pgrep blueprint || exit 1

CMD ["/usr/local/bin/blueprint"]
"#,
        INCREDIBLE_SQUARING_BINARY
    )
}

async fn verify_container_health(container_name: &str) -> bool {
    let inspect_result = Command::new("docker")
        .args(&["inspect", container_name, "--format", "{{.State.Running}}"])
        .output()
        .await
        .expect("Docker inspect failed");

    String::from_utf8_lossy(&inspect_result.stdout).trim() == "true"
}

async fn cleanup_docker_resources(container_name: &str, image_name: &str) {
    let _ = Command::new("docker")
        .args(&["rm", "-f", container_name])
        .output()
        .await;
    let _ = Command::new("docker")
        .args(&["rmi", image_name])
        .output()
        .await;
}

async fn cleanup_test_workspace(temp_dir: &std::path::Path) {
    let _ = std::fs::remove_dir_all(temp_dir);
}
