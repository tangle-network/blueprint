use crate::error::Result;
use crate::rt::ResourceLimits;
use crate::rt::service::Status;
use crate::sources::{BlueprintArgs, BlueprintEnvVars};
use k8s_openapi::api::core::v1::{Container, EnvVar, Pod, PodSpec, ResourceRequirements};
use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
use kube::api::{Patch, PatchParams};
use kube::{Api, Client};

const BLUEPRINT_NAMESPACE: &str = "blueprint-manager";

pub struct TeeInstance {
    client: Client,
    limits: ResourceLimits,
    service_name: String,
    image: String,
    env: BlueprintEnvVars,
    args: BlueprintArgs,
}

impl TeeInstance {
    /// Create a new `TeeInstance`
    pub fn new(
        kube_client: Client,
        limits: ResourceLimits,
        service_name: &str,
        image: String,
        env: BlueprintEnvVars,
        args: BlueprintArgs,
    ) -> TeeInstance {
        Self {
            client: kube_client,
            limits,
            service_name: service_name.into(),
            image,
            env,
            args,
        }
    }

    pub async fn start(&mut self) -> Result<()> {
        let pods: Api<Pod> = Api::namespaced(self.client.clone(), BLUEPRINT_NAMESPACE);

        let mem_mib = self.limits.memory_size / 1024;

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
            metadata: kube::api::ObjectMeta {
                name: Some(self.service_name.clone()),
                labels: Some(
                    [("app", "blueprint")]
                        .into_iter()
                        .map(|(k, v)| (k.to_string(), v.to_string()))
                        .collect(),
                ),
                ..Default::default()
            },
            spec: Some(PodSpec {
                runtime_class_name: Some("kata-qemu-coco-dev".to_string()),
                dns_policy: Some(String::from("ClusterFirst")),
                containers: vec![Container {
                    name: self.service_name.clone(),
                    image: Some(self.image.clone()),
                    resources: Some(ResourceRequirements {
                        claims: None,
                        limits: Some(
                            [(String::from("memory"), Quantity(format!("{mem_mib}Mi")))].into(),
                        ),
                        requests: Some(
                            [(String::from("memory"), Quantity(format!("{mem_mib}Mi")))].into(),
                        ),
                    }),
                    env: Some(env),
                    args: Some(self.args.encode()),
                    ..Default::default()
                }],
                ..Default::default()
            }),
            ..Default::default()
        };

        let pp = PatchParams::apply("blueprint-mgr").force();
        pods.patch(&self.service_name, &pp, &Patch::Apply(&pod))
            .await?;

        Ok(())
    }

    pub async fn status(&self) -> Result<Status> {
        todo!()
    }
}
