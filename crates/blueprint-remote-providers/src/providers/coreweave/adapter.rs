//! CoreWeave namespace workloads use Kubernetes lifecycle and per-lease SSH access.
#[cfg(feature = "kubernetes")]
use crate::{
    CloudProvider,
    providers::common::gpu_adapter::{DEFAULT_GPU_IMAGE, quoted_selection, starting_instance},
};
use crate::{
    ResourceSpec,
    core::error::{Error, Result},
    infra::{
        traits::{BlueprintDeploymentResult, CloudProviderAdapter},
        types::{InstanceStatus, ProvisionedInstance},
    },
    pricing::{DatedSkuPrice, dated_gpu_rates},
    providers::common::gpu_adapter::{catalog_skus, reject_tee},
};
use async_trait::async_trait;
use blueprint_std::collections::HashMap;
#[cfg(feature = "kubernetes")]
use {
    k8s_openapi::api::core::v1::{Node, Pod, Service},
    kube::{
        Client,
        api::{Api, ListParams, PostParams},
    },
};

#[cfg_attr(not(feature = "kubernetes"), allow(dead_code))]
pub struct CoreWeaveAdapter {
    namespace: String,
    token: Option<String>,
    prices: Vec<DatedSkuPrice>,
    lifecycle: tokio::sync::Mutex<()>,
    #[cfg(feature = "kubernetes")]
    client: Option<Client>,
}

impl CoreWeaveAdapter {
    pub async fn new() -> Result<Self> {
        let token = std::env::var("COREWEAVE_TOKEN")
            .map_err(|_| Error::ConfigurationError("COREWEAVE_TOKEN is required".into()))?;
        let namespace =
            std::env::var("COREWEAVE_NAMESPACE").unwrap_or_else(|_| "tenant-blueprint".into());
        Ok(Self::with_credentials(token, namespace))
    }

    /// The caller's kubeconfig owns the API server and CA; this token replaces its bearer token.
    pub fn with_credentials(token: String, namespace: String) -> Self {
        Self {
            namespace,
            token: Some(token),
            prices: dated_gpu_rates(),
            lifecycle: Default::default(),
            #[cfg(feature = "kubernetes")]
            client: None,
        }
    }

    #[cfg(feature = "kubernetes")]
    pub fn with_client(client: Client, namespace: String) -> Self {
        Self {
            namespace,
            token: None,
            prices: dated_gpu_rates(),
            lifecycle: Default::default(),
            client: Some(client),
        }
    }

    /// Replace unknown public rates with the account's dated all-in contract prices.
    pub fn with_prices(mut self, prices: Vec<DatedSkuPrice>) -> Self {
        self.prices = prices;
        self
    }

    #[cfg(feature = "kubernetes")]
    async fn client(&self) -> Result<Client> {
        if let Some(client) = &self.client {
            return Ok(client.clone());
        }
        let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
        let mut config = kube::Config::infer().await.map_err(|_| unavailable())?;
        let url = url::Url::parse(&config.cluster_url.to_string()).map_err(|_| unavailable())?;
        if url.scheme() != "https"
            && !(url.scheme() == "http"
                && matches!(url.host_str(), Some("127.0.0.1" | "localhost" | "[::1]")))
        {
            return Err(Error::ConfigurationError(
                "CoreWeave Kubernetes API requires HTTPS".into(),
            ));
        }
        if let Some(token) = &self.token {
            config.auth_info = Default::default();
            config.auth_info.token = Some(token.clone().into());
        }
        Client::try_from(config).map_err(|_| unavailable())
    }

    #[cfg(feature = "kubernetes")]
    async fn available(&self, sku: &str, region: &str, count: u32) -> Result<bool> {
        let client = self.client().await?;
        let nodes = Api::<Node>::all(client.clone())
            .list(&ListParams::default())
            .await
            .map_err(kube_error)?;
        let pods = Api::<Pod>::all(client)
            .list(&ListParams::default())
            .await
            .map_err(kube_error)?;
        Ok(nodes.items.iter().any(|node| {
            let labels = node.metadata.labels.as_ref();
            let matching = labels
                .and_then(|l| l.get("gpu.nvidia.com/class"))
                .is_some_and(|value| value == sku)
                && labels
                    .and_then(|l| l.get("topology.kubernetes.io/region"))
                    .is_some_and(|value| value == region);
            let ready = node
                .status
                .as_ref()
                .and_then(|s| s.conditions.as_ref())
                .is_some_and(|conditions| {
                    conditions
                        .iter()
                        .any(|c| c.type_ == "Ready" && c.status == "True")
                });
            let allocatable = node
                .status
                .as_ref()
                .and_then(|s| s.allocatable.as_ref())
                .and_then(|r| r.get("nvidia.com/gpu"))
                .and_then(|q| q.0.parse::<u32>().ok())
                .unwrap_or(0);
            let used: u32 = pods
                .items
                .iter()
                .filter(|p| {
                    p.spec.as_ref().and_then(|s| s.node_name.as_ref())
                        == node.metadata.name.as_ref()
                        && !p
                            .status
                            .as_ref()
                            .and_then(|s| s.phase.as_deref())
                            .is_some_and(|p| matches!(p, "Succeeded" | "Failed"))
                })
                .flat_map(|p| p.spec.iter().flat_map(|s| s.containers.iter()))
                .filter_map(|c| {
                    c.resources.as_ref().and_then(|r| {
                        r.requests
                            .as_ref()
                            .and_then(|r| r.get("nvidia.com/gpu"))
                            .or_else(|| r.limits.as_ref().and_then(|r| r.get("nvidia.com/gpu")))
                    })
                })
                .filter_map(|q| q.0.parse::<u32>().ok())
                .fold(0u32, u32::saturating_add);
            matching
                && ready
                && !node
                    .spec
                    .as_ref()
                    .is_some_and(|s| s.unschedulable == Some(true))
                && allocatable.saturating_sub(used) >= count
        }))
    }

    #[cfg(feature = "kubernetes")]
    async fn objects(&self, id: &str) -> Result<(Option<Pod>, Option<Service>)> {
        let client = self.client().await?;
        let pod = Api::<Pod>::namespaced(client.clone(), &self.namespace)
            .get_opt(id)
            .await
            .map_err(kube_error)?;
        let service = Api::<Service>::namespaced(client, &self.namespace)
            .get_opt(id)
            .await
            .map_err(kube_error)?;
        Ok((pod, service))
    }
}

fn unavailable() -> Error {
    Error::ConfigurationError(
        "CoreWeave requires the kubernetes feature and caller Kubernetes credentials".into(),
    )
}
#[cfg(feature = "kubernetes")]
fn kube_error(error: kube::Error) -> Error {
    match error {
        kube::Error::Api(e) => Error::HttpError(format!("Kubernetes HTTP {}", e.code)),
        _ => Error::HttpError("Kubernetes request failed".into()),
    }
}

fn supported_workload(spec: &ResourceSpec) -> Result<()> {
    if !spec.env.is_empty()
        || !spec.ports.is_empty()
        || spec.command.is_some()
        || spec.endpoint_port.is_some()
        || spec.source_network.is_some()
        || spec.volume_gb.is_some_and(|v| v > 0.0)
        || spec.volume_mount_path.is_some()
        || spec.manifest.is_some()
    {
        return Err(Error::ConfigurationError(
            "This adapter does not support the requested workload/network/volume fields".into(),
        ));
    }
    Ok(())
}

#[async_trait]
impl CloudProviderAdapter for CoreWeaveAdapter {
    fn quote_pricing_source(&self) -> &'static str {
        "configured"
    }
    async fn quote(
        &self,
        spec: &ResourceSpec,
        region: &str,
    ) -> Result<crate::providers::common::InstanceSelection> {
        supported_workload(spec)?;
        let skus = catalog_skus(spec, "coreweave")?;
        #[cfg(feature = "kubernetes")]
        {
            let mut best: Option<(String, f64)> = None;
            for sku in skus {
                let price = self
                    .prices
                    .iter()
                    .filter_map(|p| p.price_for(&sku, region, spec))
                    .min_by(f64::total_cmp);
                if let Some(price) = price {
                    if best.as_ref().is_none_or(|(_, current)| price < *current)
                        && self
                            .available(&sku, region, spec.gpu_count.unwrap_or(0))
                            .await?
                    {
                        best = Some((sku, price));
                    }
                }
            }
            if let Some((sku, price)) = best {
                return quoted_selection(sku, price);
            }
        }
        #[cfg(not(feature = "kubernetes"))]
        let _ = (skus, region);
        Err(Error::ConfigurationError("No in-stock CoreWeave SKU with a dated account contract price for this exact allocation".into()))
    }

    async fn provision_instance(
        &self,
        _instance_type: &str,
        _region: &str,
        _tee: bool,
    ) -> Result<ProvisionedInstance> {
        Err(Error::ConfigurationError(
            "CoreWeave requires provision_resources with class, count and per-lease access".into(),
        ))
    }

    async fn provision_resources(
        &self,
        instance_type: &str,
        region: &str,
        spec: &ResourceSpec,
        tee: bool,
    ) -> Result<ProvisionedInstance> {
        supported_workload(spec)?;
        reject_tee(tee)?;
        if !catalog_skus(spec, "coreweave")?
            .iter()
            .any(|sku| sku == instance_type)
        {
            return Err(Error::ConfigurationError(
                "CoreWeave SKU does not satisfy the request".into(),
            ));
        }
        #[cfg(feature = "kubernetes")]
        {
            let key = spec.ssh_public_key.as_ref().ok_or_else(|| {
                Error::ConfigurationError("A per-lease SSH key is required".into())
            })?;
            let id = format!("gpu-{}", uuid::Uuid::new_v4());
            let pod: Pod = serde_json::from_value(serde_json::json!({"apiVersion":"v1","kind":"Pod",
                "metadata":{"name":id,"labels":{"app":id,"managed-by":"blueprint-remote-providers"}},
                "spec":{"restartPolicy":"Never","activeDeadlineSeconds":spec.lease_duration_secs.unwrap_or(3600),
                    "nodeSelector":{"gpu.nvidia.com/class":instance_type,"topology.kubernetes.io/region":region},
                    "containers":[{"name":"gpu","image":spec.image.as_deref().unwrap_or(DEFAULT_GPU_IMAGE),
                        "env":[{"name":"PUBLIC_KEY","value":key}],"ports":[{"containerPort":22}],
                        "resources":{"requests":{"cpu":spec.cpu.to_string(),"memory":format!("{}Gi",spec.memory_gb),"ephemeral-storage":format!("{}Gi",spec.storage_gb),"nvidia.com/gpu":spec.gpu_count.unwrap_or(0).to_string()},
                            "limits":{"cpu":spec.cpu.to_string(),"memory":format!("{}Gi",spec.memory_gb),"ephemeral-storage":format!("{}Gi",spec.storage_gb),"nvidia.com/gpu":spec.gpu_count.unwrap_or(0).to_string()}}}]}}))
                .map_err(|_|Error::ConfigurationError("Invalid Kubernetes allocation".into()))?;
            let created = Api::<Pod>::namespaced(self.client().await?, &self.namespace)
                .create(&PostParams::default(), &pod)
                .await
                .map_err(kube_error)?;
            return Ok(starting_instance(
                CloudProvider::CoreWeave,
                created.metadata.name.unwrap_or(id),
                instance_type.into(),
                region.into(),
            ));
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (spec, region);
            Err(unavailable())
        }
    }

    async fn prepare_access(
        &self,
        instance: &ProvisionedInstance,
        _spec: &ResourceSpec,
    ) -> Result<()> {
        #[cfg(feature = "kubernetes")]
        {
            let _lifecycle = self.lifecycle.lock().await;
            let pod = Api::<Pod>::namespaced(self.client().await?, &self.namespace)
                .get_opt(&instance.id)
                .await
                .map_err(kube_error)?
                .ok_or_else(|| {
                    Error::ConfigurationError(
                        "CoreWeave workload was retired before access setup".into(),
                    )
                })?;
            let uid = pod.metadata.uid.ok_or_else(|| {
                Error::ConfigurationError("CoreWeave Pod has no authoritative UID".into())
            })?;
            let service: Service=serde_json::from_value(serde_json::json!({"apiVersion":"v1","kind":"Service","metadata":{"name":instance.id,"ownerReferences":[{"apiVersion":"v1","kind":"Pod","name":instance.id,"uid":uid,"controller":true,"blockOwnerDeletion":false}]},
                "spec":{"type":"LoadBalancer","selector":{"app":instance.id},"ports":[{"port":22,"targetPort":22,"protocol":"TCP"}]}}))
                .map_err(|_|unavailable())?;
            let services = Api::<Service>::namespaced(self.client().await?, &self.namespace);
            services
                .create(&PostParams::default(), &service)
                .await
                .map_err(kube_error)?;
            return Ok(());
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = instance;
            Err(unavailable())
        }
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        #[cfg(feature = "kubernetes")]
        {
            let (pod, service) = self.objects(id).await?;
            let mut instance = starting_instance(
                CloudProvider::CoreWeave,
                id.into(),
                "kubernetes-pod".into(),
                String::new(),
            );
            if pod.is_none() && service.is_none() {
                instance.status = InstanceStatus::Terminated;
                return Ok(instance);
            }
            instance.public_ip = service
                .and_then(|s| s.status)
                .and_then(|s| s.load_balancer)
                .and_then(|s| s.ingress)
                .and_then(|mut entries| {
                    if entries.is_empty() {
                        None
                    } else {
                        Some(entries.remove(0))
                    }
                })
                .and_then(|i| i.ip.or(i.hostname));
            instance.status = match pod
                .as_ref()
                .and_then(|p| p.status.as_ref())
                .and_then(|s| s.phase.as_deref())
            {
                Some("Running")
                    if instance.public_ip.is_some()
                        && pod
                            .as_ref()
                            .and_then(|p| p.status.as_ref())
                            .and_then(|s| s.conditions.as_ref())
                            .is_some_and(|c| {
                                c.iter().any(|c| c.type_ == "Ready" && c.status == "True")
                            }) =>
                {
                    InstanceStatus::Running
                }
                Some("Failed" | "Succeeded") | None => InstanceStatus::Stopping,
                _ => InstanceStatus::Starting,
            };
            return Ok(instance);
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = id;
            Err(unavailable())
        }
    }

    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        Ok(self.get_instance_details(id).await?.status)
    }

    async fn terminate_instance(&self, id: &str) -> Result<()> {
        #[cfg(feature = "kubernetes")]
        {
            let _lifecycle = self.lifecycle.lock().await;
            let client = self.client().await?;
            let services = Api::<Service>::namespaced(client.clone(), &self.namespace);
            let pods = Api::<Pod>::namespaced(client, &self.namespace);
            for result in [
                services.delete(id, &Default::default()).await.map(|_| ()),
                pods.delete(id, &Default::default()).await.map(|_| ()),
            ] {
                if let Err(error) = result {
                    if !matches!(&error,kube::Error::Api(e) if e.code==404) {
                        return Err(kube_error(error));
                    }
                }
            }
            return Ok(());
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = id;
            Err(unavailable())
        }
    }

    async fn deploy_blueprint_with_target(
        &self,
        target: &crate::core::deployment_target::DeploymentTarget,
        image: &str,
        spec: &ResourceSpec,
        env: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        use crate::core::deployment_target::DeploymentTarget;
        let namespace = match target {
            DeploymentTarget::GenericKubernetes { namespace, .. }
            | DeploymentTarget::ManagedKubernetes { namespace, .. } => namespace,
            _ => {
                return Err(Error::ConfigurationError(
                    "CoreWeave requires a Kubernetes target".into(),
                ));
            }
        };
        #[cfg(feature = "kubernetes")]
        {
            let namespace = if namespace.is_empty() {
                &self.namespace
            } else {
                namespace
            };
            let selector = std::collections::BTreeMap::from([(
                "gpu.nvidia.com/class".into(),
                crate::providers::common::gpu_adapter::catalog_sku(spec, "coreweave")?,
            )]);
            let deploy = crate::deployment::kubernetes::KubernetesDeploymentClient::with_client(
                self.client().await?,
                namespace.into(),
            )
            .with_node_selector(selector);
            let (name, ports) = deploy
                .deploy_blueprint("blueprint", image, spec, 1, env)
                .await?;
            return Ok(BlueprintDeploymentResult {
                instance: starting_instance(
                    CloudProvider::CoreWeave,
                    name.clone(),
                    "kubernetes-deployment".into(),
                    String::new(),
                ),
                blueprint_id: name,
                port_mappings: ports.into_iter().map(|p| (p, p)).collect(),
                metadata: HashMap::from([("namespace".into(), namespace.into())]),
            });
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = (namespace, image, spec, env);
            Err(unavailable())
        }
    }

    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        #[cfg(feature = "kubernetes")]
        {
            let namespace = deployment
                .metadata
                .get("namespace")
                .unwrap_or(&self.namespace);
            let deployments = Api::<k8s_openapi::api::apps::v1::Deployment>::namespaced(
                self.client().await?,
                namespace,
            );
            return Ok(deployments
                .get_opt(&deployment.blueprint_id)
                .await
                .map_err(kube_error)?
                .and_then(|d| d.status)
                .and_then(|s| s.available_replicas)
                .is_some_and(|n| n > 0));
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = deployment;
            Err(unavailable())
        }
    }
    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        #[cfg(feature = "kubernetes")]
        {
            return crate::deployment::kubernetes::KubernetesDeploymentClient::with_client(
                self.client().await?,
                deployment
                    .metadata
                    .get("namespace")
                    .unwrap_or(&self.namespace)
                    .clone(),
            )
            .cleanup_blueprint(&deployment.blueprint_id)
            .await;
        }
        #[cfg(not(feature = "kubernetes"))]
        {
            let _ = deployment;
            Err(unavailable())
        }
    }
}
