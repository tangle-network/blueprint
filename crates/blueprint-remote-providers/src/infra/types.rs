//! Types and structures for infrastructure provisioning

use crate::core::remote::CloudProvider;
use serde::{Deserialize, Serialize};

/// Provisioned instance details
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProvisionedInstance {
    pub id: String,
    pub provider: CloudProvider,
    pub instance_type: String,
    pub region: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub status: InstanceStatus,
}

/// Instance status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InstanceStatus {
    Starting,
    Running,
    Stopping,
    Stopped,
    Terminated,
    Unknown,
}

/// A recoverable provider resource carrying the caller's ownership identity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManagedResource {
    pub instance: ProvisionedInstance,
    pub namespace: String,
    pub lease_id: String,
    pub kind: ManagedResourceKind,
    pub endpoint: Option<String>,
    pub tls_mode: Option<TlsMode>,
    pub request_fingerprint: Option<String>,
    pub hourly_cost_usd: Option<f64>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub expires_at: Option<chrono::DateTime<chrono::Utc>>,
    pub expiry_source: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ManagedResourceKind {
    Instance,
    Firewall,
    Template,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TlsMode {
    Public,
    Pinned,
}

/// Retry policy configuration
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    pub max_retries: usize,
    pub base_delay: blueprint_std::time::Duration,
    pub max_delay: blueprint_std::time::Duration,
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay: blueprint_std::time::Duration::from_secs(1),
            max_delay: blueprint_std::time::Duration::from_secs(30),
        }
    }
}

impl RetryPolicy {
    pub fn delay_for_attempt(&self, attempt: usize) -> blueprint_std::time::Duration {
        let delay = self.base_delay * 2u32.pow(attempt as u32);
        delay.min(self.max_delay)
    }
}
