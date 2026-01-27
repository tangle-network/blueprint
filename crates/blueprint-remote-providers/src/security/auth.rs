//! Shared cloud authentication helpers for remote providers.

use crate::core::error::{Error, Result};
use crate::create_metadata_client;
use url::Url;

const GCP_METADATA_URL: &str =
    "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token";
const AZURE_METADATA_URL: &str = "http://169.254.169.254/metadata/identity/oauth2/token";

/// Fetch a GCP access token from env or metadata service.
pub async fn gcp_access_token() -> Result<String> {
    if let Ok(token) = std::env::var("GCP_ACCESS_TOKEN") {
        if token.trim().is_empty() {
            return Err(Error::ConfigurationError(
                "GCP_ACCESS_TOKEN is empty".into(),
            ));
        }
        return Ok(token);
    }

    validate_gcp_metadata_url(GCP_METADATA_URL)?;
    let client = create_metadata_client(2)?;

    let response = client
        .get(GCP_METADATA_URL)
        .header("Metadata-Flavor", "Google")
        .send()
        .await;

    if let Ok(resp) = response {
        if resp.status().is_success() {
            let json: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to parse token: {e}")))?;
            if let Some(token) = json["access_token"].as_str() {
                return Ok(token.to_string());
            }
        }
    }

    Err(Error::ConfigurationError(
        "No GCP credentials found. Set GCP_ACCESS_TOKEN or use a service account".into(),
    ))
}

/// Fetch an Azure management token from env, metadata service, or Azure CLI.
pub async fn azure_access_token() -> Result<String> {
    if let Ok(token) = std::env::var("AZURE_ACCESS_TOKEN") {
        if token.trim().is_empty() {
            return Err(Error::ConfigurationError(
                "AZURE_ACCESS_TOKEN is empty".into(),
            ));
        }
        return Ok(token);
    }

    validate_azure_metadata_url(AZURE_METADATA_URL)?;
    let params = [
        ("api-version", "2018-02-01"),
        ("resource", "https://management.azure.com/"),
    ];

    let client = create_metadata_client(2)?;

    let response = client
        .get(AZURE_METADATA_URL)
        .header("Metadata", "true")
        .query(&params)
        .send()
        .await;

    if let Ok(resp) = response {
        if resp.status().is_success() {
            let json: serde_json::Value = resp
                .json()
                .await
                .map_err(|e| Error::ConfigurationError(format!("Failed to parse token: {e}")))?;
            if let Some(token) = json["access_token"].as_str() {
                return Ok(token.to_string());
            }
        }
    }

    // Fall back to Azure CLI.
    let output = std::process::Command::new("az")
        .args([
            "account",
            "get-access-token",
            "--query",
            "accessToken",
            "-o",
            "tsv",
        ])
        .output()
        .map_err(|e| {
            Error::ConfigurationError(format!("Failed to get Azure token via CLI: {e}"))
        })?;

    if !output.status.success() {
        return Err(Error::ConfigurationError(
            "Failed to get Azure access token".into(),
        ));
    }

    let token = String::from_utf8(output.stdout)
        .map_err(|e| Error::ConfigurationError(format!("Invalid token format: {e}")))?;
    let token = token.trim();
    if token.is_empty() {
        return Err(Error::ConfigurationError("Azure access token empty".into()));
    }

    Ok(token.to_string())
}

fn validate_gcp_metadata_url(url: &str) -> Result<()> {
    let parsed =
        Url::parse(url).map_err(|e| Error::ConfigurationError(format!("Invalid URL: {e}")))?;
    if parsed.scheme() != "http" {
        return Err(Error::ConfigurationError(
            "GCP metadata URL must use HTTP".into(),
        ));
    }
    let host = parsed
        .host_str()
        .ok_or_else(|| Error::ConfigurationError("Missing host in URL".into()))?;
    if host != "metadata.google.internal" {
        return Err(Error::ConfigurationError(format!(
            "Unexpected GCP metadata host: {host}"
        )));
    }
    Ok(())
}

fn validate_azure_metadata_url(url: &str) -> Result<()> {
    let parsed =
        Url::parse(url).map_err(|e| Error::ConfigurationError(format!("Invalid URL: {e}")))?;
    if parsed.scheme() != "http" {
        return Err(Error::ConfigurationError(
            "Azure metadata URL must use HTTP".into(),
        ));
    }
    let host = parsed
        .host_str()
        .ok_or_else(|| Error::ConfigurationError("Missing host in URL".into()))?;
    if host != "169.254.169.254" {
        return Err(Error::ConfigurationError(format!(
            "Unexpected Azure metadata host: {host}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{
        azure_access_token, gcp_access_token, validate_azure_metadata_url,
        validate_gcp_metadata_url,
    };
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
    }

    #[test]
    fn gcp_metadata_url_validation() {
        assert!(validate_gcp_metadata_url(
            "http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token"
        )
        .is_ok());
        assert!(validate_gcp_metadata_url("https://metadata.google.internal/").is_err());
        assert!(validate_gcp_metadata_url("http://example.com").is_err());
    }

    #[test]
    fn azure_metadata_url_validation() {
        assert!(
            validate_azure_metadata_url("http://169.254.169.254/metadata/identity/oauth2/token")
                .is_ok()
        );
        assert!(validate_azure_metadata_url("https://169.254.169.254/").is_err());
        assert!(validate_azure_metadata_url("http://example.com").is_err());
    }

    #[tokio::test]
    async fn gcp_env_token_is_used() {
        let _guard = env_lock();
        unsafe {
            env::set_var("GCP_ACCESS_TOKEN", "token");
        }
        assert_eq!(gcp_access_token().await.unwrap(), "token");
        unsafe {
            env::remove_var("GCP_ACCESS_TOKEN");
        }
    }

    #[tokio::test]
    async fn azure_env_token_is_used() {
        let _guard = env_lock();
        unsafe {
            env::set_var("AZURE_ACCESS_TOKEN", "token");
        }
        assert_eq!(azure_access_token().await.unwrap(), "token");
        unsafe {
            env::remove_var("AZURE_ACCESS_TOKEN");
        }
    }

    #[tokio::test]
    async fn empty_env_tokens_fail() {
        let _guard = env_lock();
        unsafe {
            env::set_var("GCP_ACCESS_TOKEN", "");
        }
        assert!(gcp_access_token().await.is_err());
        unsafe {
            env::set_var("AZURE_ACCESS_TOKEN", "");
        }
        assert!(azure_access_token().await.is_err());
        unsafe {
            env::remove_var("GCP_ACCESS_TOKEN");
            env::remove_var("AZURE_ACCESS_TOKEN");
        }
    }
}
