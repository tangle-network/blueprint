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
    /// Spawn local anvil. Default true. Set false to use existing RPC.
    #[serde(default = "default_true")]
    pub anvil: bool,
    /// Chain ID for local anvil (default 31337).
    #[serde(default = "default_chain_id")]
    pub chain_id: u64,
    /// Stream anvil logs.
    #[serde(default)]
    pub include_anvil_logs: bool,
}

impl Default for ChainConfig {
    fn default() -> Self {
        Self {
            anvil: true,
            chain_id: default_chain_id(),
            include_anvil_logs: false,
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
}

impl Default for RouterConfig {
    fn default() -> Self {
        Self {
            spawn: false,
            port: default_router_port(),
        }
    }
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
    pub binary: Option<PathBuf>,
    /// Environment variables to inject.
    #[serde(default)]
    pub env: HashMap<String, String>,
    /// Health check path (default /health).
    #[serde(default = "default_health_path")]
    pub health_path: String,
    /// Startup timeout in seconds (default 120).
    #[serde(default = "default_startup_timeout")]
    pub startup_timeout_secs: u64,
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

    pub fn filter(&mut self, only: Option<&str>) {
        if let Some(only) = only {
            let names: Vec<&str> = only.split(',').map(str::trim).collect();
            self.blueprints
                .retain(|bp| names.contains(&bp.name.as_str()));
        }
    }
}
