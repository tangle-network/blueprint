//! Type definitions for deployment tracking

use crate::core::error::Result;
use crate::core::remote::CloudProvider;
use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use blueprint_std::collections::HashMap;

/// Deployment record tracking all necessary cleanup information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeploymentRecord {
    /// Unique deployment ID
    pub id: String,
    /// Blueprint instance ID
    pub blueprint_id: String,
    /// Type of deployment
    pub deployment_type: DeploymentType,
    /// Cloud provider (if applicable)
    pub provider: Option<CloudProvider>,
    /// Region/zone
    pub region: Option<String>,
    /// Resource specification
    pub resource_spec: crate::core::resources::ResourceSpec,
    /// Resource identifiers (instance IDs, container IDs, etc.)
    pub resource_ids: HashMap<String, String>,
    /// Deployment timestamp
    pub deployed_at: DateTime<Utc>,
    /// TTL in seconds (if applicable)
    pub ttl_seconds: Option<u64>,
    /// Expiry timestamp
    pub expires_at: Option<DateTime<Utc>>,
    /// Current status
    pub status: DeploymentStatus,
    /// Cleanup webhook URL (optional)
    pub cleanup_webhook: Option<String>,
    /// Additional metadata
    pub metadata: HashMap<String, String>,
}

impl DeploymentRecord {
    /// Create a new deployment record
    pub fn new(
        blueprint_id: String,
        deployment_type: DeploymentType,
        resource_spec: crate::core::resources::ResourceSpec,
        ttl_seconds: Option<u64>,
    ) -> Self {
        let expires_at = ttl_seconds.map(|ttl| Utc::now() + Duration::seconds(ttl as i64));
        let id = format!("dep-{}", uuid::Uuid::new_v4());

        Self {
            id,
            blueprint_id,
            deployment_type,
            provider: None,
            region: None,
            resource_spec,
            resource_ids: HashMap::new(),
            deployed_at: Utc::now(),
            ttl_seconds,
            expires_at,
            status: DeploymentStatus::Active,
            cleanup_webhook: None,
            metadata: HashMap::new(),
        }
    }

    /// Add a resource ID
    pub fn add_resource(&mut self, resource_type: String, resource_id: String) {
        self.resource_ids.insert(resource_type, resource_id);
    }

    /// Set cloud provider information
    pub fn set_cloud_info(&mut self, provider: CloudProvider, region: String) {
        self.provider = Some(provider);
        self.region = Some(region);
    }
}

/// Deployment type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DeploymentType {
    // Local deployments
    LocalDocker,
    LocalKubernetes,
    LocalHypervisor,

    // Cloud VMs
    AwsEc2,
    GcpGce,
    AzureVm,
    DigitalOceanDroplet,
    VultrInstance,

    // Kubernetes clusters
    AwsEks,
    GcpGke,
    AzureAks,
    DigitalOceanDoks,
    VultrVke,

    // Other
    SshRemote,
    BareMetal,
}

/// Deployment status
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DeploymentStatus {
    Active,
    Terminating,
    Terminated,
    Failed,
    Unknown,
}

/// Cleanup handler trait
#[async_trait::async_trait]
pub trait CleanupHandler: Send + Sync {
    /// Perform cleanup for a deployment
    async fn cleanup(&self, deployment: &DeploymentRecord) -> Result<()>;
}
