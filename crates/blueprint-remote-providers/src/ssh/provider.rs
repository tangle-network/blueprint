use crate::{
    error::{Error, Result},
    provider::{ProviderType, RemoteInfrastructureProvider},
    types::{
        Cost, DeploymentSpec, InstanceId, InstanceStatus, RemoteInstance, Resources,
        ServiceEndpoint, TunnelHandle, TunnelHub, Protocol,
    },
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Stdio;
use tokio::process::Command;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SshConfig {
    pub hosts: Vec<String>,
    pub user: String,
    pub key_path: Option<PathBuf>,
    pub port: u16,
    pub docker_socket: Option<String>,
    pub runtime: SshRuntime,
}

impl Default for SshConfig {
    fn default() -> Self {
        Self {
            hosts: Vec::new(),
            user: "root".to_string(),
            key_path: None,
            port: 22,
            docker_socket: Some("unix:///var/run/docker.sock".to_string()),
            runtime: SshRuntime::Docker,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SshRuntime {
    Docker,
    Native,
    SystemdService,
}

pub struct SshProvider {
    name: String,
    config: SshConfig,
    instances: RwLock<HashMap<InstanceId, SshInstance>>,
    next_host_index: RwLock<usize>,
}

#[derive(Clone)]
struct SshInstance {
    id: InstanceId,
    name: String,
    host: String,
    process_id: Option<String>,
    port: Option<u16>,
    status: InstanceStatus,
}

impl SshProvider {
    pub async fn new(name: impl Into<String>, config: SshConfig) -> Result<Self> {
        let name = name.into();
        
        // Validate SSH connectivity to hosts
        for host in &config.hosts {
            Self::validate_ssh_connection(&config, host).await?;
        }
        
        Ok(Self {
            name,
            config,
            instances: RwLock::new(HashMap::new()),
            next_host_index: RwLock::new(0),
        })
    }
    
    async fn validate_ssh_connection(config: &SshConfig, host: &str) -> Result<()> {
        let mut cmd = Command::new("ssh");
        
        cmd.arg("-o").arg("ConnectTimeout=5")
           .arg("-o").arg("StrictHostKeyChecking=no")
           .arg("-p").arg(config.port.to_string());
        
        if let Some(key_path) = &config.key_path {
            cmd.arg("-i").arg(key_path);
        }
        
        cmd.arg(format!("{}@{}", config.user, host))
           .arg("echo")
           .arg("connected");
        
        let output = cmd.output().await?;
        
        if !output.status.success() {
            return Err(Error::ConfigurationError(
                format!("Cannot connect to SSH host {}: {}", 
                    host, 
                    String::from_utf8_lossy(&output.stderr))
            ));
        }
        
        Ok(())
    }
    
    async fn get_next_host(&self) -> Result<String> {
        if self.config.hosts.is_empty() {
            return Err(Error::ConfigurationError("No SSH hosts configured".to_string()));
        }
        
        let mut index = self.next_host_index.write().await;
        let host = self.config.hosts[*index % self.config.hosts.len()].clone();
        *index += 1;
        Ok(host)
    }
    
    async fn execute_ssh_command(&self, host: &str, command: &str) -> Result<String> {
        let mut cmd = Command::new("ssh");
        
        cmd.arg("-o").arg("StrictHostKeyChecking=no")
           .arg("-p").arg(self.config.port.to_string());
        
        if let Some(key_path) = &self.config.key_path {
            cmd.arg("-i").arg(key_path);
        }
        
        cmd.arg(format!("{}@{}", self.config.user, host))
           .arg(command)
           .stdout(Stdio::piped())
           .stderr(Stdio::piped());
        
        let output = cmd.output().await?;
        
        if !output.status.success() {
            return Err(Error::DeploymentFailed(
                format!("SSH command failed: {}", String::from_utf8_lossy(&output.stderr))
            ));
        }
        
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
    
    async fn deploy_with_docker(&self, host: &str, spec: &DeploymentSpec) -> Result<String> {
        // Build docker run command
        let mut docker_cmd = vec![
            "docker".to_string(),
            "run".to_string(),
            "-d".to_string(),
            "--name".to_string(),
            spec.name.clone(),
        ];
        
        // Add resource limits
        if let Some(cpu) = &spec.resources.cpu {
            docker_cmd.push("--cpus".to_string());
            docker_cmd.push(cpu.clone());
        }
        if let Some(memory) = &spec.resources.memory {
            docker_cmd.push("--memory".to_string());
            docker_cmd.push(memory.clone());
        }
        
        // Add environment variables
        for (key, value) in &spec.environment {
            docker_cmd.push("-e".to_string());
            docker_cmd.push(format!("{}={}", key, value));
        }
        
        // Add port mappings
        for port in &spec.ports {
            docker_cmd.push("-p".to_string());
            if let Some(host_port) = port.host_port {
                docker_cmd.push(format!("{}:{}", host_port, port.container_port));
            } else {
                docker_cmd.push(port.container_port.to_string());
            }
        }
        
        // Add image
        docker_cmd.push(format!("{}:{}", spec.image.repository, spec.image.tag));
        
        let command = docker_cmd.join(" ");
        let output = self.execute_ssh_command(host, &command).await?;
        
        // Extract container ID from output
        Ok(output.trim().to_string())
    }
    
    async fn deploy_native(&self, host: &str, spec: &DeploymentSpec) -> Result<String> {
        // For native deployment, we would:
        // 1. SCP the binary to the host
        // 2. Create a systemd service or run directly
        // 3. Return the process ID
        
        warn!("Native SSH deployment not fully implemented");
        
        // Placeholder: just echo the deployment
        let command = format!(
            "echo 'Would deploy {} natively'", 
            spec.name
        );
        
        self.execute_ssh_command(host, &command).await?;
        Ok(format!("native-{}", spec.name))
    }
}

#[async_trait]
impl RemoteInfrastructureProvider for SshProvider {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Ssh
    }
    
    async fn deploy_instance(&self, spec: DeploymentSpec) -> Result<RemoteInstance> {
        let host = self.get_next_host().await?;
        
        info!("Deploying {} to SSH host {}", spec.name, host);
        
        let process_id = match self.config.runtime {
            SshRuntime::Docker => self.deploy_with_docker(&host, &spec).await?,
            SshRuntime::Native | SshRuntime::SystemdService => {
                self.deploy_native(&host, &spec).await?
            }
        };
        
        let instance_id = InstanceId::new(format!("{}-{}", host, process_id));
        
        let ssh_instance = SshInstance {
            id: instance_id.clone(),
            name: spec.name.clone(),
            host: host.clone(),
            process_id: Some(process_id),
            port: spec.ports.first().and_then(|p| p.host_port),
            status: InstanceStatus::Running,
        };
        
        self.instances.write().await.insert(instance_id.clone(), ssh_instance.clone());
        
        let mut instance = RemoteInstance::new(instance_id, spec.name, self.name.clone());
        instance.status = InstanceStatus::Running;
        
        if let Some(port) = ssh_instance.port {
            instance.endpoint = Some(ServiceEndpoint {
                host,
                port,
                protocol: Protocol::TCP,
                tunnel_required: false,
            });
        }
        
        Ok(instance)
    }
    
    async fn get_instance_status(&self, id: &InstanceId) -> Result<InstanceStatus> {
        if let Some(ssh_instance) = self.instances.read().await.get(id) {
            match self.config.runtime {
                SshRuntime::Docker => {
                    let command = format!(
                        "docker inspect --format='{{{{.State.Status}}}}' {}", 
                        ssh_instance.name
                    );
                    
                    match self.execute_ssh_command(&ssh_instance.host, &command).await {
                        Ok(output) => {
                            match output.trim() {
                                "running" => Ok(InstanceStatus::Running),
                                "paused" => Ok(InstanceStatus::Stopped),
                                "exited" => Ok(InstanceStatus::Stopped),
                                _ => Ok(InstanceStatus::Unknown),
                            }
                        }
                        Err(_) => Ok(InstanceStatus::Unknown),
                    }
                }
                _ => Ok(ssh_instance.status.clone()),
            }
        } else {
            Ok(InstanceStatus::Unknown)
        }
    }
    
    async fn terminate_instance(&self, id: &InstanceId) -> Result<()> {
        if let Some(ssh_instance) = self.instances.write().await.remove(id) {
            match self.config.runtime {
                SshRuntime::Docker => {
                    let command = format!("docker rm -f {}", ssh_instance.name);
                    self.execute_ssh_command(&ssh_instance.host, &command).await?;
                }
                _ => {
                    warn!("Native termination not implemented");
                }
            }
        }
        Ok(())
    }
    
    async fn get_instance_endpoint(&self, id: &InstanceId) -> Result<Option<ServiceEndpoint>> {
        if let Some(ssh_instance) = self.instances.read().await.get(id) {
            if let Some(port) = ssh_instance.port {
                Ok(Some(ServiceEndpoint {
                    host: ssh_instance.host.clone(),
                    port,
                    protocol: Protocol::TCP,
                    tunnel_required: false,
                }))
            } else {
                Ok(None)
            }
        } else {
            Ok(None)
        }
    }
    
    async fn establish_tunnel(&self, hub: &TunnelHub) -> Result<TunnelHandle> {
        // SSH tunneling could be done via port forwarding
        warn!("SSH tunnel establishment not implemented, using direct connection");
        
        Ok(TunnelHandle {
            interface: "ssh-tunnel".to_string(),
            peer_endpoint: format!("{}:{}", hub.endpoint, hub.port),
            local_address: "127.0.0.1".to_string(),
            remote_address: self.config.hosts.first()
                .unwrap_or(&"unknown".to_string())
                .clone(),
        })
    }
    
    async fn get_available_resources(&self) -> Result<Resources> {
        // Get resources from the first host
        if let Some(host) = self.config.hosts.first() {
            let cpu_cmd = "nproc";
            let mem_cmd = "free -b | awk '/^Mem:/{print $2}'";
            
            let cpu_output = self.execute_ssh_command(host, cpu_cmd).await?;
            let mem_output = self.execute_ssh_command(host, mem_cmd).await?;
            
            let cpu_count = cpu_output.trim().parse::<u32>().unwrap_or(1);
            let mem_bytes = mem_output.trim().parse::<u64>().unwrap_or(0);
            
            Ok(Resources {
                total_cpu: cpu_count.to_string(),
                total_memory: format!("{}Gi", mem_bytes / (1024 * 1024 * 1024)),
                available_cpu: cpu_count.to_string(),
                available_memory: format!("{}Gi", mem_bytes / (1024 * 1024 * 1024)),
                max_instances: 50,
                current_instances: self.instances.read().await.len() as u32,
            })
        } else {
            Ok(Resources {
                total_cpu: "0".to_string(),
                total_memory: "0".to_string(),
                available_cpu: "0".to_string(),
                available_memory: "0".to_string(),
                max_instances: 0,
                current_instances: 0,
            })
        }
    }
    
    async fn estimate_cost(&self, spec: &DeploymentSpec) -> Result<Cost> {
        // SSH deployments are typically on owned infrastructure
        // Very low cost estimate
        
        let cpu_cores = spec.resources.cpu
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0);
        
        let memory_gb = spec.resources.memory
            .as_ref()
            .and_then(|s| {
                if s.ends_with("Gi") {
                    s.trim_end_matches("Gi").parse::<f64>().ok()
                } else if s.ends_with("Mi") {
                    s.trim_end_matches("Mi").parse::<f64>().ok().map(|m| m / 1024.0)
                } else {
                    None
                }
            })
            .unwrap_or(1.0);
        
        // Minimal cost for self-hosted infrastructure
        let cpu_hour_cost = 0.001;  // Very low cost
        let memory_hour_cost = 0.0001;
        
        let hourly = (cpu_cores * cpu_hour_cost + memory_gb * memory_hour_cost) * spec.replicas as f64;
        let monthly = hourly * 730.0;
        
        let mut breakdown = HashMap::new();
        breakdown.insert("compute".to_string(), cpu_cores * cpu_hour_cost * 730.0);
        breakdown.insert("memory".to_string(), memory_gb * memory_hour_cost * 730.0);
        breakdown.insert("network".to_string(), 0.0);  // No network costs for SSH
        
        Ok(Cost {
            estimated_hourly: hourly,
            estimated_monthly: monthly,
            currency: "USD".to_string(),
            breakdown,
        })
    }
    
    async fn list_instances(&self) -> Result<Vec<RemoteInstance>> {
        let instances = self.instances.read().await;
        let mut result = Vec::new();
        
        for (id, ssh_instance) in instances.iter() {
            let mut instance = RemoteInstance::new(
                id.clone(),
                ssh_instance.name.clone(),
                self.name.clone(),
            );
            instance.status = ssh_instance.status.clone();
            
            if let Some(port) = ssh_instance.port {
                instance.endpoint = Some(ServiceEndpoint {
                    host: ssh_instance.host.clone(),
                    port,
                    protocol: Protocol::TCP,
                    tunnel_required: false,
                });
            }
            
            result.push(instance);
        }
        
        Ok(result)
    }
    
    async fn scale_instance(&self, _id: &InstanceId, replicas: u32) -> Result<()> {
        if replicas > 1 {
            return Err(Error::ConfigurationError(
                "SSH provider doesn't support scaling. Deploy multiple instances instead.".to_string()
            ));
        }
        Ok(())
    }
}