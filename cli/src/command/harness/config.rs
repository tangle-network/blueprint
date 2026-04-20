use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct HarnessConfig {
    #[serde(default)]
    pub chain: ChainConfig,
    #[serde(default)]
    pub router: RouterConfig,
    #[serde(rename = "blueprint", default)]
    pub blueprints: Vec<BlueprintSpec>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ChainConfig {
    /// Spawn local anvil. Default true. Set false to connect to an existing chain.
    #[serde(default = "default_true")]
    pub anvil: bool,
    /// Chain ID (default 31337 for local Anvil).
    #[serde(default = "default_chain_id")]
    pub chain_id: u64,
    /// Stream anvil logs.
    #[serde(default)]
    pub include_anvil_logs: bool,
    /// HTTP RPC URL for remote chains (required when anvil = false).
    #[serde(default)]
    pub rpc_url: Option<String>,
    /// WebSocket RPC URL for remote chains (required when anvil = false).
    #[serde(default)]
    pub ws_url: Option<String>,
    /// Tangle contract address on the remote chain.
    #[serde(default)]
    pub tangle_contract: Option<String>,
    /// Restaking contract address on the remote chain.
    #[serde(default)]
    pub restaking_contract: Option<String>,
    /// Path to operator keystore (required for remote chains).
    #[serde(default)]
    pub keystore_path: Option<String>,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            anvil: true,
            chain_id: default_chain_id(),
            include_anvil_logs: false,
            rpc_url: None,
            ws_url: None,
            tangle_contract: None,
            restaking_contract: None,
            keystore_path: None,
        }
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct RouterConfig {
    /// Whether to also spawn the local Tangle Router (Next.js). Default false for now.
    #[serde(default)]
    pub spawn: bool,
    /// Port for the router (default 3000).
    #[serde(default = "default_router_port")]
    pub port: u16,
    /// Router URL to register operators with after health check.
    /// Set to "https://router.tangle.tools" for production or "http://localhost:3000" for local.
    #[serde(default)]
    pub url: Option<String>,
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            spawn: false,
            port: default_router_port(),
            url: None,
        }
    }
}

/// A model served by this operator, registered with the router.
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ModelSpec {
    /// Model ID (e.g. "meta-llama/Llama-3.1-8B-Instruct")
    pub id: String,
    /// Input price per token (optional)
    #[serde(default)]
    pub input_price: f64,
    /// Output price per token (optional)
    #[serde(default)]
    pub output_price: f64,
}

/// Source for the operator binary.
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BlueprintSource {
    /// Use a local pre-built binary path.
    Binary(PathBuf),
    /// Build from source via `cargo build --release` in the blueprint repo.
    Build,
    /// Download from a GitHub Release.
    /// Format: repo = "owner/repo", tag = "v0.2.0"
    GithubRelease {
        repo: String,
        tag: String,
        /// Binary name inside the tarball (default: inferred from repo name).
        #[serde(default)]
        binary_name: Option<String>,
    },
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct BlueprintSpec {
    /// Short name for logs and CLI filtering.
    pub name: String,
    /// Path to the blueprint repo (can use ~ and env vars).
    pub path: PathBuf,
    /// Optional port override — if not set, the blueprint picks its own.
    pub port: Option<u16>,
    /// Optional: use a pre-built binary instead of cargo run.
    /// Deprecated in favor of `source`. Kept for backward compat.
    pub binary: Option<PathBuf>,
    /// How to get the operator binary. If not set, falls back to `binary` field.
    #[serde(default)]
    pub source: Option<BlueprintSource>,
    /// Environment variables to inject.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Health check path (default /health).
    #[serde(default = "default_health_path")]
    pub health_path: String,
    /// Startup timeout in seconds (default 120).
    #[serde(default = "default_startup_timeout")]
    pub startup_timeout_secs: u64,
    /// Blueprint type for router registration (e.g. "llm", "embedding", "image").
    #[serde(default)]
    pub blueprint_type: Option<String>,
    /// Models this operator serves (registered with the router for discovery).
    #[serde(default)]
    pub models: Vec<ModelSpec>,
    /// Public endpoint URL override for router registration.
    /// If not set, uses http://localhost:{port}. Set this to a tunnel URL
    /// (ngrok, cloudflare) when registering with the production router.
    #[serde(default)]
    pub public_url: Option<String>,
    /// Command to run as the E2E test after the blueprint is healthy.
    /// Used by `cargo tangle harness test`. Runs in the blueprint's `path` directory.
    #[serde(default)]
    pub test_command: Option<String>,
}

fn default_true() -> bool {
    true
}
fn default_chain_id() -> u64 {
    31337
}
fn default_router_port() -> u16 {
    3000
}
fn default_health_path() -> String {
    "/health".to_string()
}
fn default_startup_timeout() -> u64 {
    120
}

impl HarnessConfig {
    pub fn load(path: Option<&Path>) -> Result<Self> {
        let path = match path {
            Some(p) => p.to_path_buf(),
            None => Self::default_path()?,
        };

        if !path.exists() {
            return Err(eyre!(
                "harness config not found at {}. Create one or pass --config",
                path.display()
            ));
        }

        let content = std::fs::read_to_string(&path)
            .map_err(|e| eyre!("failed to read {}: {e}", path.display()))?;
        let mut config: Self = toml::from_str(&content)
            .map_err(|e| eyre!("failed to parse {}: {e}", path.display()))?;
        config.expand_paths();
        config.expand_env()?;
        Ok(config)
    }

    fn default_path() -> Result<PathBuf> {
        let cwd = std::env::current_dir()?.join("harness.toml");
        if cwd.exists() {
            return Ok(cwd);
        }
        let home = std::env::var("HOME").map_err(|_| eyre!("HOME not set"))?;
        Ok(PathBuf::from(home).join(".tangle").join("harness.toml"))
    }

    fn expand_paths(&mut self) {
        let Ok(home) = std::env::var("HOME") else {
            return;
        };
        let home = PathBuf::from(&home);
        for bp in &mut self.blueprints {
            if let Ok(s) = bp.path.strip_prefix("~") {
                bp.path = home.join(s);
            }
            if let Some(bin) = bp.binary.as_ref()
                && let Ok(s) = bin.strip_prefix("~")
            {
                bp.binary = Some(home.join(s));
            }
        }
    }

    fn expand_env(&mut self) -> Result<()> {
        for bp in &mut self.blueprints {
            for value in bp.env.values_mut() {
                if let Some(stripped) = value.strip_prefix("${").and_then(|s| s.strip_suffix('}')) {
                    *value = std::env::var(stripped).map_err(|_| {
                        eyre!(
                            "env var {stripped} not set (referenced in blueprint '{}')",
                            bp.name
                        )
                    })?;
                }
            }
        }
        Ok(())
    }

    /// Apply CLI flag overrides to the chain config.
    pub fn apply_chain_overrides(&mut self, args: &super::ChainArgs) {
        if args.no_anvil {
            self.chain.anvil = false;
        }
        if let Some(ref url) = args.rpc_url {
            self.chain.rpc_url = Some(url.clone());
            self.chain.anvil = false; // explicit RPC implies no local Anvil
        }
        if let Some(ref url) = args.ws_url {
            self.chain.ws_url = Some(url.clone());
        }
        if let Some(id) = args.chain_id {
            self.chain.chain_id = id;
        }
        if let Some(ref addr) = args.tangle_contract {
            self.chain.tangle_contract = Some(addr.clone());
        }
        if let Some(ref path) = args.keystore_path {
            self.chain.keystore_path = Some(path.clone());
        }
        if let Some(ref url) = args.router_url {
            self.router.url = Some(url.clone());
        }
    }

    pub fn filter(&mut self, only: Option<&str>) {
        if let Some(only) = only {
            let names: Vec<&str> = only.split(',').map(str::trim).collect();
            self.blueprints
                .retain(|bp| names.contains(&bp.name.as_str()));
        }
    }

    /// Compose multiple per-repo harness configs into one.
    /// Each path is a directory containing a `harness.toml`.
    /// Chain config comes from the first one; blueprint specs are merged.
    pub fn compose(paths: &[PathBuf]) -> Result<Self> {
        if paths.is_empty() {
            return Err(eyre!("--compose requires at least one blueprint path"));
        }

        let mut merged = Self::default();
        let mut first = true;

        for dir in paths {
            let config_path = if dir.is_file() && dir.ends_with("harness.toml") {
                dir.clone()
            } else {
                dir.join("harness.toml")
            };

            if !config_path.exists() {
                return Err(eyre!(
                    "no harness.toml found in {}",
                    dir.display()
                ));
            }

            let mut config = Self::load(Some(&config_path))?;

            if first {
                merged.chain = config.chain;
                merged.router = config.router;
                first = false;
            }

            merged.blueprints.append(&mut config.blueprints);
        }

        Ok(merged)
    }

    /// Discover all harness.toml files in known blueprint directories.
    /// Searches: current directory children, ~/webb/**/harness.toml,
    /// and ~/.tangle/harnesses/ registry.
    pub fn discover() -> Vec<(String, PathBuf)> {
        let mut found = Vec::new();

        // Check ~/webb/ for blueprint repos with harness.toml
        if let Ok(home) = std::env::var("HOME") {
            let webb_dir = PathBuf::from(&home).join("webb");
            if let Ok(entries) = std::fs::read_dir(&webb_dir) {
                for entry in entries.flatten() {
                    let harness = entry.path().join("harness.toml");
                    if harness.exists() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        found.push((name, harness));
                    }
                }
            }

            // Also check ~/code/
            let code_dir = PathBuf::from(&home).join("code");
            if let Ok(entries) = std::fs::read_dir(&code_dir) {
                for entry in entries.flatten() {
                    let harness = entry.path().join("harness.toml");
                    if harness.exists() {
                        let name = entry.file_name().to_string_lossy().to_string();
                        if !found.iter().any(|(n, _)| n == &name) {
                            found.push((name, harness));
                        }
                    }
                }
            }
        }

        // Check current directory
        let cwd_harness = PathBuf::from("harness.toml");
        if cwd_harness.exists() {
            let name = std::env::current_dir()
                .ok()
                .and_then(|d| d.file_name().map(|n| n.to_string_lossy().to_string()))
                .unwrap_or_else(|| ".".to_string());
            if !found.iter().any(|(_, p)| p == &cwd_harness) {
                found.insert(0, (name, cwd_harness));
            }
        }

        found.sort_by(|a, b| a.0.cmp(&b.0));
        found
    }
}
