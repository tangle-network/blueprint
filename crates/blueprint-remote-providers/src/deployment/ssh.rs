//! SSH-based deployment for bare metal servers and remote Docker hosts
//!
//! Provides direct SSH deployment capabilities for Blueprint instances
//! to bare metal servers or any SSH-accessible host with Docker/Podman.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use blueprint_std::collections::HashMap;
use blueprint_std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};
use tokio::process::Command;
use tracing::{debug, info, warn};

/// SSH deployment client for bare metal servers
pub struct SshDeploymentClient {
    /// SSH connection parameters
    connection: SshConnection,
    /// Remote runtime type (Docker, Podman, Containerd)
    runtime: ContainerRuntime,
    /// Blueprint deployment configuration
    deployment_config: DeploymentConfig,
}

impl SshDeploymentClient {
    /// Create a new SSH deployment client
    pub async fn new(
        connection: SshConnection,
        runtime: ContainerRuntime,
        deployment_config: DeploymentConfig,
    ) -> Result<Self> {
        let client = Self {
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
                info!(
                    "Container runtime verified: {}",
                    output.lines().next().unwrap_or("")
                );
                Ok(())
            }
            Err(_) => {
                warn!("Container runtime not found, attempting installation");
                self.install_runtime().await
            }
        }
    }

    /// Install container runtime on remote host
    async fn install_runtime(&self) -> Result<()> {
        let install_script = match self.runtime {
            ContainerRuntime::Docker => {
                r#"
                curl -fsSL https://get.docker.com -o get-docker.sh
                sudo sh get-docker.sh
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
            "Deploying Blueprint {} to {}",
            blueprint_image, self.connection.host
        );

        // Pull the Blueprint image
        self.pull_image(blueprint_image).await?;

        // Create container with resource limits
        let container_id = self
            .create_container(blueprint_image, spec, env_vars)
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
        let cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker pull {}", image),
            ContainerRuntime::Podman => format!("podman pull {}", image),
            ContainerRuntime::Containerd => format!("ctr image pull {}", image),
        };

        info!("Pulling image {} on remote host", image);
        self.run_remote_command(&cmd).await?;
        Ok(())
    }

    /// Create container with resource limits
    async fn create_container(
        &self,
        image: &str,
        spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
    ) -> Result<String> {
        let limits = ResourceLimits::from_spec(spec);

        let cmd = match self.runtime {
            ContainerRuntime::Docker => {
                let mut docker_cmd = format!("docker create");

                // Add resource limits
                if let Some(cpu) = limits.cpu_cores {
                    docker_cmd.push_str(&format!(" --cpus={}", cpu));
                }
                if let Some(mem) = limits.memory_mb {
                    docker_cmd.push_str(&format!(" --memory={}m", mem));
                }
                if let Some(disk) = limits.disk_gb {
                    docker_cmd.push_str(&format!(" --storage-opt size={}G", disk));
                }

                // Add environment variables
                for (key, value) in env_vars {
                    docker_cmd.push_str(&format!(" -e {}={}", key, value));
                }

                // Add network configuration - expose QoS metrics port and RPC endpoint
                docker_cmd.push_str(" -p 0.0.0.0:8080:8080");  // Blueprint endpoint
                docker_cmd.push_str(" -p 0.0.0.0:9615:9615");  // QoS gRPC metrics port
                docker_cmd.push_str(" -p 0.0.0.0:9944:9944");  // RPC endpoint for heartbeat

                // Add container name and image
                docker_cmd.push_str(&format!(
                    " --name blueprint-{} {}",
                    chrono::Utc::now().timestamp(),
                    image
                ));

                docker_cmd
            }
            ContainerRuntime::Podman => {
                let mut podman_cmd = format!("podman create");
                
                // Add resource limits
                if let Some(cpu) = limits.cpu_cores {
                    podman_cmd.push_str(&format!(" --cpus={}", cpu));
                }
                if let Some(mem) = limits.memory_mb {
                    podman_cmd.push_str(&format!(" --memory={}m", mem));
                }
                
                // Add environment variables
                for (key, value) in env_vars {
                    podman_cmd.push_str(&format!(" -e {}={}", key, value));
                }
                
                // Add network configuration - expose QoS metrics port and RPC endpoint
                podman_cmd.push_str(" -p 0.0.0.0:8080:8080");  // Blueprint endpoint
                podman_cmd.push_str(" -p 0.0.0.0:9615:9615");  // QoS gRPC metrics port
                podman_cmd.push_str(" -p 0.0.0.0:9944:9944");  // RPC endpoint for heartbeat
                
                // Add container name and image
                podman_cmd.push_str(&format!(
                    " --name blueprint-{} {}",
                    chrono::Utc::now().timestamp(),
                    image
                ));
                
                podman_cmd
            }
            ContainerRuntime::Containerd => {
                // Containerd requires more complex setup
                format!(
                    "ctr run --rm --memory-limit {} {} blueprint-{}",
                    limits.memory_mb.unwrap_or(2048) * 1024 * 1024,
                    image,
                    chrono::Utc::now().timestamp()
                )
            }
        };

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

    /// Start a container
    async fn start_container(&self, container_id: &str) -> Result<()> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker start {}", container_id),
            ContainerRuntime::Podman => format!("podman start {}", container_id),
            ContainerRuntime::Containerd => return Ok(()), // Containerd starts immediately with ctr run
        };

        self.run_remote_command(&cmd).await?;
        info!("Started container: {}", container_id);
        Ok(())
    }

    /// Get container details
    async fn get_container_details(&self, container_id: &str) -> Result<ContainerDetails> {
        let inspect_cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker inspect {}", container_id),
            ContainerRuntime::Podman => format!("podman inspect {}", container_id),
            ContainerRuntime::Containerd => format!("ctr container info {}", container_id),
        };

        let output = self.run_remote_command(&inspect_cmd).await?;
        let json: serde_json::Value = serde_json::from_str(&output).map_err(|e| {
            Error::ConfigurationError(format!("Failed to parse container info: {}", e))
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

    /// Run a command on the remote host via SSH
    async fn run_remote_command(&self, command: &str) -> Result<String> {
        let ssh_cmd = self.build_ssh_command(command);

        debug!("Running remote command: {}", command);

        let output = Command::new("sh")
            .arg("-c")
            .arg(&ssh_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("SSH command failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "Remote command failed: {}",
                stderr
            )));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    /// Build SSH command with proper options
    fn build_ssh_command(&self, command: &str) -> String {
        let mut ssh_cmd = String::from("ssh");

        // Add SSH options
        ssh_cmd.push_str(" -o StrictHostKeyChecking=no");
        ssh_cmd.push_str(" -o UserKnownHostsFile=/dev/null");

        // Add port if not default
        if self.connection.port != 22 {
            ssh_cmd.push_str(&format!(" -p {}", self.connection.port));
        }

        // Add identity file if provided
        if let Some(ref key_path) = self.connection.key_path {
            ssh_cmd.push_str(&format!(" -i {}", key_path.display()));
        }

        // Add jump host if provided
        if let Some(ref jump_host) = self.connection.jump_host {
            ssh_cmd.push_str(&format!(" -J {}", jump_host));
        }

        // Add user@host
        ssh_cmd.push_str(&format!(
            " {}@{}",
            self.connection.user, self.connection.host
        ));

        // Add the command to execute
        ssh_cmd.push_str(&format!(" '{}'", command));

        ssh_cmd
    }

    /// Copy files to remote host via SCP
    pub async fn copy_files(&self, local_path: &Path, remote_path: &str) -> Result<()> {
        let mut scp_cmd = String::from("scp");

        // Add SSH options
        scp_cmd.push_str(" -o StrictHostKeyChecking=no");
        scp_cmd.push_str(" -o UserKnownHostsFile=/dev/null");

        if self.connection.port != 22 {
            scp_cmd.push_str(&format!(" -P {}", self.connection.port));
        }

        if let Some(ref key_path) = self.connection.key_path {
            scp_cmd.push_str(&format!(" -i {}", key_path.display()));
        }

        // Add source and destination
        scp_cmd.push_str(&format!(
            " {} {}@{}:{}",
            local_path.display(),
            self.connection.user,
            self.connection.host,
            remote_path
        ));

        let output = Command::new("sh")
            .arg("-c")
            .arg(&scp_cmd)
            .output()
            .await
            .map_err(|e| Error::ConfigurationError(format!("SCP failed: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(Error::ConfigurationError(format!(
                "File copy failed: {}",
                stderr
            )));
        }

        info!("Files copied to remote host");
        Ok(())
    }

    /// Install Blueprint runtime on remote host
    pub async fn install_blueprint_runtime(&self) -> Result<()> {
        info!("Installing Blueprint runtime on remote host");

        // Create Blueprint directory structure
        self.run_remote_command("mkdir -p /opt/blueprint/{bin,config,data,logs}")
            .await?;

        // Download and install Blueprint runtime binary
        let install_script = r#"
        curl -L https://github.com/tangle-network/blueprint/releases/latest/download/blueprint-runtime -o /tmp/blueprint-runtime
        chmod +x /tmp/blueprint-runtime
        sudo mv /tmp/blueprint-runtime /opt/blueprint/bin/
        
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

        // Create configuration file
        let config_content = serde_json::to_string_pretty(config)
            .map_err(|e| Error::ConfigurationError(format!("Failed to serialize config: {}", e)))?;

        let create_config = format!(
            "echo '{}' | sudo tee /opt/blueprint/config/blueprint.json",
            config_content
        );
        self.run_remote_command(&create_config).await?;

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

    /// Monitor container logs
    pub async fn stream_logs(&self, container_id: &str, follow: bool) -> Result<String> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!(
                    "docker logs{} {}",
                    if follow { " -f" } else { "" },
                    container_id
                )
            }
            ContainerRuntime::Podman => {
                format!(
                    "podman logs{} {}",
                    if follow { " -f" } else { "" },
                    container_id
                )
            }
            ContainerRuntime::Containerd => {
                // Containerd doesn't have direct log streaming
                return Err(Error::ConfigurationError(
                    "Log streaming not supported for containerd".into(),
                ));
            }
        };

        self.run_remote_command(&cmd).await
    }

    /// Get container status
    pub async fn get_container_status(&self, container_id: &str) -> Result<String> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker ps -a --filter id={} --format '{{{{.Status}}}}'", container_id),
            ContainerRuntime::Podman => format!("podman ps -a --filter id={} --format '{{{{.Status}}}}'", container_id),
            ContainerRuntime::Containerd => format!("ctr container info {} | grep Status", container_id),
        };

        let output = self.run_remote_command(&cmd).await?;
        if output.trim().is_empty() {
            return Err(Error::ConfigurationError(format!("Container {} not found", container_id)));
        }
        Ok(output.trim().to_string())
    }

    /// Stop a container
    pub async fn stop_container(&self, container_id: &str) -> Result<()> {
        let cmd = match self.runtime {
            ContainerRuntime::Docker => format!("docker stop {}", container_id),
            ContainerRuntime::Podman => format!("podman stop {}", container_id),
            ContainerRuntime::Containerd => format!("ctr task kill {}", container_id),
        };

        self.run_remote_command(&cmd).await?;
        info!("Stopped container: {}", container_id);
        Ok(())
    }

    /// Stop and remove a deployed container
    pub async fn cleanup_deployment(&self, container_id: &str) -> Result<()> {
        let stop_cmd = match self.runtime {
            ContainerRuntime::Docker => {
                format!("docker stop {} && docker rm {}", container_id, container_id)
            }
            ContainerRuntime::Podman => {
                format!("podman stop {} && podman rm {}", container_id, container_id)
            }
            ContainerRuntime::Containerd => format!(
                "ctr task kill {} && ctr container rm {}",
                container_id, container_id
            ),
        };

        self.run_remote_command(&stop_cmd).await?;
        info!("Cleaned up container: {}", container_id);
        Ok(())
    }
}

/// SSH connection parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConnection {
    /// Hostname or IP address
    pub host: String,
    /// SSH port (default: 22)
    pub port: u16,
    /// SSH username
    pub user: String,
    /// Path to SSH private key
    pub key_path: Option<PathBuf>,
    /// SSH password (not recommended)
    pub password: Option<String>,
    /// Jump host for bastion access
    pub jump_host: Option<String>,
}

impl Default for SshConnection {
    fn default() -> Self {
        Self {
            host: "localhost".to_string(),
            port: 22,
            user: "root".to_string(),
            key_path: None,
            password: None,
            jump_host: None,
        }
    }
}

/// Container runtime type on remote host
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ContainerRuntime {
    Docker,
    Podman,
    Containerd,
}

/// Deployment configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentConfig {
    /// Deployment name
    pub name: String,
    /// Deployment namespace/project
    pub namespace: String,
    /// Auto-restart policy
    pub restart_policy: RestartPolicy,
    /// Health check configuration
    pub health_check: Option<HealthCheck>,
}

/// Container restart policy
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RestartPolicy {
    Always,
    OnFailure,
    Never,
}

impl Default for RestartPolicy {
    fn default() -> Self {
        RestartPolicy::OnFailure
    }
}

impl Default for DeploymentConfig {
    fn default() -> Self {
        Self {
            name: "blueprint-deployment".to_string(),
            namespace: "default".to_string(),
            restart_policy: RestartPolicy::default(),
            health_check: None,
        }
    }
}

/// Health check configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub command: String,
    pub interval: u32,
    pub timeout: u32,
    pub retries: u32,
}

/// Resource limits for container
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu_cores: Option<f64>,
    pub memory_mb: Option<u64>,
    pub disk_gb: Option<f64>,
    pub network_bandwidth_mbps: Option<u32>,
}

impl ResourceLimits {
    fn from_spec(spec: &ResourceSpec) -> Self {
        Self {
            cpu_cores: Some(spec.cpu as f64),
            memory_mb: Some((spec.memory_gb * 1024.0) as u64),
            disk_gb: Some(spec.storage_gb as f64),
            network_bandwidth_mbps: Some(1000), // Default 1Gbps
        }
    }
}

/// Container details
struct ContainerDetails {
    status: String,
    ports: HashMap<String, String>,
}

/// Remote deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteDeployment {
    pub host: String,
    pub container_id: String,
    pub runtime: ContainerRuntime,
    pub status: String,
    pub ports: HashMap<String, String>,
    pub resource_limits: ResourceLimits,
}

/// Native (non-containerized) deployment information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeDeployment {
    pub host: String,
    pub service_name: String,
    pub config_path: String,
    pub status: String,
}

/// Batch deployment to multiple hosts
pub struct BareMetalFleet {
    hosts: Vec<SshConnection>,
    deployments: Vec<RemoteDeployment>,
}

impl BareMetalFleet {
    /// Create a new bare metal fleet
    pub fn new(hosts: Vec<SshConnection>) -> Self {
        Self {
            hosts,
            deployments: Vec::new(),
        }
    }

    /// Deploy to all hosts in parallel
    pub async fn deploy_to_fleet(
        &mut self,
        blueprint_image: &str,
        spec: &ResourceSpec,
        env_vars: HashMap<String, String>,
        runtime: ContainerRuntime,
    ) -> Result<Vec<RemoteDeployment>> {
        use futures::future::join_all;

        let deployment_futures: Vec<_> = self
            .hosts
            .iter()
            .map(|host| {
                let connection = host.clone();
                let image = blueprint_image.to_string();
                let spec = spec.clone();
                let env = env_vars.clone();
                let rt = runtime.clone();

                async move {
                    let client = SshDeploymentClient::new(
                        connection,
                        rt,
                        DeploymentConfig {
                            name: "blueprint".to_string(),
                            namespace: "default".to_string(),
                            restart_policy: RestartPolicy::Always,
                            health_check: None,
                        },
                    )
                    .await?;

                    client.deploy_blueprint(&image, &spec, env).await
                }
            })
            .collect();

        let results = join_all(deployment_futures).await;

        for result in results {
            match result {
                Ok(deployment) => {
                    info!("Successfully deployed to {}", deployment.host);
                    self.deployments.push(deployment);
                }
                Err(e) => {
                    warn!("Failed to deploy to host: {}", e);
                }
            }
        }

        Ok(self.deployments.clone())
    }

    /// Get status of all deployments
    pub fn get_fleet_status(&self) -> HashMap<String, String> {
        self.deployments
            .iter()
            .map(|d| (d.host.clone(), d.status.clone()))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ssh_command_building() {
        let connection = SshConnection {
            host: "example.com".to_string(),
            port: 2222,
            user: "deploy".to_string(),
            key_path: Some(PathBuf::from("/home/user/.ssh/id_rsa")),
            password: None,
            jump_host: Some("bastion.example.com".to_string()),
        };

        let client = SshDeploymentClient {
            connection,
            runtime: ContainerRuntime::Docker,
            deployment_config: DeploymentConfig {
                name: "test".to_string(),
                namespace: "default".to_string(),
                restart_policy: RestartPolicy::Always,
                health_check: None,
            },
        };

        let cmd = client.build_ssh_command("ls -la");
        assert!(cmd.contains("-p 2222"));
        assert!(cmd.contains("-i /home/user/.ssh/id_rsa"));
        assert!(cmd.contains("-J bastion.example.com"));
        assert!(cmd.contains("deploy@example.com"));
    }

    #[test]
    fn test_resource_limits_conversion() {
        let spec = ResourceSpec {
            cpu: 4.0,
            memory_gb: 8.0,
            storage_gb: 100.0,
            gpu_count: None,
            allow_spot: false,
            qos: Default::default(),
        };

        let limits = ResourceLimits::from_spec(&spec);
        assert_eq!(limits.cpu_cores, Some(4.0));
        assert_eq!(limits.memory_mb, Some(8192));
        assert_eq!(limits.disk_gb, Some(100.0));
        assert_eq!(limits.network_bandwidth_mbps, Some(1000)); // Default 1Gbps
    }
}
