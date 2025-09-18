//! Kubernetes deployment tests using Kind for local testing

use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::{Container, PodSpec, PodTemplateSpec, Service, ServiceSpec, ServicePort, Namespace, Pod},
};
use kube::{
    api::{Api, PostParams, ListParams, DeleteParams},
    config::Config,
    Client,
};
use std::collections::BTreeMap;
use std::sync::Once;
use tokio::process::Command;

// Initialize rustls crypto provider once
static INIT: Once = Once::new();

fn init_crypto() {
    INIT.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .ok();
    });
}

/// Check if kind is available
async fn kind_available() -> bool {
    Command::new("kind")
        .arg("--version")
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Skip test if kind not available, otherwise ensure cluster exists
macro_rules! require_kind {
    () => {
        if !kind_available().await {
            eprintln!("⚠️  Skipping test - kind not installed. Install with: brew install kind");
            return;
        }
        ensure_test_cluster().await;
    };
}

/// Ensure test cluster exists
async fn ensure_test_cluster() {
    let output = Command::new("kind")
        .args(&["get", "clusters"])
        .output()
        .await
        .expect("Failed to list kind clusters");
    
    let clusters = String::from_utf8_lossy(&output.stdout);
    if !clusters.contains("blueprint-test") {
        println!("Creating test cluster 'blueprint-test'...");
        let create = Command::new("kind")
            .args(&["create", "cluster", "--name", "blueprint-test", "--wait", "60s"])
            .status()
            .await
            .expect("Failed to create kind cluster");
        
        assert!(create.success(), "Failed to create test cluster");
    }
}

/// Get kubeconfig for test cluster
async fn get_kubeconfig() -> Config {
    // Set KUBECONFIG environment variable to kind's kubeconfig
    let output = Command::new("kind")
        .args(&["get", "kubeconfig-path", "--name", "blueprint-test"])
        .output()
        .await;
    
    // If that fails, try the newer command
    let kubeconfig_path = if output.is_err() || !output.as_ref().unwrap().status.success() {
        // Newer versions of kind don't have kubeconfig-path, just export to temp file
        let temp_path = "/tmp/kind-blueprint-test.kubeconfig";
        let export = Command::new("kind")
            .args(&["export", "kubeconfig", "--name", "blueprint-test", "--kubeconfig", temp_path])
            .status()
            .await
            .expect("Failed to export kubeconfig");
        
        if !export.success() {
            panic!("Failed to export kubeconfig from kind");
        }
        temp_path.to_string()
    } else {
        String::from_utf8_lossy(&output.unwrap().stdout).trim().to_string()
    };
    
    // Use the kubeconfig file
    unsafe {
        std::env::set_var("KUBECONFIG", &kubeconfig_path);
    }
    Config::infer().await
        .expect("Failed to infer config from KUBECONFIG")
}

#[tokio::test]
async fn test_deploy_blueprint_to_kubernetes() {
    init_crypto();
    require_kind!();
    
    let config = get_kubeconfig().await;
    let client = Client::try_from(config)
        .expect("Failed to create Kubernetes client");
    
    let namespace = "default";
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    
    // Create deployment
    let deployment = create_blueprint_deployment("test-blueprint", "nginx:alpine", 1);
    
    deployments
        .create(&PostParams::default(), &deployment)
        .await
        .expect("Failed to create deployment");
    
    // Wait for pods to start
    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    
    // Verify deployment exists
    let deployed = deployments.get("test-blueprint").await
        .expect("Failed to get deployment");
    
    assert_eq!(deployed.metadata.name, Some("test-blueprint".to_string()));
    assert_eq!(deployed.status.unwrap().replicas, Some(1));
    
    // Verify pods are created
    let pods: Api<Pod> = Api::namespaced(client.clone(), namespace);
    let pod_list = pods.list(&ListParams::default().labels("app=test-blueprint")).await
        .expect("Failed to list pods");
    
    assert_eq!(pod_list.items.len(), 1, "Expected 1 pod");
    
    // Cleanup
    deployments
        .delete("test-blueprint", &DeleteParams::default())
        .await
        .expect("Failed to delete deployment");
}

#[tokio::test]
async fn test_multi_namespace_deployment() {
    init_crypto();
    require_kind!();
    
    let config = get_kubeconfig().await;
    let client = Client::try_from(config).unwrap();
    
    // Create namespaces
    let namespaces: Api<Namespace> = Api::all(client.clone());
    for ns_name in &["dev", "staging", "prod"] {
        let ns = Namespace {
            metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                name: Some(ns_name.to_string()),
                ..Default::default()
            },
            ..Default::default()
        };
        namespaces.create(&PostParams::default(), &ns).await.ok();
    }
    
    // Deploy to each namespace
    for ns in &["dev", "staging", "prod"] {
        let deployments: Api<Deployment> = Api::namespaced(client.clone(), ns);
        let deployment = create_blueprint_deployment(
            &format!("blueprint-{}", ns),
            "alpine:latest",
            1,
        );
        
        deployments
            .create(&PostParams::default(), &deployment)
            .await
            .expect(&format!("Failed to create deployment in {}", ns));
    }
    
    // Verify all deployments exist
    for ns in &["dev", "staging", "prod"] {
        let deployments: Api<Deployment> = Api::namespaced(client.clone(), ns);
        let list = deployments.list(&ListParams::default()).await.unwrap();
        assert_eq!(list.items.len(), 1, "Expected 1 deployment in {}", ns);
        
        // Cleanup
        deployments.delete(&format!("blueprint-{}", ns), &DeleteParams::default()).await.ok();
    }
}

#[tokio::test]
async fn test_k8s_service_exposure() {
    init_crypto();
    require_kind!();
    
    let config = get_kubeconfig().await;
    let client = Client::try_from(config).unwrap();
    
    let namespace = "default";
    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    
    // Test different service types
    let test_cases = vec![
        ("loadbalancer-svc", "LoadBalancer", 8080),
        ("clusterip-svc", "ClusterIP", 8081),
        ("nodeport-svc", "NodePort", 8082),
    ];
    
    for (name, svc_type, port) in test_cases {
        let service = create_blueprint_service(name, svc_type, port);
        
        services
            .create(&PostParams::default(), &service)
            .await
            .expect(&format!("Failed to create {} service", svc_type));
        
        // Verify service
        let created = services.get(name).await.unwrap();
        assert_eq!(created.spec.unwrap().type_, Some(svc_type.to_string()));
        
        // Cleanup
        services.delete(name, &DeleteParams::default()).await.ok();
    }
}

#[tokio::test]
async fn test_k8s_resource_limits() {
    init_crypto();
    require_kind!();
    
    let config = get_kubeconfig().await;
    let client = Client::try_from(config).unwrap();
    
    let namespace = "default";
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    
    // Create deployment with resource limits
    let mut deployment = create_blueprint_deployment("resource-test", "alpine:latest", 1);
    
    // Set resource limits
    if let Some(spec) = deployment.spec.as_mut() {
        if let Some(pod_spec) = spec.template.spec.as_mut() {
            pod_spec.containers[0].resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
                limits: Some(BTreeMap::from([
                    ("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("500m".to_string())),
                    ("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("512Mi".to_string())),
                ])),
                requests: Some(BTreeMap::from([
                    ("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("100m".to_string())),
                    ("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("128Mi".to_string())),
                ])),
                ..Default::default()
            });
            pod_spec.containers[0].command = Some(vec!["sleep".to_string(), "3600".to_string()]);
        }
    }
    
    deployments
        .create(&PostParams::default(), &deployment)
        .await
        .expect("Failed to create deployment with resource limits");
    
    // Verify resource limits are set
    let created = deployments.get("resource-test").await.unwrap();
    let container = &created.spec.unwrap().template.spec.unwrap().containers[0];
    
    let resources = container.resources.as_ref().expect("No resources set");
    let limits = resources.limits.as_ref().expect("No limits set");
    let requests = resources.requests.as_ref().expect("No requests set");
    
    assert!(limits.contains_key("cpu"));
    assert!(limits.contains_key("memory"));
    assert!(requests.contains_key("cpu"));
    assert!(requests.contains_key("memory"));
    
    // Cleanup
    deployments.delete("resource-test", &DeleteParams::default()).await.ok();
}

#[tokio::test]
async fn test_k8s_rolling_update() {
    init_crypto();
    require_kind!();
    
    let config = get_kubeconfig().await;
    let client = Client::try_from(config).unwrap();
    
    let namespace = "default";
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    
    // Create initial deployment with 3 replicas
    let deployment = create_blueprint_deployment("rolling-update", "nginx:1.19", 3);
    
    deployments
        .create(&PostParams::default(), &deployment)
        .await
        .expect("Failed to create deployment");
    
    // Wait for initial deployment
    tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
    
    // Update to new image version
    let mut updated = deployments.get("rolling-update").await.unwrap();
    updated.spec.as_mut().unwrap()
        .template.spec.as_mut().unwrap()
        .containers[0].image = Some("nginx:1.20".to_string());
    
    deployments
        .replace("rolling-update", &PostParams::default(), &updated)
        .await
        .expect("Failed to update deployment");
    
    // Wait for rolling update
    tokio::time::sleep(tokio::time::Duration::from_secs(15)).await;
    
    // Verify update completed
    let final_deployment = deployments.get("rolling-update").await.unwrap();
    let container = &final_deployment.spec.unwrap().template.spec.unwrap().containers[0];
    assert_eq!(container.image, Some("nginx:1.20".to_string()));
    
    // Cleanup
    deployments.delete("rolling-update", &DeleteParams::default()).await.ok();
}

#[tokio::test]
async fn test_namespace_isolation() {
    init_crypto();
    require_kind!();
    
    let config = get_kubeconfig().await;
    let client = Client::try_from(config).unwrap();
    
    // Create isolated namespace
    let namespaces: Api<Namespace> = Api::all(client.clone());
    let isolated_ns = Namespace {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some("isolated".to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    
    namespaces.create(&PostParams::default(), &isolated_ns).await.ok();
    
    // Deploy to isolated namespace
    let isolated_deployments: Api<Deployment> = Api::namespaced(client.clone(), "isolated");
    let deployment = create_blueprint_deployment("isolated-app", "nginx:alpine", 1);
    
    isolated_deployments
        .create(&PostParams::default(), &deployment)
        .await
        .expect("Failed to create deployment in isolated namespace");
    
    // Verify it exists in isolated namespace
    let exists_isolated = isolated_deployments.get("isolated-app").await;
    assert!(exists_isolated.is_ok());
    
    // Verify it does NOT exist in default namespace
    let default_deployments: Api<Deployment> = Api::namespaced(client.clone(), "default");
    let exists_default = default_deployments.get("isolated-app").await;
    assert!(exists_default.is_err());
    
    // Cleanup
    isolated_deployments.delete("isolated-app", &DeleteParams::default()).await.ok();
    namespaces.delete("isolated", &DeleteParams::default()).await.ok();
}

// Helper functions

fn create_blueprint_deployment(name: &str, image: &str, replicas: i32) -> Deployment {
    Deployment {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_string()),
            labels: Some(BTreeMap::from([
                ("app".to_string(), name.to_string()),
                ("managed-by".to_string(), "blueprint-manager".to_string()),
            ])),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(replicas),
            selector: k8s_openapi::apimachinery::pkg::apis::meta::v1::LabelSelector {
                match_labels: Some(BTreeMap::from([
                    ("app".to_string(), name.to_string()),
                ])),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
                    labels: Some(BTreeMap::from([
                        ("app".to_string(), name.to_string()),
                    ])),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.to_string(),
                        image: Some(image.to_string()),
                        ..Default::default()
                    }],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn create_blueprint_service(name: &str, service_type: &str, port: i32) -> Service {
    Service {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_string()),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some(service_type.to_string()),
            selector: Some(BTreeMap::from([
                ("app".to_string(), name.to_string()),
            ])),
            ports: Some(vec![ServicePort {
                port,
                target_port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(port)),
                ..Default::default()
            }]),
            ..Default::default()
        }),
        ..Default::default()
    }
}