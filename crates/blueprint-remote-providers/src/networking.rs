use crate::error::Result;
use crate::remote::CloudProvider;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info};

/// Networking extensions for remote deployments
/// 
/// This works with the existing bridge/proxy system to add
/// cloud-specific networking configurations and tunneling.
pub struct TunnelManager {
    tunnels: Arc<RwLock<HashMap<String, TunnelConnection>>>,
    mode: NetworkingMode,
}

impl TunnelManager {
    pub fn new(mode: NetworkingMode) -> Self {
        Self {
            tunnels: Arc::new(RwLock::new(HashMap::new())),
            mode,
        }
    }
    
    /// Establish tunnel to remote cluster if needed
    /// 
    /// This extends the existing bridge communication to work
    /// across cloud boundaries.
    pub async fn establish_if_needed(
        &self,
        cluster_name: &str,
        provider: &CloudProvider,
        endpoint: &str,
    ) -> Result<Option<TunnelConnection>> {
        if !provider.requires_tunnel() {
            debug!("Provider {} doesn't require tunnel", cluster_name);
            return Ok(None);
        }
        
        info!("Establishing tunnel to {} cluster", cluster_name);
        
        // Check if tunnel already exists
        if let Some(existing) = self.tunnels.read().await.get(cluster_name) {
            if existing.is_healthy().await {
                debug!("Reusing existing tunnel to {}", cluster_name);
                return Ok(Some(existing.clone()));
            }
        }
        
        // Create new tunnel based on mode
        let tunnel = match self.mode {
            NetworkingMode::Direct => {
                // No tunnel, direct connection
                return Ok(None);
            }
            NetworkingMode::WireGuard { ref config } => {
                self.create_wireguard_tunnel(cluster_name, endpoint, config).await?
            }
            NetworkingMode::SSHTunnel { ref jump_host } => {
                self.create_ssh_tunnel(cluster_name, endpoint, jump_host).await?
            }
        };
        
        self.tunnels.write().await.insert(cluster_name.to_string(), tunnel.clone());
        
        Ok(Some(tunnel))
    }
    
    async fn create_wireguard_tunnel(
        &self,
        cluster_name: &str,
        endpoint: &str,
        config: &WireGuardConfig,
    ) -> Result<TunnelConnection> {
        // In production, this would shell out to wg-quick or use a WireGuard library
        // For now, we create a mock tunnel configuration
        
        info!("Creating WireGuard tunnel to {}", endpoint);
        
        Ok(TunnelConnection {
            name: format!("wg-{}", cluster_name),
            tunnel_type: TunnelType::WireGuard,
            local_endpoint: config.local_address.clone(),
            remote_endpoint: endpoint.to_string(),
            interface: format!("wg-{}", cluster_name.chars().take(8).collect::<String>()),
        })
    }
    
    async fn create_ssh_tunnel(
        &self,
        cluster_name: &str,
        endpoint: &str,
        jump_host: &str,
    ) -> Result<TunnelConnection> {
        // In production, this would create an SSH tunnel
        // For now, we create a mock tunnel configuration
        
        info!("Creating SSH tunnel via {} to {}", jump_host, endpoint);
        
        Ok(TunnelConnection {
            name: format!("ssh-{}", cluster_name),
            tunnel_type: TunnelType::SSH,
            local_endpoint: "127.0.0.1:0".to_string(), // Dynamic port
            remote_endpoint: endpoint.to_string(),
            interface: "ssh".to_string(),
        })
    }
    
    /// Get existing tunnel for a cluster
    pub async fn get_tunnel(&self, cluster_name: &str) -> Option<TunnelConnection> {
        self.tunnels.read().await.get(cluster_name).cloned()
    }
    
    /// Tear down tunnel
    pub async fn teardown(&self, cluster_name: &str) -> Result<()> {
        if let Some(tunnel) = self.tunnels.write().await.remove(cluster_name) {
            info!("Tearing down tunnel: {}", tunnel.name);
            // In production, would actually tear down the tunnel
        }
        Ok(())
    }
}

/// Networking mode for remote connections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum NetworkingMode {
    /// Direct connection (no tunnel)
    Direct,
    /// WireGuard VPN tunnel
    WireGuard { config: WireGuardConfig },
    /// SSH tunnel through jump host
    SSHTunnel { jump_host: String },
}

impl Default for NetworkingMode {
    fn default() -> Self {
        NetworkingMode::Direct
    }
}

/// WireGuard configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireGuardConfig {
    pub private_key_path: String,
    pub public_key: String,
    pub local_address: String,
    pub peer_endpoint: String,
    pub peer_public_key: String,
    pub allowed_ips: Vec<String>,
}

/// Active tunnel connection
#[derive(Debug, Clone)]
pub struct TunnelConnection {
    pub name: String,
    pub tunnel_type: TunnelType,
    pub local_endpoint: String,
    pub remote_endpoint: String,
    pub interface: String,
}

impl TunnelConnection {
    /// Check if tunnel is healthy
    pub async fn is_healthy(&self) -> bool {
        // In production, would actually check tunnel health
        // For now, always return true
        true
    }
    
    /// Get the endpoint to use for connections through this tunnel
    pub fn get_connection_endpoint(&self) -> String {
        match self.tunnel_type {
            TunnelType::WireGuard => self.local_endpoint.clone(),
            TunnelType::SSH => self.local_endpoint.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum TunnelType {
    WireGuard,
    SSH,
}

/// Extension for existing bridge to work with tunnels
pub struct RemoteBridgeExtension {
    tunnel_manager: Arc<TunnelManager>,
}

impl RemoteBridgeExtension {
    pub fn new(tunnel_manager: Arc<TunnelManager>) -> Self {
        Self { tunnel_manager }
    }
    
    /// Get the appropriate endpoint for a remote cluster
    /// 
    /// This determines whether to use direct connection or tunnel
    /// and returns the correct endpoint for the existing bridge to use.
    pub async fn get_remote_endpoint(
        &self,
        cluster_name: &str,
        direct_endpoint: &str,
    ) -> Result<String> {
        if let Some(tunnel) = self.tunnel_manager.get_tunnel(cluster_name).await {
            // Use tunnel endpoint
            Ok(tunnel.get_connection_endpoint())
        } else {
            // Use direct endpoint
            Ok(direct_endpoint.to_string())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_tunnel_not_required() {
        let manager = TunnelManager::new(NetworkingMode::Direct);
        
        let result = manager.establish_if_needed(
            "aws-cluster",
            &CloudProvider::AWS,
            "eks.amazonaws.com",
        ).await.unwrap();
        
        assert!(result.is_none(), "AWS shouldn't require tunnel");
    }
    
    #[tokio::test]
    async fn test_tunnel_required() {
        let config = WireGuardConfig {
            private_key_path: "/tmp/wg.key".to_string(),
            public_key: "test-public-key".to_string(),
            local_address: "10.0.0.2".to_string(),
            peer_endpoint: "vpn.example.com:51820".to_string(),
            peer_public_key: "peer-public-key".to_string(),
            allowed_ips: vec!["10.0.0.0/24".to_string()],
        };
        
        let manager = TunnelManager::new(NetworkingMode::WireGuard { config });
        
        let result = manager.establish_if_needed(
            "generic-cluster",
            &CloudProvider::Generic,
            "cluster.local",
        ).await.unwrap();
        
        assert!(result.is_some(), "Generic should require tunnel");
        
        let tunnel = result.unwrap();
        assert_eq!(tunnel.tunnel_type as i32, TunnelType::WireGuard as i32);
        assert!(tunnel.name.contains("generic-cluster"));
    }
    
    #[tokio::test]
    async fn test_tunnel_reuse() {
        let manager = TunnelManager::new(NetworkingMode::SSHTunnel {
            jump_host: "jump.example.com".to_string(),
        });
        
        // First call creates tunnel
        let tunnel1 = manager.establish_if_needed(
            "bare-metal",
            &CloudProvider::BareMetal(vec!["192.168.1.10".to_string()]),
            "192.168.1.10",
        ).await.unwrap();
        
        assert!(tunnel1.is_some());
        
        // Second call reuses tunnel
        let tunnel2 = manager.establish_if_needed(
            "bare-metal",
            &CloudProvider::BareMetal(vec!["192.168.1.10".to_string()]),
            "192.168.1.10",
        ).await.unwrap();
        
        assert!(tunnel2.is_some());
        assert_eq!(tunnel1.unwrap().name, tunnel2.unwrap().name);
    }
}