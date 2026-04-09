use crate::command::harness::config::{BlueprintSpec, HarnessConfig};
use crate::command::service::build_request_params;
use crate::command::tangle::DevnetStack;
use alloy_primitives::{Address, Bytes, U256};
use blueprint_client_tangle::{TangleClient, TangleClientConfig, TangleSettings};
use color_eyre::eyre::{Result, eyre};
use std::io::Write as _;
use std::net::TcpListener;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::task::JoinHandle;

/// A spawned blueprint-manager subprocess with isolated env.
struct SpawnedBlueprint {
    name: String,
    service_id: u64,
    port: u16,
    child: Child,
    _log_task: JoinHandle<()>,
    _settings_dir: tempfile::TempDir,
}

pub struct Orchestrator {
    stack: DevnetStack,
    client: TangleClient,
    blueprints: Vec<SpawnedBlueprint>,
}

impl Orchestrator {
    /// Boot the local devnet (anvil + Tangle Core contracts + keystore).
    /// Creates a TangleClient for on-chain service creation.
    pub async fn bootstrap(config: &HarnessConfig) -> Result<Self> {
        if !config.chain.anvil {
            return Err(eyre!(
                "remote RPC mode not yet supported — set chain.anvil = true"
            ));
        }

        println!("Starting local Tangle devnet (anvil + contracts)...");
        let stack = DevnetStack::spawn(config.chain.include_anvil_logs).await?;
        println!("  HTTP RPC:  {}", stack.http_rpc_url());
        println!("  WS RPC:    {}", stack.ws_rpc_url());
        println!("  Tangle:    {:?}", stack.tangle_contract());

        // Build a TangleClient for on-chain operations (service creation, etc.)
        let settings = TangleSettings {
            blueprint_id: 0,
            service_id: None,
            tangle_contract: stack.tangle_contract(),
            restaking_contract: stack.restaking_contract(),
            status_registry_contract: stack.status_registry_contract(),
        };
        let client_config = TangleClientConfig::new(
            stack.http_rpc_url(),
            stack.ws_rpc_url(),
            stack.keystore_path(),
            settings,
        );
        let client = TangleClient::new(client_config)
            .await
            .map_err(|e| eyre!("failed to build TangleClient: {e}"))?;
        println!("  Operator:  {:?}", client.account());
        println!();

        Ok(Self {
            stack,
            client,
            blueprints: Vec::new(),
        })
    }

    /// For each blueprint in config, spawn a `cargo-tangle blueprint run` subprocess
    /// with its own settings.env, env vars, and service_id.
    pub async fn spawn_blueprints(&mut self, config: &HarnessConfig) -> Result<()> {
        let self_exe = std::env::current_exe()
            .map_err(|e| eyre!("failed to find cargo-tangle binary: {e}"))?;

        let operator_address = self.client.account();

        for (idx, bp) in config.blueprints.iter().enumerate() {
            let port = bp
                .port
                .unwrap_or_else(|| allocate_free_port().unwrap_or(9000 + idx as u16));

            // Create a real on-chain service for this blueprint via request_service.
            // This is the production-faithful flow: blueprint_id 0 (pre-seeded), but
            // each config entry gets its own service_id so managers don't conflict.
            let blueprint_id = 0u64; // pre-seeded by DevnetStack
            let params = build_request_params(
                blueprint_id,
                vec![operator_address], // this operator serves it
                None,                   // no operator_exposures
                vec![],                 // any caller permitted
                7200,                   // ttl_blocks (contract min is 3600)
                Address::ZERO,          // native token for payment
                U256::ZERO,             // no initial payment in dev
                Bytes::new(),           // empty service config
            );
            let (_tx, service_id) = self
                .client
                .request_service(params)
                .await
                .map_err(|e| eyre!("[{}] failed to create on-chain service: {e}", bp.name))?;

            println!(
                "[{}] Service created on-chain: service_id={}, port={}, path={}",
                bp.name,
                service_id,
                port,
                bp.path.display()
            );

            // Write a per-blueprint settings.env with the on-chain config
            let settings_dir = tempfile::TempDir::new()
                .map_err(|e| eyre!("failed to create temp settings dir: {e}"))?;
            let settings_path = settings_dir.path().join("settings.env");
            write_settings_env(&settings_path, blueprint_id, service_id, &self.stack)?;

            // Build the subprocess command
            let binary = bp
                .binary
                .as_ref()
                .map(|p| p.to_path_buf())
                .unwrap_or_else(|| self_exe.clone());

            let mut cmd = Command::new(&binary);

            // If using self-exe (cargo-tangle), add subcommand args
            if bp.binary.is_none() {
                cmd.args(["tangle", "blueprint", "run", "--no-vm", "--settings-file"]);
                cmd.arg(&settings_path);
                cmd.args(["--http-rpc-url"]);
                cmd.arg(self.stack.http_rpc_url().as_str());
                cmd.args(["--ws-rpc-url"]);
                cmd.arg(self.stack.ws_rpc_url().as_str());
                cmd.args(["--keystore-path"]);
                cmd.arg(&self.stack.keystore_path());
            }

            // Per-blueprint env isolation: clear and inject only what's needed
            cmd.env_clear();
            // Inherit essential system env
            for key in &[
                "PATH",
                "HOME",
                "USER",
                "TMPDIR",
                "RUST_LOG",
                "RUST_BACKTRACE",
            ] {
                if let Ok(val) = std::env::var(key) {
                    cmd.env(key, &val);
                }
            }

            // Inject Tangle protocol env vars — needed by BlueprintEnvironment::load()
            // in operator binaries (every #[arg(env)] field reads from env vars)
            cmd.env("HTTP_RPC_URL", self.stack.http_rpc_url().as_str());
            cmd.env("WS_RPC_URL", self.stack.ws_rpc_url().as_str());
            cmd.env("KEYSTORE_URI", &self.stack.keystore_path());
            cmd.env("DATA_DIR", self.stack.data_dir().display().to_string());
            cmd.env("BLUEPRINT_ID", blueprint_id.to_string());
            cmd.env("SERVICE_ID", service_id.to_string());
            cmd.env(
                "TANGLE_CONTRACT",
                format!("{:?}", self.stack.tangle_contract()),
            );
            cmd.env(
                "RESTAKING_CONTRACT",
                format!("{:?}", self.stack.restaking_contract()),
            );
            cmd.env(
                "STATUS_REGISTRY_CONTRACT",
                format!("{:?}", self.stack.status_registry_contract()),
            );
            cmd.env("PROTOCOL", "tangle");
            cmd.env("TEST_MODE", "true");

            // Inject per-blueprint env (MODEL, API keys, etc.)
            for (k, v) in &bp.env {
                cmd.env(k, v);
            }
            // Inject port
            cmd.env("PORT", port.to_string());

            // Working directory is the blueprint repo
            cmd.current_dir(&bp.path);

            // Pipe stdout/stderr for log forwarding
            cmd.stdout(std::process::Stdio::piped());
            cmd.stderr(std::process::Stdio::piped());

            let mut child = cmd.spawn().map_err(|e| {
                eyre!(
                    "failed to spawn blueprint-manager for '{}': {e}\n  binary: {}\n  cwd: {}",
                    bp.name,
                    binary.display(),
                    bp.path.display()
                )
            })?;

            // Spawn log forwarder
            let log_task =
                spawn_log_forwarder(bp.name.clone(), child.stdout.take(), child.stderr.take());

            self.blueprints.push(SpawnedBlueprint {
                name: bp.name.clone(),
                service_id,
                port,
                child,
                _log_task: log_task,
                _settings_dir: settings_dir,
            });
        }

        // Health checks
        for bp in &self.blueprints {
            let spec = config
                .blueprints
                .iter()
                .find(|s| s.name == bp.name)
                .unwrap();
            let timeout = Duration::from_secs(spec.startup_timeout_secs);
            let health_url = format!("http://127.0.0.1:{}{}", bp.port, spec.health_path);

            println!("[{}] Waiting for health at {} ...", bp.name, health_url);
            match wait_for_health(&health_url, timeout).await {
                Ok(()) => println!("[{}] Healthy", bp.name),
                Err(e) => {
                    eprintln!("[{}] Health check failed: {e}", bp.name);
                }
            }
        }

        // Register operators with the router (if configured)
        if let Some(router_url) = &config.router.url {
            println!();
            println!("Registering operators with router at {router_url} ...");
            for bp in &self.blueprints {
                let spec = config
                    .blueprints
                    .iter()
                    .find(|s| s.name == bp.name)
                    .unwrap();
                let endpoint_url = spec
                    .public_url
                    .clone()
                    .unwrap_or_else(|| format!("http://127.0.0.1:{}", bp.port));
                match register_with_router(router_url, &bp.name, &endpoint_url, spec).await {
                    Ok(()) => println!("[{}] Registered with router", bp.name),
                    Err(e) => eprintln!("[{}] Router registration failed: {e}", bp.name),
                }
            }
        }

        Ok(())
    }

    /// Block until Ctrl-C or a blueprint exits, then clean up everything.
    pub async fn run_until_shutdown(mut self) -> Result<()> {
        println!();
        println!(
            "Harness up. {} blueprint(s) running.",
            self.blueprints.len()
        );
        println!("Press Ctrl+C to stop.");
        println!();

        // Wait for either Ctrl-C or any child to exit unexpectedly
        tokio::select! {
            _ = tokio::signal::ctrl_c() => {
                println!();
                println!("Shutdown signal received, stopping blueprints...");
            }
            result = wait_for_any_exit(&mut self.blueprints) => {
                match result {
                    Some((name, code)) => {
                        eprintln!();
                        eprintln!("[{name}] exited unexpectedly with code {code:?}, shutting down...");
                    }
                    None => {
                        eprintln!("All blueprints exited.");
                    }
                }
            }
        }

        // Graceful shutdown: SIGTERM → wait 5s → SIGKILL
        for bp in &mut self.blueprints {
            let _ = bp.child.start_kill();
        }
        for bp in &mut self.blueprints {
            let _ = tokio::time::timeout(Duration::from_secs(5), bp.child.wait()).await;
        }

        self.stack.shutdown().await;
        println!("Harness stopped.");
        Ok(())
    }
}

/// Write a settings.env file for one blueprint subprocess.
fn write_settings_env(
    path: &std::path::Path,
    blueprint_id: u64,
    service_id: u64,
    stack: &DevnetStack,
) -> Result<()> {
    let mut f = std::fs::File::create(path)
        .map_err(|e| eyre!("failed to create settings.env at {}: {e}", path.display()))?;
    writeln!(f, "BLUEPRINT_ID={blueprint_id}")?;
    writeln!(f, "SERVICE_ID={service_id}")?;
    writeln!(f, "TANGLE_CONTRACT={:?}", stack.tangle_contract())?;
    writeln!(f, "RESTAKING_CONTRACT={:?}", stack.restaking_contract())?;
    writeln!(
        f,
        "STATUS_REGISTRY_CONTRACT={:?}",
        stack.status_registry_contract()
    )?;
    Ok(())
}

/// Allocate a free port by briefly binding to :0.
fn allocate_free_port() -> Result<u16> {
    let listener =
        TcpListener::bind("127.0.0.1:0").map_err(|e| eyre!("failed to allocate free port: {e}"))?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

/// Poll an HTTP health endpoint until 200 or timeout.
/// Uses raw TCP + HTTP/1.1 to avoid adding reqwest as a CLI dependency.
async fn wait_for_health(url: &str, timeout: Duration) -> Result<()> {
    let start = tokio::time::Instant::now();
    let mut last_error = String::new();

    // Parse host:port from url (expects http://127.0.0.1:{port}{path})
    let url = url.trim_start_matches("http://");
    let (addr, path) = url.split_once('/').unwrap_or((url, ""));
    let path = format!("/{path}");

    loop {
        if start.elapsed() > timeout {
            return Err(eyre!(
                "health check timed out after {}s — last error: {last_error}",
                timeout.as_secs()
            ));
        }

        match tokio::net::TcpStream::connect(addr).await {
            Ok(mut stream) => {
                use tokio::io::{AsyncReadExt, AsyncWriteExt};
                let req =
                    format!("GET {path} HTTP/1.1\r\nHost: {addr}\r\nConnection: close\r\n\r\n");
                if stream.write_all(req.as_bytes()).await.is_ok() {
                    let mut buf = vec![0u8; 256];
                    if let Ok(n) = stream.read(&mut buf).await {
                        let response = String::from_utf8_lossy(&buf[..n]);
                        if response.contains("200") {
                            return Ok(());
                        }
                        last_error = response.lines().next().unwrap_or("unknown").to_string();
                    }
                }
            }
            Err(e) => {
                last_error = e.to_string();
            }
        }

        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

/// Spawn a task that reads stdout+stderr and prefixes each line with [name].
fn spawn_log_forwarder(
    name: String,
    stdout: Option<tokio::process::ChildStdout>,
    stderr: Option<tokio::process::ChildStderr>,
) -> JoinHandle<()> {
    tokio::spawn(async move {
        let name2 = name.clone();

        let stdout_task = tokio::spawn(async move {
            if let Some(out) = stdout {
                let reader = BufReader::new(out);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    println!("[{name}] {line}");
                }
            }
        });

        let stderr_task = tokio::spawn(async move {
            if let Some(err) = stderr {
                let reader = BufReader::new(err);
                let mut lines = reader.lines();
                while let Ok(Some(line)) = lines.next_line().await {
                    eprintln!("[{name2}] {line}");
                }
            }
        });

        let _ = tokio::join!(stdout_task, stderr_task);
    })
}

/// Wait until any child process exits. Returns the name and exit code.
async fn wait_for_any_exit(blueprints: &mut [SpawnedBlueprint]) -> Option<(String, Option<i32>)> {
    loop {
        for bp in blueprints.iter_mut() {
            match bp.child.try_wait() {
                Ok(Some(status)) => {
                    return Some((bp.name.clone(), status.code()));
                }
                Ok(None) => {} // still running
                Err(_) => {
                    return Some((bp.name.clone(), None));
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(250)).await;
    }
}

/// Register an operator with the Tangle Router via POST /api/operators.
async fn register_with_router(
    router_url: &str,
    name: &str,
    endpoint_url: &str,
    spec: &BlueprintSpec,
) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let blueprint_type = spec.blueprint_type.as_deref().unwrap_or("inference");

    // Build the JSON payload
    let models_json: Vec<String> = spec
        .models
        .iter()
        .map(|m| {
            format!(
                r#"{{"modelId":"{}","inputPrice":{},"outputPrice":{}}}"#,
                m.id, m.input_price, m.output_price
            )
        })
        .collect();
    let models_array = format!("[{}]", models_json.join(","));

    let body = format!(
        r#"{{"name":"{}","endpointUrl":"{}","blueprintType":"{}","models":{}}}"#,
        name, endpoint_url, blueprint_type, models_array
    );

    // Parse the router URL to get host:port
    let url = router_url
        .trim_start_matches("https://")
        .trim_start_matches("http://");
    let is_https = router_url.starts_with("https://");

    if is_https {
        // For HTTPS (production router), shell out to curl
        let output = tokio::process::Command::new("curl")
            .args([
                "-s",
                "-X",
                "POST",
                &format!("{router_url}/api/operators"),
                "-H",
                "Content-Type: application/json",
                "-d",
                &body,
            ])
            .output()
            .await
            .map_err(|e| eyre!("curl failed: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(eyre!("router registration failed: {stderr}"));
        }
        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.contains("error") {
            return Err(eyre!("router returned error: {stdout}"));
        }
    } else {
        // For HTTP (local router), use raw TCP
        let (addr, _) = url.split_once('/').unwrap_or((url, ""));
        let mut stream = tokio::net::TcpStream::connect(addr)
            .await
            .map_err(|e| eyre!("failed to connect to router at {addr}: {e}"))?;

        let req = format!(
            "POST /api/operators HTTP/1.1\r\nHost: {addr}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
            body.len()
        );
        stream.write_all(req.as_bytes()).await?;

        let mut buf = vec![0u8; 4096];
        let n = stream.read(&mut buf).await?;
        let response = String::from_utf8_lossy(&buf[..n]);

        if !response.contains("200") && !response.contains("201") {
            return Err(eyre!(
                "router registration failed: {}",
                response.lines().next().unwrap_or("unknown")
            ));
        }
    }

    Ok(())
}
