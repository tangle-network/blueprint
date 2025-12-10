//! Test utilities for blueprint remote providers

use crate::core::remote::CloudProvider;
use crate::core::resources::ResourceSpec;
use crate::infra::types::ProvisionedInstance;

/// Create a minimal resource specification for testing
pub fn minimal_resource_spec() -> ResourceSpec {
    ResourceSpec {
        cpu: 1.0,
        memory_gb: 1.0,
        storage_gb: 10.0,
        gpu_count: None,
        allow_spot: false,
        qos: Default::default(),
    }
}

/// Create a mock provisioned instance for testing
pub fn mock_provisioned_instance() -> ProvisionedInstance {
    ProvisionedInstance {
        id: "test-instance-123".to_string(),
        public_ip: Some("203.0.113.1".to_string()),
        private_ip: Some("10.0.1.1".to_string()),
        status: crate::infra::types::InstanceStatus::Running,
        provider: CloudProvider::AWS,
        region: "us-east-1".to_string(),
        instance_type: "t3.micro".to_string(),
    }
}
