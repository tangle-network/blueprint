//! CLI commands that drive the blueprint-manager's *local* authz RPC.
//!
//! These never hit the chain — they talk to `http://<manager-url>/upgrades/*`
//! and let MANUAL-policy operators pre-authorize the manager to swap into
//! specific versions without writing an on-chain ack tx. The on-chain
//! `UpgradePolicy` stays `MANUAL`, the audit trail stays unchanged, and yet
//! the swap pipeline (download → sha256 → attestation → drain → respawn)
//! runs the moment the desired version is effective.
//!
//! Manager URL resolution order (highest wins):
//!   1. `--manager-url <URL>` flag
//!   2. `BLUEPRINT_MANAGER_URL` env var
//!   3. `http://127.0.0.1:9000` (the manager's default localhost bind)
//!
//! Every command surfaces a clear error if the manager isn't reachable —
//! these subcommands are an explicit "drive the local manager" surface, not
//! a silent fallback to chain calls.

use color_eyre::eyre::{Context, Result, bail};
use dialoguer::console::style;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::time::Duration;
use url::Url;

const DEFAULT_MANAGER_URL: &str = "http://127.0.0.1:9000";
const MANAGER_URL_ENV: &str = "BLUEPRINT_MANAGER_URL";

/// Resolve which manager to talk to. `flag` wins over env wins over default.
pub fn resolve_manager_url(flag: Option<&Url>) -> Result<Url> {
    if let Some(url) = flag {
        return Ok(url.clone());
    }
    if let Ok(s) = std::env::var(MANAGER_URL_ENV) {
        return Url::parse(&s).with_context(|| format!("parsing {MANAGER_URL_ENV}=`{s}` as a URL"));
    }
    Ok(Url::parse(DEFAULT_MANAGER_URL).expect("literal URL"))
}

fn http_client() -> Result<reqwest::Client> {
    reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .context("building http client")
}

async fn get_json<T: serde::de::DeserializeOwned>(base: &Url, path: &str) -> Result<T> {
    let mut url = base.clone();
    url.set_path(path);
    let client = http_client()?;
    let resp = client.get(url.clone()).send().await.with_context(|| {
        format!("GET {url} — is blueprint-manager running on this host? (set --manager-url or {MANAGER_URL_ENV})")
    })?;
    let status = resp.status();
    let body = resp.text().await.context("reading response body")?;
    if !status.is_success() {
        bail!("GET {url} failed (HTTP {status}): {body}");
    }
    serde_json::from_str(&body).with_context(|| format!("decoding response body: {body}"))
}

async fn post_json<B: Serialize, T: serde::de::DeserializeOwned>(
    base: &Url,
    path: &str,
    body: &B,
) -> Result<T> {
    let mut url = base.clone();
    url.set_path(path);
    let client = http_client()?;
    let resp = client
        .post(url.clone())
        .json(body)
        .send()
        .await
        .with_context(|| {
            format!("POST {url} — is blueprint-manager running? (set --manager-url or {MANAGER_URL_ENV})")
        })?;
    let status = resp.status();
    let raw = resp.text().await.context("reading response body")?;
    if !status.is_success() {
        bail!("POST {url} failed (HTTP {status}): {raw}");
    }
    serde_json::from_str(&raw).with_context(|| format!("decoding response body: {raw}"))
}

// ─────────────────────────────────────────────────────────────────────────────
// Wire types — mirror crate::upgrade::rpc serialized shapes.
// ─────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Deserialize)]
pub struct AvailableEntry {
    pub version_id: u64,
    pub sha256: String,
    pub binary_uri: String,
    pub attestation_hash: String,
    pub published_at: u64,
    pub deprecated: bool,
    pub running: bool,
}

#[derive(Debug, Deserialize)]
pub struct AvailableList {
    pub service_id: u64,
    pub blueprint_id: u64,
    pub active_version_id: u64,
    pub versions: Vec<AvailableEntry>,
}

#[derive(Debug, Deserialize)]
pub struct AuthzView {
    pub service_id: u64,
    pub policy_onchain: String,
    pub whitelisted: Vec<u64>,
    pub pinned: Option<u64>,
    pub skipped: Vec<SkipEntry>,
    pub running: Option<RunningEntry>,
}

#[derive(Debug, Deserialize)]
pub struct SkipEntry {
    pub version_id: u64,
    pub reason: String,
}

#[derive(Debug, Deserialize)]
pub struct RunningEntry {
    pub version_id: u64,
    pub sha256: String,
}

#[derive(Debug, Serialize)]
struct PinBody {
    version_id: u64,
    dry_run: bool,
}

#[derive(Debug, Deserialize)]
pub struct PinResult {
    pub ok: bool,
    pub status: String,
    pub pinned: Option<u64>,
}

#[derive(Debug, Serialize)]
struct WhitelistBody {
    versions: Vec<u64>,
}

#[derive(Debug, Deserialize)]
pub struct WhitelistResult {
    pub whitelisted: Vec<u64>,
}

#[derive(Debug, Serialize)]
struct SkipBody {
    version_id: u64,
    reason: String,
}

#[derive(Debug, Deserialize)]
pub struct SkipResult {
    pub skipped: Vec<SkipEntry>,
}

// ─────────────────────────────────────────────────────────────────────────────
// Public command entrypoints
// ─────────────────────────────────────────────────────────────────────────────

pub async fn list_upgrades(manager: &Url, service_id: u64) -> Result<AvailableList> {
    get_json(manager, &format!("/upgrades/{service_id}/available")).await
}

pub async fn show_authz(manager: &Url, service_id: u64) -> Result<AuthzView> {
    get_json(manager, &format!("/upgrades/{service_id}/authz")).await
}

pub async fn pin_version(
    manager: &Url,
    service_id: u64,
    version_id: u64,
    dry_run: bool,
) -> Result<PinResult> {
    post_json(
        manager,
        &format!("/upgrades/{service_id}/pin"),
        &PinBody {
            version_id,
            dry_run,
        },
    )
    .await
}

pub async fn set_whitelist(
    manager: &Url,
    service_id: u64,
    versions: Vec<u64>,
) -> Result<WhitelistResult> {
    post_json(
        manager,
        &format!("/upgrades/{service_id}/whitelist"),
        &WhitelistBody { versions },
    )
    .await
}

pub async fn add_skip(
    manager: &Url,
    service_id: u64,
    version_id: u64,
    reason: String,
) -> Result<SkipResult> {
    post_json(
        manager,
        &format!("/upgrades/{service_id}/skip"),
        &SkipBody { version_id, reason },
    )
    .await
}

// ─────────────────────────────────────────────────────────────────────────────
// Pretty-printers
// ─────────────────────────────────────────────────────────────────────────────

pub fn print_available(list: &AvailableList, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "service_id": list.service_id,
                "blueprint_id": list.blueprint_id,
                "active_version_id": list.active_version_id,
                "versions": list.versions.iter().map(|v| serde_json::json!({
                    "version_id": v.version_id,
                    "sha256": v.sha256,
                    "binary_uri": v.binary_uri,
                    "attestation_hash": v.attestation_hash,
                    "published_at": v.published_at,
                    "deprecated": v.deprecated,
                    "running": v.running,
                })).collect::<Vec<_>>(),
            }))
            .unwrap()
        );
        return;
    }
    println!(
        "Service {} → blueprint {} (active version {})",
        style(list.service_id).green().bold(),
        list.blueprint_id,
        style(list.active_version_id).cyan().bold(),
    );
    if list.versions.is_empty() {
        println!("  (no versions published)");
        return;
    }
    for v in &list.versions {
        let badge = if v.running {
            style("RUNNING").green().bold().to_string()
        } else if v.deprecated {
            style("deprecated").red().to_string()
        } else if v.version_id == list.active_version_id {
            style("active   ").cyan().to_string()
        } else {
            style("available").dim().to_string()
        };
        println!(
            "  v{:>3}  {}  {}  uri={}",
            v.version_id,
            badge,
            short_hash(&v.sha256),
            v.binary_uri,
        );
    }
}

pub fn print_authz(authz: &AuthzView, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "service_id": authz.service_id,
                "policy_onchain": authz.policy_onchain,
                "whitelisted": authz.whitelisted,
                "pinned": authz.pinned,
                "skipped": authz.skipped.iter().map(|s| serde_json::json!({
                    "version_id": s.version_id,
                    "reason": s.reason,
                })).collect::<Vec<_>>(),
                "running": authz.running.as_ref().map(|r| serde_json::json!({
                    "version_id": r.version_id,
                    "sha256": r.sha256,
                })),
            }))
            .unwrap()
        );
        return;
    }
    println!(
        "Service {} — on-chain policy {}",
        style(authz.service_id).green().bold(),
        style(&authz.policy_onchain).cyan().bold(),
    );
    if let Some(r) = &authz.running {
        println!(
            "  Running:     v{} ({})",
            style(r.version_id).cyan(),
            short_hash(&r.sha256)
        );
    } else {
        println!("  Running:     (not currently served)");
    }
    if let Some(p) = authz.pinned {
        println!("  Pinned:      v{} (one-shot)", style(p).yellow().bold());
    } else {
        println!("  Pinned:      (none)");
    }
    if authz.whitelisted.is_empty() {
        println!("  Whitelist:   (empty)");
    } else {
        println!(
            "  Whitelist:   {}",
            authz
                .whitelisted
                .iter()
                .map(|v| format!("v{v}"))
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
    if authz.skipped.is_empty() {
        println!("  Skipped:     (none)");
    } else {
        println!("  Skipped:");
        for s in &authz.skipped {
            println!("    - v{}: {}", s.version_id, s.reason);
        }
    }
}

pub fn print_pin_result(service_id: u64, r: &PinResult, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(
                &serde_json::to_value(PinJson {
                    ok: r.ok,
                    status: r.status.clone(),
                    pinned: r.pinned,
                    service_id,
                })
                .unwrap()
            )
            .unwrap()
        );
        return;
    }
    let tag = match r.status.as_str() {
        s if s.starts_with("dry_run") => style("(dry-run)").yellow().to_string(),
        "already_running" => style("already_running").dim().to_string(),
        "pinned_swap" => style("pinned_swap").green().bold().to_string(),
        "not_published" => style("not_published").red().bold().to_string(),
        other => other.to_string(),
    };
    let pinned = r
        .pinned
        .map(|p| format!("v{p}"))
        .unwrap_or_else(|| "(cleared)".into());
    println!(
        "{} service {} pin={} status={}",
        style("OK").green().bold(),
        service_id,
        pinned,
        tag,
    );
}

#[derive(Serialize)]
struct PinJson {
    ok: bool,
    status: String,
    pinned: Option<u64>,
    service_id: u64,
}

pub fn print_whitelist_result(service_id: u64, r: &WhitelistResult, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "service_id": service_id,
                "whitelisted": r.whitelisted,
            }))
            .unwrap()
        );
        return;
    }
    if r.whitelisted.is_empty() {
        println!(
            "{} service {} whitelist cleared",
            style("OK").green().bold(),
            service_id
        );
        return;
    }
    println!(
        "{} service {} whitelist now: {}",
        style("OK").green().bold(),
        service_id,
        r.whitelisted
            .iter()
            .map(|v| format!("v{v}"))
            .collect::<Vec<_>>()
            .join(", ")
    );
}

pub fn print_skip_result(service_id: u64, r: &SkipResult, json_out: bool) {
    if json_out {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "service_id": service_id,
                "skipped": r.skipped.iter().map(|s| serde_json::json!({
                    "version_id": s.version_id,
                    "reason": s.reason,
                })).collect::<Vec<_>>(),
            }))
            .unwrap()
        );
        return;
    }
    println!(
        "{} service {} now skipping {} version(s)",
        style("OK").green().bold(),
        service_id,
        r.skipped.len()
    );
    for s in &r.skipped {
        println!("  - v{}: {}", s.version_id, s.reason);
    }
}

fn short_hash(hex: &str) -> String {
    let stripped = hex.strip_prefix("0x").unwrap_or(hex);
    if stripped.len() <= 12 {
        format!("0x{stripped}")
    } else {
        format!("0x{}…", &stripped[..12])
    }
}

/// Parse a comma-separated version list like "1,4,7" into Vec<u64>. Used by
/// `--versions` on the whitelist subcommand.
pub fn parse_version_list(s: &str) -> Result<Vec<u64>> {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return Ok(vec![]);
    }
    trimmed
        .split(',')
        .map(|p| {
            p.trim()
                .parse::<u64>()
                .with_context(|| format!("parsing version `{p}` in list `{s}`"))
        })
        .collect()
}

/// Try to coerce a `Value` to a `Vec<u64>`. Helper used by tests and JSON
/// post-processing on the call sites.
pub fn versions_from_json(value: &Value) -> Result<Vec<u64>> {
    let arr = value
        .as_array()
        .ok_or_else(|| color_eyre::eyre::eyre!("expected JSON array, got {value:?}"))?;
    arr.iter()
        .map(|v| {
            v.as_u64()
                .ok_or_else(|| color_eyre::eyre::eyre!("expected u64, got {v:?}"))
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_version_list_handles_whitespace_and_empty() {
        assert_eq!(parse_version_list("1,2,3").unwrap(), vec![1, 2, 3]);
        assert_eq!(parse_version_list(" 1 , 2 , 3 ").unwrap(), vec![1, 2, 3]);
        assert!(parse_version_list("").unwrap().is_empty());
        assert!(parse_version_list("not-a-number").is_err());
    }

    #[test]
    fn manager_url_resolution_prefers_flag_over_env() {
        let url = Url::parse("http://override:9999").unwrap();
        let resolved = resolve_manager_url(Some(&url)).unwrap();
        assert_eq!(resolved.as_str(), "http://override:9999/");
    }

    #[test]
    fn manager_url_default_falls_back_to_localhost() {
        // Important: when no flag and no env, the default is the manager's
        // documented localhost bind. If this changes, every operator's
        // existing playbook breaks.
        // SAFETY: integration tests run single-threaded for env var changes.
        let prev = std::env::var(MANAGER_URL_ENV).ok();
        // SAFETY: This test mutates process-global env. We restore in a
        // best-effort sense; the goal is just to avoid leaving a stray var
        // that breaks downstream tests in this same process.
        unsafe {
            std::env::remove_var(MANAGER_URL_ENV);
        }
        let resolved = resolve_manager_url(None).unwrap();
        assert_eq!(resolved.as_str(), "http://127.0.0.1:9000/");
        if let Some(prev) = prev {
            unsafe {
                std::env::set_var(MANAGER_URL_ENV, prev);
            }
        }
    }

    #[test]
    fn short_hash_truncates_long() {
        let h = "0xabcdef0123456789deadbeef";
        assert_eq!(short_hash(h), "0xabcdef012345…");
    }

    #[test]
    fn short_hash_preserves_short() {
        // Defensive: short_hash must not panic on inputs shorter than 12
        // hex chars. A truncation slice would panic without this branch.
        assert_eq!(short_hash("0xabcd"), "0xabcd");
        assert_eq!(short_hash("abcdef"), "0xabcdef");
    }
}
