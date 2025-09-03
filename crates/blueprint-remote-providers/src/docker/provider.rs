use crate::{
    error::{Error, Result},
    provider::{ProviderType, RemoteInfrastructureProvider},
    types::{
        Cost, DeploymentSpec, InstanceId, InstanceStatus, RemoteInstance, Resources,
        ServiceEndpoint, TunnelHandle, TunnelHub, Protocol,
    },
};
use async_trait::async_trait;
use bollard::{
    container::{
        Config, CreateContainerOptions, ListContainersOptions, RemoveContainerOptions,
        StartContainerOptions, StopContainerOptions,
    },
    models::{ContainerInspectResponse, ContainerSummary, HostConfig, PortBinding},
    Docker,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DockerConfig {
    pub endpoint: Option<String>,
    pub tls_cert_path: Option<PathBuf>,
    pub tls_key_path: Option<PathBuf>,
    pub tls_ca_path: Option<PathBuf>,
    pub network: Option<String>,
    pub labels: HashMap<String, String>,
}

impl Default for DockerConfig {
    fn default() -> Self {
        Self {
            endpoint: None,
            tls_cert_path: None,
            tls_key_path: None,
            tls_ca_path: None,
            network: Some("bridge".to_string()),
            labels: HashMap::new(),
        }
    }
}

pub struct DockerProvider {
    name: String,
    config: DockerConfig,
    client: Docker,
}

impl DockerProvider {
    pub async fn new(name: impl Into<String>, config: DockerConfig) -> Result<Self> {
        let client = if let Some(ref endpoint) = config.endpoint {
            if endpoint.starts_with("tcp://") || endpoint.starts_with("https://") {
                // Remote Docker with optional TLS
                if config.tls_cert_path.is_some() {
                    Docker::connect_with_ssl(
                        endpoint,
                        &config.tls_key_path.as_ref()
                            .ok_or_else(|| Error::ConfigurationError("TLS key required".to_string()))?,
                        &config.tls_cert_path.as_ref()
                            .ok_or_else(|| Error::ConfigurationError("TLS cert required".to_string()))?,
                        &config.tls_ca_path.as_ref()
                            .ok_or_else(|| Error::ConfigurationError("TLS CA required".to_string()))?,
                        120,
                    )?
                } else {
                    Docker::connect_with_http(endpoint, 120, bollard::API_DEFAULT_VERSION)?
                }
            } else if endpoint.starts_with("unix://") {
                Docker::connect_with_unix(endpoint, 120, bollard::API_DEFAULT_VERSION)?
            } else {
                return Err(Error::ConfigurationError(format!("Invalid Docker endpoint: {}", endpoint)));
            }
        } else {
            // Local Docker
            Docker::connect_with_local_defaults()?
        };
        
        // Test connection
        client.version().await?;
        
        Ok(Self {
            name: name.into(),
            config,
            client,
        })
    }
    
    fn container_config_from_spec(&self, spec: &DeploymentSpec) -> Config<String> {
        let mut labels = self.config.labels.clone();
        labels.insert("blueprint.managed".to_string(), "true".to_string());
        labels.insert("blueprint.name".to_string(), spec.name.clone());
        labels.insert("blueprint.provider".to_string(), self.name.clone());
        
        for (k, v) in &spec.labels {
            labels.insert(format!("blueprint.user.{}", k), v.clone());
        }
        
        let mut env = Vec::new();
        for (k, v) in &spec.environment {
            env.push(format!("{}={}", k, v));
        }
        
        let mut exposed_ports = HashMap::new();
        let mut port_bindings = HashMap::new();
        
        for port in &spec.ports {
            let container_port = format!("{}/{}", 
                port.container_port,
                match port.protocol {
                    Protocol::TCP => "tcp",
                    Protocol::UDP => "udp",
                }
            );
            
            exposed_ports.insert(container_port.clone(), HashMap::new());
            
            if let Some(host_port) = port.host_port {
                port_bindings.insert(
                    container_port,
                    Some(vec![PortBinding {
                        host_ip: Some("0.0.0.0".to_string()),
                        host_port: Some(host_port.to_string()),
                    }]),
                );
            }
        }
        
        let mut host_config = HostConfig {
            network_mode: self.config.network.clone(),
            port_bindings: if port_bindings.is_empty() { None } else { Some(port_bindings) },
            ..Default::default()
        };
        
        // Set resource limits
        if let Some(cpu) = &spec.resources.cpu {
            if let Ok(cores) = cpu.parse::<f64>() {
                host_config.nano_cpus = Some((cores * 1_000_000_000.0) as i64);
            }
        }
        
        if let Some(memory) = &spec.resources.memory {
            if let Some(bytes) = parse_memory_string(memory) {
                host_config.memory = Some(bytes);
            }
        }
        
        Config {
            image: Some(format!("{}:{}", spec.image.repository, spec.image.tag)),
            env: if env.is_empty() { None } else { Some(env) },
            exposed_ports: if exposed_ports.is_empty() { None } else { Some(exposed_ports) },
            host_config: Some(host_config),
            labels: Some(labels),
            ..Default::default()
        }
    }
    
    async fn get_container_by_id(&self, id: &str) -> Result<Option<ContainerInspectResponse>> {
        match self.client.inspect_container(id, None).await {
            Ok(container) => Ok(Some(container)),
            Err(bollard::errors::Error::DockerResponseServerError { status_code: 404, .. }) => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    async fn get_container_by_name(&self, name: &str) -> Result<Option<ContainerSummary>> {
        let mut filters = HashMap::new();
        filters.insert("name".to_string(), vec![name.to_string()]);
        
        let options = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };
        
        let containers = self.client.list_containers(Some(options)).await?;
        Ok(containers.into_iter().next())
    }
}

fn parse_memory_string(mem: &str) -> Option<i64> {
    if mem.ends_with("Gi") {
        mem.trim_end_matches("Gi").parse::<i64>().ok().map(|g| g * 1024 * 1024 * 1024)
    } else if mem.ends_with("Mi") {
        mem.trim_end_matches("Mi").parse::<i64>().ok().map(|m| m * 1024 * 1024)
    } else if mem.ends_with("Ki") {
        mem.trim_end_matches("Ki").parse::<i64>().ok().map(|k| k * 1024)
    } else {
        mem.parse::<i64>().ok()
    }
}

#[async_trait]
impl RemoteInfrastructureProvider for DockerProvider {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Docker
    }
    
    async fn deploy_instance(&self, spec: DeploymentSpec) -> Result<RemoteInstance> {
        let config = self.container_config_from_spec(&spec);
        
        let options = CreateContainerOptions {
            name: spec.name.clone(),
            platform: None,
        };
        
        let container = self.client.create_container(Some(options), config).await?;
        
        self.client.start_container::<String>(&container.id, None).await?;
        
        let mut instance = RemoteInstance::new(container.id, spec.name, self.name.clone());
        instance.status = InstanceStatus::Running;
        
        // Get container info for endpoint
        if let Some(info) = self.get_container_by_id(&instance.id.as_str()).await? {
            if let Some(network_settings) = info.network_settings {
                if let Some(ports) = network_settings.ports {
                    for (container_port, bindings) in ports {
                        if let Some(bindings) = bindings {
                            if let Some(binding) = bindings.first() {
                                let port_parts: Vec<&str> = container_port.split('/').collect();
                                if let Ok(port_num) = port_parts[0].parse::<u16>() {
                                    instance.endpoint = Some(ServiceEndpoint {
                                        host: binding.host_ip.clone().unwrap_or_else(|| "127.0.0.1".to_string()),
                                        port: binding.host_port.clone()
                                            .and_then(|p| p.parse::<u16>().ok())
                                            .unwrap_or(port_num),
                                        protocol: if port_parts.get(1) == Some(&"udp") {
                                            Protocol::UDP
                                        } else {
                                            Protocol::TCP
                                        },
                                        tunnel_required: false,
                                    });
                                    break;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(instance)
    }
    
    async fn get_instance_status(&self, id: &InstanceId) -> Result<InstanceStatus> {
        match self.get_container_by_id(id.as_str()).await? {
            Some(info) => {
                if let Some(state) = info.state {
                    Ok(match state.status.as_deref() {
                        Some("running") => InstanceStatus::Running,
                        Some("paused") => InstanceStatus::Stopped,
                        Some("restarting") => InstanceStatus::Pending,
                        Some("removing") => InstanceStatus::Stopping,
                        Some("exited") | Some("dead") => {
                            if let Some(error) = state.error {
                                InstanceStatus::Failed(error)
                            } else {
                                InstanceStatus::Stopped
                            }
                        }
                        _ => InstanceStatus::Unknown,
                    })
                } else {
                    Ok(InstanceStatus::Unknown)
                }
            }
            None => Ok(InstanceStatus::Unknown),
        }
    }
    
    async fn terminate_instance(&self, id: &InstanceId) -> Result<()> {
        // Stop container
        self.client.stop_container(
            id.as_str(),
            Some(StopContainerOptions { t: 30 }),
        ).await.ok(); // Ignore if already stopped
        
        // Remove container
        self.client.remove_container(
            id.as_str(),
            Some(RemoveContainerOptions {
                force: true,
                v: true,
                ..Default::default()
            }),
        ).await?;
        
        Ok(())
    }
    
    async fn get_instance_endpoint(&self, id: &InstanceId) -> Result<Option<ServiceEndpoint>> {
        if let Some(info) = self.get_container_by_id(id.as_str()).await? {
            if let Some(network_settings) = info.network_settings {
                if let Some(ports) = network_settings.ports {
                    for (container_port, bindings) in ports {
                        if let Some(bindings) = bindings {
                            if let Some(binding) = bindings.first() {
                                let port_parts: Vec<&str> = container_port.split('/').collect();
                                if let Ok(port_num) = port_parts[0].parse::<u16>() {
                                    return Ok(Some(ServiceEndpoint {
                                        host: binding.host_ip.clone().unwrap_or_else(|| "127.0.0.1".to_string()),
                                        port: binding.host_port.clone()
                                            .and_then(|p| p.parse::<u16>().ok())
                                            .unwrap_or(port_num),
                                        protocol: if port_parts.get(1) == Some(&"udp") {
                                            Protocol::UDP
                                        } else {
                                            Protocol::TCP
                                        },
                                        tunnel_required: false,
                                    }));
                                }
                            }
                        }
                    }
                }
            }
        }
        
        Ok(None)
    }
    
    async fn establish_tunnel(&self, hub: &TunnelHub) -> Result<TunnelHandle> {
        // Docker provider typically doesn't need tunneling for local deployments
        // For remote Docker, this would establish WireGuard tunnel
        warn!("Tunnel establishment not yet implemented for Docker provider");
        
        Ok(TunnelHandle {
            interface: "docker0".to_string(),
            peer_endpoint: format!("{}:{}", hub.endpoint, hub.port),
            local_address: "172.17.0.1".to_string(),
            remote_address: "172.17.0.2".to_string(),
        })
    }
    
    async fn get_available_resources(&self) -> Result<Resources> {
        let info = self.client.info().await?;
        
        Ok(Resources {
            total_cpu: info.ncpu.unwrap_or(0).to_string(),
            total_memory: format!("{}Gi", info.mem_total.unwrap_or(0) / (1024 * 1024 * 1024)),
            available_cpu: info.ncpu.unwrap_or(0).to_string(),
            available_memory: format!("{}Gi", info.mem_total.unwrap_or(0) / (1024 * 1024 * 1024)),
            max_instances: 100,
            current_instances: info.containers.unwrap_or(0) as u32,
        })
    }
    
    async fn estimate_cost(&self, spec: &DeploymentSpec) -> Result<Cost> {
        // Docker deployments are typically on owned infrastructure
        // Cost estimation would depend on the underlying infrastructure
        
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
        let cpu_hour_cost = 0.01;
        let memory_hour_cost = 0.001;
        
        let hourly = (cpu_cores * cpu_hour_cost + memory_gb * memory_hour_cost) * spec.replicas as f64;
        let monthly = hourly * 730.0;
        
        let mut breakdown = HashMap::new();
        breakdown.insert("compute".to_string(), cpu_cores * cpu_hour_cost * 730.0 * spec.replicas as f64);
        breakdown.insert("memory".to_string(), memory_gb * memory_hour_cost * 730.0 * spec.replicas as f64);
        
        Ok(Cost {
            estimated_hourly: hourly,
            estimated_monthly: monthly,
            currency: "USD".to_string(),
            breakdown,
        })
    }
    
    async fn list_instances(&self) -> Result<Vec<RemoteInstance>> {
        let mut filters = HashMap::new();
        filters.insert("label".to_string(), vec!["blueprint.managed=true".to_string()]);
        
        let options = ListContainersOptions {
            all: true,
            filters,
            ..Default::default()
        };
        
        let containers = self.client.list_containers(Some(options)).await?;
        let mut instances = Vec::new();
        
        for container in containers {
            let id = container.id.clone().unwrap_or_default();
            let names = container.names.clone().unwrap_or_default();
            let name = names.first()
                .map(|n| n.trim_start_matches('/').to_string())
                .unwrap_or_else(|| id.clone());
            
            let status = match container.state.as_deref() {
                Some("running") => InstanceStatus::Running,
                Some("paused") => InstanceStatus::Stopped,
                Some("restarting") => InstanceStatus::Pending,
                Some("removing") => InstanceStatus::Stopping,
                Some("exited") | Some("dead") => InstanceStatus::Stopped,
                _ => InstanceStatus::Unknown,
            };
            
            let mut instance = RemoteInstance::new(id, name, self.name.clone());
            instance.status = status;
            
            // Extract endpoint from ports
            if let Some(ports) = container.ports {
                for port in ports {
                    if let (Some(public_port), Some(private_port)) = (port.public_port, port.private_port) {
                        instance.endpoint = Some(ServiceEndpoint {
                            host: port.ip.unwrap_or_else(|| "127.0.0.1".to_string()),
                            port: public_port,
                            protocol: match port.r#type.as_deref() {
                                Some("udp") => Protocol::UDP,
                                _ => Protocol::TCP,
                            },
                            tunnel_required: false,
                        });
                        break;
                    }
                }
            }
            
            instances.push(instance);
        }
        
        Ok(instances)
    }
    
    async fn scale_instance(&self, id: &InstanceId, replicas: u32) -> Result<()> {
        // Docker doesn't support native scaling like Kubernetes
        // This would need to be implemented with Docker Swarm or by creating additional containers
        
        if replicas == 0 {
            self.client.stop_container(id.as_str(), Some(StopContainerOptions { t: 30 })).await?;
        } else if replicas == 1 {
            self.client.start_container::<String>(id.as_str(), None).await?;
        } else {
            return Err(Error::ConfigurationError(
                "Docker provider doesn't support scaling beyond 1 replica. Use Docker Swarm or Kubernetes for scaling.".to_string()
            ));
        }
        
        Ok(())
    }
}