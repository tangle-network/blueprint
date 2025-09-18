//! Traits for cloud provider adapters

use crate::core::error::Result;
use crate::infra::types::{InstanceStatus, ProvisionedInstance};
use async_trait::async_trait;

/// Common adapter trait for all cloud providers
#[async_trait]
pub trait CloudProviderAdapter: Send + Sync {
    /// Provision a new instance of the specified type in the given region
    async fn provision_instance(
        &self,
        instance_type: &str,
        region: &str,
    ) -> Result<ProvisionedInstance>;
    
    /// Terminate an existing instance
    async fn terminate_instance(&self, instance_id: &str) -> Result<()>;
    
    /// Get the current status of an instance
    async fn get_instance_status(&self, instance_id: &str) -> Result<InstanceStatus>;
}