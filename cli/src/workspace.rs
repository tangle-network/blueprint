//! Per-directory Tangle workspace config (`.tangle.toml`).
//!
//! A workspace file lets a project pin the RPC endpoints, contract addresses, and defaults
//! for every cargo-tangle command in that directory. Commands walk up from CWD looking for
//! a `.tangle.toml`, and fall back to `TANGLE_CONFIG=<path>` if set. This turns 11-arg
//! invocations into `cargo-tangle jobs submit --job 0 --payload-hex ...` against the active
//! network.
//!
//! Typical file:
//! ```toml
//! network = "local"                           # default network for commands
//!
//! [networks.local]
//! http_rpc_url = "http://127.0.0.1:8545"
//! ws_rpc_url = "ws://127.0.0.1:8545"
//! tangle_contract = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
//! staking_contract = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
//! status_registry_contract = "0x8f86403A4DE0bb5791fa46B8e795C547942fE4Cf"
//! chain_id = 31337
//!
//! [defaults]
//! keystore_path = "./keystore"
//! blueprint_id = 0
//! service_id = 0
//! ```

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::{env, fs};

use alloy_primitives::Address;
use color_eyre::eyre::{Context, Result, eyre};
use serde::{Deserialize, Serialize};
use url::Url;

pub const WORKSPACE_FILE: &str = ".tangle.toml";
pub const WORKSPACE_ENV: &str = "TANGLE_CONFIG";

/// A Tangle workspace loaded from a `.tangle.toml` file.
#[derive(Debug, Clone)]
pub struct TangleWorkspace {
    /// Path to the file that was loaded.
    pub source: PathBuf,
    /// Name of the active network (default `"local"`).
    pub active: String,
    /// All configured networks.
    pub networks: HashMap<String, Network>,
    /// Command defaults.
    pub defaults: Defaults,
}

/// A single network configuration (local devnet, testnet, mainnet, ...).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Network {
    pub http_rpc_url: Url,
    pub ws_rpc_url: Url,
    pub tangle_contract: Address,
    pub staking_contract: Address,
    #[serde(default)]
    pub status_registry_contract: Option<Address>,
    #[serde(default)]
    pub chain_id: Option<u64>,
}

/// Default values for commands that the workspace can fill in.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Defaults {
    #[serde(default)]
    pub keystore_path: Option<PathBuf>,
    #[serde(default)]
    pub blueprint_id: Option<u64>,
    #[serde(default)]
    pub service_id: Option<u64>,
}

/// Raw disk representation — deserialised directly from TOML, then validated.
#[derive(Debug, Deserialize, Serialize, Default)]
struct Raw {
    #[serde(default)]
    network: Option<String>,
    #[serde(default)]
    networks: HashMap<String, Network>,
    #[serde(default)]
    defaults: Defaults,
}

impl TangleWorkspace {
    /// Load an explicit workspace file.
    pub fn load(path: &Path) -> Result<Self> {
        let content =
            fs::read_to_string(path).with_context(|| format!("reading {}", path.display()))?;
        let raw: Raw =
            toml::from_str(&content).with_context(|| format!("parsing {}", path.display()))?;

        let active = raw.network.unwrap_or_else(|| "local".to_string());
        if !raw.networks.contains_key(&active) {
            return Err(eyre!(
                "{}: active network '{active}' is not defined under [networks.{active}]",
                path.display()
            ));
        }

        Ok(Self {
            source: path.to_path_buf(),
            active,
            networks: raw.networks,
            defaults: raw.defaults,
        })
    }

    /// Try to find a workspace. Order:
    ///   1. `$TANGLE_CONFIG` env var if set
    ///   2. walk up from CWD looking for `.tangle.toml`
    ///
    /// Returns `Ok(None)` when none is found (not an error).
    pub fn discover() -> Result<Option<Self>> {
        if let Ok(path) = env::var(WORKSPACE_ENV) {
            let path = PathBuf::from(path);
            return Self::load(&path).map(Some);
        }

        let raw = env::current_dir().context("resolving CWD for workspace discovery")?;
        // Canonicalise so the walk resolves through symlinks into the real
        // project tree — otherwise a symlinked project dir never finds its
        // real parent.
        let start = fs::canonicalize(&raw).unwrap_or(raw);
        let mut dir: Option<&Path> = Some(&start);
        while let Some(d) = dir {
            let candidate = d.join(WORKSPACE_FILE);
            if candidate.is_file() {
                return Self::load(&candidate).map(Some);
            }
            dir = d.parent();
        }
        Ok(None)
    }

    /// Network selected by the `network = "…"` key, or the only one defined.
    pub fn active_network(&self) -> Result<&Network> {
        self.network(&self.active)
    }

    /// Named network lookup with a precise error.
    pub fn network(&self, name: &str) -> Result<&Network> {
        self.networks.get(name).ok_or_else(|| {
            let available: Vec<&String> = self.networks.keys().collect();
            eyre!(
                "network '{name}' not defined in {}. Available: {:?}",
                self.source.display(),
                available
            )
        })
    }

    /// Serialise + atomically write to disk. Optionally prepend a header
    /// (e.g. a managed-by marker) so external tooling can recognise the file.
    pub fn write_with_header(&self, header: Option<&str>) -> Result<()> {
        let raw = Raw {
            network: Some(self.active.clone()),
            networks: self.networks.clone(),
            defaults: self.defaults.clone(),
        };
        let mut body = String::new();
        if let Some(h) = header {
            body.push_str(h);
            if !h.ends_with('\n') {
                body.push('\n');
            }
            body.push('\n');
        }
        body.push_str(&toml::to_string_pretty(&raw).context("serialising workspace")?);
        atomic_write(&self.source, &body)
    }

    /// Serialise + atomically write to disk without a header.
    pub fn write(&self) -> Result<()> {
        self.write_with_header(None)
    }
}

/// Atomically write `body` to `path`.
///
/// Uses a PID- and nanosecond-suffixed sibling file so an interrupted write
/// never leaves an ambiguous `.tangle.tmp` that could shadow a real file (the
/// naive `path.with_extension("tmp")` collapses `.tangle.toml` -> `.tangle.tmp`
/// because `set_extension` strips the last extension including on dotfiles).
/// Stale siblings from prior crashes are cleaned up opportunistically.
fn atomic_write(path: &Path, body: &str) -> Result<()> {
    use std::time::{SystemTime, UNIX_EPOCH};
    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let fname = path
        .file_name()
        .and_then(|s| s.to_str())
        .ok_or_else(|| eyre!("non-utf8 workspace path"))?;
    // Best-effort sweep of stale siblings from previous crashes. Temp files are
    // siblings named `<fname>.tmp.<pid>.<nonce>` (no extra leading dot, since
    // `fname` is typically already a dotfile).
    let tmp_prefix = format!("{fname}.tmp.");
    if let Ok(entries) = fs::read_dir(parent) {
        for e in entries.flatten() {
            if let Some(name) = e.file_name().to_str() {
                if name.starts_with(&tmp_prefix) {
                    let _ = fs::remove_file(e.path());
                }
            }
        }
    }
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or_default();
    let tmp = parent.join(format!("{fname}.tmp.{}.{nonce}", std::process::id()));
    fs::write(&tmp, body).with_context(|| format!("writing {}", tmp.display()))?;
    fs::rename(&tmp, path)
        .with_context(|| format!("renaming {} -> {}", tmp.display(), path.display()))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    fn sample() -> &'static str {
        r#"
network = "local"

[networks.local]
http_rpc_url = "http://127.0.0.1:8545"
ws_rpc_url   = "ws://127.0.0.1:8545"
tangle_contract          = "0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9"
staking_contract       = "0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512"
status_registry_contract = "0x8f86403A4DE0bb5791fa46B8e795C547942fE4Cf"
chain_id = 31337

[defaults]
blueprint_id = 0
service_id   = 0
"#
    }

    #[test]
    fn loads_and_resolves_active_network() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(WORKSPACE_FILE);
        fs::write(&path, sample()).unwrap();
        let ws = TangleWorkspace::load(&path).unwrap();
        assert_eq!(ws.active, "local");
        let net = ws.active_network().unwrap();
        assert_eq!(net.chain_id, Some(31337));
        assert_eq!(
            net.tangle_contract.to_string().to_lowercase(),
            "0xcf7ed3acca5a467e9e704c703e8d87f634fb0fc9"
        );
        assert_eq!(ws.defaults.blueprint_id, Some(0));
    }

    #[test]
    fn missing_active_network_errors() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(WORKSPACE_FILE);
        fs::write(&path, "network = \"ghost\"\n[networks.local]\nhttp_rpc_url=\"http://x\"\nws_rpc_url=\"ws://x\"\ntangle_contract=\"0x0000000000000000000000000000000000000000\"\nstaking_contract=\"0x0000000000000000000000000000000000000000\"\n").unwrap();
        assert!(TangleWorkspace::load(&path).is_err());
    }

    #[test]
    fn roundtrip_write_load() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(WORKSPACE_FILE);
        fs::write(&path, sample()).unwrap();
        let ws = TangleWorkspace::load(&path).unwrap();
        let out = dir.path().join("out.toml");
        let mut ws2 = ws.clone();
        ws2.source = out.clone();
        ws2.write().unwrap();
        let reloaded = TangleWorkspace::load(&out).unwrap();
        assert_eq!(reloaded.active, ws.active);
        assert_eq!(reloaded.active_network().unwrap().chain_id, Some(31337));
    }

    #[test]
    fn malformed_toml_gives_specific_error() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(WORKSPACE_FILE);
        fs::write(&path, "this = is [ not valid toml").unwrap();
        let err = TangleWorkspace::load(&path).unwrap_err();
        assert!(err.to_string().contains("parsing"), "{err}");
    }

    #[test]
    fn write_with_header_roundtrip_prepends_marker() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(WORKSPACE_FILE);
        fs::write(&path, sample()).unwrap();
        let ws = TangleWorkspace::load(&path).unwrap();
        let out = dir.path().join("header.toml");
        let mut ws2 = ws.clone();
        ws2.source = out.clone();
        ws2.write_with_header(Some("# managed-by = \"test\""))
            .unwrap();
        let body = fs::read_to_string(&out).unwrap();
        assert!(body.starts_with("# managed-by = \"test\""), "{body}");
        // Marker doesn't break parsing.
        let reloaded = TangleWorkspace::load(&out).unwrap();
        assert_eq!(reloaded.active, ws.active);
    }

    #[test]
    fn atomic_write_cleans_up_orphan_tmp_siblings() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join(".tangle.toml");
        // Plant an orphan tmp sibling from a hypothetical prior crash.
        let orphan = dir.path().join(".tangle.toml.tmp.9999999.123");
        fs::write(&orphan, "garbage").unwrap();
        assert!(orphan.exists());

        fs::write(&path, sample()).unwrap();
        let ws = TangleWorkspace::load(&path).unwrap();
        ws.write().unwrap();
        // The write should have swept the orphan.
        assert!(!orphan.exists(), "orphan tmp file should be cleaned up");
        assert!(path.exists());
    }

    #[test]
    fn env_override_wins_over_discovery() {
        let dir = TempDir::new().unwrap();
        let path = dir.path().join("custom-name.toml");
        fs::write(&path, sample()).unwrap();
        // SAFETY: single-threaded test; no other test in this module touches
        // $TANGLE_CONFIG concurrently.
        // SAFETY: set_var/remove_var are unsafe since Rust 1.87.
        unsafe {
            std::env::set_var(WORKSPACE_ENV, &path);
        }
        let loaded = TangleWorkspace::discover()
            .unwrap()
            .expect("env var should make discovery succeed");
        assert_eq!(loaded.source, path);
        unsafe {
            std::env::remove_var(WORKSPACE_ENV);
        }
    }
}
