use crate::{
    error::Result,
    types::{
        Cost, DeploymentSpec, InstanceId, InstanceStatus, RemoteInstance, Resources,
        ServiceEndpoint, TunnelHandle, TunnelHub,
    },
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[auto_impl::auto_impl(&, Arc)]
#[async_trait]
pub trait RemoteInfrastructureProvider: Send + Sync + 'static {
    fn name(&self) -> &str;
    
    fn provider_type(&self) -> ProviderType;
    
    async fn deploy_instance(&self, spec: DeploymentSpec) -> Result<RemoteInstance>;
    
    async fn get_instance_status(&self, id: &InstanceId) -> Result<InstanceStatus>;
    
    async fn terminate_instance(&self, id: &InstanceId) -> Result<()>;
    
    async fn get_instance_endpoint(&self, id: &InstanceId) -> Result<Option<ServiceEndpoint>>;
    
    async fn establish_tunnel(&self, hub: &TunnelHub) -> Result<TunnelHandle>;
    
    async fn get_available_resources(&self) -> Result<Resources>;
    
    async fn estimate_cost(&self, spec: &DeploymentSpec) -> Result<Cost>;
    
    async fn list_instances(&self) -> Result<Vec<RemoteInstance>>;
    
    async fn scale_instance(&self, id: &InstanceId, replicas: u32) -> Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProviderType {
    Kubernetes,
    Docker,
    Ssh,
    Mock,
}

pub struct ProviderRegistry {
    providers: Arc<RwLock<HashMap<String, Arc<dyn RemoteInfrastructureProvider>>>>,
}

impl ProviderRegistry {
    pub fn new() -> Self {
        Self {
            providers: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    pub async fn register(
        &self,
        name: impl Into<String>,
        provider: Arc<dyn RemoteInfrastructureProvider>,
    ) {
        let mut providers = self.providers.write().await;
        providers.insert(name.into(), provider);
    }
    
    pub async fn get(&self, name: &str) -> Option<Arc<dyn RemoteInfrastructureProvider>> {
        let providers = self.providers.read().await;
        providers.get(name).cloned()
    }
    
    pub async fn list(&self) -> Vec<String> {
        let providers = self.providers.read().await;
        providers.keys().cloned().collect()
    }
    
    pub async fn remove(&self, name: &str) -> bool {
        let mut providers = self.providers.write().await;
        providers.remove(name).is_some()
    }
}

impl Default for ProviderRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockProvider;
    
    #[tokio::test]
    async fn test_provider_registry() {
        let registry = ProviderRegistry::new();
        let mock_provider = Arc::new(MockProvider::new("test-provider"));
        
        registry.register("test", mock_provider.clone()).await;
        
        let providers = registry.list().await;
        assert_eq!(providers.len(), 1);
        assert!(providers.contains(&"test".to_string()));
        
        let retrieved = registry.get("test").await;
        assert!(retrieved.is_some());
        assert_eq!(retrieved.unwrap().name(), "test-provider");
        
        let removed = registry.remove("test").await;
        assert!(removed);
        
        let providers = registry.list().await;
        assert!(providers.is_empty());
    }
    
    #[tokio::test]
    async fn test_deployment_lifecycle() {
        let mock = MockProvider::new("test-provider");
        mock.set_deployment_result(Ok(RemoteInstance::new(
            "test-123",
            "test-instance",
            "mock",
        )))
        .await;
        
        let spec = DeploymentSpec::default();
        let result = mock.deploy_instance(spec).await;
        
        assert!(result.is_ok());
        let instance = result.unwrap();
        assert_eq!(instance.id.as_str(), "test-123");
        assert_eq!(instance.name, "test-instance");
        
        mock.set_status_result(Ok(InstanceStatus::Running)).await;
        let status = mock.get_instance_status(&instance.id).await;
        assert!(matches!(status, Ok(InstanceStatus::Running)));
        
        let terminate = mock.terminate_instance(&instance.id).await;
        assert!(terminate.is_ok());
    }
    
    #[tokio::test]
    async fn test_resource_management() {
        let mock = MockProvider::new("test-provider");
        
        let resources = Resources {
            total_cpu: "16".to_string(),
            total_memory: "64Gi".to_string(),
            available_cpu: "8".to_string(),
            available_memory: "32Gi".to_string(),
            max_instances: 10,
            current_instances: 2,
        };
        
        mock.set_resources_result(Ok(resources.clone())).await;
        
        let result = mock.get_available_resources().await;
        assert!(result.is_ok());
        let retrieved = result.unwrap();
        assert_eq!(retrieved.total_cpu, "16");
        assert_eq!(retrieved.available_memory, "32Gi");
        assert_eq!(retrieved.current_instances, 2);
    }
    
    #[tokio::test]
    async fn test_cost_estimation() {
        let mock = MockProvider::new("test-provider");
        
        let mut cost = Cost::default();
        cost.estimated_hourly = 0.10;
        cost.estimated_monthly = 72.0;
        cost.breakdown.insert("compute".to_string(), 50.0);
        cost.breakdown.insert("storage".to_string(), 22.0);
        
        mock.set_cost_result(Ok(cost.clone())).await;
        
        let spec = DeploymentSpec::default();
        let result = mock.estimate_cost(&spec).await;
        
        assert!(result.is_ok());
        let estimated = result.unwrap();
        assert_eq!(estimated.estimated_hourly, 0.10);
        assert_eq!(estimated.estimated_monthly, 72.0);
        assert_eq!(estimated.breakdown.get("compute"), Some(&50.0));
    }
    
    #[tokio::test]
    async fn test_tunnel_establishment() {
        let mock = MockProvider::new("test-provider");
        
        let handle = TunnelHandle {
            interface: "wg0".to_string(),
            peer_endpoint: "10.0.0.1:51820".to_string(),
            local_address: "10.0.0.2".to_string(),
            remote_address: "10.0.0.1".to_string(),
        };
        
        mock.set_tunnel_result(Ok(handle.clone())).await;
        
        let hub = TunnelHub::new("hub.example.com", 51820, "public-key");
        let result = mock.establish_tunnel(&hub).await;
        
        assert!(result.is_ok());
        let established = result.unwrap();
        assert_eq!(established.interface, "wg0");
        assert_eq!(established.local_address, "10.0.0.2");
    }
}