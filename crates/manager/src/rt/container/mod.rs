use crate::config::BlueprintManagerContext;
use crate::error::{Error, Result};
use crate::rt::ResourceLimits;
use crate::rt::service::Status;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use blueprint_core::{info, warn};
use k8s_openapi::api::core::v1::{
    Container, EndpointAddress, EndpointPort, EndpointSubset, Endpoints, EnvVar,
    HostPathVolumeSource, Namespace, Node, Pod, PodSpec, ResourceRequirements, Service,
    ServicePort, ServiceSpec, Volume, VolumeMount,
};
use k8s_openapi::api::node::v1::RuntimeClass;
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use kube::Client;
use kube::api::{Api, DeleteParams, ListParams, Patch, PatchParams, PostParams};
use std::collections::BTreeMap;
use std::net::IpAddr;
use url::{Host, Url};

const BLUEPRINT_NAMESPACE: &str = "blueprint-manager";
const BLUEPRINT_SERVICE: &str = "blueprint-service";
const KEYSTORE_PATH: &str = "/mnt/keystore";

async fn detect_kata(client: Client) -> Result<bool> {
    let runtimeclass_api: Api<RuntimeClass> = Api::all(client.clone());
    let runtimeclass_list = runtimeclass_api.list(&ListParams::default()).await?;

    for runtimeclass in runtimeclass_list.items {
        if runtimeclass.metadata.name.as_deref() == Some("kata") {
            return Ok(true);
        }
    }

    warn!("Kata Containers runtime not found, blueprint will **not** be sandboxed");
    Ok(false)
}

pub struct ContainerInstance {
    client: Client,
    local_ip: IpAddr,
    service_port: u16,

    limits: ResourceLimits,
    service_name: String,
    image: String,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
    debug: bool,
}

impl ContainerInstance {
    /// Create a new `ContainerInstance`
    pub async fn new(
        ctx: &BlueprintManagerContext,
        limits: ResourceLimits,
        service_name: &str,
        image: String,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
        debug: bool,
    ) -> ContainerInstance {
        Self {
            client: ctx.containers.kube_client.clone(),
            local_ip: ctx.containers.local_ip,
            service_port: ctx.kube_service_port().await,

            limits,
            service_name: service_name.into(),
            image,
            env,
            args,
            debug,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        /// TODO: actually resolve the hosts to see if they're loopback
        // For local testnets, we need to translate IPs to the host
        fn translate_local_ip(url: &mut Url, host_ip: IpAddr) {
            match url.host() {
                Some(Host::Ipv4(ip)) if ip.is_loopback() => {
                    let _ = url.set_ip_host(host_ip).ok();
                }
                _ => {}
            }
        }

        self.ensure_namespace().await?;
        self.ensure_host_service().await?;
        self.ensure_host_endpoints().await?;

        let runtime = if matches!(detect_kata(self.client.clone()).await, Ok(true)) {
            Some(String::from("kata"))
        } else {
            None
        };

        let pods: Api<Pod> = Api::namespaced(self.client.clone(), BLUEPRINT_NAMESPACE);

        let mem_mib = (self.limits.memory_size / 1024) / 1024;

        let host_keystore_path = self.env.keystore_uri.clone();
        self.env.keystore_uri = KEYSTORE_PATH.to_string();

        translate_local_ip(&mut self.env.http_rpc_endpoint, self.local_ip);
        translate_local_ip(&mut self.env.ws_rpc_endpoint, self.local_ip);

        let env = self
            .env
            .encode()
            .into_iter()
            .map(|(k, v)| EnvVar {
                name: k,
                value: Some(v),
                value_from: None,
            })
            .collect::<Vec<_>>();

        let pod = Pod {
            metadata: ObjectMeta {
                name: Some(self.service_name.clone()),
                labels: Some([("app".to_string(), "blueprint".to_string())].into()),
                annotations: {
                    let mut annotations = BTreeMap::new();
                    if let Some(runtime) = runtime.clone() {
                        annotations
                            .insert(String::from("io.containerd.cri.runtime-handler"), runtime);
                    }
                    Some(annotations)
                },
                ..Default::default()
            },
            spec: Some(PodSpec {
                runtime_class_name: runtime.clone(),
                dns_policy: Some("ClusterFirst".to_string()),
                containers: vec![Container {
                    name: self.service_name.clone(),
                    image: Some(self.image.clone()),
                    resources: Some(ResourceRequirements {
                        limits: Some(
                            [("memory".to_string(), Quantity(format!("{mem_mib}Mi")))].into(),
                        ),
                        requests: Some(
                            [("memory".to_string(), Quantity(format!("{mem_mib}Mi")))].into(),
                        ),
                        ..Default::default()
                    }),
                    env: Some(env),
                    args: Some(self.args.encode(false)),
                    volume_mounts: Some(vec![VolumeMount {
                        name: "keystore-volume".to_string(),
                        mount_path: KEYSTORE_PATH.to_string(),
                        read_only: Some(true),
                        ..Default::default()
                    }]),
                    ..Default::default()
                }],
                volumes: Some(vec![Volume {
                    name: "keystore-volume".to_string(),
                    host_path: Some(HostPathVolumeSource {
                        path: host_keystore_path,
                        type_: Some("Directory".to_string()),
                    }),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };

        let pp = PatchParams::apply("blueprint-mgr").force();
        pods.patch(&self.service_name, &pp, &Patch::Apply(&pod))
            .await?;

        info!(target: "containers", service_name = self.service_name, "Pod started successfully.");
        Ok(())
    }

    /// Fetches the current status of the Pod from Kubernetes
    pub async fn status(&self) -> Result<Status> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), BLUEPRINT_NAMESPACE);
        let pod = pods.get(&self.service_name).await?;

        let phase = pod.status.and_then(|s| s.phase).unwrap_or_default();
        info!(target: "containers", service_name = self.service_name, phase = phase, "Checked pod status");

        let status = match phase.as_str() {
            "Running" => Status::Running,
            "Pending" => Status::Pending,
            "Failed" => Status::Error,
            "Succeeded" => Status::Finished,
            _ => Status::Unknown,
        };

        Ok(status)
    }

    /// Deletes the Pod from the Kubernetes cluster
    pub async fn shutdown(self) -> Result<()> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), BLUEPRINT_NAMESPACE);
        info!(target: "containers", service_name = self.service_name, "Shutting down pod...");

        match pods
            .delete(&self.service_name, &DeleteParams::default())
            .await
        {
            Ok(_) => {
                info!(target: "containers", service_name = self.service_name, "Pod deleted successfully.");
                Ok(())
            }
            Err(kube::Error::Api(e)) if e.code == 404 => {
                warn!(target: "containers", service_name = self.service_name, "Pod was already deleted.");
                Ok(())
            }
            Err(e) => Err(e.into()),
        }
    }

    async fn ensure_namespace(&self) -> Result<()> {
        let namespaces: Api<Namespace> = Api::all(self.client.clone());

        let new_ns = Namespace {
            metadata: ObjectMeta {
                name: Some(BLUEPRINT_NAMESPACE.to_string()),
                ..Default::default()
            },
            ..Default::default()
        };

        match namespaces.create(&PostParams::default(), &new_ns).await {
            Ok(o) => {
                info!(target: "containers", "Created namespace '{}'", o.metadata.name.unwrap_or_default());
            }
            // Already exists
            Err(kube::Error::Api(e)) if e.code == 409 => {}
            Err(e) => return Err(e.into()),
        }
        Ok(())
    }

    async fn ensure_host_service(&self) -> Result<()> {
        let services: Api<Service> = Api::namespaced(self.client.clone(), BLUEPRINT_NAMESPACE);

        let service = Service {
            metadata: ObjectMeta {
                name: Some(String::from(BLUEPRINT_SERVICE)),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                ports: Some(vec![ServicePort {
                    port: 8080,
                    protocol: Some("TCP".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }),
            ..Default::default()
        };

        let pp = PatchParams::apply("blueprint-mgr").force();
        services
            .patch(BLUEPRINT_SERVICE, &pp, &Patch::Apply(&service))
            .await?;
        Ok(())
    }

    async fn ensure_host_endpoints(&self) -> Result<()> {
        let endpoints_api: Api<Endpoints> =
            Api::namespaced(self.client.clone(), BLUEPRINT_NAMESPACE);

        let endpoints = Endpoints {
            metadata: ObjectMeta {
                name: Some(String::from(BLUEPRINT_SERVICE)),
                ..Default::default()
            },
            subsets: Some(vec![EndpointSubset {
                addresses: Some(vec![EndpointAddress {
                    ip: self.local_ip.to_string(),
                    ..Default::default()
                }]),
                ports: Some(vec![EndpointPort {
                    port: i32::from(self.service_port),
                    protocol: Some("TCP".to_string()),
                    ..Default::default()
                }]),
                ..Default::default()
            }]),
        };

        let pp = PatchParams::apply("blueprint-mgr").force();
        endpoints_api
            .patch(BLUEPRINT_SERVICE, &pp, &Patch::Apply(&endpoints))
            .await?;
        Ok(())
    }
}
