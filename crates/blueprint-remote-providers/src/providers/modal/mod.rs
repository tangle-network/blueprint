//! Modal GPU Sandboxes over the native HTTP/2 API.
//!
//! GPU sandboxes use the V1 backend, an explicit command, bounded CPU/RAM,
//! native lifetime enforcement, and provider-observed encrypted tunnels.

mod client;
mod pricing;
pub use pricing::ModalRateKeys;
mod wire;

use crate::infra::traits::BlueprintDeploymentResult;
use crate::infra::types::TlsMode;
use crate::infra::{CloudProviderAdapter, InstanceStatus, ProvisionedInstance};
use crate::providers::common::{
    InstanceSelection,
    gpu_adapter::{
        catalog_class_for_sku, catalog_skus, managed_resource_name, worker_tls_enabled,
        workload_environment,
    },
};
use crate::{CloudProvider, Error, ManagedResource, ManagedResourceKind, ResourceSpec, Result};
use async_trait::async_trait;
use std::collections::{HashMap, HashSet};
use wire::*;

const APP: &str = "tangle-gpu-leases";
const NAMESPACE: &str = "tangle-lease-namespace";
const LEASE: &str = "tangle-lease-id";
const FINGERPRINT: &str = "tangle-request-fingerprint";
const ENDPOINT: &str = "tangle-endpoint-port";

/// Native API client; account credentials are supplied by the caller.
pub struct ModalAdapter {
    rpc: client::Rpc,
    environment: String,
    rate_keys: Option<ModalRateKeys>,
}

impl ModalAdapter {
    pub fn with_credentials(token_id: String, token_secret: String) -> Result<Self> {
        Self::with_endpoint(token_id, token_secret, "https://api.modal.com:443")
    }

    /// HTTP is restricted to loopback protocol peers; production uses TLS.
    pub fn with_endpoint(token_id: String, token_secret: String, endpoint: &str) -> Result<Self> {
        Ok(Self {
            rpc: client::Rpc::new(token_id, token_secret, endpoint)?,
            environment: String::new(),
            rate_keys: None,
        })
    }

    pub fn with_environment(mut self, environment: String) -> Self {
        self.environment = environment;
        self
    }

    fn shape(spec: &ResourceSpec, region: &str) -> Result<(u32, u32)> {
        spec.validate().map_err(Error::ConfigurationError)?;
        let image = spec.image.as_deref().unwrap_or("");
        if spec.tee_required
            || spec.ssh_public_key.is_some()
            || !spec.allow_spot
            || spec.gpu_count.is_none()
            || spec.command.as_ref().is_none_or(Vec::is_empty)
            || image.is_empty()
            || !image
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b"._/:@-".contains(&c))
            || spec.storage_gb > 512.0
            || spec.volume_gb.is_some()
            || spec.volume_mount_path.is_some()
            || spec.manifest.is_some()
            || !region.is_empty()
            || spec.region.as_ref().is_some_and(|value| !value.is_empty())
            || spec
                .lease_duration_secs
                .is_some_and(|ttl| ttl == 0 || ttl > 86_400)
            || worker_tls_enabled(spec)?
        {
            return Err(Error::ConfigurationError("Modal requires an explicit image/command, preemptible GPUs, native proxy TLS, default-region quota storage and no SSH/TEE/volume".into()));
        }
        let milli_cpu = (f64::from(spec.cpu) * 1000.0).ceil().max(125.0);
        let memory_mib = (f64::from(spec.memory_gb) * 1024.0).ceil();
        if milli_cpu > u32::MAX as f64 || memory_mib > u32::MAX as f64 {
            return Err(Error::ConfigurationError(
                "Modal resource request exceeds protocol bounds".into(),
            ));
        }
        Ok((milli_cpu as u32, memory_mib as u32))
    }

    async fn app(&self, create: bool) -> std::result::Result<String, tonic::Status> {
        let response: Id = self
            .rpc
            .call(
                "/modal.client.ModalClient/AppGetOrCreate",
                AppRequest {
                    name: APP.into(),
                    environment: self.environment.clone(),
                    creation_type: i32::from(create),
                },
            )
            .await?;
        if response.id.is_empty() {
            return Err(tonic::Status::data_loss("Missing app identity"));
        }
        Ok(response.id)
    }

    async fn image(&self, app_id: &str, registry: &str) -> Result<String> {
        let environment: Environment = self
            .rpc
            .call(
                "/modal.client.ModalClient/EnvironmentGetOrCreate",
                Id {
                    id: self.environment.clone(),
                },
            )
            .await
            .map_err(client::error)?;
        let builder = environment
            .metadata
            .and_then(|metadata| metadata.settings)
            .map(|settings| settings.builder)
            .filter(|value| !value.is_empty())
            .ok_or_else(|| {
                Error::ConfigurationError("Modal environment has no image builder setting".into())
            })?;
        let response: ImageResponse = self
            .rpc
            .call(
                "/modal.client.ModalClient/ImageGetOrCreate",
                ImageRequest {
                    app_id: app_id.into(),
                    builder,
                    image: Some(Image {
                        commands: vec![format!("FROM {registry}")],
                    }),
                },
            )
            .await
            .map_err(client::error)?;
        if response.id.is_empty() {
            return Err(Error::HttpError("Modal image identity missing".into()));
        }
        if let Some(result) = response.result.filter(|result| result.status != 0) {
            if result.status == 1 {
                return Ok(response.id);
            }
            return Err(Error::Other("Modal image build failed".into()));
        }
        let image_id = response.id;
        let mut cursor = String::new();
        // Image build logs may contain secrets. Consume only terminal status and cursor.
        let deadline = tokio::time::Instant::now() + std::time::Duration::from_secs(600);
        loop {
            if tokio::time::Instant::now() >= deadline {
                return Err(Error::Other("Modal image build timed out".into()));
            }
            let mut stream = self
                .rpc
                .stream::<_, ImageJoinResponse>(
                    "/modal.client.ModalClient/ImageJoinStreaming",
                    ImageJoinRequest {
                        id: image_id.clone(),
                        timeout: 30.0,
                        last_entry: cursor.clone(),
                    },
                )
                .await
                .map_err(client::error)?;
            while let Some(item) = stream.message().await.map_err(client::error)? {
                if !item.entry.is_empty() {
                    cursor = item.entry;
                }
                if let Some(result) = item.result.filter(|result| result.status != 0) {
                    return if result.status == 1 {
                        Ok(image_id)
                    } else {
                        Err(Error::Other("Modal image build failed".into()))
                    };
                }
            }
        }
    }

    async fn inventory(&self, namespace: Option<&str>) -> Result<Vec<Info>> {
        let app_id = match self.app(false).await {
            Ok(id) => id,
            Err(status) if status.code() == tonic::Code::NotFound => return Ok(Vec::new()),
            Err(status) => return Err(client::error(status)),
        };
        let mut cursor = 0.0;
        let mut seen = HashSet::new();
        let mut resources = Vec::new();
        loop {
            let tags = namespace
                .map(|value| {
                    vec![Tag {
                        name: NAMESPACE.into(),
                        value: value.into(),
                    }]
                })
                .unwrap_or_default();
            let response: ListResponse = self
                .rpc
                .call(
                    "/modal.client.ModalClient/SandboxList",
                    List {
                        app_id: app_id.clone(),
                        before: cursor,
                        environment: self.environment.clone(),
                        finished: false,
                        tags,
                    },
                )
                .await
                .map_err(client::error)?;
            if response.resources.is_empty() {
                return Ok(resources);
            }
            let next = response.resources.last().unwrap().created;
            if !next.is_finite() || next <= 0.0 || (cursor != 0.0 && next >= cursor) {
                return Err(Error::HttpError(
                    "Modal inventory pagination did not advance".into(),
                ));
            }
            for resource in response.resources {
                if resource.id.is_empty() || !seen.insert(resource.id.clone()) {
                    return Err(Error::HttpError(
                        "Modal inventory repeated a resource".into(),
                    ));
                }
                resources.push(resource);
            }
            cursor = next;
        }
    }

    fn status(info: &Info) -> InstanceStatus {
        if info
            .task
            .as_ref()
            .and_then(|task| task.result.as_ref())
            .is_some_and(|result| result.status != 0)
        {
            InstanceStatus::Stopped
        } else if info
            .ready
            .is_some_and(|ready| ready.is_finite() && ready > 0.0)
        {
            InstanceStatus::Running
        } else {
            InstanceStatus::Starting
        }
    }

    fn instance(info: &Info) -> ProvisionedInstance {
        let gpu = info
            .task
            .as_ref()
            .map(|task| {
                task.gpu
                    .as_ref()
                    .map(|gpu| gpu.kind.as_str())
                    .unwrap_or(task.gpu_type.as_str())
            })
            .filter(|value| !value.is_empty())
            .unwrap_or("unknown");
        ProvisionedInstance {
            id: info.id.clone(),
            provider: CloudProvider::Modal,
            instance_type: gpu.into(),
            region: info.regions.join(","),
            public_ip: None,
            private_ip: None,
            status: Self::status(info),
        }
    }

    fn endpoint(tunnels: &[Tunnel], port: u16) -> Result<Option<String>> {
        let Some(tunnel) = tunnels
            .iter()
            .find(|tunnel| tunnel.container_port == u32::from(port))
        else {
            return Ok(None);
        };
        let host = &tunnel.host;
        if host.is_empty()
            || !host
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || b".-".contains(&c))
            || tunnel.port == 0
            || tunnel.port > u16::MAX as u32
        {
            return Err(Error::HttpError("Modal returned an invalid tunnel".into()));
        }
        Ok(Some(if tunnel.port == 443 {
            format!("https://{host}")
        } else {
            format!("https://{host}:{}", tunnel.port)
        }))
    }
}

#[async_trait]
impl CloudProviderAdapter for ModalAdapter {
    async fn quote(&self, spec: &ResourceSpec, region: &str) -> Result<InstanceSelection> {
        let (cpu, memory) = Self::shape(spec, region)?;
        let rates: Rates = self
            .rpc
            .call("/modal.client.ModalClient/WorkspaceBillingRates", Empty {})
            .await
            .map_err(client::error)?;
        let candidates = catalog_skus(spec, "modal")?;
        // Billing key meanings must be bound from an observed account response;
        // opaque or deprecated keys cannot authorize a paid allocation.
        self.select_rate(spec, cpu, memory, candidates, rates)
    }

    async fn provision_instance(&self, _: &str, _: &str, _: bool) -> Result<ProvisionedInstance> {
        Err(Error::ConfigurationError(
            "Modal requires a full workload ResourceSpec".into(),
        ))
    }

    async fn provision_resources(
        &self,
        sku: &str,
        region: &str,
        spec: &ResourceSpec,
        require_tee: bool,
    ) -> Result<ProvisionedInstance> {
        if require_tee {
            return Err(Error::ConfigurationError(
                "Modal has no TEE allocation path".into(),
            ));
        }
        let (cpu, memory) = Self::shape(spec, region)?;
        let selection = self.quote(spec, region).await?;
        if selection.instance_type != sku
            || selection
                .estimated_hourly_cost
                .is_none_or(|price| spec.max_hourly_cost_usd.is_some_and(|cap| price > cap))
        {
            return Err(Error::ConfigurationError(
                "Modal SKU or live rate changed before allocation".into(),
            ));
        }
        let identity = spec.ownership.as_ref().ok_or_else(|| {
            Error::ConfigurationError("Modal creation requires managed ownership".into())
        })?;
        identity.validate().map_err(Error::ConfigurationError)?;
        let name = managed_resource_name(&identity.namespace, &identity.lease_id)?;
        let class = catalog_class_for_sku(spec, "modal", sku)?;
        let fingerprint = crate::managed::request_fingerprint(spec)?;
        let existing = self.list_managed(&identity.namespace).await?;
        if let Some(existing) = existing
            .into_iter()
            .find(|resource| resource.lease_id == identity.lease_id)
        {
            return if existing.request_fingerprint.as_deref() == Some(&fingerprint) {
                Ok(existing.instance)
            } else {
                Err(Error::ConfigurationError(
                    "Modal lease identity belongs to another allocation request".into(),
                ))
            };
        }
        let app_id = self.app(true).await.map_err(client::error)?;
        let image = self.image(&app_id, spec.image.as_deref().unwrap()).await?;
        let environment = workload_environment(spec, Some("public"))?;
        let secret: Id = self
            .rpc
            .call(
                "/modal.client.ModalClient/SecretGetOrCreate",
                SecretRequest {
                    environment: self.environment.clone(),
                    creation_type: 5,
                    values: environment.into_iter().collect(),
                },
            )
            .await
            .map_err(client::error)?;
        if secret.id.is_empty() {
            return Err(Error::HttpError(
                "Modal ephemeral secret identity missing".into(),
            ));
        }
        let mut tags = vec![
            Tag {
                name: NAMESPACE.into(),
                value: identity.namespace.clone(),
            },
            Tag {
                name: LEASE.into(),
                value: identity.lease_id.clone(),
            },
            Tag {
                name: FINGERPRINT.into(),
                value: fingerprint,
            },
            Tag {
                name: "tangle-gpu-class".into(),
                value: class,
            },
        ];
        if let Some(port) = spec.endpoint_port {
            tags.push(Tag {
                name: ENDPOINT.into(),
                value: port.to_string(),
            });
        }
        let created: Id = self
            .rpc
            .call(
                "/modal.client.ModalClient/SandboxCreate",
                Create {
                    app_id,
                    tags,
                    definition: Some(Sandbox {
                        command: spec.command.clone().unwrap(),
                        image,
                        secrets: vec![secret.id],
                        resources: Some(Resources {
                            memory_mib: memory,
                            memory_limit_mib: memory,
                            milli_cpu: cpu,
                            milli_cpu_limit: cpu,
                            gpu: Some(Gpu {
                                kind: sku.into(),
                                count: spec.gpu_count.unwrap(),
                            }),
                        }),
                        ttl: spec.lease_duration_secs.unwrap_or(300) as u32,
                        ports: Some(Ports {
                            ports: spec
                                .ports
                                .iter()
                                .map(|port| Port {
                                    port: u32::from(*port),
                                    unencrypted: false,
                                })
                                .collect(),
                        }),
                        runtime: Some("gvisor".into()),
                        name: Some(name),
                        inbound_cidrs: spec.source_network.iter().cloned().collect(),
                    }),
                },
            )
            .await
            .map_err(client::error)?;
        if created.id.is_empty() {
            return Err(Error::HttpError("Modal sandbox identity missing".into()));
        }
        Ok(ProvisionedInstance {
            id: created.id,
            provider: CloudProvider::Modal,
            instance_type: sku.into(),
            region: String::new(),
            public_ip: None,
            private_ip: None,
            status: InstanceStatus::Starting,
        })
    }

    async fn list_managed(&self, namespace: &str) -> Result<Vec<ManagedResource>> {
        let mut resources = Vec::new();
        for info in self.inventory(Some(namespace)).await? {
            let tags: HashMap<_, _> = info
                .tags
                .iter()
                .map(|tag| (tag.name.as_str(), tag.value.as_str()))
                .collect();
            if tags.get(NAMESPACE).copied() != Some(namespace) {
                continue;
            }
            let Some(lease_id) = tags.get(LEASE) else {
                return Err(Error::HttpError(
                    "Managed Modal resource is missing its lease identity".into(),
                ));
            };
            let identity = blueprint_core::resources::ManagedLeaseIdentity {
                namespace: namespace.into(),
                lease_id: (*lease_id).into(),
            };
            identity.validate().map_err(Error::ConfigurationError)?;
            let endpoint = tags
                .get(ENDPOINT)
                .and_then(|value| value.parse().ok())
                .map(|port| Self::endpoint(&info.tunnels, port))
                .transpose()?
                .flatten();
            resources.push(ManagedResource {
                instance: Self::instance(&info),
                namespace: namespace.into(),
                lease_id: (*lease_id).into(),
                kind: ManagedResourceKind::Instance,
                tls_mode: endpoint.as_ref().map(|_| TlsMode::Public),
                endpoint,
                request_fingerprint: tags.get(FINGERPRINT).map(|value| (*value).into()),
                hourly_cost_usd: None,
                expires_at: crate::providers::common::gpu_adapter::provider_created_at(
                    &serde_json::json!(info.created),
                )
                .and_then(|created| {
                    (info.ttl > 0).then_some(info.ttl).and_then(|ttl| {
                        created.checked_add_signed(chrono::Duration::seconds(i64::from(ttl)))
                    })
                }),
                expiry_source: (info.ttl > 0 && info.created.is_finite() && info.created > 0.0)
                    .then(|| "derived".into()),
                created_at: info.task.as_ref().and_then(|task| {
                    crate::providers::common::gpu_adapter::provider_created_at(&serde_json::json!(
                        task.started
                    ))
                }),
            });
        }
        Ok(resources)
    }

    async fn application_endpoint(
        &self,
        instance: &ProvisionedInstance,
        spec: &ResourceSpec,
    ) -> Result<Option<String>> {
        let Some(port) = spec.endpoint_port else {
            return Ok(None);
        };
        let response: Tunnels = self
            .rpc
            .call(
                "/modal.client.ModalClient/SandboxGetTunnels",
                Poll {
                    id: instance.id.clone(),
                    timeout: 0.0,
                },
            )
            .await
            .map_err(client::error)?;
        Self::endpoint(&response.tunnels, port)
    }

    fn application_tls_mode(&self, spec: &ResourceSpec) -> Option<TlsMode> {
        spec.endpoint_port.map(|_| TlsMode::Public)
    }

    async fn get_instance_status(&self, id: &str) -> Result<InstanceStatus> {
        match self
            .rpc
            .call::<_, PollResponse>(
                "/modal.client.ModalClient/SandboxWait",
                Poll {
                    id: id.into(),
                    timeout: 0.0,
                },
            )
            .await
        {
            Ok(response) => {
                if response.result.is_some_and(|result| result.status != 0) {
                    return Ok(InstanceStatus::Terminated);
                }
                Ok(self
                    .inventory(None)
                    .await?
                    .iter()
                    .find(|info| info.id == id)
                    .map(Self::status)
                    .unwrap_or(InstanceStatus::Starting))
            }
            Err(status) if status.code() == tonic::Code::NotFound => Ok(InstanceStatus::Terminated),
            Err(status) => Err(client::error(status)),
        }
    }

    async fn get_instance_details(&self, id: &str) -> Result<ProvisionedInstance> {
        self.inventory(None)
            .await?
            .iter()
            .find(|info| info.id == id)
            .map(Self::instance)
            .ok_or_else(|| Error::Other("Modal sandbox not found".into()))
    }

    async fn terminate_instance(&self, id: &str) -> Result<()> {
        match self
            .rpc
            .call::<_, Empty>(
                "/modal.client.ModalClient/SandboxTerminate",
                Id { id: id.into() },
            )
            .await
        {
            Ok(_) => Ok(()),
            Err(status) if status.code() == tonic::Code::NotFound => Ok(()),
            Err(status) => Err(client::error(status)),
        }
    }

    async fn deploy_blueprint_with_target(
        &self,
        _: &crate::core::deployment_target::DeploymentTarget,
        _: &str,
        _: &ResourceSpec,
        _: HashMap<String, String>,
    ) -> Result<BlueprintDeploymentResult> {
        Err(Error::ConfigurationError(
            "Modal starts the supplied workload atomically at creation".into(),
        ))
    }
    async fn health_check_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<bool> {
        Ok(self
            .get_instance_details(&deployment.instance.id)
            .await?
            .status
            == InstanceStatus::Running)
    }
    async fn cleanup_blueprint(&self, deployment: &BlueprintDeploymentResult) -> Result<()> {
        self.terminate_instance(&deployment.instance.id).await
    }
}
