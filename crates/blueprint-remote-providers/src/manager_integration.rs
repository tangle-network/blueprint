// Integration module for Blueprint Manager
// This shows how remote providers integrate with the existing manager infrastructure

#[cfg(feature = "blueprint-manager")]
pub mod integration {
    use crate::{
        error::Result,
        provider::RemoteInfrastructureProvider,
        types::{DeploymentSpec, InstanceId, RemoteInstance},
    };
    use blueprint_manager::{
        config::BlueprintManagerConfig,
        rt::service::{Service, ServiceId},
    };
    use std::sync::Arc;
    
    /// Extension trait for BlueprintManager to support remote deployments
    pub trait RemoteDeploymentExt {
        async fn deploy_remote(
            &self,
            provider: Arc<dyn RemoteInfrastructureProvider>,
            spec: DeploymentSpec,
        ) -> Result<ServiceId>;
        
        async fn list_remote_services(
            &self,
            provider: Arc<dyn RemoteInfrastructureProvider>,
        ) -> Result<Vec<RemoteInstance>>;
    }
    
    /// Adapter to convert remote instances to manager services
    pub struct RemoteServiceAdapter {
        provider: Arc<dyn RemoteInfrastructureProvider>,
        instance: RemoteInstance,
    }
    
    impl RemoteServiceAdapter {
        pub fn new(
            provider: Arc<dyn RemoteInfrastructureProvider>,
            instance: RemoteInstance,
        ) -> Self {
            Self { provider, instance }
        }
        
        pub async fn to_service(&self) -> Result<Service> {
            // Convert remote instance to manager service
            // This would integrate with the existing Service struct
            todo!("Implement service conversion")
        }
        
        pub fn instance_id(&self) -> &InstanceId {
            &self.instance.id
        }
        
        pub fn provider_name(&self) -> &str {
            self.provider.name()
        }
    }
    
    /// Configuration extension for remote providers
    #[derive(Debug, Clone)]
    pub struct RemoteProviderConfig {
        pub provider_type: String,
        pub endpoint: Option<String>,
        pub credentials: Option<String>,
        pub region: Option<String>,
        pub tunnel_enabled: bool,
    }
    
    impl RemoteProviderConfig {
        pub fn for_kubernetes(kubeconfig: Option<String>, context: Option<String>) -> Self {
            Self {
                provider_type: "kubernetes".to_string(),
                endpoint: kubeconfig,
                credentials: context,
                region: None,
                tunnel_enabled: true,
            }
        }
        
        pub fn for_docker(endpoint: Option<String>) -> Self {
            Self {
                provider_type: "docker".to_string(),
                endpoint,
                credentials: None,
                region: None,
                tunnel_enabled: false,
            }
        }
    }
}

/// Example of how to use remote providers with Blueprint Manager
#[cfg(all(test, feature = "blueprint-manager"))]
mod tests {
    use super::integration::*;
    use crate::{
        provider::ProviderRegistry,
        testing::MockProvider,
        types::DeploymentSpec,
    };
    use std::sync::Arc;
    
    #[tokio::test]
    async fn test_manager_integration() {
        // Create provider registry
        let registry = ProviderRegistry::new();
        
        // Register mock provider
        let provider = Arc::new(MockProvider::new("test-remote"));
        registry.register("remote", provider.clone()).await;
        
        // Create deployment spec
        let spec = DeploymentSpec::default();
        
        // Deploy instance
        let instance = provider.deploy_instance(spec).await.unwrap();
        
        // Create service adapter
        let adapter = RemoteServiceAdapter::new(provider, instance);
        
        // Verify adapter
        assert_eq!(adapter.provider_name(), "test-remote");
        assert!(!adapter.instance_id().as_str().is_empty());
    }
}