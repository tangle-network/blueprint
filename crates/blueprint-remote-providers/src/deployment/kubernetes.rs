//! Kubernetes deployment support for Blueprint remote providers
//!
//! Provides Kubernetes deployment capabilities for Blueprint instances,
//! ensuring QoS metrics ports are exposed for remote monitoring.

use crate::core::error::{Error, Result};
use crate::core::resources::ResourceSpec;
use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::{
        Container, ContainerPort, PodSpec, PodTemplateSpec, Service, ServicePort, ServiceSpec,
    },
};
use kube::{
    Client,
    api::{Api, PostParams},
};
use std::collections::BTreeMap;
use tracing::{debug, info};

/// Kubernetes deployment client for Blueprint services
pub struct KubernetesDeploymentClient {
    client: Client,
    namespace: String,
}

impl KubernetesDeploymentClient {
    /// Create a new Kubernetes deployment client
    pub async fn new(namespace: Option<String>) -> Result<Self> {
        let config = kube::Config::infer()
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to infer k8s config: {}", e)))?;

        let client = Client::try_from(config).map_err(|e| {
            Error::ConfigurationError(format!("Failed to create k8s client: {}", e))
        })?;

        let namespace = namespace.unwrap_or_else(|| "default".to_string());

        Ok(Self { client, namespace })
    }

    /// Deploy a Blueprint service to Kubernetes with QoS port exposure
    pub async fn deploy_blueprint(
        &self,
        name: &str,
        image: &str,
        spec: &ResourceSpec,
        replicas: i32,
    ) -> Result<(String, Vec<u16>)> {
        info!(
            "Deploying Blueprint {} to Kubernetes namespace {}",
            name, self.namespace
        );

        // Create deployment with QoS port exposure
        let deployment = self.create_blueprint_deployment(name, image, spec, replicas);
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.namespace);

        let deployment_result = deployments
            .create(&PostParams::default(), &deployment)
            .await
            .map_err(|e| {
                Error::ConfigurationError(format!("Failed to create deployment: {}", e))
            })?;

        let deployment_name = deployment_result
            .metadata
            .name
            .ok_or_else(|| Error::ConfigurationError("Deployment has no name".into()))?;

        // Create service with QoS port exposure
        let (service, exposed_ports) = self.create_blueprint_service(name);
        let services: Api<Service> = Api::namespaced(self.client.clone(), &self.namespace);

        services
            .create(&PostParams::default(), &service)
            .await
            .map_err(|e| Error::ConfigurationError(format!("Failed to create service: {}", e)))?;

        info!(
            "Successfully deployed Blueprint {} (deployment: {}, exposed ports: {:?})",
            name, deployment_name, exposed_ports
        );

        Ok((deployment_name, exposed_ports))
    }

    /// Create a Blueprint deployment with proper resource limits and QoS port exposure
    fn create_blueprint_deployment(
        &self,
        name: &str,
        image: &str,
        spec: &ResourceSpec,
        replicas: i32,
    ) -> Deployment {
        let container_ports = vec![
            ContainerPort {
                container_port: 8080,
                name: Some("blueprint".to_string()),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            },
            ContainerPort {
                container_port: 9615,
                name: Some("qos-metrics".to_string()),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            },
            ContainerPort {
                container_port: 9944,
                name: Some("rpc".to_string()),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            },
        ];

        let mut container = Container {
            name: name.to_string(),
            image: Some(image.to_string()),
            ports: Some(container_ports),
            ..Default::default()
        };

        // Add resource limits if specified
        if spec.memory_gb > 0.0 || spec.cpu > 0.0 {
            use k8s_openapi::apimachinery::pkg::api::resource::Quantity;
            let mut limits = BTreeMap::new();
            let mut requests = BTreeMap::new();

            if spec.cpu > 0.0 {
                limits.insert(
                    "cpu".to_string(),
                    Quantity(format!("{}m", (spec.cpu * 1000.0) as u64)),
                );
                requests.insert(
                    "cpu".to_string(),
                    Quantity(format!("{}m", (spec.cpu * 500.0) as u64)),
                ); // 50% request
            }

            if spec.memory_gb > 0.0 {
                limits.insert(
                    "memory".to_string(),
                    Quantity(format!("{}Gi", spec.memory_gb)),
                );
                requests.insert(
                    "memory".to_string(),
                    Quantity(format!("{}Gi", spec.memory_gb * 0.5)),
                ); // 50% request
            }

            container.resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
                limits: Some(limits),
                requests: Some(requests),
                ..Default::default()
            });
        }

        Deployment {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(name.to_string()),
                labels: Some(BTreeMap::from([
                    ("app".to_string(), name.to_string()),
                    (
                        "managed-by".to_string(),
                        "blueprint-remote-providers".to_string(),
                    ),
                    ("qos-enabled".to_string(), "true".to_string()),
                ])),
                ..Default::default()
            },
            spec: Some(DeploymentSpec {
                replicas: Some(replicas),
                selector: k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
                    match_labels: Some(BTreeMap::from([("app".to_string(), name.to_string())])),
                    ..Default::default()
                },
                template: PodTemplateSpec {
                    metadata: Some(k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                        labels: Some(BTreeMap::from([
                            ("app".to_string(), name.to_string()),
                            ("qos-enabled".to_string(), "true".to_string()),
                        ])),
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

    /// Create a service that exposes all Blueprint ports including QoS metrics
    fn create_blueprint_service(&self, name: &str) -> (Service, Vec<u16>) {
        let service_ports = vec![
            ServicePort {
                port: 8080,
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(8080),
                ),
                name: Some("blueprint".to_string()),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            },
            ServicePort {
                port: 9615,
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(9615),
                ),
                name: Some("qos-metrics".to_string()),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            },
            ServicePort {
                port: 9944,
                target_port: Some(
                    k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(9944),
                ),
                name: Some("rpc".to_string()),
                protocol: Some("TCP".to_string()),
                ..Default::default()
            },
        ];

        let exposed_ports = vec![8080, 9615, 9944];

        let service = Service {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(format!("{}-service", name)),
                labels: Some(BTreeMap::from([
                    ("app".to_string(), name.to_string()),
                    (
                        "managed-by".to_string(),
                        "blueprint-remote-providers".to_string(),
                    ),
                ])),
                ..Default::default()
            },
            spec: Some(ServiceSpec {
                type_: Some("LoadBalancer".to_string()), // Expose externally for metrics collection
                selector: Some(BTreeMap::from([("app".to_string(), name.to_string())])),
                ports: Some(service_ports),
                ..Default::default()
            }),
            ..Default::default()
        };

        (service, exposed_ports)
    }

    /// Get service external endpoint for QoS metrics collection
    pub async fn get_service_endpoint(&self, service_name: &str) -> Result<Option<String>> {
        let services: Api<Service> = Api::namespaced(self.client.clone(), &self.namespace);

        match services.get(service_name).await {
            Ok(service) => {
                if let Some(status) = service.status {
                    if let Some(lb) = status.load_balancer {
                        if let Some(ingresses) = lb.ingress {
                            if let Some(ingress) = ingresses.first() {
                                if let Some(ip) = &ingress.ip {
                                    return Ok(Some(ip.clone()));
                                }
                                if let Some(hostname) = &ingress.hostname {
                                    return Ok(Some(hostname.clone()));
                                }
                            }
                        }
                    }
                }
                Ok(None) // Service exists but no external endpoint yet
            }
            Err(e) => Err(Error::ConfigurationError(format!(
                "Failed to get service: {}",
                e
            ))),
        }
    }

    /// Cleanup a Blueprint deployment and service
    pub async fn cleanup_blueprint(&self, name: &str) -> Result<()> {
        debug!("Cleaning up Blueprint deployment: {}", name);

        // Delete service
        let services: Api<Service> = Api::namespaced(self.client.clone(), &self.namespace);
        let service_name = format!("{}-service", name);
        if let Err(e) = services.delete(&service_name, &Default::default()).await {
            debug!("Service {} may not exist: {}", service_name, e);
        }

        // Delete deployment
        let deployments: Api<Deployment> = Api::namespaced(self.client.clone(), &self.namespace);
        if let Err(e) = deployments.delete(name, &Default::default()).await {
            debug!("Deployment {} may not exist: {}", name, e);
        }

        info!("Cleaned up Blueprint deployment: {}", name);
        Ok(())
    }
}
