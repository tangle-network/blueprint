//! Remote deployment preflight checks and bootstrap helpers.

use super::config::{CloudConfig, CloudProvider};
use color_eyre::{Result, eyre::eyre};
use std::fmt::Write as _;
use std::net::IpAddr;
use std::path::Path;
use std::path::PathBuf;

pub async fn run(
    provider: Option<CloudProvider>,
    tee_required: bool,
    bootstrap_env: bool,
    write_env_file: Option<PathBuf>,
) -> Result<()> {
    let config = CloudConfig::load()?;
    let providers = selected_providers(&config, provider);

    if providers.is_empty() {
        println!("No cloud providers configured in CLI config.");
        println!("Run `cargo tangle cloud configure <provider>` first.");
        return Err(eyre!("No cloud providers configured"));
    }

    println!("Cloud preflight checks\n");

    let attestation_policy = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY")
        .unwrap_or_else(|_| "cryptographic".to_string())
        .trim()
        .to_ascii_lowercase();
    let policy_valid = matches!(attestation_policy.as_str(), "cryptographic" | "structural");
    let cryptographic = attestation_policy == "cryptographic";
    let cryptographic_cmd = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD").ok();

    let mut any_fail = false;

    for selected in &providers {
        let configured = config.providers.contains_key(selected);
        let credentials_ok = credentials_present(*selected);
        let supports_tee = provider_supports_tee(*selected);
        let tee_ok = !tee_required || supports_tee;
        let gcp_cidrs_ok = if *selected == CloudProvider::GCP {
            gcp_cidr_prerequisites_met()
        } else {
            true
        };

        let mut notes = Vec::new();
        if !configured {
            notes.push("not configured via `cloud configure`".to_string());
        }
        if !credentials_ok {
            notes.push("credentials/env not detected".to_string());
        }
        if tee_required && !supports_tee {
            notes.push("provider does not support confidential compute".to_string());
        }
        if *selected == CloudProvider::GCP && !gcp_cidrs_ok {
            notes.push(
                "set BLUEPRINT_ALLOWED_SSH_CIDRS and BLUEPRINT_ALLOWED_QOS_CIDRS".to_string(),
            );
        }

        let pass = configured && credentials_ok && tee_ok && gcp_cidrs_ok;
        if !pass {
            any_fail = true;
        }

        println!("- {}: {}", selected, if pass { "PASS" } else { "FAIL" });
        println!("  configured: {}", yes_no(configured));
        println!("  credentials: {}", yes_no(credentials_ok));
        if tee_required {
            println!("  tee-capable: {}", yes_no(supports_tee));
        }
        if !notes.is_empty() {
            println!("  notes: {}", notes.join("; "));
        }
    }

    if tee_required {
        println!("\nTEE policy");
        println!("- BLUEPRINT_REMOTE_TEE_REQUIRED=true");
        println!("- BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY={attestation_policy}");
        if !policy_valid {
            any_fail = true;
            println!(
                "- BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY: INVALID (use 'cryptographic' or 'structural')"
            );
        } else if cryptographic {
            let ok = cryptographic_cmd
                .as_deref()
                .map(command_spec_is_executable)
                .unwrap_or(false);
            if ok {
                println!(
                    "- BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD: configured ({})",
                    cryptographic_cmd.as_deref().unwrap_or_default()
                );
            } else {
                any_fail = true;
                println!(
                    "- BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD: MISSING or NOT EXECUTABLE (required for cryptographic policy)"
                );
            }
        }
    }

    if bootstrap_env || write_env_file.is_some() {
        let env_output = render_bootstrap_env(
            &config,
            providers[0],
            tee_required,
            &attestation_policy,
            cryptographic_cmd.as_deref(),
        );

        println!("\nBootstrap env");
        println!("{env_output}");

        if let Some(path) = write_env_file {
            std::fs::write(&path, &env_output)?;
            println!("Wrote bootstrap env file: {}", path.display());
        }
    }

    if any_fail {
        println!("\nPreflight result: FAIL");
        return Err(eyre!("Cloud preflight checks failed"));
    } else {
        println!("\nPreflight result: PASS");
    }

    Ok(())
}

fn selected_providers(config: &CloudConfig, provider: Option<CloudProvider>) -> Vec<CloudProvider> {
    if let Some(p) = provider {
        return vec![p];
    }
    if let Some(default) = config.default_provider {
        return vec![default];
    }
    let mut providers: Vec<_> = config.providers.keys().copied().collect();
    providers.sort_by_key(|p| p.to_string());
    providers
}

fn yes_no(v: bool) -> &'static str {
    if v { "yes" } else { "no" }
}

fn provider_supports_tee(provider: CloudProvider) -> bool {
    matches!(
        provider,
        CloudProvider::AWS | CloudProvider::GCP | CloudProvider::Azure
    )
}

fn credentials_present(provider: CloudProvider) -> bool {
    match provider {
        CloudProvider::AWS => {
            let env_ok = std::env::var("AWS_ACCESS_KEY_ID").is_ok()
                && std::env::var("AWS_SECRET_ACCESS_KEY").is_ok();
            let file_ok = dirs::home_dir()
                .map(|home| home.join(".aws").join("credentials"))
                .map(|path| path.exists())
                .unwrap_or(false);
            env_ok || file_ok
        }
        CloudProvider::GCP => {
            std::env::var("GCP_PROJECT_ID").is_ok()
                && std::env::var("GOOGLE_APPLICATION_CREDENTIALS")
                    .ok()
                    .map(PathBuf::from)
                    .map(|path| path.exists())
                    .unwrap_or(false)
        }
        CloudProvider::Azure => {
            std::env::var("AZURE_SUBSCRIPTION_ID").is_ok()
                && std::env::var("AZURE_CLIENT_ID").is_ok()
                && std::env::var("AZURE_CLIENT_SECRET").is_ok()
                && std::env::var("AZURE_TENANT_ID").is_ok()
        }
        CloudProvider::DigitalOcean => std::env::var("DIGITALOCEAN_TOKEN").is_ok(),
        CloudProvider::Vultr => std::env::var("VULTR_API_KEY").is_ok(),
        CloudProvider::Hetzner => std::env::var("HETZNER_API_TOKEN").is_ok(),
        CloudProvider::RunPod => std::env::var("RUNPOD_API_KEY").is_ok(),
        CloudProvider::LambdaLabs => std::env::var("LAMBDA_LABS_API_KEY").is_ok(),
        CloudProvider::PrimeIntellect => std::env::var("PRIME_INTELLECT_API_KEY").is_ok(),
        CloudProvider::VastAi => std::env::var("VAST_AI_API_KEY").is_ok(),
        CloudProvider::Crusoe => {
            std::env::var("CRUSOE_API_KEY").is_ok() && std::env::var("CRUSOE_API_SECRET").is_ok()
        }
    }
}

fn gcp_cidr_prerequisites_met() -> bool {
    let ssh_cidrs = std::env::var("BLUEPRINT_ALLOWED_SSH_CIDRS").ok();
    let qos_cidrs = std::env::var("BLUEPRINT_ALLOWED_QOS_CIDRS").ok();
    gcp_cidr_prerequisites_met_with_values(ssh_cidrs.as_deref(), qos_cidrs.as_deref())
}

fn gcp_cidr_prerequisites_met_with_values(
    ssh_cidrs: Option<&str>,
    qos_cidrs: Option<&str>,
) -> bool {
    ssh_cidrs.is_some_and(cidr_list_is_valid) && qos_cidrs.is_some_and(cidr_list_is_valid)
}

fn cidr_list_is_valid(raw: &str) -> bool {
    let mut saw_entry = false;
    for cidr in raw.split(',').map(str::trim) {
        if cidr.is_empty() {
            continue;
        }
        saw_entry = true;
        if !cidr_is_valid(cidr) {
            return false;
        }
    }
    saw_entry
}

fn cidr_is_valid(raw: &str) -> bool {
    let (ip_raw, prefix_raw) = match raw.split_once('/') {
        Some(parts) => parts,
        None => return false,
    };
    let ip: IpAddr = match ip_raw.trim().parse() {
        Ok(ip) => ip,
        Err(_) => return false,
    };
    let prefix: u8 = match prefix_raw.trim().parse() {
        Ok(prefix) => prefix,
        Err(_) => return false,
    };
    match ip {
        IpAddr::V4(_) => prefix <= 32,
        IpAddr::V6(_) => prefix <= 128,
    }
}

fn command_spec_is_executable(spec: &str) -> bool {
    let Some(executable) = spec.split_whitespace().next().filter(|v| !v.is_empty()) else {
        return false;
    };

    let executable_path = Path::new(executable);
    if executable_path.is_absolute() || executable.contains(std::path::MAIN_SEPARATOR) {
        return is_executable_file(executable_path);
    }

    std::env::var_os("PATH")
        .map(|path| {
            std::env::split_paths(&path).any(|dir| {
                let candidate = dir.join(executable);
                is_executable_file(&candidate)
            })
        })
        .unwrap_or(false)
}

fn is_executable_file(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::metadata(path)
            .map(|metadata| metadata.permissions().mode() & 0o111 != 0)
            .unwrap_or(false)
    }

    #[cfg(not(unix))]
    {
        true
    }
}

fn render_bootstrap_env(
    config: &CloudConfig,
    provider: CloudProvider,
    tee_required: bool,
    attestation_policy: &str,
    cryptographic_cmd: Option<&str>,
) -> String {
    let mut out = String::new();
    let _ = writeln!(&mut out, "# Blueprint remote deployment bootstrap");
    let _ = writeln!(
        &mut out,
        "{}={}",
        region_env_var(provider),
        region_for_provider(config, provider)
    );
    let _ = writeln!(
        &mut out,
        "BLUEPRINT_REMOTE_TEE_REQUIRED={}",
        if tee_required { "true" } else { "false" }
    );
    let _ = writeln!(
        &mut out,
        "BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY={}",
        attestation_policy
    );
    if tee_required {
        if let Some(backend) = default_tee_backend(provider) {
            let _ = writeln!(&mut out, "TEE_BACKEND={backend}");
        }
    }
    if provider == CloudProvider::GCP {
        let _ = writeln!(&mut out, "BLUEPRINT_ALLOWED_SSH_CIDRS=10.0.0.0/8");
        let _ = writeln!(&mut out, "BLUEPRINT_ALLOWED_QOS_CIDRS=10.0.0.0/8");
    }
    if attestation_policy.eq_ignore_ascii_case("cryptographic") {
        let value = cryptographic_cmd.unwrap_or("/usr/local/bin/verify-tee-attestation");
        let _ = writeln!(
            &mut out,
            "BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD={value}"
        );
    }
    out
}

fn region_env_var(provider: CloudProvider) -> &'static str {
    match provider {
        CloudProvider::AWS => "AWS_DEFAULT_REGION",
        CloudProvider::GCP => "GCP_DEFAULT_REGION",
        CloudProvider::Azure => "AZURE_DEFAULT_REGION",
        CloudProvider::DigitalOcean => "DO_REGION",
        CloudProvider::Vultr => "VULTR_DEFAULT_REGION",
        CloudProvider::Hetzner => "HETZNER_REGION",
        CloudProvider::RunPod => "RUNPOD_REGION",
        CloudProvider::LambdaLabs => "LAMBDA_LABS_REGION",
        CloudProvider::PrimeIntellect => "PRIME_INTELLECT_REGION",
        CloudProvider::VastAi => "VAST_AI_REGION",
        CloudProvider::Crusoe => "CRUSOE_REGION",
    }
}

fn region_for_provider(config: &CloudConfig, provider: CloudProvider) -> String {
    config
        .providers
        .get(&provider)
        .map(|s| s.region.clone())
        .unwrap_or_else(|| default_region(provider).to_string())
}

fn default_region(provider: CloudProvider) -> &'static str {
    match provider {
        CloudProvider::AWS => "us-east-1",
        CloudProvider::GCP => "us-central1",
        CloudProvider::Azure => "eastus",
        CloudProvider::DigitalOcean => "nyc3",
        CloudProvider::Vultr => "ewr",
        CloudProvider::Hetzner => "fsn1",
        CloudProvider::RunPod => "US",
        CloudProvider::LambdaLabs => "us-west-1",
        CloudProvider::PrimeIntellect => "us-east",
        CloudProvider::VastAi => "any",
        CloudProvider::Crusoe => "us-east1",
    }
}

fn default_tee_backend(provider: CloudProvider) -> Option<&'static str> {
    match provider {
        CloudProvider::AWS => Some("aws-nitro"),
        CloudProvider::GCP => Some("gcp-confidential"),
        CloudProvider::Azure => Some("azure-skr"),
        CloudProvider::DigitalOcean
        | CloudProvider::Vultr
        | CloudProvider::Hetzner
        | CloudProvider::RunPod
        | CloudProvider::LambdaLabs
        | CloudProvider::PrimeIntellect
        | CloudProvider::VastAi
        | CloudProvider::Crusoe => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::command::cloud::config::ProviderSettings;
    use std::collections::HashMap;

    #[test]
    fn selected_providers_prefers_default_provider() {
        let mut providers = HashMap::new();
        providers.insert(
            CloudProvider::AWS,
            ProviderSettings {
                region: "us-east-1".to_string(),
                project_id: None,
                configured: true,
            },
        );
        providers.insert(
            CloudProvider::GCP,
            ProviderSettings {
                region: "us-central1".to_string(),
                project_id: Some("proj".to_string()),
                configured: true,
            },
        );

        let cfg = CloudConfig {
            default_provider: Some(CloudProvider::GCP),
            providers,
        };
        let selected = selected_providers(&cfg, None);
        assert_eq!(selected, vec![CloudProvider::GCP]);
    }

    #[test]
    fn bootstrap_env_contains_expected_entries() {
        let mut providers = HashMap::new();
        providers.insert(
            CloudProvider::AWS,
            ProviderSettings {
                region: "us-west-2".to_string(),
                project_id: None,
                configured: true,
            },
        );

        let cfg = CloudConfig {
            default_provider: Some(CloudProvider::AWS),
            providers,
        };
        let output = render_bootstrap_env(&cfg, CloudProvider::AWS, true, "structural", None);
        assert!(output.contains("AWS_DEFAULT_REGION=us-west-2"));
        assert!(output.contains("BLUEPRINT_REMOTE_TEE_REQUIRED=true"));
        assert!(output.contains("TEE_BACKEND=aws-nitro"));
    }

    #[test]
    fn gcp_cidr_prerequisites_require_both_values() {
        assert!(gcp_cidr_prerequisites_met_with_values(
            Some("10.0.0.0/8"),
            Some("192.168.0.0/16")
        ));
        assert!(!gcp_cidr_prerequisites_met_with_values(
            Some("10.0.0.0/8"),
            None
        ));
        assert!(!gcp_cidr_prerequisites_met_with_values(
            None,
            Some("192.168.0.0/16")
        ));
        assert!(!gcp_cidr_prerequisites_met_with_values(
            Some(""),
            Some("192.168.0.0/16")
        ));
    }

    #[test]
    fn gcp_cidr_prerequisites_require_valid_cidrs() {
        assert!(gcp_cidr_prerequisites_met_with_values(
            Some("10.0.0.0/8,192.168.0.0/16"),
            Some("172.16.0.0/12")
        ));
        assert!(!gcp_cidr_prerequisites_met_with_values(
            Some("not-a-cidr"),
            Some("172.16.0.0/12")
        ));
        assert!(!gcp_cidr_prerequisites_met_with_values(
            Some("10.0.0.0/33"),
            Some("172.16.0.0/12")
        ));
        assert!(!gcp_cidr_prerequisites_met_with_values(
            Some("10.0.0.0/8"),
            Some("172.16.0.0")
        ));
    }

    #[test]
    fn command_spec_requires_executable() {
        assert!(command_spec_is_executable("/bin/sh"));
        assert!(!command_spec_is_executable("/definitely/missing/verifier"));
        assert!(!command_spec_is_executable(""));
    }

    #[cfg(unix)]
    #[test]
    fn command_spec_rejects_non_executable_files() {
        use std::os::unix::fs::PermissionsExt;

        let temp = tempfile::NamedTempFile::new().expect("create temp file");
        let path = temp.path();
        std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o644))
            .expect("set non-executable mode");
        assert!(!command_spec_is_executable(path.to_string_lossy().as_ref()));
    }
}
