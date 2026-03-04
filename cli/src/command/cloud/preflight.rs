//! Remote deployment preflight checks and bootstrap helpers.

use super::config::{CloudConfig, CloudProvider};
use color_eyre::Result;
use std::fmt::Write as _;
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
        return Ok(());
    }

    println!("Cloud preflight checks\n");

    let attestation_policy = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_POLICY")
        .unwrap_or_else(|_| "structural".to_string());
    let cryptographic = attestation_policy.eq_ignore_ascii_case("cryptographic");
    let cryptographic_cmd = std::env::var("BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD").ok();

    let mut any_fail = false;

    for selected in &providers {
        let configured = config.providers.contains_key(selected);
        let credentials_ok = credentials_present(*selected);
        let supports_tee = provider_supports_tee(*selected);
        let tee_ok = !tee_required || supports_tee;

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
        if tee_required && *selected == CloudProvider::GCP {
            if std::env::var("BLUEPRINT_ALLOWED_SSH_CIDRS").is_err()
                || std::env::var("BLUEPRINT_ALLOWED_QOS_CIDRS").is_err()
            {
                notes.push(
                    "set BLUEPRINT_ALLOWED_SSH_CIDRS and BLUEPRINT_ALLOWED_QOS_CIDRS".to_string(),
                );
            }
        }

        let pass = configured && credentials_ok && tee_ok;
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
        if cryptographic {
            let ok = cryptographic_cmd
                .as_deref()
                .map(|v| !v.trim().is_empty())
                .unwrap_or(false);
            if ok {
                println!(
                    "- BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD: configured ({})",
                    cryptographic_cmd.as_deref().unwrap_or_default()
                );
            } else {
                any_fail = true;
                println!(
                    "- BLUEPRINT_REMOTE_TEE_ATTESTATION_VERIFY_CMD: MISSING (required for cryptographic policy)"
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
        if provider == CloudProvider::GCP {
            let _ = writeln!(&mut out, "BLUEPRINT_ALLOWED_SSH_CIDRS=10.0.0.0/8");
            let _ = writeln!(&mut out, "BLUEPRINT_ALLOWED_QOS_CIDRS=10.0.0.0/8");
        }
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
    }
}

fn default_tee_backend(provider: CloudProvider) -> Option<&'static str> {
    match provider {
        CloudProvider::AWS => Some("aws-nitro"),
        CloudProvider::GCP => Some("gcp-confidential"),
        CloudProvider::Azure => Some("azure-skr"),
        CloudProvider::DigitalOcean | CloudProvider::Vultr => None,
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
}
