//! Reference HTTP FaaS Server
//!
//! A minimal but fully-featured test server implementing the Custom FaaS Platform Spec.
//! This server is intended for local testing of the HTTP FaaS executor without requiring
//! cloud credentials.
//!
//! Run with:
//! ```bash
//! cargo run --example reference_faas_server --features custom
//! ```
//!
//! Server runs on http://localhost:8080

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::convert::Infallible;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Arc;
use tokio::sync::RwLock;
use warp::http::StatusCode;
use warp::{Filter, Rejection, Reply};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FaasConfig {
    memory_mb: u32,
    timeout_secs: u32,
    #[serde(default)]
    max_concurrency: u32,
    #[serde(default)]
    env_vars: HashMap<String, String>,
}

impl Default for FaasConfig {
    fn default() -> Self {
        Self {
            memory_mb: 512,
            timeout_secs: 300,
            max_concurrency: 10,
            env_vars: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Serialize)]
struct DeploymentInfo {
    function_id: String,
    endpoint: String,
    status: String,
    cold_start_ms: u64,
    memory_mb: u32,
    timeout_secs: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    deployed_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    binary_size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InvokeRequest {
    job_id: u32,
    args: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct InvokeResponse {
    job_id: u32,
    result: Vec<u8>,
    success: bool,
    #[serde(default)]
    execution_ms: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    memory_used_mb: Option<u32>,
}

#[derive(Debug, Clone, Serialize)]
struct HealthResponse {
    function_id: String,
    status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_invocation: Option<String>,
    #[serde(default)]
    total_invocations: u64,
}

#[derive(Debug, Clone, Serialize)]
struct ErrorResponse {
    error: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    function_id: Option<String>,
}

#[derive(Debug, Clone)]
struct FunctionMetadata {
    binary_path: PathBuf,
    config: FaasConfig,
    deployed_at: String,
    invocations: u64,
    last_invocation: Option<String>,
}

type FunctionStore = Arc<RwLock<HashMap<String, FunctionMetadata>>>;

/// Deploy a new function
async fn deploy_function(
    function_id: String,
    config_header: Option<String>,
    bytes: bytes::Bytes,
    store: FunctionStore,
) -> Result<impl Reply, Rejection> {
    // Parse config from header
    let config = if let Some(config_b64) = config_header {
        use base64::Engine;
        match base64::engine::general_purpose::STANDARD.decode(&config_b64) {
            Ok(decoded) => match serde_json::from_slice::<FaasConfig>(&decoded) {
                Ok(cfg) => cfg,
                Err(e) => {
                    eprintln!("Failed to parse config: {e}");
                    FaasConfig::default()
                }
            },
            Err(e) => {
                eprintln!("Failed to decode base64 config: {e}");
                FaasConfig::default()
            }
        }
    } else {
        FaasConfig::default()
    };

    // Check if function already exists
    {
        let functions = store.read().await;
        if functions.contains_key(&function_id) {
            let response = ErrorResponse {
                error: "Function already exists".to_string(),
                code: Some("CONFLICT".to_string()),
                function_id: Some(function_id.clone()),
            };
            return Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::CONFLICT,
            ));
        }
    }

    // Create functions directory
    let functions_dir = PathBuf::from("/tmp/blueprint-faas-test/functions");
    let function_dir = functions_dir.join(&function_id);
    if let Err(e) = fs::create_dir_all(&function_dir) {
        let response = ErrorResponse {
            error: format!("Failed to create function directory: {e}"),
            code: Some("INFRASTRUCTURE_ERROR".to_string()),
            function_id: Some(function_id),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Save and extract zip
    let zip_path = function_dir.join("function.zip");
    if let Err(e) = fs::write(&zip_path, bytes.as_ref()) {
        let response = ErrorResponse {
            error: format!("Failed to write zip file: {e}"),
            code: Some("INFRASTRUCTURE_ERROR".to_string()),
            function_id: Some(function_id),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Extract bootstrap executable
    let binary_path = function_dir.join("bootstrap");
    if let Err(e) = Command::new("unzip")
        .arg("-o")
        .arg(&zip_path)
        .arg("bootstrap")
        .arg("-d")
        .arg(&function_dir)
        .output()
    {
        let response = ErrorResponse {
            error: format!("Failed to extract zip: {e}"),
            code: Some("INFRASTRUCTURE_ERROR".to_string()),
            function_id: Some(function_id),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Make bootstrap executable
    if let Err(e) = Command::new("chmod").arg("+x").arg(&binary_path).output() {
        let response = ErrorResponse {
            error: format!("Failed to make bootstrap executable: {e}"),
            code: Some("INFRASTRUCTURE_ERROR".to_string()),
            function_id: Some(function_id),
        };
        return Ok(warp::reply::with_status(
            warp::reply::json(&response),
            StatusCode::INTERNAL_SERVER_ERROR,
        ));
    }

    // Store function metadata
    let metadata = FunctionMetadata {
        binary_path: binary_path.clone(),
        config: config.clone(),
        deployed_at: chrono::Utc::now().to_rfc3339(),
        invocations: 0,
        last_invocation: None,
    };

    {
        let mut functions = store.write().await;
        functions.insert(function_id.clone(), metadata);
    }

    let response = DeploymentInfo {
        function_id: function_id.clone(),
        endpoint: format!("http://localhost:8080/api/functions/{function_id}/invoke"),
        status: "deployed".to_string(),
        cold_start_ms: 500,
        memory_mb: config.memory_mb,
        timeout_secs: config.timeout_secs,
        deployed_at: Some(chrono::Utc::now().to_rfc3339()),
        binary_size_bytes: Some(bytes.len() as u64),
    };

    Ok(warp::reply::with_status(
        warp::reply::json(&response),
        StatusCode::OK,
    ))
}

/// Invoke a deployed function
async fn invoke_function(
    function_id: String,
    request: InvokeRequest,
    store: FunctionStore,
) -> Result<impl Reply, Rejection> {
    let metadata = {
        let functions = store.read().await;
        match functions.get(&function_id) {
            Some(meta) => meta.clone(),
            None => {
                let response = ErrorResponse {
                    error: "Function not found".to_string(),
                    code: Some("NOT_FOUND".to_string()),
                    function_id: Some(function_id),
                };
                return Ok(warp::reply::with_status(
                    warp::reply::json(&response),
                    StatusCode::NOT_FOUND,
                ));
            }
        }
    };

    // Create input JSON
    let input_json = serde_json::json!({
        "job_id": request.job_id,
        "args": request.args
    });

    let start = std::time::Instant::now();

    // Execute binary
    let output = Command::new(&metadata.binary_path)
        .stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .and_then(|mut child| {
            use std::io::Write;
            if let Some(mut stdin) = child.stdin.take() {
                let _ = stdin.write_all(input_json.to_string().as_bytes());
            }
            child.wait_with_output()
        });

    let execution_ms = start.elapsed().as_millis() as u64;

    // Update invocation stats
    {
        let mut functions = store.write().await;
        if let Some(meta) = functions.get_mut(&function_id) {
            meta.invocations += 1;
            meta.last_invocation = Some(chrono::Utc::now().to_rfc3339());
        }
    }

    match output {
        Ok(output) if output.status.success() => {
            // Parse output JSON
            match serde_json::from_slice::<serde_json::Value>(&output.stdout) {
                Ok(json_output) => {
                    let response = InvokeResponse {
                        job_id: json_output["job_id"]
                            .as_u64()
                            .unwrap_or(request.job_id as u64)
                            as u32,
                        result: json_output["result"]
                            .as_array()
                            .map(|arr| {
                                arr.iter()
                                    .filter_map(|v| v.as_u64().map(|n| n as u8))
                                    .collect()
                            })
                            .unwrap_or_default(),
                        success: json_output["success"].as_bool().unwrap_or(true),
                        execution_ms,
                        memory_used_mb: Some(128), // Mock value
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&response),
                        StatusCode::OK,
                    ))
                }
                Err(e) => {
                    eprintln!("Failed to parse function output: {e}");
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    eprintln!("stdout: {stdout}");
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("stderr: {stderr}");
                    let response = InvokeResponse {
                        job_id: request.job_id,
                        result: vec![],
                        success: false,
                        execution_ms,
                        memory_used_mb: None,
                    };
                    Ok(warp::reply::with_status(
                        warp::reply::json(&response),
                        StatusCode::INTERNAL_SERVER_ERROR,
                    ))
                }
            }
        }
        Ok(output) => {
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Function execution failed: {stderr}");
            let response = InvokeResponse {
                job_id: request.job_id,
                result: vec![],
                success: false,
                execution_ms,
                memory_used_mb: None,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
        Err(e) => {
            eprintln!("Failed to execute function: {e}");
            let response = ErrorResponse {
                error: format!("Failed to execute function: {e}"),
                code: Some("EXECUTION_ERROR".to_string()),
                function_id: Some(function_id),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::INTERNAL_SERVER_ERROR,
            ))
        }
    }
}

/// Health check for a deployed function
async fn health_check(function_id: String, store: FunctionStore) -> Result<impl Reply, Rejection> {
    let functions = store.read().await;
    match functions.get(&function_id) {
        Some(meta) => {
            let response = HealthResponse {
                function_id,
                status: "healthy".to_string(),
                last_invocation: meta.last_invocation.clone(),
                total_invocations: meta.invocations,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        None => {
            let response = ErrorResponse {
                error: "Function not found".to_string(),
                code: Some("NOT_FOUND".to_string()),
                function_id: Some(function_id),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}

/// Get deployment info for a function
async fn get_deployment(
    function_id: String,
    store: FunctionStore,
) -> Result<impl Reply, Rejection> {
    let functions = store.read().await;
    match functions.get(&function_id) {
        Some(meta) => {
            let response = DeploymentInfo {
                function_id: function_id.clone(),
                endpoint: format!("http://localhost:8080/api/functions/{function_id}/invoke"),
                status: "deployed".to_string(),
                cold_start_ms: 500,
                memory_mb: meta.config.memory_mb,
                timeout_secs: meta.config.timeout_secs,
                deployed_at: Some(meta.deployed_at.clone()),
                binary_size_bytes: None,
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        None => {
            let response = ErrorResponse {
                error: "Function not found".to_string(),
                code: Some("NOT_FOUND".to_string()),
                function_id: Some(function_id),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}

/// Undeploy a function
async fn undeploy_function(
    function_id: String,
    store: FunctionStore,
) -> Result<impl Reply, Rejection> {
    let removed = {
        let mut functions = store.write().await;
        functions.remove(&function_id)
    };

    match removed {
        Some(_meta) => {
            // Clean up function directory
            let function_dir =
                PathBuf::from("/tmp/blueprint-faas-test/functions").join(&function_id);
            let _ = fs::remove_dir_all(&function_dir);

            let response = serde_json::json!({
                "function_id": function_id,
                "status": "deleted"
            });
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        None => {
            let response = ErrorResponse {
                error: "Function not found".to_string(),
                code: Some("NOT_FOUND".to_string()),
                function_id: Some(function_id),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}

/// Warm a function (pre-allocate instances)
async fn warm_function(function_id: String, store: FunctionStore) -> Result<impl Reply, Rejection> {
    let functions = store.read().await;
    match functions.get(&function_id) {
        Some(_meta) => {
            let response = serde_json::json!({
                "function_id": function_id,
                "status": "warm",
                "instances_warmed": 3
            });
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::OK,
            ))
        }
        None => {
            let response = ErrorResponse {
                error: "Function not found".to_string(),
                code: Some("NOT_FOUND".to_string()),
                function_id: Some(function_id),
            };
            Ok(warp::reply::with_status(
                warp::reply::json(&response),
                StatusCode::NOT_FOUND,
            ))
        }
    }
}

#[tokio::main]
async fn main() {
    // Initialize function store
    let store: FunctionStore = Arc::new(RwLock::new(HashMap::new()));

    // Deploy function: PUT /api/functions/{function_id}
    let deploy = warp::put()
        .and(warp::path!("api" / "functions" / String))
        .and(warp::header::optional::<String>("x-blueprint-config"))
        .and(warp::body::bytes())
        .and(with_store(store.clone()))
        .and_then(
            |function_id: String,
             config_header: Option<String>,
             bytes: bytes::Bytes,
             store: FunctionStore| {
                deploy_function(function_id, config_header, bytes, store)
            },
        );

    // Invoke function: POST /api/functions/{function_id}/invoke
    let invoke = warp::post()
        .and(warp::path!("api" / "functions" / String / "invoke"))
        .and(warp::body::json())
        .and(with_store(store.clone()))
        .and_then(invoke_function);

    // Health check: GET /api/functions/{function_id}/health
    let health = warp::get()
        .and(warp::path!("api" / "functions" / String / "health"))
        .and(with_store(store.clone()))
        .and_then(health_check);

    // Get deployment: GET /api/functions/{function_id}
    let get_deploy = warp::get()
        .and(warp::path!("api" / "functions" / String))
        .and(with_store(store.clone()))
        .and_then(get_deployment);

    // Undeploy function: DELETE /api/functions/{function_id}
    let undeploy = warp::delete()
        .and(warp::path!("api" / "functions" / String))
        .and(with_store(store.clone()))
        .and_then(undeploy_function);

    // Warm function: POST /api/functions/{function_id}/warm
    let warm = warp::post()
        .and(warp::path!("api" / "functions" / String / "warm"))
        .and(with_store(store.clone()))
        .and_then(warm_function);

    let routes = deploy
        .or(invoke)
        .or(health)
        .or(get_deploy)
        .or(undeploy)
        .or(warm);

    println!("Reference HTTP FaaS Server");
    println!("==========================");
    println!();
    println!("Server running on http://localhost:8080");
    println!();
    println!("Endpoints:");
    println!("  PUT    /api/functions/{{id}}          - Deploy function");
    println!("  POST   /api/functions/{{id}}/invoke   - Invoke function");
    println!("  GET    /api/functions/{{id}}/health   - Health check");
    println!("  GET    /api/functions/{{id}}          - Get deployment info");
    println!("  DELETE /api/functions/{{id}}          - Undeploy function");
    println!("  POST   /api/functions/{{id}}/warm     - Warm function");
    println!();
    println!("Implements: Custom FaaS Platform Specification v1.0");
    println!("Functions stored in: /tmp/blueprint-faas-test/functions/");
    println!();

    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

fn with_store(
    store: FunctionStore,
) -> impl Filter<Extract = (FunctionStore,), Error = Infallible> + Clone {
    warp::any().map(move || store.clone())
}
