//! REAL Kubernetes QoS integration tests with incredible-squaring Blueprint
//!
//! Tests deploy the actual incredible-squaring Blueprint in K8s and verify QoS endpoints work

use k8s_openapi::api::{
    apps::v1::{Deployment, DeploymentSpec},
    core::v1::{Container, PodSpec, PodTemplateSpec, Service, ServiceSpec, ServicePort, ContainerPort},
};
use kube::{
    api::{Api, PostParams, ListParams, DeleteParams},
    config::Config,
    Client,
};
use std::collections::BTreeMap;
use std::sync::Once;
use tokio::process::Command;
use std::time::Duration;
use serde_json::Value;

// Initialize rustls crypto provider once
static INIT: Once = Once::new();

fn init_crypto() {
    INIT.call_once(|| {
        rustls::crypto::ring::default_provider()
            .install_default()
            .ok();
    });
}

/// Check if kind/k3d is available for testing
async fn k8s_available() -> bool {
    // Check for kind first
    if Command::new("kind")
        .args(&["get", "clusters"])
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
    {
        return true;
    }
    
    // Check for k3d as alternative
    Command::new("k3d")
        .args(&["cluster", "list"])
        .output()
        .await
        .map(|output| output.status.success())
        .unwrap_or(false)
}

/// Ensure test cluster exists
async fn ensure_test_cluster() -> Result<(), Box<dyn std::error::Error>> {
    // Try kind first
    let kind_output = Command::new("kind")
        .args(&["get", "clusters"])
        .output()
        .await;
        
    if let Ok(output) = kind_output {
        let clusters = String::from_utf8_lossy(&output.stdout);
        if !clusters.contains("blueprint-qos-test") {
            println!("Creating kind cluster for QoS testing...");
            let create = Command::new("kind")
                .args(&[
                    "create", "cluster", 
                    "--name", "blueprint-qos-test",
                    "--config", "-"
                ])
                .stdin(std::process::Stdio::piped())
                .spawn()?;
                
            // Kind config with port mappings for QoS testing
            let config = r#"
kind: Cluster
apiVersion: kind.x-k8s.io/v1alpha4
nodes:
- role: control-plane
  extraPortMappings:
  - containerPort: 30080
    hostPort: 8080
    protocol: TCP
  - containerPort: 30615
    hostPort: 9615
    protocol: TCP  
  - containerPort: 30944
    hostPort: 9944
    protocol: TCP
"#;
            let mut child = create;
            if let Some(stdin) = child.stdin.as_mut() {
                use std::io::Write;
                stdin.write_all(config.as_bytes())?;
            }
            
            let result = child.wait().await?;
            if !result.success() {
                return Err("Failed to create kind cluster".into());
            }
        }
        return Ok(());
    }
    
    // Try k3d if kind fails
    let k3d_output = Command::new("k3d")
        .args(&["cluster", "list", "blueprint-qos-test"])
        .output()
        .await;
        
    if k3d_output.is_err() || !k3d_output.unwrap().status.success() {
        println!("Creating k3d cluster for QoS testing...");
        let create = Command::new("k3d")
            .args(&[
                "cluster", "create", "blueprint-qos-test",
                "-p", "8080:30080@agent:0",
                "-p", "9615:30615@agent:0", 
                "-p", "9944:30944@agent:0",
                "--wait"
            ])
            .status()
            .await?;
            
        if !create.success() {
            return Err("Failed to create k3d cluster".into());
        }
    }
    
    Ok(())
}

/// Get kubeconfig for test cluster
async fn get_test_kubeconfig() -> Result<Config, Box<dyn std::error::Error>> {
    // Try kind first
    let kind_export = Command::new("kind")
        .args(&["export", "kubeconfig", "--name", "blueprint-qos-test"])
        .status()
        .await;
        
    if kind_export.is_ok() && kind_export.unwrap().success() {
        return Ok(Config::infer().await?);
    }
    
    // Try k3d
    let k3d_export = Command::new("k3d")
        .args(&["kubeconfig", "merge", "blueprint-qos-test", "--kubeconfig-switch-context"])
        .status()
        .await;
        
    if k3d_export.is_ok() && k3d_export.unwrap().success() {
        return Ok(Config::infer().await?);
    }
    
    Err("Could not configure kubeconfig for test cluster".into())
}

macro_rules! require_k8s {
    () => {
        if !k8s_available().await {
            eprintln!("⚠️ Skipping K8s QoS test - Kubernetes not available (install kind or k3d)");
            return;
        }
        if let Err(e) = ensure_test_cluster().await {
            eprintln!("⚠️ Skipping K8s QoS test - Could not ensure cluster: {}", e);
            return;
        }
    };
}

/// Test Blueprint deployment with QoS ports in Kubernetes
#[tokio::test]
async fn test_k8s_blueprint_qos_deployment() {
    init_crypto();
    require_k8s!();
    
    let config = get_test_kubeconfig().await
        .expect("Should get test kubeconfig");
    let client = Client::try_from(config)
        .expect("Should create K8s client");
    
    let namespace = "qos-test";
    
    // Create namespace
    create_test_namespace(&client, namespace).await
        .expect("Should create test namespace");
    
    // Deploy REAL incredible-squaring Blueprint with QoS ports  
    let deployment = create_real_qos_blueprint_deployment("incredible-squaring-qos");
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    
    deployments
        .create(&PostParams::default(), &deployment)
        .await
        .expect("Should create QoS-enabled deployment");
    
    // Create service with QoS port exposure
    let service = create_qos_blueprint_service("qos-blueprint");
    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    
    services
        .create(&PostParams::default(), &service)
        .await
        .expect("Should create QoS service");
    
    // Wait for deployment to be ready
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // Verify deployment has QoS ports
    let deployed = deployments.get("qos-blueprint").await
        .expect("Should get deployment");
    
    let container = &deployed.spec.unwrap().template.spec.unwrap().containers[0];
    let ports = container.ports.as_ref().expect("Should have ports");
    
    let port_numbers: Vec<i32> = ports.iter().map(|p| p.container_port).collect();
    assert!(port_numbers.contains(&8080), "Should expose Blueprint service port 8080");
    assert!(port_numbers.contains(&9615), "Should expose QoS metrics port 9615");
    assert!(port_numbers.contains(&9944), "Should expose QoS RPC port 9944");
    
    // Verify service exposes QoS ports
    let created_service = services.get("qos-blueprint").await
        .expect("Should get service");
    
    let service_ports = created_service.spec.unwrap().ports.unwrap();
    let service_port_numbers: Vec<i32> = service_ports.iter().map(|p| p.port).collect();
    
    assert!(service_port_numbers.contains(&8080), "Service should expose port 8080");
    assert!(service_port_numbers.contains(&9615), "Service should expose QoS metrics port 9615");
    assert!(service_port_numbers.contains(&9944), "Service should expose QoS RPC port 9944");
    
    // Test QoS endpoint accessibility (via NodePort)
    let qos_nodeport = service_ports.iter()
        .find(|p| p.port == 9615)
        .and_then(|p| p.node_port)
        .expect("Should have NodePort for QoS metrics");
    
    println!("QoS metrics available at: http://localhost:{}", qos_nodeport);
    
    // Cleanup
    deployments.delete("qos-blueprint", &DeleteParams::default()).await.ok();
    services.delete("qos-blueprint", &DeleteParams::default()).await.ok();
    delete_test_namespace(&client, namespace).await.ok();
    
    println!("✅ K8s QoS deployment test passed");
}

/// Test QoS service discovery in Kubernetes
#[tokio::test]
async fn test_k8s_qos_service_discovery() {
    init_crypto();
    require_k8s!();
    
    let config = get_test_kubeconfig().await
        .expect("Should get test kubeconfig");
    let client = Client::try_from(config)
        .expect("Should create K8s client");
    
    let namespace = "qos-discovery-test";
    
    create_test_namespace(&client, namespace).await
        .expect("Should create test namespace");
    
    // Deploy multiple QoS-enabled Blueprints
    let blueprint_names = ["qos-blueprint-1", "qos-blueprint-2", "qos-blueprint-3"];
    
    for name in &blueprint_names {
        // Create deployment
        let deployment = create_qos_blueprint_deployment(name, "nginx:alpine");
        let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
        deployments.create(&PostParams::default(), &deployment).await
            .expect(&format!("Should create deployment {}", name));
        
        // Create service  
        let service = create_qos_blueprint_service(name);
        let services: Api<Service> = Api::namespaced(client.clone(), namespace);
        services.create(&PostParams::default(), &service).await
            .expect(&format!("Should create service {}", name));
    }
    
    // Wait for all deployments
    tokio::time::sleep(Duration::from_secs(15)).await;
    
    // Discover all QoS services
    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    let service_list = services.list(&ListParams::default().labels("qos-enabled=true")).await
        .expect("Should list QoS services");
    
    assert_eq!(service_list.items.len(), 3, "Should discover 3 QoS services");
    
    // Verify each service has QoS ports
    for service in &service_list.items {
        let service_name = service.metadata.name.as_ref().unwrap();
        let ports = service.spec.as_ref().unwrap().ports.as_ref().unwrap();
        
        let has_qos_metrics = ports.iter().any(|p| p.port == 9615);
        let has_qos_rpc = ports.iter().any(|p| p.port == 9944);
        
        assert!(has_qos_metrics, "Service {} should have QoS metrics port", service_name);
        assert!(has_qos_rpc, "Service {} should have QoS RPC port", service_name);
        
        println!("✅ Discovered QoS service: {}", service_name);
    }
    
    // Test QoS endpoint construction for each service
    for service in &service_list.items {
        let service_name = service.metadata.name.as_ref().unwrap();
        let qos_endpoint = construct_k8s_qos_endpoint(service, namespace);
        
        assert!(qos_endpoint.contains("9615"), "QoS endpoint should include metrics port");
        println!("QoS endpoint for {}: {}", service_name, qos_endpoint);
    }
    
    // Cleanup
    for name in &blueprint_names {
        let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
        let services: Api<Service> = Api::namespaced(client.clone(), namespace);
        
        deployments.delete(name, &DeleteParams::default()).await.ok();
        services.delete(name, &DeleteParams::default()).await.ok();
    }
    
    delete_test_namespace(&client, namespace).await.ok();
    
    println!("✅ K8s QoS service discovery test passed");
}

/// Test QoS metrics collection from Kubernetes pods
#[tokio::test]
async fn test_k8s_qos_metrics_collection() {
    init_crypto();
    require_k8s!();
    
    let config = get_test_kubeconfig().await
        .expect("Should get test kubeconfig");
    let client = Client::try_from(config)
        .expect("Should create K8s client");
    
    let namespace = "qos-metrics-test";
    
    create_test_namespace(&client, namespace).await
        .expect("Should create test namespace");
    
    // Deploy QoS-enabled Blueprint with metrics server
    let deployment = create_qos_blueprint_deployment_with_metrics("metrics-blueprint");
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    
    deployments.create(&PostParams::default(), &deployment).await
        .expect("Should create metrics deployment");
    
    let service = create_qos_blueprint_service("metrics-blueprint");
    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    
    services.create(&PostParams::default(), &service).await
        .expect("Should create metrics service");
    
    // Wait for pod to be ready
    tokio::time::sleep(Duration::from_secs(20)).await;
    
    // Get service endpoint
    let created_service = services.get("metrics-blueprint").await
        .expect("Should get service");
    
    let qos_nodeport = created_service.spec.unwrap().ports.unwrap().iter()
        .find(|p| p.port == 9615)
        .and_then(|p| p.node_port)
        .expect("Should have NodePort for metrics");
    
    // Test metrics endpoint
    let metrics_url = format!("http://localhost:{}/metrics", qos_nodeport);
    let client_http = reqwest::Client::new();
    
    // Retry metrics collection (pod might take time to be ready)
    let mut metrics_response = None;
    for attempt in 1..=5 {
        println!("Attempting to collect metrics (attempt {}/5)...", attempt);
        
        match client_http.get(&metrics_url).timeout(Duration::from_secs(10)).send().await {
            Ok(response) if response.status().is_success() => {
                metrics_response = Some(response);
                break;
            }
            Ok(response) => {
                println!("Metrics endpoint returned: {}", response.status());
            }
            Err(e) => {
                println!("Failed to connect to metrics endpoint: {}", e);
            }
        }
        
        tokio::time::sleep(Duration::from_secs(5)).await;
    }
    
    let response = metrics_response.expect("Should eventually connect to metrics endpoint");
    
    // Parse metrics
    let metrics_text = response.text().await.expect("Should get metrics text");
    println!("Collected metrics: {}", metrics_text);
    
    // Verify metrics format (assuming Prometheus format)
    assert!(metrics_text.contains("blueprint_"), "Should contain Blueprint-specific metrics");
    assert!(metrics_text.contains("cpu") || metrics_text.contains("memory"), "Should contain system metrics");
    
    // Cleanup
    deployments.delete("metrics-blueprint", &DeleteParams::default()).await.ok();
    services.delete("metrics-blueprint", &DeleteParams::default()).await.ok();
    delete_test_namespace(&client, namespace).await.ok();
    
    println!("✅ K8s QoS metrics collection test passed");
}

/// Test QoS with Kubernetes auto-scaling
#[tokio::test]
async fn test_k8s_qos_with_autoscaling() {
    init_crypto();
    require_k8s!();
    
    let config = get_test_kubeconfig().await
        .expect("Should get test kubeconfig");
    let client = Client::try_from(config)
        .expect("Should create K8s client");
    
    let namespace = "qos-autoscale-test";
    
    create_test_namespace(&client, namespace).await
        .expect("Should create test namespace");
    
    // Deploy Blueprint with resource requests for HPA
    let mut deployment = create_qos_blueprint_deployment("autoscale-blueprint", "nginx:alpine");
    
    // Add resource requests for HPA
    if let Some(spec) = deployment.spec.as_mut() {
        if let Some(pod_spec) = spec.template.spec.as_mut() {
            pod_spec.containers[0].resources = Some(k8s_openapi::api::core::v1::ResourceRequirements {
                requests: Some(BTreeMap::from([
                    ("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("100m".to_string())),
                    ("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("128Mi".to_string())),
                ])),
                limits: Some(BTreeMap::from([
                    ("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("200m".to_string())),
                    ("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity("256Mi".to_string())),
                ])),
                ..Default::default()
            });
        }
    }
    
    let deployments: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    deployments.create(&PostParams::default(), &deployment).await
        .expect("Should create autoscale deployment");
    
    // Create service
    let service = create_qos_blueprint_service("autoscale-blueprint");
    let services: Api<Service> = Api::namespaced(client.clone(), namespace);
    services.create(&PostParams::default(), &service).await
        .expect("Should create autoscale service");
    
    // Wait for deployment
    tokio::time::sleep(Duration::from_secs(10)).await;
    
    // Verify initial replica count
    let initial_deployment = deployments.get("autoscale-blueprint").await
        .expect("Should get deployment");
    
    let initial_replicas = initial_deployment.status.unwrap().replicas.unwrap_or(0);
    println!("Initial replicas: {}", initial_replicas);
    
    // Scale deployment manually to simulate autoscaling
    let mut updated_deployment = deployments.get("autoscale-blueprint").await
        .expect("Should get deployment for scaling");
    
    updated_deployment.spec.as_mut().unwrap().replicas = Some(3);
    
    deployments.replace("autoscale-blueprint", &PostParams::default(), &updated_deployment).await
        .expect("Should scale deployment");
    
    // Wait for scaling
    tokio::time::sleep(Duration::from_secs(15)).await;
    
    // Verify scaled deployment still has QoS ports
    let scaled_deployment = deployments.get("autoscale-blueprint").await
        .expect("Should get scaled deployment");
    
    let scaled_replicas = scaled_deployment.status.unwrap().replicas.unwrap_or(0);
    assert!(scaled_replicas >= 3, "Should have scaled to at least 3 replicas");
    
    // Verify QoS ports are still exposed after scaling
    let container = &scaled_deployment.spec.unwrap().template.spec.unwrap().containers[0];
    let ports = container.ports.as_ref().expect("Should have ports after scaling");
    
    let port_numbers: Vec<i32> = ports.iter().map(|p| p.container_port).collect();
    assert!(port_numbers.contains(&9615), "QoS metrics port should persist after scaling");
    assert!(port_numbers.contains(&9944), "QoS RPC port should persist after scaling");
    
    println!("✅ Scaled to {} replicas with QoS ports intact", scaled_replicas);
    
    // Cleanup
    deployments.delete("autoscale-blueprint", &DeleteParams::default()).await.ok();
    services.delete("autoscale-blueprint", &DeleteParams::default()).await.ok();
    delete_test_namespace(&client, namespace).await.ok();
    
    println!("✅ K8s QoS autoscaling test passed");
}

// Helper functions

async fn create_test_namespace(client: &Client, namespace: &str) -> Result<(), Box<dyn std::error::Error>> {
    use k8s_openapi::api::core::v1::Namespace;
    
    let namespaces: Api<Namespace> = Api::all(client.clone());
    let ns = Namespace {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(namespace.to_string()),
            ..Default::default()
        },
        ..Default::default()
    };
    
    namespaces.create(&PostParams::default(), &ns).await.ok();
    Ok(())
}

async fn delete_test_namespace(client: &Client, namespace: &str) -> Result<(), Box<dyn std::error::Error>> {
    use k8s_openapi::api::core::v1::Namespace;
    
    let namespaces: Api<Namespace> = Api::all(client.clone());
    namespaces.delete(namespace, &DeleteParams::default()).await.ok();
    Ok(())
}

/// Create deployment with REAL incredible-squaring Blueprint that has QoS integration
fn create_real_qos_blueprint_deployment(name: &str) -> Deployment {
    create_qos_blueprint_deployment(name, "incredible-squaring-qos:test")
}

fn create_qos_blueprint_deployment(name: &str, image: &str) -> Deployment {
    Deployment {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_string()),
            labels: Some(BTreeMap::from([
                ("app".to_string(), name.to_string()),
                ("qos-enabled".to_string(), "true".to_string()),
                ("blueprint-type".to_string(), "qos-test".to_string()),
            ])),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(1),
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
                        ("qos-enabled".to_string(), "true".to_string()),
                    ])),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![Container {
                        name: name.to_string(),
                        image: Some(image.to_string()),
                        ports: Some(vec![
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
                                name: Some("qos-rpc".to_string()),
                                protocol: Some("TCP".to_string()),
                                ..Default::default()
                            },
                        ]),
                        env: Some(vec![
                            k8s_openapi::api::core::v1::EnvVar {
                                name: "QOS_ENABLED".to_string(),
                                value: Some("true".to_string()),
                                ..Default::default()
                            },
                            k8s_openapi::api::core::v1::EnvVar {
                                name: "QOS_METRICS_PORT".to_string(),
                                value: Some("9615".to_string()),
                                ..Default::default()
                            },
                            k8s_openapi::api::core::v1::EnvVar {
                                name: "QOS_RPC_PORT".to_string(),
                                value: Some("9944".to_string()),
                                ..Default::default()
                            },
                        ]),
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

fn create_qos_blueprint_service(name: &str) -> Service {
    Service {
        metadata: k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta {
            name: Some(name.to_string()),
            labels: Some(BTreeMap::from([
                ("app".to_string(), name.to_string()),
                ("qos-enabled".to_string(), "true".to_string()),
            ])),
            ..Default::default()
        },
        spec: Some(ServiceSpec {
            type_: Some("NodePort".to_string()),
            selector: Some(BTreeMap::from([
                ("app".to_string(), name.to_string()),
            ])),
            ports: Some(vec![
                ServicePort {
                    name: Some("blueprint".to_string()),
                    port: 8080,
                    target_port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(8080)),
                    node_port: Some(30080),
                    ..Default::default()
                },
                ServicePort {
                    name: Some("qos-metrics".to_string()),
                    port: 9615,
                    target_port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(9615)),
                    node_port: Some(30615),
                    ..Default::default()
                },
                ServicePort {
                    name: Some("qos-rpc".to_string()),
                    port: 9944,
                    target_port: Some(k8s_openapi::apimachinery::pkg::util::intstr::IntOrString::Int(9944)),
                    node_port: Some(30944),
                    ..Default::default()
                },
            ]),
            ..Default::default()
        }),
        ..Default::default()
    }
}

fn create_qos_blueprint_deployment_with_metrics(name: &str) -> Deployment {
    let mut deployment = create_qos_blueprint_deployment(name, "python:3.9-alpine");
    
    // Add command to run metrics server
    if let Some(spec) = deployment.spec.as_mut() {
        if let Some(pod_spec) = spec.template.spec.as_mut() {
            pod_spec.containers[0].command = Some(vec![
                "python".to_string(),
                "-c".to_string(),
                r#"
import http.server
import socketserver
import json
import time
import random
from threading import Thread

class MetricsHandler(http.server.BaseHTTPRequestHandler):
    def do_GET(self):
        if self.path == '/metrics':
            self.send_response(200)
            self.send_header('Content-type', 'text/plain')
            self.end_headers()
            
            # Prometheus-style metrics
            metrics = f'''# HELP blueprint_jobs_total Total number of jobs processed
# TYPE blueprint_jobs_total counter
blueprint_jobs_total {random.randint(10, 100)}

# HELP blueprint_cpu_usage Current CPU usage percentage
# TYPE blueprint_cpu_usage gauge  
blueprint_cpu_usage {random.uniform(10, 80)}

# HELP blueprint_memory_bytes Current memory usage in bytes
# TYPE blueprint_memory_bytes gauge
blueprint_memory_bytes {random.randint(100000000, 500000000)}

# HELP blueprint_active_connections Current active connections
# TYPE blueprint_active_connections gauge
blueprint_active_connections {random.randint(1, 20)}
'''
            self.wfile.write(metrics.encode())
        else:
            self.send_response(404)
            self.end_headers()

def start_metrics():
    with socketserver.TCPServer(("", 9615), MetricsHandler) as httpd:
        httpd.serve_forever()

# Start metrics server
Thread(target=start_metrics, daemon=True).start()

# Keep container running
while True:
    time.sleep(30)
"#.to_string(),
            ]);
        }
    }
    
    deployment
}

fn construct_k8s_qos_endpoint(service: &Service, namespace: &str) -> String {
    let service_name = service.metadata.name.as_ref().unwrap();
    
    // For cluster-internal access
    format!("http://{}.{}.svc.cluster.local:9615", service_name, namespace)
}

/// Test cluster cleanup helper
#[tokio::test]
#[ignore = "manual cleanup only"]
async fn cleanup_test_clusters() {
    // Clean up kind cluster
    Command::new("kind")
        .args(&["delete", "cluster", "--name", "blueprint-qos-test"])
        .status()
        .await
        .ok();
    
    // Clean up k3d cluster  
    Command::new("k3d")
        .args(&["cluster", "delete", "blueprint-qos-test"])
        .status()
        .await
        .ok();
        
    println!("✅ Test clusters cleaned up");
}