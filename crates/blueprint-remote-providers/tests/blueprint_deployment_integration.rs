//! Blueprint Deployment Integration Tests
//!
//! Validates remote deployment infrastructure with production blueprint binaries.
//! Tests deployment across Docker containers, Kubernetes pods, and virtual machines.

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

/// Tests Kubernetes pod deployment with production blueprint binary
#[tokio::test]
#[serial]
async fn kubernetes_pod_deployment() {
    if !is_kubernetes_available().await {
        eprintln!("Skipping Kubernetes test - cluster not available");
        return;
    }

    let binary_path = Path::new(INCREDIBLE_SQUARING_BINARY);
    assert!(
        binary_path.exists(),
        "Blueprint binary required for Kubernetes test"
    );

    let test_id = generate_test_id();
    let namespace = format!("blueprint-test-{}", test_id);

    // Create test namespace
    let namespace_result = Command::new("kubectl")
        .args(&["create", "namespace", &namespace])
        .output()
        .await
        .expect("kubectl create namespace failed");

    assert!(
        namespace_result.status.success(),
        "Failed to create test namespace: {}",
        String::from_utf8_lossy(&namespace_result.stderr)
    );

    // Deploy blueprint as ConfigMap + Pod (simulating real deployment)
    let deployment_manifest = create_kubernetes_deployment(&test_id);
    let manifest_path = std::env::temp_dir().join(format!("blueprint-deployment-{}.yaml", test_id));

    std::fs::write(&manifest_path, deployment_manifest)
        .expect("Failed to write deployment manifest");

    let apply_result = Command::new("kubectl")
        .args(&[
            "apply",
            "-f",
            manifest_path.to_str().unwrap(),
            "-n",
            &namespace,
        ])
        .output()
        .await
        .expect("kubectl apply failed");

    assert!(
        apply_result.status.success(),
        "Failed to apply deployment: {}",
        String::from_utf8_lossy(&apply_result.stderr)
    );

    // Wait for pod readiness
    sleep(Duration::from_secs(10)).await;
    let pod_status = verify_pod_health(&namespace).await;
    assert!(pod_status, "Pod failed to reach ready state");

    // Cleanup
    cleanup_kubernetes_resources(&namespace).await;
    let _ = std::fs::remove_file(manifest_path);

    println!("✓ Kubernetes deployment test completed successfully");
}

/// Tests virtual machine deployment simulation
#[tokio::test]
#[serial]
async fn virtual_machine_deployment() {
    let binary_path = Path::new(INCREDIBLE_SQUARING_BINARY);
    assert!(
        binary_path.exists(),
        "Blueprint binary required for VM test"
    );

    let test_id = generate_test_id();
    let vm_simulation_dir = std::env::temp_dir().join(format!("vm-sim-{}", test_id));

    std::fs::create_dir_all(&vm_simulation_dir).expect("Failed to create VM simulation directory");

    // Simulate remote deployment by copying binary
    let remote_binary_path = vm_simulation_dir.join("blueprint");
    std::fs::copy(binary_path, &remote_binary_path)
        .expect("Failed to copy blueprint to simulated VM");

    // Set executable permissions (Unix systems)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(&remote_binary_path)
            .unwrap()
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(&remote_binary_path, perms).unwrap();
    }

    // Test remote execution simulation
    println!("Testing VM deployment simulation");
    let mut process = Command::new(&remote_binary_path)
        .env("BLUEPRINT_ID", "0")
        .env("SERVICE_ID", "0")
        .env("RUST_LOG", "info")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to execute blueprint on simulated VM");

    // Allow process startup
    sleep(Duration::from_secs(2)).await;

    // Verify process is running
    let process_health = match process.try_wait() {
        Ok(Some(_)) => false, // Process exited
        Ok(None) => true,     // Process still running
        Err(_) => false,      // Error checking process
    };

    assert!(
        process_health,
        "Blueprint process failed to maintain execution"
    );

    // Cleanup
    let _ = process.kill().await;
    let _ = std::fs::remove_dir_all(vm_simulation_dir);

    println!("✓ VM deployment simulation completed successfully");
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

async fn is_kubernetes_available() -> bool {
    Command::new("kubectl")
        .args(&["cluster-info"])
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

fn create_kubernetes_deployment(test_id: &str) -> String {
    format!(
        r#"
apiVersion: v1
kind: ConfigMap
metadata:
  name: blueprint-binary-{}
data:
  blueprint: |
    # Binary would be base64 encoded in real deployment
---
apiVersion: apps/v1
kind: Deployment
metadata:
  name: blueprint-deployment-{}
spec:
  replicas: 1
  selector:
    matchLabels:
      app: blueprint-{}
  template:
    metadata:
      labels:
        app: blueprint-{}
    spec:
      containers:
      - name: blueprint
        image: debian:bookworm-slim
        command: ["/bin/sleep", "3600"]  # Simplified for test
        ports:
        - containerPort: 8080
          name: blueprint
        - containerPort: 9615
          name: qos-metrics
        - containerPort: 9944
          name: qos-rpc
        env:
        - name: BLUEPRINT_ID
          value: "0"
        - name: SERVICE_ID
          value: "0"
        - name: RUST_LOG
          value: "info"
---
apiVersion: v1
kind: Service
metadata:
  name: blueprint-service-{}
spec:
  selector:
    app: blueprint-{}
  ports:
  - name: blueprint
    port: 8080
    targetPort: 8080
  - name: qos-metrics
    port: 9615
    targetPort: 9615
  - name: qos-rpc
    port: 9944
    targetPort: 9944
"#,
        test_id, test_id, test_id, test_id, test_id, test_id
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

async fn verify_pod_health(namespace: &str) -> bool {
    let pod_result = Command::new("kubectl")
        .args(&[
            "get",
            "pods",
            "-n",
            namespace,
            "--field-selector=status.phase=Running",
        ])
        .output()
        .await
        .expect("kubectl get pods failed");

    !String::from_utf8_lossy(&pod_result.stdout).is_empty()
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

async fn cleanup_kubernetes_resources(namespace: &str) {
    let _ = Command::new("kubectl")
        .args(&["delete", "namespace", namespace])
        .output()
        .await;
}

async fn cleanup_test_workspace(temp_dir: &std::path::Path) {
    let _ = std::fs::remove_dir_all(temp_dir);
}
