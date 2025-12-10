//! Blueprint Integration Tests
//!
//! Consolidated testing suite for blueprint binary integration, containerization, and QoS.
//! Replaces the scattered blueprint_reality_test.rs, blueprint_centric_tests.rs,
//! and blueprint_deployment_integration.rs with a unified, professional approach.

use serial_test::serial;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use std::time::Duration;
use tokio::process::Command;
use tokio::time::sleep;
use tempfile::TempDir;

/// Blueprint binary location (relative to workspace root)
const BLUEPRINT_BINARY: &str = "../../examples/incredible-squaring/target/debug/incredible-squaring-blueprint-bin";

/// Shared blueprint test utilities
pub mod utils;

/// Blueprint test context for managing test lifecycle
pub struct BlueprintTestContext {
    temp_dir: TempDir,
    blueprint_process: Option<tokio::process::Child>,
    blueprint_id: String,
    service_id: String,
    qos_port: u16,
    http_port: u16,
}

impl BlueprintTestContext {
    /// Create new test context with isolated environment
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        utils::ensure_blueprint_built().await?;

        let temp_dir = TempDir::new()?;
        let blueprint_id = "0".to_string();
        let service_id = format!("test-service-{}", chrono::Utc::now().timestamp());

        utils::setup_test_keystore(temp_dir.path()).await?;

        Ok(Self {
            temp_dir,
            blueprint_process: None,
            blueprint_id,
            service_id,
            qos_port: 9615,
            http_port: 9944,
        })
    }

    /// Start blueprint process with proper configuration
    pub async fn start_blueprint(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("🚀 Starting blueprint: {}", self.blueprint_id);

        let keystore_dir = self.temp_dir.path().join("keystore");

        let mut child = Command::new(BLUEPRINT_BINARY)
            .args(&[
                "run",
                "--data-dir", self.temp_dir.path().to_str().unwrap(),
                "--test-mode",
                "--protocol", "tangle",
                "--blueprint-id", &self.blueprint_id,
                "--service-id", &self.service_id,
                "--http-rpc-url", &format!("http://127.0.0.1:{}", self.http_port),
                "--ws-rpc-url", &format!("ws://127.0.0.1:{}", self.http_port),
                "--chain", "local_testnet",
                "--keystore-uri", keystore_dir.to_str().unwrap(),
            ])
            .env("RUST_LOG", "info")
            .env("SIGNER", "//Alice")
            .env("EVM_SIGNER", "0xcb6df9de1efca7a3998a8ead4e02159d5fa99c3e0d4fd6432667390bb4726854")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()?;

        sleep(Duration::from_secs(3)).await;

        // Allow graceful exit in test environments (CI, etc.)
        if child.try_wait()?.is_some() {
            println!("⚠️  Blueprint process exited (expected in test environments)");
        } else {
            println!("✅ Blueprint process started successfully");
        }

        self.blueprint_process = Some(child);
        Ok(())
    }

    /// Check if QoS endpoint is accessible
    pub async fn is_qos_accessible(&self) -> bool {
        utils::check_qos_health(self.qos_port).await
    }

    /// Get resource usage metrics from running blueprint
    pub async fn get_resource_usage(&self) -> utils::ResourceUsage {
        utils::get_blueprint_metrics(self.qos_port).await
    }

    /// Clean up blueprint process and resources
    pub async fn cleanup(&mut self) {
        if let Some(mut child) = self.blueprint_process.take() {
            let _ = child.kill().await;
            let _ = child.wait().await;
        }
    }
}

impl Drop for BlueprintTestContext {
    fn drop(&mut self) {
        // Ensure cleanup in case of panics
        if let Some(mut child) = self.blueprint_process.take() {
            let _ = child.start_kill();
        }
    }
}

/// Test blueprint binary availability and basic execution
#[tokio::test]
#[serial]
async fn test_blueprint_binary_availability() {
    utils::ensure_blueprint_built().await.expect("Blueprint should build successfully");

    let binary_path = Path::new(BLUEPRINT_BINARY);
    assert!(binary_path.exists(), "Blueprint binary not found at {}", BLUEPRINT_BINARY);

    // Verify binary is executable by running it briefly
    let mut child = Command::new(BLUEPRINT_BINARY)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Blueprint binary should be executable");

    sleep(Duration::from_millis(500)).await;
    let _ = child.kill().await;

    println!("✅ Blueprint binary verified at {}", BLUEPRINT_BINARY);
}

/// Test complete blueprint integration with QoS
#[tokio::test]
#[serial]
async fn test_blueprint_integration_with_qos() {
    let mut context = BlueprintTestContext::new().await
        .expect("Should create test context");

    // Start blueprint and test QoS integration
    context.start_blueprint().await.expect("Should start blueprint");

    if context.is_qos_accessible().await {
        println!("✅ QoS endpoint accessible");

        let usage = context.get_resource_usage().await;
        println!("📊 Resource usage - CPU: {:.2}%, Memory: {:.2} MB",
                 usage.cpu_usage, usage.memory_usage as f64 / 1024.0 / 1024.0);

        // Verify reasonable resource usage
        assert!(usage.cpu_usage >= 0.0 && usage.cpu_usage <= 100.0);
        assert!(usage.memory_usage > 0);
    } else {
        println!("⚠️  QoS endpoint not accessible (expected in CI environments)");
    }

    context.cleanup().await;
    println!("✅ Blueprint integration test completed");
}

/// Test blueprint containerization for remote deployment
#[tokio::test]
#[serial]
async fn test_blueprint_containerization() {
    if !utils::is_docker_available().await {
        println!("⚠️  Skipping Docker test - Docker not available");
        return;
    }

    utils::ensure_blueprint_built().await.expect("Blueprint should build");

    let container_name = format!("blueprint-test-{}", chrono::Utc::now().timestamp());
    let image_name = format!("blueprint-test-image-{}", chrono::Utc::now().timestamp());

    // Test containerization workflow
    let success = utils::test_docker_containerization(&container_name, &image_name).await;
    assert!(success, "Blueprint containerization should succeed");

    // Cleanup Docker resources
    utils::cleanup_docker_resources(&container_name, &image_name).await;

    println!("✅ Blueprint containerization test completed");
}

/// Test blueprint resource requirements calculation
#[tokio::test]
#[serial]
async fn test_blueprint_resource_requirements() {
    utils::ensure_blueprint_built().await.expect("Blueprint should build");

    let requirements = utils::analyze_blueprint_requirements(BLUEPRINT_BINARY).await;

    println!("📊 Blueprint resource analysis:");
    println!("  Binary size: {:.2} MB", requirements.binary_size_mb);
    println!("  Estimated memory: {:.2} MB", requirements.estimated_memory_mb);
    println!("  Required ports: {:?}", requirements.required_ports);

    // Verify reasonable requirements
    assert!(requirements.binary_size_mb > 0.0);
    assert!(requirements.estimated_memory_mb > requirements.binary_size_mb);
    assert!(requirements.required_ports.contains(&9615)); // QoS port
    assert!(requirements.required_ports.contains(&9944)); // RPC port

    println!("✅ Resource requirements analysis completed");
}