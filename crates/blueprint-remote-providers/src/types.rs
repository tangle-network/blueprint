use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct InstanceId(String);

impl InstanceId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }
    
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for InstanceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentSpec {
    pub name: String,
    pub image: ContainerImage,
    pub resources: ResourceLimits,
    pub environment: HashMap<String, String>,
    pub ports: Vec<PortMapping>,
    pub volumes: Vec<VolumeMount>,
    pub region: Option<String>,
    pub labels: HashMap<String, String>,
    pub replicas: u32,
}

impl Default for DeploymentSpec {
    fn default() -> Self {
        Self {
            name: "blueprint-instance".to_string(),
            image: ContainerImage::default(),
            resources: ResourceLimits::default(),
            environment: HashMap::new(),
            ports: Vec::new(),
            volumes: Vec::new(),
            region: None,
            labels: HashMap::new(),
            replicas: 1,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContainerImage {
    pub repository: String,
    pub tag: String,
    pub pull_policy: PullPolicy,
}

impl Default for ContainerImage {
    fn default() -> Self {
        Self {
            repository: "blueprint".to_string(),
            tag: "latest".to_string(),
            pull_policy: PullPolicy::IfNotPresent,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PullPolicy {
    Always,
    IfNotPresent,
    Never,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceLimits {
    pub cpu: Option<String>,
    pub memory: Option<String>,
    pub storage: Option<String>,
}

impl Default for ResourceLimits {
    fn default() -> Self {
        Self {
            cpu: Some("1".to_string()),
            memory: Some("1Gi".to_string()),
            storage: Some("10Gi".to_string()),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortMapping {
    pub name: String,
    pub container_port: u16,
    pub host_port: Option<u16>,
    pub protocol: Protocol,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Protocol {
    TCP,
    UDP,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VolumeMount {
    pub name: String,
    pub mount_path: PathBuf,
    pub read_only: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceStatus {
    Pending,
    Running,
    Stopping,
    Stopped,
    Failed(String),
    Unknown,
}

impl fmt::Display for InstanceStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pending => write!(f, "Pending"),
            Self::Running => write!(f, "Running"),
            Self::Stopping => write!(f, "Stopping"),
            Self::Stopped => write!(f, "Stopped"),
            Self::Failed(msg) => write!(f, "Failed: {}", msg),
            Self::Unknown => write!(f, "Unknown"),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteInstance {
    pub id: InstanceId,
    pub name: String,
    pub provider: String,
    pub region: Option<String>,
    pub status: InstanceStatus,
    pub endpoint: Option<ServiceEndpoint>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub metadata: HashMap<String, String>,
}

impl RemoteInstance {
    pub fn new(id: impl Into<String>, name: impl Into<String>, provider: impl Into<String>) -> Self {
        Self {
            id: InstanceId::new(id),
            name: name.into(),
            provider: provider.into(),
            region: None,
            status: InstanceStatus::Pending,
            endpoint: None,
            created_at: chrono::Utc::now(),
            metadata: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceEndpoint {
    pub host: String,
    pub port: u16,
    pub protocol: Protocol,
    pub tunnel_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Resources {
    pub total_cpu: String,
    pub total_memory: String,
    pub available_cpu: String,
    pub available_memory: String,
    pub max_instances: u32,
    pub current_instances: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cost {
    pub estimated_hourly: f64,
    pub estimated_monthly: f64,
    pub currency: String,
    pub breakdown: HashMap<String, f64>,
}

impl Default for Cost {
    fn default() -> Self {
        Self {
            estimated_hourly: 0.0,
            estimated_monthly: 0.0,
            currency: "USD".to_string(),
            breakdown: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TunnelConfig {
    pub endpoint: String,
    pub port: u16,
    pub private_key: Option<String>,
    pub public_key: Option<String>,
    pub allowed_ips: Vec<String>,
    pub persistent_keepalive: Option<u32>,
}

#[derive(Debug, Clone)]
pub struct TunnelHandle {
    pub interface: String,
    pub peer_endpoint: String,
    pub local_address: String,
    pub remote_address: String,
}

#[derive(Debug, Clone)]
pub struct TunnelHub {
    pub endpoint: String,
    pub port: u16,
    pub public_key: String,
}

impl TunnelHub {
    pub fn new(endpoint: impl Into<String>, port: u16, public_key: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            port,
            public_key: public_key.into(),
        }
    }
}