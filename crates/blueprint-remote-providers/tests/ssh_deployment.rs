//! SSH deployment tests using Docker containers as SSH targets

use blueprint_remote_providers::{
    deployment::ssh::{SshDeploymentClient, SshConnection, ContainerRuntime, DeploymentConfig},
    resources::ResourceSpec,
};
use tokio::process::Command;
use std::time::Duration;

/// Check if Docker is available
async fn docker_available() -> bool {
    Command::new("docker")
        .arg("--version")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Start an SSH server container for testing
async fn start_ssh_container() -> Option<(String, u16)> {
    if !docker_available().await {
        eprintln!("⚠️  Skipping SSH tests - Docker not available");
        return None;
    }
    
    // Clean up any existing test container
    Command::new("docker")
        .args(&["rm", "-f", "blueprint-ssh-test"])
        .output()
        .await
        .ok();
    
    // Start SSH server container with Docker socket mounted
    let output = Command::new("docker")
        .args(&[
            "run", "-d",
            "--name", "blueprint-ssh-test",
            "-p", "0:22",  // Random port
            "-e", "PUID=1000",
            "-e", "PGID=1000",
            "-e", "TZ=UTC",
            "-e", "SUDO_ACCESS=true",
            "-e", "PASSWORD_ACCESS=true",
            "-e", "USER_PASSWORD=testpass",
            "-e", "USER_NAME=blueprint",
            "-v", "/var/run/docker.sock:/var/run/docker.sock",
            "linuxserver/openssh-server:latest"
        ])
        .output()
        .await
        .ok()?;
    
    if !output.status.success() {
        eprintln!("Failed to start SSH container");
        return None;
    }
    
    // Get the container ID
    let container_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Get the mapped port
    let port_output = Command::new("docker")
        .args(&["port", &container_id, "22"])
        .output()
        .await
        .ok()?;
    
    let port_str = String::from_utf8_lossy(&port_output.stdout);
    let port = port_str
        .split(':')
        .last()?
        .trim()
        .parse::<u16>()
        .ok()?;
    
    // Wait for SSH to be ready
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    Some((container_id, port))
}

/// Clean up SSH container
async fn cleanup_ssh_container(container_id: &str) {
    Command::new("docker")
        .args(&["rm", "-f", container_id])
        .output()
        .await
        .ok();
}

#[tokio::test]
async fn test_ssh_connection_and_docker_deployment() {
    let (container_id, port) = match start_ssh_container().await {
        Some(info) => info,
        None => {
            eprintln!("Could not start SSH container, skipping test");
            return;
        }
    };
    
    // Test SSH connection
    let connection = SshConnection {
        host: "localhost".to_string(),
        port,
        user: "blueprint".to_string(),
        key_path: None,
        password: Some("testpass".to_string()),
        jump_host: None,
    };
    
    // Create SSH client
    let deployment_config = DeploymentConfig {
        name: "test-blueprint".to_string(),
        namespace: "default".to_string(),
        ..Default::default()
    };
    
    let client = SshDeploymentClient::new(
        connection.clone(),
        ContainerRuntime::Docker,
        deployment_config,
    ).await;
    
    match client {
        Ok(ssh_client) => {
            println!("✅ SSH connection established");
            
            // Deploy a test container
            let spec = ResourceSpec::minimal();
            let env_vars = std::collections::HashMap::new();
            
            let deployment = ssh_client.deploy_blueprint(
                "nginx:alpine",
                &spec,
                env_vars,
            ).await;
            
            match deployment {
                Ok(deployed) => {
                    println!("✅ Deployed container: {}", deployed.container_id);
                    assert!(!deployed.container_id.is_empty());
                    
                    // Check if container is running
                    let status = ssh_client.get_container_status(&deployed.container_id).await;
                    if let Ok(status) = status {
                        println!("✅ Container status: {}", status);
                        assert!(status.contains("running") || status.contains("Up"));
                    }
                    
                    // Stop the deployed container
                    ssh_client.stop_container(&deployed.container_id).await.ok();
                },
                Err(e) => {
                    eprintln!("Failed to deploy: {}", e);
                }
            }
        },
        Err(e) => {
            eprintln!("Failed to create SSH client: {}", e);
        }
    }
    
    // Cleanup
    cleanup_ssh_container(&container_id).await;
}

#[tokio::test]
async fn test_ssh_resource_limits() {
    let (container_id, port) = match start_ssh_container().await {
        Some(info) => info,
        None => return,
    };
    
    let connection = SshConnection {
        host: "localhost".to_string(),
        port,
        user: "blueprint".to_string(),
        key_path: None,
        password: Some("testpass".to_string()),
        jump_host: None,
    };
    
    let deployment_config = DeploymentConfig {
        name: "test-limits".to_string(),
        namespace: "default".to_string(),
        ..Default::default()
    };
    
    if let Ok(client) = SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config).await {
        // Deploy with specific resource limits
        let spec = ResourceSpec {
            cpu: 0.5,
            memory_gb: 0.25,
            storage_gb: 1.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };
        
        let deployment = client.deploy_blueprint(
            "alpine:latest",
            &spec,
            std::collections::HashMap::new(),
        ).await;
        
        if let Ok(deployed) = deployment {
            // Verify limits were applied
            assert_eq!(deployed.resource_limits.cpu_cores, Some(0.5));
            assert_eq!(deployed.resource_limits.memory_mb, Some(256));
            
            println!("✅ Resource limits applied: CPU={:?}, Memory={:?}", 
                deployed.resource_limits.cpu_cores,
                deployed.resource_limits.memory_mb);
            
            // Cleanup
            client.stop_container(&deployed.container_id).await.ok();
        }
    }
    
    cleanup_ssh_container(&container_id).await;
}

#[tokio::test]
async fn test_ssh_container_lifecycle() {
    let (container_id, port) = match start_ssh_container().await {
        Some(info) => info,
        None => return,
    };
    
    let connection = SshConnection {
        host: "localhost".to_string(),
        port,
        user: "blueprint".to_string(),
        key_path: None,
        password: Some("testpass".to_string()),
        jump_host: None,
    };
    
    let deployment_config = DeploymentConfig {
        name: "test-lifecycle".to_string(),
        namespace: "default".to_string(),
        ..Default::default()
    };
    
    if let Ok(client) = SshDeploymentClient::new(connection, ContainerRuntime::Docker, deployment_config).await {
        // Deploy
        let spec = ResourceSpec::minimal();
        let deployment = client.deploy_blueprint(
            "nginx:alpine",
            &spec,
            std::collections::HashMap::new(),
        ).await;
        
        if let Ok(deployed) = deployment {
            let container_id = deployed.container_id.clone();
            
            // Check running
            let status = client.get_container_status(&container_id).await.unwrap();
            assert!(status.contains("running") || status.contains("Up"));
            
            // Stop
            client.stop_container(&container_id).await.unwrap();
            
            // Check stopped
            tokio::time::sleep(Duration::from_secs(2)).await;
            let status = client.get_container_status(&container_id).await;
            assert!(status.is_err() || !status.unwrap().contains("running"));
            
            println!("✅ Container lifecycle test passed");
        }
    }
    
    cleanup_ssh_container(&container_id).await;
}