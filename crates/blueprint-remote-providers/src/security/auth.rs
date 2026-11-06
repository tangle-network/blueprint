//! Shared cloud authentication helpers for remote providers.

use crate::core::error::{Error, Result};
use crate::create_metadata_client;
use std::sync::Arc;
use tokio::sync::OnceCell;
use url::Url;

const AZURE_METADATA_URL: &str = "http://169.254.169.254/metadata/identity/oauth2/token";

/// Fetch a GCP access token. `GCP_ACCESS_TOKEN` is an explicit override; otherwise
/// `gcp_auth` tries `GOOGLE_APPLICATION_CREDENTIALS`, application-default credentials,
/// the GCE metadata server and the `gcloud` CLI, in that order.
pub async fn gcp_access_token() -> Result<String> {
    if let Ok(token) = std::env::var("GCP_ACCESS_TOKEN") {
        if token.trim().is_empty() {
            return Err(Error::ConfigurationError(
                "GCP_ACCESS_TOKEN is empty".into(),
            ));
        }
        return Ok(token);
    }

    // Discovery probes files, the metadata server and `gcloud`, so run it once per
    // process; the provider caches and refreshes its token.
    static PROVIDER: OnceCell<Arc<dyn gcp_auth::TokenProvider>> = OnceCell::const_new();
    let provider = PROVIDER
        .get_or_try_init(gcp_auth::provider)
        .await
        .map_err(gcp_login_error)?;
    let token = provider
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await
        .map_err(gcp_login_error)?;
    Ok(token.as_str().to_owned())
}

fn gcp_login_error(err: gcp_auth::Error) -> Error {
    let detail = match &err {
        gcp_auth::Error::NoAuthMethod(gcloud, metadata, adc) => format!(
            "application-default credentials: {}; GCE metadata: {}; gcloud: {}",
            error_chain(adc),
            error_chain(metadata),
            error_chain(gcloud)
        ),
        other => error_chain(other),
    };
    Error::ConfigurationError(format!(
        "No usable GCP credentials ({detail}). Log in with one of: GCP_ACCESS_TOKEN, \
         GOOGLE_APPLICATION_CREDENTIALS=<service-account.json>, \
         `gcloud auth application-default login` (or delete a stale \
         application_default_credentials.json), `gcloud auth login`, or a GCE service account"
    ))
}

fn error_chain(err: &(dyn std::error::Error + 'static)) -> String {
    let mut text = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        let cause_text = cause.to_string();
        if !text.contains(&cause_text) {
            text = format!("{text}: {cause_text}");
        }
        source = cause.source();
    }
    text
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
#[allow(clippy::await_holding_lock)]
mod tests {
    use super::{azure_access_token, gcp_access_token, validate_azure_metadata_url};
    use std::env;
    use std::sync::{Mutex, OnceLock};

    fn env_lock() -> std::sync::MutexGuard<'static, ()> {
        static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
        LOCK.get_or_init(|| Mutex::new(())).lock().unwrap()
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
