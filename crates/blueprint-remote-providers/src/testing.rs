use crate::{
    error::Result,
    provider::{ProviderType, RemoteInfrastructureProvider},
    types::{
        Cost, DeploymentSpec, InstanceId, InstanceStatus, RemoteInstance, Resources,
        ServiceEndpoint, TunnelHandle, TunnelHub,
    },
};
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct MockProvider {
    name: String,
    deployment_result: RwLock<Result<RemoteInstance>>,
    status_result: RwLock<Result<InstanceStatus>>,
    terminate_result: RwLock<Result<()>>,
    endpoint_result: RwLock<Result<Option<ServiceEndpoint>>>,
    tunnel_result: RwLock<Result<TunnelHandle>>,
    resources_result: RwLock<Result<Resources>>,
    cost_result: RwLock<Result<Cost>>,
    instances: RwLock<HashMap<InstanceId, RemoteInstance>>,
}

impl MockProvider {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            deployment_result: RwLock::new(Ok(RemoteInstance::new("mock-1", "mock-instance", "mock"))),
            status_result: RwLock::new(Ok(InstanceStatus::Running)),
            terminate_result: RwLock::new(Ok(())),
            endpoint_result: RwLock::new(Ok(None)),
            tunnel_result: RwLock::new(Ok(TunnelHandle {
                interface: "wg0".to_string(),
                peer_endpoint: "mock-endpoint".to_string(),
                local_address: "10.0.0.1".to_string(),
                remote_address: "10.0.0.2".to_string(),
            })),
            resources_result: RwLock::new(Ok(Resources {
                total_cpu: "8".to_string(),
                total_memory: "16Gi".to_string(),
                available_cpu: "4".to_string(),
                available_memory: "8Gi".to_string(),
                max_instances: 10,
                current_instances: 0,
            })),
            cost_result: RwLock::new(Ok(Cost::default())),
            instances: RwLock::new(HashMap::new()),
        }
    }
    
    pub async fn set_deployment_result(&self, result: Result<RemoteInstance>) {
        *self.deployment_result.write().await = result;
    }
    
    pub async fn set_status_result(&self, result: Result<InstanceStatus>) {
        *self.status_result.write().await = result;
    }
    
    pub async fn set_terminate_result(&self, result: Result<()>) {
        *self.terminate_result.write().await = result;
    }
    
    pub async fn set_endpoint_result(&self, result: Result<Option<ServiceEndpoint>>) {
        *self.endpoint_result.write().await = result;
    }
    
    pub async fn set_tunnel_result(&self, result: Result<TunnelHandle>) {
        *self.tunnel_result.write().await = result;
    }
    
    pub async fn set_resources_result(&self, result: Result<Resources>) {
        *self.resources_result.write().await = result;
    }
    
    pub async fn set_cost_result(&self, result: Result<Cost>) {
        *self.cost_result.write().await = result;
    }
}

#[async_trait]
impl RemoteInfrastructureProvider for MockProvider {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Mock
    }
    
    async fn deploy_instance(&self, _spec: DeploymentSpec) -> Result<RemoteInstance> {
        let result = self.deployment_result.read().await.clone();
        if let Ok(ref instance) = result {
            self.instances.write().await.insert(instance.id.clone(), instance.clone());
        }
        result
    }
    
    async fn get_instance_status(&self, _id: &InstanceId) -> Result<InstanceStatus> {
        self.status_result.read().await.clone()
    }
    
    async fn terminate_instance(&self, id: &InstanceId) -> Result<()> {
        self.instances.write().await.remove(id);
        self.terminate_result.read().await.clone()
    }
    
    async fn get_instance_endpoint(&self, _id: &InstanceId) -> Result<Option<ServiceEndpoint>> {
        self.endpoint_result.read().await.clone()
    }
    
    async fn establish_tunnel(&self, _hub: &TunnelHub) -> Result<TunnelHandle> {
        self.tunnel_result.read().await.clone()
    }
    
    async fn get_available_resources(&self) -> Result<Resources> {
        self.resources_result.read().await.clone()
    }
    
    async fn estimate_cost(&self, _spec: &DeploymentSpec) -> Result<Cost> {
        self.cost_result.read().await.clone()
    }
    
    async fn list_instances(&self) -> Result<Vec<RemoteInstance>> {
        Ok(self.instances.read().await.values().cloned().collect())
    }
    
    async fn scale_instance(&self, id: &InstanceId, replicas: u32) -> Result<()> {
        if let Some(instance) = self.instances.write().await.get_mut(id) {
            instance.metadata.insert("replicas".to_string(), replicas.to_string());
            Ok(())
        } else {
            Err(crate::Error::InstanceNotFound(id.to_string()))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_mock_provider_default_behavior() {
        let mock = MockProvider::new("test");
        
        assert_eq!(mock.name(), "test");
        assert_eq!(mock.provider_type(), ProviderType::Mock);
        
        let spec = DeploymentSpec::default();
        let instance = mock.deploy_instance(spec).await.unwrap();
        assert_eq!(instance.id.as_str(), "mock-1");
        
        let status = mock.get_instance_status(&instance.id).await.unwrap();
        assert!(matches!(status, InstanceStatus::Running));
        
        let instances = mock.list_instances().await.unwrap();
        assert_eq!(instances.len(), 1);
        
        mock.terminate_instance(&instance.id).await.unwrap();
        let instances = mock.list_instances().await.unwrap();
        assert_eq!(instances.len(), 0);
    }
    
    #[tokio::test]
    async fn test_mock_provider_custom_results() {
        let mock = MockProvider::new("custom");
        
        let custom_instance = RemoteInstance::new("custom-id", "custom-name", "custom-provider");
        mock.set_deployment_result(Ok(custom_instance.clone())).await;
        
        let spec = DeploymentSpec::default();
        let instance = mock.deploy_instance(spec).await.unwrap();
        assert_eq!(instance.id.as_str(), "custom-id");
        assert_eq!(instance.name, "custom-name");
        
        mock.set_status_result(Ok(InstanceStatus::Failed("test error".to_string()))).await;
        let status = mock.get_instance_status(&instance.id).await.unwrap();
        assert!(matches!(status, InstanceStatus::Failed(_)));
    }
    
    #[tokio::test]
    async fn test_mock_provider_error_handling() {
        let mock = MockProvider::new("error");
        
        mock.set_deployment_result(Err(crate::Error::DeploymentFailed("test failure".to_string()))).await;
        
        let spec = DeploymentSpec::default();
        let result = mock.deploy_instance(spec).await;
        assert!(result.is_err());
        
        if let Err(e) = result {
            assert!(matches!(e, crate::Error::DeploymentFailed(_)));
        }
    }
}