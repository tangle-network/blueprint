//! AWS provider implementation

#[cfg(feature = "aws")]
pub mod adapter;
#[cfg(feature = "aws")]
pub mod instance_mapper;
#[cfg(feature = "aws")]
pub mod provisioner;

#[cfg(feature = "aws")]
pub use adapter::AwsAdapter;
#[cfg(feature = "aws")]
pub use instance_mapper::AwsInstanceMapper;
#[cfg(feature = "aws")]
pub use provisioner::AwsProvisioner;

#[cfg(not(feature = "aws"))]
pub mod adapter {
    use crate::core::error::{Error, Result};
    use crate::core::resources::ResourceSpec;
    use crate::infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter};
    use crate::infra::types::{InstanceStatus, ProvisionedInstance};
    use async_trait::async_trait;
    use blueprint_std::collections::HashMap;

    pub struct AwsAdapter;

    impl AwsAdapter {
        pub async fn new() -> Result<Self> {
            Err(Error::ConfigurationError(
                "AWS support not enabled. Enable the 'aws' feature".into(),
            ))
        }
    }

    #[async_trait]
    impl CloudProviderAdapter for AwsAdapter {
        async fn provision_instance(
            &self,
            _instance_type: &str,
            _region: &str,
        ) -> Result<ProvisionedInstance> {
            Err(Error::ConfigurationError(
                "AWS support not enabled. Enable the 'aws' feature".into(),
            ))
        }

        async fn terminate_instance(&self, _instance_id: &str) -> Result<()> {
            Err(Error::ConfigurationError(
                "AWS support not enabled. Enable the 'aws' feature".into(),
            ))
        }

        async fn get_instance_status(&self, _instance_id: &str) -> Result<InstanceStatus> {
            Err(Error::ConfigurationError(
                "AWS support not enabled. Enable the 'aws' feature".into(),
            ))
        }

        async fn deploy_blueprint_with_target(
            &self,
            _target: &crate::core::deployment_target::DeploymentTarget,
            _blueprint_image: &str,
            _resource_spec: &ResourceSpec,
            _env_vars: HashMap<String, String>,
        ) -> Result<BlueprintDeploymentResult> {
            Err(Error::ConfigurationError(
                "AWS support not enabled. Enable the 'aws' feature".into(),
            ))
        }

        async fn health_check_blueprint(
            &self,
            _deployment: &BlueprintDeploymentResult,
        ) -> Result<bool> {
            Ok(false)
        }

        async fn cleanup_blueprint(&self, _deployment: &BlueprintDeploymentResult) -> Result<()> {
            Err(Error::ConfigurationError(
                "AWS support not enabled. Enable the 'aws' feature".into(),
            ))
        }
    }
}
