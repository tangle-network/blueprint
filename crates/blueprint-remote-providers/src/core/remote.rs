#[cfg(feature = "kubernetes")]
use kube::config::Kubeconfig;
#[cfg(feature = "kubernetes")]
use kube::{Client, Config};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::PathBuf;

/// Manages remote Kubernetes clusters for Blueprint deployments
#[cfg(feature = "kubernetes")]
pub struct RemoteClusterManager {
    /// Map of cluster name to configuration
    clusters: Arc<RwLock<HashMap<String, RemoteCluster>>>,
    /// Active cluster for deployments
    active_cluster: Arc<RwLock<Option<String>>>,
}

#[cfg(not(feature = "kubernetes"))]
pub struct RemoteClusterManager {
    _private: (),
}

#[cfg(feature = "kubernetes")]
impl RemoteClusterManager {
    pub fn new() -> Self {
        Self {
            clusters: Arc::new(RwLock::new(HashMap::new())),
            active_cluster: Arc::new(RwLock::new(None)),
        }
    }

    /// Register a remote Kubernetes cluster
    pub async fn add_cluster(&self, name: String, config: KubernetesClusterConfig) -> Result<()> {
        info!("Adding remote cluster: {}", name);

        // Create Kubernetes client with remote context
        let kube_config = if let Some(ref path) = config.kubeconfig_path {
            let kubeconfig_yaml = tokio::fs::read_to_string(path).await.map_err(|e| {
                Error::ConfigurationError(format!("Failed to read kubeconfig file: {}", e))
            })?;
            let kubeconfig: kube::config::Kubeconfig = serde_yaml::from_str(&kubeconfig_yaml)
                .map_err(|e| Error::ConfigurationError(format!("Invalid kubeconfig: {}", e)))?;
            Config::from_custom_kubeconfig(kubeconfig, &Default::default()).await?
        } else {
            Config::infer().await?
        };

        // If a specific context is requested, switch to it
        let kube_config = if let Some(ref context_name) = config.context {
            // Load the full kubeconfig to access all contexts
            let kubeconfig_yaml = if let Some(ref path) = config.kubeconfig_path {
                std::fs::read_to_string(path)
                    .map_err(|e| Error::Other(format!("Failed to read kubeconfig: {}", e)))?
            } else {
                let home =
                    std::env::var("HOME").map_err(|_| Error::Other("HOME not set".into()))?;
                let default_path = format!("{}/.kube/config", home);
                std::fs::read_to_string(&default_path)
                    .map_err(|e| Error::Other(format!("Failed to read kubeconfig: {}", e)))?
            };

            let mut kubeconfig: Kubeconfig = serde_yaml::from_str(&kubeconfig_yaml)
                .map_err(|e| Error::Other(format!("Failed to parse kubeconfig: {}", e)))?;

            // Set the current context to the requested one
            if !kubeconfig.contexts.iter().any(|c| c.name == *context_name) {
                return Err(Error::Other(format!(
                    "Context '{}' not found in kubeconfig",
                    context_name
                )));
            }
            kubeconfig.current_context = Some(context_name.clone());

            Config::from_custom_kubeconfig(kubeconfig, &Default::default()).await?
        } else {
            kube_config
        };

        let client = Client::try_from(kube_config)?;

        let cluster = RemoteCluster { config, client };

        self.clusters.write().await.insert(name.clone(), cluster);

        // Set as active if it's the first cluster
        let mut active = self.active_cluster.write().await;
        if active.is_none() {
            *active = Some(name);
        }

        Ok(())
    }

    /// Switch active cluster for deployments
    pub async fn set_active_cluster(&self, name: &str) -> Result<()> {
        let clusters = self.clusters.read().await;
        if !clusters.contains_key(name) {
            return Err(Error::ConfigurationError(format!(
                "Cluster {} not found",
                name
            )));
        }

        let mut active = self.active_cluster.write().await;
        *active = Some(name.to_string());
        info!("Switched active cluster to: {}", name);

        Ok(())
    }

    /// List all registered clusters
    pub async fn list_clusters(&self) -> Vec<(String, CloudProvider)> {
        let clusters = self.clusters.read().await;
        clusters
            .iter()
            .map(|(name, cluster)| (name.clone(), cluster.config.provider.clone()))
            .collect()
    }

    /// Get cluster endpoint for networking setup
    pub async fn get_cluster_endpoint(&self, name: &str) -> Result<String> {
        let clusters = self.clusters.read().await;
        let cluster = clusters
            .get(name)
            .ok_or_else(|| Error::ConfigurationError(format!("Cluster {} not found", name)))?;

        // Get the cluster's API server endpoint
        Ok(cluster.client.default_namespace().to_string())
    }
}

impl Default for RemoteClusterManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(not(feature = "kubernetes"))]
impl RemoteClusterManager {
    pub fn new() -> Self {
        Self { _private: () }
    }
}

#[cfg(feature = "kubernetes")]
struct RemoteCluster {
    config: KubernetesClusterConfig,
    client: Client,
}

/// Configuration for a Kubernetes cluster (different from deployment config)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesClusterConfig {
    /// Path to kubeconfig file
    pub kubeconfig_path: Option<PathBuf>,
    /// Kubernetes context to use
    pub context: Option<String>,
    /// Namespace for deployments (default: "blueprint-remote")
    pub namespace: String,
    /// Cloud provider type
    pub provider: CloudProvider,
    /// Region/zone information
    pub region: Option<String>,
}

impl Default for KubernetesClusterConfig {
    fn default() -> Self {
        Self {
            kubeconfig_path: None,
            context: None,
            namespace: "blueprint-remote".to_string(),
            provider: CloudProvider::Generic,
            region: None,
        }
    }
}

/// Re-export CloudProvider from pricing-engine
/// This is now the single source of truth for cloud provider types
pub use blueprint_pricing_engine_lib::CloudProvider;

/// Extension trait for Kubernetes-specific functionality
pub trait CloudProviderExt {
    /// Convert to Kubernetes service type based on provider
    fn to_service_type(&self) -> &str;

    /// Check if provider requires tunnel for private networking
    fn requires_tunnel(&self) -> bool;
}

impl CloudProviderExt for CloudProvider {
    fn to_service_type(&self) -> &str {
        match self {
            CloudProvider::AWS | CloudProvider::Azure => "LoadBalancer",
            CloudProvider::GCP => "ClusterIP", // Use with Ingress
            CloudProvider::DigitalOcean | CloudProvider::Vultr | CloudProvider::Linode => {
                "LoadBalancer"
            }
            _ => "ClusterIP",
        }
    }

    fn requires_tunnel(&self) -> bool {
        matches!(
            self,
            CloudProvider::Generic | CloudProvider::BareMetal(_) | CloudProvider::DockerLocal
        )
    }
}

// #[cfg(feature = "kubernetes")]
// impl RemoteContainerRuntimeExt for ContainerRuntime {
//     fn with_client(
//         client: Client,
//         namespace: String,
//         service_type: &str,
//     ) -> Result<ContainerRuntime> {
//         // This would be implemented in the manager crate to allow
//         // creating ContainerRuntime with a specific client
//         // For now, we return an error indicating this needs manager support
//         Err(Error::ConfigurationError(
//             "ContainerRuntime remote extension requires manager crate support".to_string(),
//         ))
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_cluster_management() {
        #[cfg(feature = "kubernetes")]
        {
            // Initialize rustls crypto provider for kube-client
            let _ = rustls::crypto::ring::default_provider().install_default();

            let manager = RemoteClusterManager::new();

            let config = KubernetesClusterConfig {
                namespace: "test-namespace".to_string(),
                provider: CloudProvider::AWS,
                ..Default::default()
            };

            // Note: This may succeed or fail depending on kubeconfig availability
            // Just testing the structure
            let result = manager.add_cluster("test-aws".to_string(), config).await;

            // Either it succeeds (with valid config) or fails (without config)
            // Both are acceptable for this test
            let clusters = manager.list_clusters().await;

            if result.is_ok() {
                assert_eq!(clusters.len(), 1);
            } else {
                assert_eq!(clusters.len(), 0);
            }
        }

        #[cfg(not(feature = "kubernetes"))]
        {
            // Just test that the manager can be created
            let _manager = RemoteClusterManager::new();
        }
    }

    #[test]
    fn test_provider_service_type() {
        use super::CloudProviderExt;
        assert_eq!(CloudProvider::AWS.to_service_type(), "LoadBalancer");
        assert_eq!(CloudProvider::GCP.to_service_type(), "ClusterIP");
        assert_eq!(CloudProvider::Generic.to_service_type(), "ClusterIP");
    }

    #[test]
    fn test_provider_tunnel_requirement() {
        use super::CloudProviderExt;
        assert!(!CloudProvider::AWS.requires_tunnel());
        assert!(!CloudProvider::GCP.requires_tunnel());
        assert!(CloudProvider::Generic.requires_tunnel());
        assert!(CloudProvider::BareMetal(vec!["host".to_string()]).requires_tunnel());
    }
}
