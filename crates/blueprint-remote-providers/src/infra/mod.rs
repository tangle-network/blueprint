//! Infrastructure provisioning and deployment

pub mod adapters;
pub mod auto;
pub mod mapper;
pub mod provisioner;
pub mod traits;
pub mod types;

// Re-export main provisioning interfaces
#[cfg(feature = "aws")]
pub use adapters::AwsAdapter;
pub use adapters::DigitalOceanAdapter;
#[cfg(feature = "gcp")]
pub use adapters::GcpAdapter;
pub use auto::AutoDeploymentManager;
pub use mapper::InstanceTypeMapper;
pub use provisioner::CloudProvisioner;
pub use traits::CloudProviderAdapter;
pub use types::{InstanceStatus, ProvisionedInstance, RetryPolicy};
