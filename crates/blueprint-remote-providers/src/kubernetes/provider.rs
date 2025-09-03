use crate::{
    error::{Error, Result},
    provider::{ProviderType, RemoteInfrastructureProvider},
    types::{
        Cost, DeploymentSpec, InstanceId, InstanceStatus, RemoteInstance, Resources,
        ServiceEndpoint, TunnelHandle, TunnelHub, Protocol,
    },
};
use async_trait::async_trait;
use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec as K8sDeploymentSpec},
    core::v1::{
        Container, ContainerPort, EnvVar, Namespace, Pod, PodSpec, PodTemplateSpec,
        ResourceRequirements, Service, ServicePort, ServiceSpec, VolumeMount as K8sVolumeMount,
    },
};
use kube::{
    api::{Api, ListParams, ObjectMeta, PostParams},
    Client, Config, ResourceExt,
};
use serde::{Deserialize, Serialize};
use std::collections::{BTreeMap, HashMap};
use std::path::PathBuf;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KubernetesConfig {
    pub kubeconfig_path: Option<PathBuf>,
    pub context: Option<String>,
    pub namespace: String,
    pub service_type: ServiceType,
    pub storage_class: Option<String>,
    pub image_pull_secret: Option<String>,
}

impl Default for KubernetesConfig {
    fn default() -> Self {
        Self {
            kubeconfig_path: None,
            context: None,
            namespace: "blueprint-remote".to_string(),
            service_type: ServiceType::ClusterIP,
            storage_class: None,
            image_pull_secret: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServiceType {
    ClusterIP,
    NodePort,
    LoadBalancer,
}

pub struct KubernetesProvider {
    name: String,
    config: KubernetesConfig,
    client: Client,
}

impl KubernetesProvider {
    pub async fn new(name: impl Into<String>, config: KubernetesConfig) -> Result<Self> {
        let kube_config = if let Some(ref path) = config.kubeconfig_path {
            Config::from_kubeconfig(&tokio::fs::read_to_string(path).await?).await?
        } else {
            Config::infer().await?
        };
        
        let kube_config = if let Some(ref context) = config.context {
            kube_config.with_context(context.clone())
                .ok_or_else(|| Error::ConfigurationError(format!("Context {} not found", context)))?
        } else {
            kube_config
        };
        
        let client = Client::try_from(kube_config)?;
        
        Self::ensure_namespace(&client, &config.namespace).await?;
        
        Ok(Self {
            name: name.into(),
            config,
            client,
        })
    }
    
    async fn ensure_namespace(client: &Client, namespace: &str) -> Result<()> {
        let namespaces: Api<Namespace> = Api::all(client.clone());
        
        match namespaces.get(namespace).await {
            Ok(_) => {
                debug!("Namespace {} already exists", namespace);
                Ok(())
            }
            Err(kube::Error::Api(err)) if err.code == 404 => {
                info!("Creating namespace {}", namespace);
                let ns = Namespace {
                    metadata: ObjectMeta {
                        name: Some(namespace.to_string()),
                        labels: Some({
                            let mut labels = BTreeMap::new();
                            labels.insert("managed-by".to_string(), "blueprint-remote".to_string());
                            labels
                        }),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                
                namespaces.create(&PostParams::default(), &ns).await?;
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }
    
    fn deployment_from_spec(&self, spec: &DeploymentSpec) -> Deployment {
        let labels = {
            let mut labels = BTreeMap::new();
            labels.insert("app".to_string(), spec.name.clone());
            labels.insert("managed-by".to_string(), "blueprint-remote".to_string());
            for (k, v) in &spec.labels {
                labels.insert(k.clone(), v.clone());
            }
            labels
        };
        
        let env_vars: Vec<EnvVar> = spec
            .environment
            .iter()
            .map(|(k, v)| EnvVar {
                name: k.clone(),
                value: Some(v.clone()),
                ..Default::default()
            })
            .collect();
        
        let ports: Vec<ContainerPort> = spec
            .ports
            .iter()
            .map(|p| ContainerPort {
                name: Some(p.name.clone()),
                container_port: p.container_port as i32,
                protocol: Some(match p.protocol {
                    Protocol::TCP => "TCP".to_string(),
                    Protocol::UDP => "UDP".to_string(),
                }),
                ..Default::default()
            })
            .collect();
        
        let volume_mounts: Vec<K8sVolumeMount> = spec
            .volumes
            .iter()
            .map(|v| K8sVolumeMount {
                name: v.name.clone(),
                mount_path: v.mount_path.to_string_lossy().to_string(),
                read_only: Some(v.read_only),
                ..Default::default()
            })
            .collect();
        
        let mut resources = ResourceRequirements::default();
        let mut limits = BTreeMap::new();
        let mut requests = BTreeMap::new();
        
        if let Some(cpu) = &spec.resources.cpu {
            limits.insert("cpu".to_string(), cpu.clone());
            requests.insert("cpu".to_string(), cpu.clone());
        }
        if let Some(memory) = &spec.resources.memory {
            limits.insert("memory".to_string(), memory.clone());
            requests.insert("memory".to_string(), memory.clone());
        }
        
        resources.limits = Some(limits);
        resources.requests = Some(requests);
        
        let container = Container {
            name: spec.name.clone(),
            image: Some(format!("{}:{}", spec.image.repository, spec.image.tag)),
            env: if env_vars.is_empty() { None } else { Some(env_vars) },
            ports: if ports.is_empty() { None } else { Some(ports) },
            volume_mounts: if volume_mounts.is_empty() { None } else { Some(volume_mounts) },
            resources: Some(resources),
            ..Default::default()
        };
        
        Deployment {
            metadata: ObjectMeta {
                name: Some(spec.name.clone()),
                namespace: Some(self.config.namespace.clone()),
                labels: Some(labels.clone()),
                ..Default::default()
            },
            spec: Some(K8sDeploymentSpec {
                replicas: Some(spec.replicas as i32),
                selector: k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
                    match_labels: Some(labels.clone()),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(ObjectMeta {
                        labels: Some(labels),
                        ..Default::default()
                    }),
                    spec: Some(PodSpec {
                        containers: vec![container],
                        ..Default::default()
                    }),
                },
                ..Default::default()
            }),
            ..Default::default()
        }
    }
    
    fn service_from_spec(&self, spec: &DeploymentSpec) -> Service {
        let labels = {
            let mut labels = BTreeMap::new();
            labels.insert("app".to_string(), spec.name.clone());
            labels.insert("managed-by".to_string(), "blueprint-remote".to_string());
            labels
        };
        
        let ports: Vec<ServicePort> = spec
            .ports
            .iter()
            .map(|p| ServicePort {
                name: Some(p.name.clone()),
                port: p.container_port as i32,
                target_port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(
                    p.container_port as i32,
                )),
                protocol: Some(match p.protocol {
                    Protocol::TCP => "TCP".to_string(),
                    Protocol::UDP => "UDP".to_string(),
                }),
                ..Default::default()
            })
            .collect();
        
        Service {
            metadata: ObjectMeta {
                name: Some(spec.name.clone()),
                namespace: Some(self.config.namespace.clone()),
                labels: Some(labels.clone()),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                selector: Some(labels),
                ports: if ports.is_empty() { None } else { Some(ports) },
                type_: Some(match self.config.service_type {
                    ServiceType::ClusterIP => "ClusterIP",
                    ServiceType::NodePort => "NodePort",
                    ServiceType::LoadBalancer => "LoadBalancer",
                }.to_string()),
                ..Default::default()
            }),
            ..Default::default()
        }
    }
    
    async fn get_deployment(&self, name: &str) -> Result<Option<Deployment>> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        match deployments.get(name).await {
            Ok(deployment) => Ok(Some(deployment)),
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
    
    async fn get_service_endpoint(&self, name: &str) -> Result<Option<ServiceEndpoint>> {
        let services: Api<Service> = Api::namespaced(self.client.clone(), &self.config.namespace);
        
        match services.get(name).await {
            Ok(service) => {
                if let Some(spec) = service.spec {
                    if let Some(ports) = spec.ports {
                        if let Some(port) = ports.first() {
                            let endpoint = match self.config.service_type {
                                ServiceType::LoadBalancer => {
                                    if let Some(status) = service.status {
                                        if let Some(lb) = status.load_balancer {
                                            if let Some(ingress) = lb.ingress {
                                                if let Some(first) = ingress.first() {
                                                    first.hostname.clone()
                                                        .or_else(|| first.ip.clone())
                                                        .unwrap_or_else(|| spec.cluster_ip.unwrap_or_default())
                                                } else {
                                                    spec.cluster_ip.unwrap_or_default()
                                                }
                                            } else {
                                                spec.cluster_ip.unwrap_or_default()
                                            }
                                        } else {
                                            spec.cluster_ip.unwrap_or_default()
                                        }
                                    } else {
                                        spec.cluster_ip.unwrap_or_default()
                                    }
                                }
                                _ => spec.cluster_ip.unwrap_or_default(),
                            };
                            
                            return Ok(Some(ServiceEndpoint {
                                host: endpoint,
                                port: port.port as u16,
                                protocol: Protocol::TCP,
                                tunnel_required: matches!(self.config.service_type, ServiceType::ClusterIP),
                            }));
                        }
                    }
                }
                Ok(None)
            }
            Err(kube::Error::Api(err)) if err.code == 404 => Ok(None),
            Err(e) => Err(e.into()),
        }
    }
}

#[async_trait]
impl RemoteInfrastructureProvider for KubernetesProvider {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn provider_type(&self) -> ProviderType {
        ProviderType::Kubernetes
    }
    
    async fn deploy_instance(&self, spec: DeploymentSpec) -> Result<RemoteInstance> {
        let deployment = self.deployment_from_spec(&spec);
        let service = self.service_from_spec(&spec);
        
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let services: Api<Service> = Api::namespaced(self.client.clone(), &self.config.namespace);
        
        let created_deployment = deployments.create(&PostParams::default(), &deployment).await?;
        let deployment_uid = created_deployment.metadata.uid.unwrap_or_default();
        
        if !spec.ports.is_empty() {
            services.create(&PostParams::default(), &service).await?;
        }
        
        let mut instance = RemoteInstance::new(deployment_uid, spec.name, self.name.clone());
        instance.region = self.config.context.clone();
        instance.status = InstanceStatus::Pending;
        
        if let Some(endpoint) = self.get_service_endpoint(&instance.name).await? {
            instance.endpoint = Some(endpoint);
        }
        
        Ok(instance)
    }
    
    async fn get_instance_status(&self, id: &InstanceId) -> Result<InstanceStatus> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let list_params = ListParams::default().fields(&format!("metadata.uid={}", id.as_str()));
        
        let deployment_list = deployments.list(&list_params).await?;
        
        if let Some(deployment) = deployment_list.items.first() {
            if let Some(status) = &deployment.status {
                let replicas = status.replicas.unwrap_or(0);
                let ready_replicas = status.ready_replicas.unwrap_or(0);
                let unavailable = status.unavailable_replicas.unwrap_or(0);
                
                if unavailable > 0 {
                    return Ok(InstanceStatus::Pending);
                }
                
                if ready_replicas == replicas && replicas > 0 {
                    return Ok(InstanceStatus::Running);
                }
                
                if replicas == 0 {
                    return Ok(InstanceStatus::Stopped);
                }
                
                return Ok(InstanceStatus::Pending);
            }
        }
        
        Ok(InstanceStatus::Unknown)
    }
    
    async fn terminate_instance(&self, id: &InstanceId) -> Result<()> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let services: Api<Service> = Api::namespaced(self.client.clone(), &self.config.namespace);
        
        let list_params = ListParams::default().fields(&format!("metadata.uid={}", id.as_str()));
        let deployment_list = deployments.list(&list_params).await?;
        
        if let Some(deployment) = deployment_list.items.first() {
            let name = deployment.metadata.name.clone().unwrap_or_default();
            
            deployments.delete(&name, &Default::default()).await?;
            
            match services.delete(&name, &Default::default()).await {
                Ok(_) => {}
                Err(kube::Error::Api(err)) if err.code == 404 => {}
                Err(e) => return Err(e.into()),
            }
        }
        
        Ok(())
    }
    
    async fn get_instance_endpoint(&self, id: &InstanceId) -> Result<Option<ServiceEndpoint>> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let list_params = ListParams::default().fields(&format!("metadata.uid={}", id.as_str()));
        
        let deployment_list = deployments.list(&list_params).await?;
        
        if let Some(deployment) = deployment_list.items.first() {
            let name = deployment.metadata.name.clone().unwrap_or_default();
            return self.get_service_endpoint(&name).await;
        }
        
        Ok(None)
    }
    
    async fn establish_tunnel(&self, hub: &TunnelHub) -> Result<TunnelHandle> {
        // This would integrate with WireGuard to establish a tunnel
        // For now, return a mock implementation
        warn!("Tunnel establishment not yet implemented for Kubernetes provider");
        
        Ok(TunnelHandle {
            interface: "wg0".to_string(),
            peer_endpoint: format!("{}:{}", hub.endpoint, hub.port),
            local_address: "10.100.0.2".to_string(),
            remote_address: "10.100.0.1".to_string(),
        })
    }
    
    async fn get_available_resources(&self) -> Result<Resources> {
        let nodes: Api<k8s_openapi::api::core::v1::Node> = Api::all(self.client.clone());
        let node_list = nodes.list(&ListParams::default()).await?;
        
        let mut total_cpu = 0i64;
        let mut total_memory = 0i64;
        let mut available_cpu = 0i64;
        let mut available_memory = 0i64;
        
        for node in node_list.items {
            if let Some(status) = node.status {
                if let Some(allocatable) = status.allocatable {
                    if let Some(cpu) = allocatable.get("cpu") {
                        if let Some(quantity) = cpu.0.parse::<i64>().ok() {
                            available_cpu += quantity;
                        }
                    }
                    if let Some(memory) = allocatable.get("memory") {
                        if let Some(quantity) = memory.0.parse::<i64>().ok() {
                            available_memory += quantity;
                        }
                    }
                }
                
                if let Some(capacity) = status.capacity {
                    if let Some(cpu) = capacity.get("cpu") {
                        if let Some(quantity) = cpu.0.parse::<i64>().ok() {
                            total_cpu += quantity;
                        }
                    }
                    if let Some(memory) = capacity.get("memory") {
                        if let Some(quantity) = memory.0.parse::<i64>().ok() {
                            total_memory += quantity;
                        }
                    }
                }
            }
        }
        
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let deployment_list = deployments.list(&ListParams::default()).await?;
        
        Ok(Resources {
            total_cpu: total_cpu.to_string(),
            total_memory: format!("{}Gi", total_memory / (1024 * 1024 * 1024)),
            available_cpu: available_cpu.to_string(),
            available_memory: format!("{}Gi", available_memory / (1024 * 1024 * 1024)),
            max_instances: 100,
            current_instances: deployment_list.items.len() as u32,
        })
    }
    
    async fn estimate_cost(&self, spec: &DeploymentSpec) -> Result<Cost> {
        let cpu_cores = spec.resources.cpu
            .as_ref()
            .and_then(|s| s.parse::<f64>().ok())
            .unwrap_or(1.0);
        
        let memory_gb = spec.resources.memory
            .as_ref()
            .and_then(|s| {
                if s.ends_with("Gi") {
                    s.trim_end_matches("Gi").parse::<f64>().ok()
                } else if s.ends_with("Mi") {
                    s.trim_end_matches("Mi").parse::<f64>().ok().map(|m| m / 1024.0)
                } else {
                    None
                }
            })
            .unwrap_or(1.0);
        
        // Rough cloud cost estimates
        let cpu_hour_cost = 0.0464;  // ~$34/month per vCPU
        let memory_hour_cost = 0.004; // ~$3/month per GB
        
        let hourly = (cpu_cores * cpu_hour_cost + memory_gb * memory_hour_cost) * spec.replicas as f64;
        let monthly = hourly * 730.0;
        
        let mut breakdown = HashMap::new();
        breakdown.insert("compute".to_string(), cpu_cores * cpu_hour_cost * 730.0 * spec.replicas as f64);
        breakdown.insert("memory".to_string(), memory_gb * memory_hour_cost * 730.0 * spec.replicas as f64);
        
        Ok(Cost {
            estimated_hourly: hourly,
            estimated_monthly: monthly,
            currency: "USD".to_string(),
            breakdown,
        })
    }
    
    async fn list_instances(&self) -> Result<Vec<RemoteInstance>> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let deployment_list = deployments.list(&ListParams::default()).await?;
        
        let mut instances = Vec::new();
        
        for deployment in deployment_list.items {
            let uid = deployment.metadata.uid.unwrap_or_default();
            let name = deployment.metadata.name.unwrap_or_default();
            
            let status = if let Some(status) = &deployment.status {
                let ready = status.ready_replicas.unwrap_or(0);
                let total = status.replicas.unwrap_or(0);
                
                if ready == total && total > 0 {
                    InstanceStatus::Running
                } else if total == 0 {
                    InstanceStatus::Stopped
                } else {
                    InstanceStatus::Pending
                }
            } else {
                InstanceStatus::Unknown
            };
            
            let mut instance = RemoteInstance::new(uid, name.clone(), self.name.clone());
            instance.status = status;
            instance.region = self.config.context.clone();
            
            if let Some(endpoint) = self.get_service_endpoint(&name).await? {
                instance.endpoint = Some(endpoint);
            }
            
            instances.push(instance);
        }
        
        Ok(instances)
    }
    
    async fn scale_instance(&self, id: &InstanceId, replicas: u32) -> Result<()> {
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.config.namespace);
        let list_params = ListParams::default().fields(&format!("metadata.uid={}", id.as_str()));
        
        let deployment_list = deployments.list(&list_params).await?;
        
        if let Some(mut deployment) = deployment_list.items.into_iter().next() {
            if let Some(spec) = &mut deployment.spec {
                spec.replicas = Some(replicas as i32);
            }
            
            let name = deployment.metadata.name.clone().unwrap_or_default();
            deployments.replace(&name, &PostParams::default(), &deployment).await?;
            
            Ok(())
        } else {
            Err(Error::InstanceNotFound(id.to_string()))
        }
    }
}