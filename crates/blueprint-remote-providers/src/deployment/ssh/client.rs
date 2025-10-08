//! SSH deployment client implementation

use super::types::*;
use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use crate::deployment::secure_commands::{SecureConfigManager, SecureContainerCommands};
use crate::deployment::secure_ssh::{SecureSshClient, SecureSshConnection};
use crate::monitoring::health::{ApplicationHealthChecker, HealthStatus};
#[allow(unused_imports)]
use crate::monitoring::logs::LogStreamer;
use blueprint_core::{debug, info, warn};
use std::collections::HashMap;
use std::path::Path;
use tokio::sync::mpsc;

/// SSH deployment client for bare metal servers
pub struct SshDeploymentClient {
    /// Secure SSH connection
    ssh_client: SecureSshClient,
    /// SSH connection parameters
    connection: SshConnection,
    /// Remote runtime type (Docker, Podman, Containerd)
    runtime: ContainerRuntime,
    /// Blueprint deployment configuration
    deployment_config: DeploymentConfig,
}

impl SshDeploymentClient {
    /// Create a new SSH deployment client with secure connection
    pub async fn new(
        connection: SshConnection,
        runtime: ContainerRuntime,
        deployment_config: DeploymentConfig,
    ) -> Result<Self> {
        // Create secure SSH connection with validation
        let secure_connection =
            SecureSshConnection::new(connection.host.clone(), connection.user.clone())?
                .with_port(connection.port)?
                .with_strict_host_checking(false); // Disabled for dynamic cloud instances

        let secure_connection = if let Some(ref key_path) = connection.key_path {
            secure_connection.with_key_path(key_path)?
        } else {
            secure_connection
        };

        let secure_connection = if let Some(ref jump_host) = connection.jump_host {
            secure_connection.with_jump_host(jump_host.clone())?
        } else {
            secure_connection
        };

        let ssh_client = SecureSshClient::new(secure_connection);

        let client = Self {
            ssh_client,
            connection,
            runtime,
            deployment_config,
        };

        // Test SSH connection
        client.test_connection().await?;

        // Verify runtime is installed
        client.verify_runtime().await?;

        Ok(client)
    }

    /// Test SSH connection to the remote host
    async fn test_connection(&self) -> Result<()> {
        let output = self.run_remote_command("echo 'Connection test'").await?;
        if output.contains("Connection test") {
            info!("SSH connection to {} successful", self.connection.host);
            Ok(())
        } else {
            Err(Error::ConfigurationError(
                "Failed to establish SSH connection".into(),
            ))
        }
    }

    /// Verify container runtime is installed on remote host
    async fn verify_runtime(&self) -> Result<()> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => "docker --version",
            ContainerRuntime::Podman => "podman --version",
            ContainerRuntime::Containerd => "ctr version",
        };

        match self.run_remote_command(cmd).await {
            Ok(output) => {
                let first_line = output.lines().next().unwrap_or("unknown");
                info!("Container runtime verified: {}", first_line);
                Ok(())
            }
            Err(_) => {
                warn!("Container runtime not found, attempting installation");
                self.install_runtime().await
            }
        }
    }

    /// Install container runtime on remote host using official package repositories.
    ///
    /// This uses OS package managers exclusively to avoid security risks from
    /// downloading and executing arbitrary scripts from the internet.
    async fn install_runtime(&self) -> Result<()> {
        let install_script = match self.runtime {
            ContainerRuntime::Docker => {
                // Use official Docker repository via package manager
                // This is secure as it verifies package signatures and uses trusted repos
                r#"
                # Add Docker's official GPG key and repository
                sudo apt-get update
                sudo apt-get install -y ca-certificates curl gnupg
                sudo install -m 0755 -d /etc/apt/keyrings
                curl -fsSL https://download.docker.com/linux/ubuntu/gpg | sudo gpg --dearmor -o /etc/apt/keyrings/docker.gpg
                sudo chmod a+r /etc/apt/keyrings/docker.gpg

                # Add Docker repository
                echo \
                  "deb [arch=$(dpkg --print-architecture) signed-by=/etc/apt/keyrings/docker.gpg] https://download.docker.com/linux/ubuntu \
                  $(. /etc/os-release && echo "$VERSION_CODENAME") stable" | \
                  sudo tee /etc/apt/sources.list.d/docker.list > /dev/null

                # Install Docker from official repository
                sudo apt-get update
                sudo apt-get install -y docker-ce docker-ce-cli containerd.io docker-buildx-plugin docker-compose-plugin
                sudo usermod -aG docker $USER
                sudo systemctl enable docker
                sudo systemctl start docker
                "#
            }
            ContainerRuntime::Podman => {
                r#"
                sudo apt-get update
                sudo apt-get install -y podman
                "#
            }
            ContainerRuntime::Containerd => {
                r#"
                sudo apt-get update
                sudo apt-get install -y containerd
                sudo systemctl enable containerd
                sudo systemctl start containerd
                "#
            }
        };

        self.run_remote_command(install_script).await?;
        info!("Container runtime installed successfully");
        Ok(())
    }

    /// Deploy Blueprint to remote host
    pub async fn deploy_blueprint(
        &self,
        blueprint_image: &str,
        spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<RemoteDeployment> {
        info!(
            "Deploying Blueprint {} to {} (deployment: {}, namespace: {})",
            blueprint_image,
            self.connection.host,
            self.deployment_config.name,
            self.deployment_config.namespace
        );

        // Pull the Blueprint image
        self.pull_image(blueprint_image).await?;

        // Create container with deployment config-based naming and settings
        let container_id = self
            .create_container_with_config(blueprint_image, spec, env_vars)
            .await?;

        // Start the container
        self.start_container(&container_id).await?;

        // Get container details
        let details = self.get_container_details(&container_id).await?;

        let deployment = RemoteDeployment {
            host: self.connection.host.clone(),
            container_id: container_id.clone(),
            runtime: self.runtime.clone(),
            status: details.status,
            ports: details.ports.clone(),
            resource_limits: ResourceLimits::from_spec(spec),
        };

        // Log QoS endpoint for integration
        if let Some(qos_port) = details.ports.get("9615/tcp") {
            info!(
                "Remote Blueprint QoS endpoint available at {}:{}",
                self.connection.host, qos_port
            );
        }

        Ok(deployment)
    }

    /// Pull container image on remote host
    async fn pull_image(&self, image: &str) -> Result<()> {
        let runtime_str = match self.runtime {
            ContainerRuntime::Docker => "docker",
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Containerd => "ctr",
        };

        // Use secure command building to prevent injection
        let cmd = SecureContainerCommands::build_pull_command(runtime_str, image)?;

        info!("Pulling image {} on remote host", image);
        self.run_remote_command(&cmd).await?;
        Ok(())
    }

    /// Create container with resource limits (SECURITY: Fixed command injection vulnerabilities)
    #[allow(dead_code)]
    async fn create_container(
        &self,
        image: &str,
        spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<String> {
        let limits = ResourceLimits::from_spec(spec);

        let runtime_str = match self.runtime {
            ContainerRuntime::Docker => "docker",
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Containerd => "ctr",
        };

        // Use secure command building to prevent injection attacks
        let cmd = SecureContainerCommands::build_create_command(
            runtime_str,
            image,
            &env_vars,
            limits.cpu_cores.map(|c| c as f32),
            limits.memory_mb.map(|m| m as u32),
            limits.disk_gb.map(|d| d as u32),
        )?;

        let output = self.run_remote_command(&cmd).await?;

        // Extract container ID from output
        let container_id = output
            .lines()
            .next()
            .ok_or_else(|| Error::ConfigurationError("Failed to get container ID".into()))?
            .trim()
            .to_string();

        info!("Created container: {}", container_id);
        Ok(container_id)
    }

    /// Create container with deployment config-based naming and restart policies
    async fn create_container_with_config(
        &self,
        image: &str,
        spec: &ResourceSpec,
        mut env_vars: HashMap<String, String>,
    ) -> Result<String> {
        let limits = ResourceLimits::from_spec(spec);

        // Add deployment config variables to environment
        env_vars.insert(
            "BLUEPRINT_DEPLOYMENT_NAME".to_string(),
            self.deployment_config.name.clone(),
        );
        env_vars.insert(
            "BLUEPRINT_NAMESPACE".to_string(),
            self.deployment_config.namespace.clone(),
        );
        env_vars.insert(
            "BLUEPRINT_RESTART_POLICY".to_string(),
            format!("{:?}", self.deployment_config.restart_policy),
        );

        let runtime_str = match self.runtime {
            ContainerRuntime::Docker => "docker",
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Containerd => "ctr",
        };

        // Build container name based on deployment config
        let container_name = format!(
            "{}-{}",
            self.deployment_config.name, self.deployment_config.namespace
        );

        // Use secure command building to prevent injection attacks
        let mut cmd = SecureContainerCommands::build_create_command(
            runtime_str,
            image,
            &env_vars,
            limits.cpu_cores.map(|c| c as f32),
            limits.memory_mb.map(|m| m as u32),
            limits.disk_gb.map(|d| d as u32),
        )?;

        // Apply restart policy based on deployment config
        let restart_policy_flag = match self.deployment_config.restart_policy {
            RestartPolicy::Always => "--restart=always",
            RestartPolicy::OnFailure => "--restart=on-failure",
            RestartPolicy::Never => "--restart=no",
        };

        // Insert restart policy and name into command
        if runtime_str != "ctr" {
            cmd = cmd.replace(
                "run -d",
                &format!("run -d --name {container_name} {restart_policy_flag}"),
            );
        }

        // Add health check if configured
        if let Some(ref health_check) = self.deployment_config.health_check {
            if runtime_str == "docker" {
                let health_cmd = format!(
                    "--health-cmd='{}' --health-interval={}s --health-timeout={}s --health-retries={}",
                    health_check.command,
                    health_check.interval,
                    health_check.timeout,
                    health_check.retries
                );
                cmd = cmd.replace("run -d", &format!("run -d {health_cmd}"));
            }
        }

        let output = self.run_remote_command(&cmd).await?;

        // Extract container ID from output
        let container_id = output
            .lines()
            .next()
            .ok_or_else(|| Error::ConfigurationError("Failed to get container ID".into()))?
            .trim()
            .to_string();

        info!("Created container: {} with deployment config", container_id);
        Ok(container_id)
    }

    /// Start a container (SECURITY: Fixed command injection vulnerabilities)
    async fn start_container(&self, container_id: &str) -> Result<()> {
        let runtime_str = match self.runtime {
            ContainerRuntime::Docker => "docker",
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Containerd => return Ok(()), // Containerd starts immediately with ctr run
        };

        // Use secure command building to prevent injection
        let cmd = SecureContainerCommands::build_container_command(
            runtime_str,
            "start",
            container_id,
            None,
        )?;

        self.run_remote_command(&cmd).await?;
        info!("Started container: {}", container_id);
        Ok(())
    }

    /// Get container details
    async fn get_container_details(&self, container_id: &str) -> Result<ContainerDetails> {
        let inspect_cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker inspect {container_id}"),
            ContainerRuntime::Podman => format!("podman inspect {container_id}"),
            ContainerRuntime::Containerd => format!("ctr container info {container_id}"),
        };

        let output = self.run_remote_command(&inspect_cmd).await?;
        let json: serde_json::Value = serde_json::from_str(&output).map_err(|e| {
            Error::ConfigurationError(format!("Failed to parse container info: {e}"))
        })?;

        // Parse container details from JSON
        let status = if self.runtime == ContainerRuntime::Containerd {
            json["Status"].as_str().unwrap_or("unknown").to_string()
        } else {
            json[0]["State"]["Status"]
                .as_str()
                .unwrap_or("unknown")
                .to_string()
        };

        let ports = if self.runtime != ContainerRuntime::Containerd {
            json[0]["NetworkSettings"]["Ports"]
                .as_object()
                .map(|ports| {
                    ports
                        .iter()
                        .filter_map(|(internal, bindings)| {
                            bindings[0]["HostPort"]
                                .as_str()
                                .map(|host_port| (internal.clone(), host_port.to_string()))
                        })
                        .collect()
                })
                .unwrap_or_default()
        } else {
            HashMap::new()
        };

        Ok(ContainerDetails { status, ports })
    }

    /// Run a command on the remote host via secure SSH
    async fn run_remote_command(&self, command: &str) -> Result<String> {
        debug!("Running secure remote command: {}", command);
        self.ssh_client.run_remote_command(command).await
    }

    /// Copy files to remote host via secure SCP
    pub async fn copy_files(&self, local_path: &Path, remote_path: &str) -> Result<()> {
        info!(
            "Copying files via secure SCP: {} -> {}",
            local_path.display(),
            remote_path
        );
        self.ssh_client.copy_files(local_path, remote_path).await
    }

    /// Install Blueprint runtime on remote host
    pub async fn install_blueprint_runtime(&self) -> Result<()> {
        info!("Installing Blueprint runtime on remote host");

        // Create Blueprint directory structure
        self.run_remote_command("mkdir -p /opt/blueprint/{bin,config,data,logs}")
            .await?;

        // Download and install Blueprint runtime binary with checksum verification
        let install_script = r#"
        # Download binary and checksum
        curl -L https://github.com/tangle-network/blueprint/releases/latest/download/blueprint-runtime -o /tmp/blueprint-runtime
        curl -L https://github.com/tangle-network/blueprint/releases/latest/download/blueprint-runtime.sha256 -o /tmp/blueprint-runtime.sha256

        # Verify SHA256 checksum
        cd /tmp
        if ! sha256sum -c blueprint-runtime.sha256 2>/dev/null; then
            echo "ERROR: Checksum verification failed for blueprint-runtime" >&2
            rm -f blueprint-runtime blueprint-runtime.sha256
            exit 1
        fi

        # Install verified binary
        chmod +x /tmp/blueprint-runtime
        sudo mv /tmp/blueprint-runtime /opt/blueprint/bin/
        rm -f /tmp/blueprint-runtime.sha256

        # Create systemd service
        sudo tee /etc/systemd/system/blueprint-runtime.service > /dev/null <<EOF
        [Unit]
        Description=Blueprint Runtime
        After=network.target

        [Service]
        Type=simple
        User=blueprint
        WorkingDirectory=/opt/blueprint
        ExecStart=/opt/blueprint/bin/blueprint-runtime
        Restart=always
        RestartSec=10

        [Install]
        WantedBy=multi-user.target
        EOF

        # Create blueprint user
        sudo useradd -r -s /bin/false blueprint || true
        sudo chown -R blueprint:blueprint /opt/blueprint

        # Enable and start service
        sudo systemctl daemon-reload
        sudo systemctl enable blueprint-runtime
        sudo systemctl start blueprint-runtime
        "#;

        self.run_remote_command(install_script).await?;

        // Verify installation
        let status = self
            .run_remote_command("sudo systemctl status blueprint-runtime")
            .await?;
        if status.contains("active (running)") {
            info!("Blueprint runtime installed and running");
            Ok(())
        } else {
            Err(Error::ConfigurationError(
                "Blueprint runtime installation failed".into(),
            ))
        }
    }

    /// Deploy Blueprint as native process (without container)
    pub async fn deploy_native_blueprint(
        &self,
        blueprint_path: &Path,
        spec: &ResourceSpec,
        config: &HashMap<String, String>,
    ) -> Result<NativeDeployment> {
        info!("Deploying native Blueprint to {}", self.connection.host);

        // Copy Blueprint binary to remote host
        self.copy_files(blueprint_path, "/opt/blueprint/bin/")
            .await?;

        // Create configuration file (SECURITY: Fixed command injection vulnerability)
        let config_content = serde_json::to_string_pretty(config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize config: {e}")))?;

        // Use secure config management to prevent injection
        SecureConfigManager::write_config_file(
            &config_content,
            "/opt/blueprint/config/blueprint.json",
        )
        .await?;

        // Set resource limits using systemd
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
            (spec.cpu * 100.0) as u32,
            (spec.memory_gb * 1024.0) as u32,
            1000
        );

        self.run_remote_command(&systemd_limits).await?;

        // Restart service with new limits
        self.run_remote_command(
            "sudo systemctl daemon-reload && sudo systemctl restart blueprint-runtime",
        )
        .await?;

        Ok(NativeDeployment {
            host: self.connection.host.clone(),
            service_name: "blueprint-runtime".to_string(),
            config_path: "/opt/blueprint/config/blueprint.json".to_string(),
            status: "running".to_string(),
        })
    }

    /// Monitor container logs (basic version without integration)
    pub async fn stream_logs(&self, container_id: &str, follow: bool) -> Result<String> {
        let runtime_str = match self.runtime {
            ContainerRuntime::Docker => "docker",
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Containerd => {
                // Containerd doesn't have direct log streaming
                return Err(Error::ConfigurationError(
                    "Log streaming not supported for containerd".into(),
                ));
            }
        };

        // Use secure command building to prevent injection
        let cmd = SecureContainerCommands::build_container_command(
            runtime_str,
            "logs",
            container_id,
            Some(follow),
        )?;

        self.run_remote_command(&cmd).await
    }

    /// Get deployment configuration
    pub fn get_deployment_config(&self) -> &DeploymentConfig {
        &self.deployment_config
    }

    /// Get container status
    pub async fn get_container_status(&self, container_id: &str) -> Result<String> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!("docker ps -a --filter id={container_id} --format '{{{{.Status}}}}'")
            }
            ContainerRuntime::Podman => {
                format!("podman ps -a --filter id={container_id} --format '{{{{.Status}}}}'")
            }
            ContainerRuntime::Containerd => {
                format!("ctr container info {container_id} | grep Status")
            }
        };

        let output = self.run_remote_command(&cmd).await?;
        if output.trim().is_empty() {
            return Err(Error::ConfigurationError(format!(
                "Container {container_id} not found"
            )));
        }
        Ok(output.trim().to_string())
    }

    /// Stop a container
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker stop {container_id}"),
            ContainerRuntime::Podman => format!("podman stop {container_id}"),
            ContainerRuntime::Containerd => format!("ctr task kill {container_id}"),
        };

        self.run_remote_command(&cmd).await?;
        info!("Stopped container: {}", container_id);
        Ok(())
    }

    /// Stop and remove a deployed container
    pub async fn cleanup_deployment(&self, container_id: &str) -> Result<()> {
        let stop_cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!("docker stop {container_id} && docker rm {container_id}")
            }
            ContainerRuntime::Podman => {
                format!("podman stop {container_id} && podman rm {container_id}")
            }
            ContainerRuntime::Containerd => {
                format!("ctr task kill {container_id} && ctr container rm {container_id}")
            }
        };

        self.run_remote_command(&stop_cmd).await?;
        info!("Cleaned up container: {}", container_id);
        Ok(())
    }

    /// Deploy a container with environment variables
    pub async fn deploy_container(
        &self,
        image: &str,
        env_vars: HashMap<String, String>,
    ) -> Result<String> {
        let spec = ResourceSpec::basic();
        self.create_container_with_config(image, &spec, env_vars)
            .await
    }

    /// Deploy a container with a specific name
    pub async fn deploy_container_with_name(
        &self,
        image: &str,
        name: &str,
        env_vars: HashMap<String, String>,
    ) -> Result<String> {
        // Use default resource limits if not specified
        self.deploy_container_with_resources(image, name, env_vars, None)
            .await
    }

    /// Deploy a container with specific name and resource limits
    pub async fn deploy_container_with_resources(
        &self,
        image: &str,
        name: &str,
        env_vars: HashMap<String, String>,
        resource_spec: Option<&ResourceSpec>,
    ) -> Result<String> {
        let runtime_str = match self.runtime {
            ContainerRuntime::Docker => "docker",
            ContainerRuntime::Podman => "podman",
            ContainerRuntime::Containerd => "ctr",
        };

        // Build command with specific name
        let mut cmd = format!("{runtime_str} run -d --name {name}");

        // Add resource limits if specified
        if let Some(spec) = resource_spec {
            match self.runtime {
                ContainerRuntime::Docker | ContainerRuntime::Podman => {
                    // CPU limits (in CPU units, e.g., 1.5 = 1.5 CPUs)
                    cmd.push_str(&format!(" --cpus={}", spec.cpu));

                    // Memory limits (convert GB to format like "2g")
                    cmd.push_str(&format!(" --memory={}g", spec.memory_gb));

                    // GPU support if requested
                    if let Some(gpu_count) = spec.gpu_count {
                        if gpu_count > 0 {
                            cmd.push_str(&format!(" --gpus={gpu_count}"));
                        }
                    }
                }
                ContainerRuntime::Containerd => {
                    // Containerd uses different syntax for resource limits
                    if spec.cpu > 0.0 {
                        cmd.push_str(&format!(" --cpu-quota={}", (spec.cpu * 100000.0) as u64));
                    }
                    if spec.memory_gb > 0.0 {
                        cmd.push_str(&format!(" --memory-limit={}g", spec.memory_gb));
                    }
                }
            }
        }

        // Add environment variables
        for (key, value) in &env_vars {
            cmd.push_str(&format!(" -e {key}={value}"));
        }

        // Add image
        cmd.push_str(&format!(" {image}"));

        let output = self.run_remote_command(&cmd).await?;

        let container_id = output
            .lines()
            .next()
            .ok_or_else(|| Error::ConfigurationError("Failed to get container ID".into()))?
            .trim()
            .to_string();

        info!(
            "Created container {} with name {} and resource limits: {:?}",
            container_id, name, resource_spec
        );
        Ok(container_id)
    }

    /// Update a container (stop old, start new with same config)
    pub async fn update_container(
        &self,
        new_image: &str,
        env_vars: HashMap<String, String>,
    ) -> Result<String> {
        // Use default resource limits if not specified
        self.update_container_with_resources(new_image, env_vars, None)
            .await
    }

    /// Update a container with specific resource limits
    pub async fn update_container_with_resources(
        &self,
        new_image: &str,
        env_vars: HashMap<String, String>,
        resource_spec: Option<&ResourceSpec>,
    ) -> Result<String> {
        // Get current container name from deployment config
        let container_name = format!(
            "{}-{}",
            self.deployment_config.name, self.deployment_config.namespace
        );

        // Stop and remove old container
        let stop_cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!("docker stop {container_name} && docker rm {container_name}")
            }
            ContainerRuntime::Podman => {
                format!("podman stop {container_name} && podman rm {container_name}")
            }
            ContainerRuntime::Containerd => {
                format!("ctr task kill {container_name} && ctr container rm {container_name}")
            }
        };

        // Try to stop and remove old container (might not exist)
        match self.run_remote_command(&stop_cmd).await {
            Ok(_) => info!(
                "Successfully stopped and removed old container: {}",
                container_name
            ),
            Err(e) => debug!(
                "Old container cleanup failed (expected if not exists): {}",
                e
            ),
        }

        // Deploy new container with same name and resource limits
        self.deploy_container_with_resources(new_image, &container_name, env_vars, resource_spec)
            .await
    }

    /// Remove a container
    pub async fn remove_container(&self, container_id: &str) -> Result<()> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker rm -f {container_id}"),
            ContainerRuntime::Podman => format!("podman rm -f {container_id}"),
            ContainerRuntime::Containerd => format!("ctr container rm {container_id}"),
        };

        self.run_remote_command(&cmd).await?;
        info!("Removed container: {}", container_id);
        Ok(())
    }

    /// Check if a container is healthy
    pub async fn health_check_container(&self, container_id: &str) -> Result<bool> {
        // First check if container is running
        let status = self.get_container_status(container_id).await?;
        if !status.contains("Up") && !status.contains("running") {
            return Ok(false);
        }

        // Check container health status if available (Docker only)
        if self.runtime == ContainerRuntime::Docker {
            let cmd =
                format!("docker inspect --format='{{{{.State.Health.Status}}}}' {container_id}");
            match self.run_remote_command(&cmd).await {
                Ok(health) => {
                    let health = health.trim();
                    if health == "healthy" {
                        return Ok(true);
                    } else if health == "unhealthy" {
                        return Ok(false);
                    }
                    // If no health check configured, fall through to basic check
                }
                Err(_) => {
                    // Health check not configured, fall through to basic check
                }
            }
        }

        // Basic connectivity check - try to execute a simple command in the container
        let test_cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker exec {container_id} echo ok"),
            ContainerRuntime::Podman => format!("podman exec {container_id} echo ok"),
            ContainerRuntime::Containerd => format!("ctr task exec {container_id} echo ok"),
        };

        match self.run_remote_command(&test_cmd).await {
            Ok(output) => Ok(output.trim() == "ok"),
            Err(_) => Ok(false),
        }
    }

    /// Switch traffic to a new container (update load balancer/proxy config)
    pub async fn switch_traffic_to(&self, new_container_name: &str) -> Result<()> {
        // This would typically update nginx/haproxy/envoy configuration
        // For now, we'll implement a basic nginx config update

        let nginx_config = format!(
            r#"
upstream backend {{
    server {new_container_name}:8080;
}}

server {{
    listen 80;
    location / {{
        proxy_pass http://backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }}
}}
"#
        );

        // Write new nginx config
        self.run_remote_command(&format!(
            "echo '{nginx_config}' | sudo tee /etc/nginx/sites-available/blueprint"
        ))
        .await?;

        // Reload nginx
        self.run_remote_command("sudo nginx -s reload").await?;

        info!("Switched traffic to container: {}", new_container_name);
        Ok(())
    }

    /// Reconnect SSH connection
    pub async fn reconnect(&mut self) -> Result<()> {
        info!("Reconnecting SSH to {}", self.connection.host);

        // Create new secure connection
        let secure_connection =
            SecureSshConnection::new(self.connection.host.clone(), self.connection.user.clone())?
                .with_port(self.connection.port)?
                .with_strict_host_checking(false);

        let secure_connection = if let Some(ref key_path) = self.connection.key_path {
            secure_connection.with_key_path(key_path)?
        } else {
            secure_connection
        };

        let secure_connection = if let Some(ref jump_host) = self.connection.jump_host {
            secure_connection.with_jump_host(jump_host.clone())?
        } else {
            secure_connection
        };

        // Replace SSH client
        self.ssh_client = SecureSshClient::new(secure_connection);

        // Test reconnection
        self.test_connection().await?;

        info!("SSH reconnection successful");
        Ok(())
    }

    /// Deploy a blueprint to the remote host (main deployment entry point)
    pub async fn deploy(
        &self,
        host_ip: &str,
        binary_path: &Path,
        service_name: &str,
        env_vars: HashMap<String, String>,
        arguments: Vec<String>,
    ) -> Result<()> {
        info!("Deploying blueprint '{}' to {}", service_name, host_ip);

        // Ensure we're connected to the right host
        if self.connection.host != host_ip {
            return Err(Error::ConfigurationError(format!(
                "Host mismatch: expected {}, got {}",
                self.connection.host, host_ip
            )));
        }

        // Copy binary to remote host
        let remote_binary_path = format!("/opt/blueprint/bin/{service_name}");
        self.copy_files(binary_path, &remote_binary_path).await?;

        // Make binary executable
        self.run_remote_command(&format!("chmod +x {remote_binary_path}"))
            .await?;

        // Create service configuration
        let mut service_env = env_vars;
        for (i, arg) in arguments.iter().enumerate() {
            service_env.insert(format!("ARG_{i}"), arg.clone());
        }

        // Create systemd service unit for the blueprint
        let service_unit = format!(
            r#"
[Unit]
Description=Blueprint Service: {}
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=always
RestartSec=10
User=blueprint
Group=blueprint
WorkingDirectory=/opt/blueprint
{}

[Install]
WantedBy=multi-user.target
"#,
            service_name,
            remote_binary_path,
            service_env
                .iter()
                .map(|(k, v)| format!("Environment={k}={v}"))
                .collect::<Vec<_>>()
                .join("\n")
        );

        // Write service file
        let service_file = format!("/etc/systemd/system/blueprint-{service_name}.service");
        self.run_remote_command(&format!(
            "sudo tee {service_file} > /dev/null << 'EOF'\n{service_unit}\nEOF"
        ))
        .await?;

        // Enable and start service
        self.run_remote_command("sudo systemctl daemon-reload")
            .await?;
        self.run_remote_command(&format!("sudo systemctl enable blueprint-{service_name}"))
            .await?;
        self.run_remote_command(&format!("sudo systemctl start blueprint-{service_name}"))
            .await?;

        // Verify service is running
        let status = self
            .run_remote_command(&format!(
                "sudo systemctl is-active blueprint-{service_name}"
            ))
            .await?;
        if status.trim() == "active" {
            info!(
                "✅ Blueprint service '{}' deployed and running",
                service_name
            );
            Ok(())
        } else {
            Err(Error::ConfigurationError(format!(
                "Failed to start blueprint service: {status}"
            )))
        }
    }

    /// Stream container logs integrated with LogStreamer for aggregation
    pub async fn stream_container_logs(
        &self,
        container_id: &str,
    ) -> Result<mpsc::Receiver<String>> {
        info!("Starting log stream for container {}", container_id);

        let (tx, rx) = mpsc::channel(100);
        let runtime = self.runtime.clone();
        let ssh_client = self.ssh_client.clone();
        let container = container_id.to_string();

        // Spawn background task to stream logs
        tokio::spawn(async move {
            let cmd = match runtime {
                ContainerRuntime::Docker => format!("docker logs -f {container}"),
                ContainerRuntime::Podman => format!("podman logs -f {container}"),
                ContainerRuntime::Containerd => {
                    warn!("Log streaming not supported for containerd");
                    return;
                }
            };

            // This would ideally use SSH session with PTY for real-time streaming
            // For now, we poll logs periodically
            loop {
                match ssh_client
                    .run_remote_command(&cmd.replace("-f", "--tail=10"))
                    .await
                {
                    Ok(logs) => {
                        for line in logs.lines() {
                            if tx.send(line.to_string()).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(e) => {
                        warn!("Failed to fetch logs: {}", e);
                        break;
                    }
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
        });

        Ok(rx)
    }

    /// Collect container metrics for QoS monitoring
    pub async fn collect_container_metrics(&self, container_id: &str) -> Result<serde_json::Value> {
        info!("Collecting metrics for container {}", container_id);

        let stats_cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!("docker stats {container_id} --no-stream --format json")
            }
            ContainerRuntime::Podman => {
                format!("podman stats {container_id} --no-stream --format json")
            }
            ContainerRuntime::Containerd => {
                // Containerd doesn't have direct stats, use cgroup info
                return Err(Error::ConfigurationError(
                    "Metrics collection not supported for containerd".into(),
                ));
            }
        };

        let output = self.run_remote_command(&stats_cmd).await?;

        // Parse stats JSON
        let stats: serde_json::Value = serde_json::from_str(&output)
            .map_err(|e| Error::ConfigurationError(format!("Failed to parse stats: {e}")))?;

        // Transform into QoS-compatible format
        let qos_metrics = serde_json::json!({
            "cpu_usage_percent": stats["CPUPerc"].as_str().unwrap_or("0%").replace("%", ""),
            "memory_usage_mb": self.parse_memory_usage(stats["MemUsage"].as_str().unwrap_or("0MiB")),
            "network_io": stats["NetIO"].as_str().unwrap_or("0B / 0B"),
            "block_io": stats["BlockIO"].as_str().unwrap_or("0B / 0B"),
            "pids": stats["PIDs"].as_str().unwrap_or("0"),
            "timestamp": chrono::Utc::now().to_rfc3339(),
        });

        Ok(qos_metrics)
    }

    /// Parse memory usage string (e.g., "100MiB / 1GiB") to MB
    fn parse_memory_usage(&self, mem_str: &str) -> f64 {
        let parts: Vec<&str> = mem_str.split('/').collect();
        if let Some(used) = parts.first() {
            let used = used.trim();
            if used.ends_with("GiB") {
                used.replace("GiB", "").trim().parse::<f64>().unwrap_or(0.0) * 1024.0
            } else if used.ends_with("MiB") {
                used.replace("MiB", "").trim().parse::<f64>().unwrap_or(0.0)
            } else if used.ends_with("KiB") {
                used.replace("KiB", "").trim().parse::<f64>().unwrap_or(0.0) / 1024.0
            } else {
                0.0
            }
        } else {
            0.0
        }
    }

    /// Check blueprint-specific health endpoints
    pub async fn check_blueprint_health(&self, container_id: &str) -> Result<HealthStatus> {
        info!("Checking blueprint health for container {}", container_id);

        // First check container is running
        if !self.health_check_container(container_id).await? {
            return Ok(HealthStatus::Unhealthy);
        }

        // Get container IP for health checks
        let ip_cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!(
                    "docker inspect -f '{{{{.NetworkSettings.IPAddress}}}}' {container_id}"
                )
            }
            ContainerRuntime::Podman => {
                format!(
                    "podman inspect -f '{{{{.NetworkSettings.IPAddress}}}}' {container_id}"
                )
            }
            ContainerRuntime::Containerd => {
                return Ok(HealthStatus::Unknown);
            }
        };

        let container_ip = self.run_remote_command(&ip_cmd).await?.trim().to_string();

        if container_ip.is_empty() || container_ip == "<no value>" {
            warn!("No IP address found for container {}", container_id);
            return Ok(HealthStatus::Unknown);
        }

        // Check blueprint-specific endpoints
        let health_checker = ApplicationHealthChecker::new();

        // Check main health endpoint
        let health_url = format!("http://{container_ip}:8080/health");
        match health_checker.check_http(&health_url).await {
            HealthStatus::Healthy => {
                info!("Blueprint health endpoint healthy");

                // Also check metrics endpoint
                let metrics_url = format!("http://{container_ip}:9615/metrics");
                match health_checker.check_http(&metrics_url).await {
                    HealthStatus::Healthy => {
                        info!("Blueprint metrics endpoint also healthy");
                        Ok(HealthStatus::Healthy)
                    }
                    _ => {
                        warn!("Metrics endpoint not responding");
                        Ok(HealthStatus::Degraded)
                    }
                }
            }
            status => Ok(status),
        }
    }

    /// Deploy blueprint as a systemd service
    pub async fn deploy_binary_as_service(
        &self,
        binary_path: &Path,
        service_name: &str,
        env_vars: HashMap<String, String>,
        resource_spec: &ResourceSpec,
    ) -> Result<()> {
        info!("Deploying {} as systemd service", service_name);

        // Copy binary to remote
        let remote_path = format!("/opt/blueprint/bin/{service_name}");
        self.copy_files(binary_path, &remote_path).await?;

        // Make executable
        self.run_remote_command(&format!("chmod +x {remote_path}"))
            .await?;

        // Create systemd unit with resource limits
        let env_section = env_vars
            .iter()
            .map(|(k, v)| format!("Environment={k}={v}"))
            .collect::<Vec<_>>()
            .join("\n");

        let service_unit = format!(
            r#"
[Unit]
Description=Blueprint Service: {}
After=network.target

[Service]
Type=simple
ExecStart={}
Restart=always
RestartSec=10
User=blueprint
Group=blueprint
WorkingDirectory=/opt/blueprint
{}
CPUQuota={}%
MemoryMax={}M
TasksMax=1000

[Install]
WantedBy=multi-user.target
"#,
            service_name,
            remote_path,
            env_section,
            (resource_spec.cpu * 100.0) as u32,
            (resource_spec.memory_gb * 1024.0) as u32
        );

        // Write service file
        let service_file = format!("/etc/systemd/system/blueprint-{service_name}.service");
        self.run_remote_command(&format!(
            "sudo tee {service_file} > /dev/null << 'EOF'\n{service_unit}\nEOF"
        ))
        .await?;

        // Enable and start
        self.run_remote_command("sudo systemctl daemon-reload")
            .await?;
        self.run_remote_command(&format!("sudo systemctl enable blueprint-{service_name}"))
            .await?;
        self.run_remote_command(&format!("sudo systemctl start blueprint-{service_name}"))
            .await?;

        // Verify it's running
        let status = self
            .run_remote_command(&format!(
                "sudo systemctl is-active blueprint-{service_name}"
            ))
            .await?;

        if status.trim() == "active" {
            info!("Service {} deployed and running", service_name);
            Ok(())
        } else {
            Err(Error::ConfigurationError(format!(
                "Failed to start service: {status}"
            )))
        }
    }

    /// Create a new client with localhost settings for testing
    pub fn localhost() -> Self {
        // This is a simplified constructor for basic usage
        // In production, proper connection details should be provided
        Self {
            ssh_client: SecureSshClient::new(SecureSshConnection {
                host: "localhost".to_string(),
                port: 22,
                user: "root".to_string(),
                key_path: Some("~/.ssh/id_rsa".into()),
                jump_host: None,
                known_hosts_file: None,
                strict_host_checking: false,
            }),
            connection: SshConnection {
                host: "localhost".to_string(),
                port: 22,
                user: "root".to_string(),
                key_path: Some("~/.ssh/id_rsa".into()),
                password: None,
                jump_host: None,
            },
            runtime: ContainerRuntime::Docker,
            deployment_config: DeploymentConfig {
                name: "default".to_string(),
                namespace: "blueprint".to_string(),
                restart_policy: RestartPolicy::Always,
                health_check: None,
            },
        }
    }
}
