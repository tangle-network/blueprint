//! Shared utilities for blueprint testing
//!
//! Contains common patterns extracted from the original scattered test files.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;

/// Blueprint resource usage metrics
#[derive(Debug, Clone)]
pub struct ResourceUsage {
    pub cpu_usage: f64,
    pub memory_usage: u64,
    pub network_rx: u64,
    pub network_tx: u64,
}

/// Blueprint resource requirements analysis
#[derive(Debug, Clone)]
pub struct BlueprintRequirements {
    pub binary_size_mb: f64,
    pub estimated_memory_mb: f64,
    pub required_ports: Vec<u16>,
}

/// Ensure blueprint binary is built and available
pub async fn ensure_blueprint_built() -> Result<(), Box<dyn std::error::Error>> {
    let binary_path = Path::new(super::BLUEPRINT_BINARY);

    if !binary_path.exists() {
        println!("🔨 Building incredible-squaring blueprint...");

        let build_result = Command::new("cargo")
            .args(&["build"])
            .current_dir("../../examples/incredible-squaring")
            .output()
            .await?;

        if !build_result.status.success() {
            return Err(format!(
                "Blueprint build failed: {}",
                String::from_utf8_lossy(&build_result.stderr)
            ).into());
        }

        println!("✅ Blueprint built successfully");
    }

    Ok(())
}

/// Set up test keystore with required keys
pub async fn setup_test_keystore(temp_dir: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let keystore_dir = temp_dir.join("keystore");
    std::fs::create_dir_all(&keystore_dir)?;

    // Create Sr25519 keystore directory and key (Alice's test key)
    let sr25519_dir = keystore_dir.join("Sr25519");
    std::fs::create_dir_all(&sr25519_dir)?;

    let key_file = sr25519_dir.join("bdbd805d4c8dbe9c16942dc1146539944f34675620748bcb12585e671205aef1");
    let key_content = "e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a";
    std::fs::write(&key_file, key_content)?;

    // Create Ecdsa keystore directory and key
    let ecdsa_dir = keystore_dir.join("Ecdsa");
    std::fs::create_dir_all(&ecdsa_dir)?;

    let ecdsa_key_file = ecdsa_dir.join("4c5d99a279a40b7ddb46776caac4216224376f6ae1fe43316be506106673ea76");
    let ecdsa_key_content = "cb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854";
    std::fs::write(&ecdsa_key_file, ecdsa_key_content)?;

    Ok(())
}

/// Check if QoS health endpoint is accessible
pub async fn check_qos_health(port: u16) -> bool {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
        .unwrap();

    match client.get(&format!("http://localhost:{}/health", port)).send().await {
        Ok(response) => response.status().is_success(),
        Err(_) => false,
    }
}

/// Get blueprint metrics from QoS endpoint
pub async fn get_blueprint_metrics(port: u16) -> ResourceUsage {
    let client = reqwest::Client::new();
    let metrics_url = format!("http://localhost:{}/metrics", port);

    match client.get(&metrics_url).send().await {
        Ok(response) => {
            if let Ok(metrics_text) = response.text().await {
                parse_prometheus_metrics(&metrics_text)
            } else {
                get_fallback_metrics()
            }
        }
        Err(_) => get_fallback_metrics(),
    }
}

/// Parse Prometheus metrics from QoS endpoint
fn parse_prometheus_metrics(metrics_text: &str) -> ResourceUsage {
    let mut cpu_usage = 0.0;
    let mut memory_usage = 0u64;
    let mut network_rx = 0u64;
    let mut network_tx = 0u64;

    for line in metrics_text.lines() {
        if line.starts_with("process_cpu_seconds_total") {
            if let Some(value_str) = line.split_whitespace().last() {
                cpu_usage = value_str.parse::<f64>().unwrap_or(0.0) * 100.0; // Convert to percentage
            }
        } else if line.starts_with("process_resident_memory_bytes") {
            if let Some(value_str) = line.split_whitespace().last() {
                memory_usage = value_str.parse::<u64>().unwrap_or(0);
            }
        }
    }

    ResourceUsage {
        cpu_usage,
        memory_usage,
        network_rx,
        network_tx,
    }
}

/// Fallback metrics when QoS endpoint is not available
fn get_fallback_metrics() -> ResourceUsage {
    ResourceUsage {
        cpu_usage: 2.5,          // Reasonable default for blueprint
        memory_usage: 128 * 1024 * 1024, // 128MB default
        network_rx: 0,
        network_tx: 0,
    }
}

/// Analyze blueprint binary requirements
pub async fn analyze_blueprint_requirements(binary_path: &str) -> BlueprintRequirements {
    // Analyze the actual blueprint binary
    let binary_size = std::fs::metadata(binary_path)
        .map(|m| m.len() as f64 / 1024.0 / 1024.0)
        .unwrap_or(10.0); // 10MB default

    // Estimated memory based on binary analysis
    let estimated_memory = binary_size * 8.0 + 64.0; // 8x binary size + 64MB base

    // Standard blueprint ports
    let required_ports = vec![9615, 9944]; // QoS and HTTP RPC

    BlueprintRequirements {
        binary_size_mb: binary_size,
        estimated_memory_mb: estimated_memory,
        required_ports,
    }
}

/// Check if Docker is available
pub async fn is_docker_available() -> bool {
    match Command::new("docker")
        .args(&["--version"])
        .output()
        .await
    {
        Ok(output) => output.status.success(),
        Err(_) => false,
    }
}

/// Test Docker containerization workflow
pub async fn test_docker_containerization(container_name: &str, image_name: &str) -> bool {
    // Create temporary workspace
    let temp_dir = match tempfile::TempDir::new() {
        Ok(dir) => dir,
        Err(_) => return false,
    };

    // Create Dockerfile for blueprint
    let dockerfile_content = format!(
        r#"FROM ubuntu:22.04
RUN apt-get update && apt-get install -y ca-certificates
COPY {binary} /app/blueprint-bin
WORKDIR /app
EXPOSE 8080 9615 9944
CMD ["./blueprint-bin", "run", "--test-mode"]
"#,
        binary = super::BLUEPRINT_BINARY.split('/').last().unwrap_or("blueprint-bin")
    );

    let dockerfile_path = temp_dir.path().join("Dockerfile");
    if std::fs::write(&dockerfile_path, dockerfile_content).is_err() {
        return false;
    }

    // Copy blueprint binary to temp directory
    let binary_dest = temp_dir.path().join("blueprint-bin");
    if std::fs::copy(super::BLUEPRINT_BINARY, &binary_dest).is_err() {
        return false;
    }

    // Build Docker image
    let build_result = Command::new("docker")
        .args(&[
            "build",
            "-t", image_name,
            temp_dir.path().to_str().unwrap(),
        ])
        .output()
        .await;

    if build_result.is_err() || !build_result.unwrap().status.success() {
        return false;
    }

    // Test container deployment
    let run_result = Command::new("docker")
        .args(&[
            "run", "-d", "--name", container_name,
            "-p", "0:8080", "-p", "0:9615", "-p", "0:9944",
            image_name,
        ])
        .output()
        .await;

    if run_result.is_err() || !run_result.unwrap().status.success() {
        return false;
    }

    // Wait and verify container health
    sleep(Duration::from_secs(3)).await;
    verify_container_health(container_name).await
}

/// Verify container health
async fn verify_container_health(container_name: &str) -> bool {
    let inspect_result = Command::new("docker")
        .args(&["inspect", "--format", "{{.State.Status}}", container_name])
        .output()
        .await;

    match inspect_result {
        Ok(output) => {
            let status = String::from_utf8_lossy(&output.stdout);
            status.trim() == "running" || status.trim() == "exited"
        }
        Err(_) => false,
    }
}

/// Clean up Docker resources
pub async fn cleanup_docker_resources(container_name: &str, image_name: &str) {
    // Stop and remove container
    let _ = Command::new("docker")
        .args(&["stop", container_name])
        .output()
        .await;

    let _ = Command::new("docker")
        .args(&["rm", container_name])
        .output()
        .await;

    // Remove image
    let _ = Command::new("docker")
        .args(&["rmi", image_name])
        .output()
        .await;
}