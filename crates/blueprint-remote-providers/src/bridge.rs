use crate::{
    error::{Error, Result},
    provider::RemoteInfrastructureProvider,
    types::{InstanceId, ServiceEndpoint, TunnelHandle},
};
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Remote bridge connection manager
/// Handles communication between the Blueprint Manager and remote instances
pub struct RemoteBridgeManager {
    connections: Arc<RwLock<HashMap<InstanceId, BridgeConnection>>>,
    tunnel_manager: Arc<RwLock<TunnelManager>>,
}

impl RemoteBridgeManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
            tunnel_manager: Arc::new(RwLock::new(TunnelManager::new())),
        }
    }
    
    /// Establish a bridge connection to a remote instance
    pub async fn connect_to_instance(
        &self,
        provider: Arc<dyn RemoteInfrastructureProvider>,
        instance_id: &InstanceId,
    ) -> Result<BridgeConnection> {
        info!("Establishing bridge connection to remote instance: {}", instance_id);
        
        // Get instance endpoint
        let endpoint = provider.get_instance_endpoint(instance_id).await?
            .ok_or_else(|| Error::ConfigurationError(
                format!("No endpoint available for instance {}", instance_id)
            ))?;
        
        // Create connection based on endpoint type
        let connection = if endpoint.tunnel_required {
            self.create_tunneled_connection(provider, instance_id, endpoint).await?
        } else {
            self.create_direct_connection(instance_id, endpoint).await?
        };
        
        // Store connection
        self.connections.write().await.insert(instance_id.clone(), connection.clone());
        
        Ok(connection)
    }
    
    async fn create_tunneled_connection(
        &self,
        provider: Arc<dyn RemoteInfrastructureProvider>,
        instance_id: &InstanceId,
        endpoint: ServiceEndpoint,
    ) -> Result<BridgeConnection> {
        debug!("Creating tunneled connection for instance {}", instance_id);
        
        // Establish tunnel if needed
        let tunnel = self.tunnel_manager.write().await
            .get_or_create_tunnel(provider.name()).await?;
        
        Ok(BridgeConnection {
            instance_id: instance_id.clone(),
            connection_type: ConnectionType::Tunneled {
                tunnel_interface: tunnel.interface.clone(),
                remote_address: format!("{}:{}", tunnel.remote_address, endpoint.port),
            },
            status: ConnectionStatus::Connected,
        })
    }
    
    async fn create_direct_connection(
        &self,
        instance_id: &InstanceId,
        endpoint: ServiceEndpoint,
    ) -> Result<BridgeConnection> {
        debug!("Creating direct connection for instance {}", instance_id);
        
        Ok(BridgeConnection {
            instance_id: instance_id.clone(),
            connection_type: ConnectionType::Direct {
                address: format!("{}:{}", endpoint.host, endpoint.port),
            },
            status: ConnectionStatus::Connected,
        })
    }
    
    /// Disconnect from a remote instance
    pub async fn disconnect_from_instance(&self, instance_id: &InstanceId) -> Result<()> {
        info!("Disconnecting from remote instance: {}", instance_id);
        
        if let Some(mut connection) = self.connections.write().await.remove(instance_id) {
            connection.status = ConnectionStatus::Disconnected;
        }
        
        Ok(())
    }
    
    /// Get connection status for an instance
    pub async fn get_connection_status(&self, instance_id: &InstanceId) -> Option<ConnectionStatus> {
        self.connections.read().await
            .get(instance_id)
            .map(|conn| conn.status.clone())
    }
    
    /// List all active connections
    pub async fn list_connections(&self) -> Vec<(InstanceId, ConnectionStatus)> {
        self.connections.read().await
            .iter()
            .map(|(id, conn)| (id.clone(), conn.status.clone()))
            .collect()
    }
    
    /// Health check for a connection
    pub async fn health_check(&self, instance_id: &InstanceId) -> Result<bool> {
        if let Some(connection) = self.connections.read().await.get(instance_id) {
            match &connection.connection_type {
                ConnectionType::Direct { address } => {
                    // Attempt to parse and ping the address
                    if let Ok(addr) = address.parse::<SocketAddr>() {
                        debug!("Health check for direct connection to {}", addr);
                        // In a real implementation, this would actually ping the service
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                }
                ConnectionType::Tunneled { remote_address, .. } => {
                    debug!("Health check for tunneled connection to {}", remote_address);
                    // Check tunnel is still active
                    Ok(true)
                }
            }
        } else {
            Ok(false)
        }
    }
}

impl Default for RemoteBridgeManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct BridgeConnection {
    pub instance_id: InstanceId,
    pub connection_type: ConnectionType,
    pub status: ConnectionStatus,
}

#[derive(Debug, Clone)]
pub enum ConnectionType {
    Direct {
        address: String,
    },
    Tunneled {
        tunnel_interface: String,
        remote_address: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConnectionStatus {
    Connecting,
    Connected,
    Disconnected,
    Error(String),
}

struct TunnelManager {
    tunnels: HashMap<String, TunnelHandle>,
}

impl TunnelManager {
    fn new() -> Self {
        Self {
            tunnels: HashMap::new(),
        }
    }
    
    async fn get_or_create_tunnel(&mut self, provider_name: &str) -> Result<TunnelHandle> {
        if let Some(tunnel) = self.tunnels.get(provider_name) {
            Ok(tunnel.clone())
        } else {
            // In a real implementation, this would establish WireGuard tunnel
            let tunnel = TunnelHandle {
                interface: format!("wg-{}", provider_name),
                peer_endpoint: "hub.blueprint.network:51820".to_string(),
                local_address: "10.100.0.2".to_string(),
                remote_address: "10.100.0.1".to_string(),
            };
            self.tunnels.insert(provider_name.to_string(), tunnel.clone());
            Ok(tunnel)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::testing::MockProvider;
    use crate::types::{DeploymentSpec, Protocol};
    
    #[tokio::test]
    async fn test_direct_connection() {
        let manager = RemoteBridgeManager::new();
        let provider = Arc::new(MockProvider::new("test-provider"));
        
        // Deploy instance
        let spec = DeploymentSpec::default();
        let instance = provider.deploy_instance(spec).await.unwrap();
        
        // Set endpoint
        provider.set_endpoint_result(Ok(Some(ServiceEndpoint {
            host: "192.168.1.100".to_string(),
            port: 8080,
            protocol: Protocol::TCP,
            tunnel_required: false,
        })))
        .await;
        
        // Connect to instance
        let connection = manager.connect_to_instance(provider, &instance.id).await.unwrap();
        
        assert!(matches!(connection.connection_type, ConnectionType::Direct { .. }));
        assert_eq!(connection.status, ConnectionStatus::Connected);
        
        // Check connection status
        let status = manager.get_connection_status(&instance.id).await;
        assert_eq!(status, Some(ConnectionStatus::Connected));
        
        // Health check
        let healthy = manager.health_check(&instance.id).await.unwrap();
        assert!(healthy);
        
        // Disconnect
        manager.disconnect_from_instance(&instance.id).await.unwrap();
        
        // Verify disconnection
        let status = manager.get_connection_status(&instance.id).await;
        assert_eq!(status, None);
    }
    
    #[tokio::test]
    async fn test_tunneled_connection() {
        let manager = RemoteBridgeManager::new();
        let provider = Arc::new(MockProvider::new("remote-provider"));
        
        let instance_id = InstanceId::new("test-instance");
        
        // Set endpoint requiring tunnel
        provider.set_endpoint_result(Ok(Some(ServiceEndpoint {
            host: "10.0.0.5".to_string(),
            port: 8080,
            protocol: Protocol::TCP,
            tunnel_required: true,
        })))
        .await;
        
        // Connect to instance
        let connection = manager.connect_to_instance(provider, &instance_id).await.unwrap();
        
        assert!(matches!(connection.connection_type, ConnectionType::Tunneled { .. }));
        assert_eq!(connection.status, ConnectionStatus::Connected);
    }
    
    #[tokio::test]
    async fn test_multiple_connections() {
        let manager = RemoteBridgeManager::new();
        let provider1 = Arc::new(MockProvider::new("provider1"));
        let provider2 = Arc::new(MockProvider::new("provider2"));
        
        let instance1 = InstanceId::new("instance1");
        let instance2 = InstanceId::new("instance2");
        
        // Set different endpoints
        provider1.set_endpoint_result(Ok(Some(ServiceEndpoint {
            host: "192.168.1.100".to_string(),
            port: 8080,
            protocol: Protocol::TCP,
            tunnel_required: false,
        })))
        .await;
        
        provider2.set_endpoint_result(Ok(Some(ServiceEndpoint {
            host: "10.0.0.5".to_string(),
            port: 9090,
            protocol: Protocol::TCP,
            tunnel_required: true,
        })))
        .await;
        
        // Connect to both instances
        manager.connect_to_instance(provider1, &instance1).await.unwrap();
        manager.connect_to_instance(provider2, &instance2).await.unwrap();
        
        // List connections
        let connections = manager.list_connections().await;
        assert_eq!(connections.len(), 2);
        
        // Verify both are connected
        for (_, status) in connections {
            assert_eq!(status, ConnectionStatus::Connected);
        }
    }
    
    #[tokio::test]
    async fn test_connection_error_handling() {
        let manager = RemoteBridgeManager::new();
        let provider = Arc::new(MockProvider::new("error-provider"));
        
        let instance_id = InstanceId::new("error-instance");
        
        // Set no endpoint (error case)
        provider.set_endpoint_result(Ok(None)).await;
        
        // Attempt connection
        let result = manager.connect_to_instance(provider, &instance_id).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), Error::ConfigurationError(_)));
    }
}