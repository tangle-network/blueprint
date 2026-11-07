//! Shared cloud authentication helpers for remote providers.

use crate::core::error::{Error, Result};
use crate::create_metadata_client;
use gcp_auth::TokenProvider;
use std::sync::Arc;
use tokio::sync::OnceCell;

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

    // A key file, application-default credentials and the metadata server carry their token's real
    // expiry and refresh it, so the first of them found is kept for the process. gcp_auth gives a
    // `gcloud` token a fixed hour instead, which would keep a revoked or expired token in use for up
    // to 3600 s; without one of the others, each call asks `gcloud`, whose own cache decides.
    static SELF_REFRESHING: OnceCell<Option<Arc<dyn TokenProvider>>> = OnceCell::const_new();
    let provider = match SELF_REFRESHING.get_or_init(self_refreshing_provider).await {
        Some(provider) => provider.clone(),
        None => gcp_auth::provider().await.map_err(gcp_login_error)?,
    };
    let token = provider
        .token(&["https://www.googleapis.com/auth/cloud-platform"])
        .await
        .map_err(gcp_login_error)?;
    Ok(token.as_str().to_owned())
}

async fn self_refreshing_provider() -> Option<Arc<dyn TokenProvider>> {
    if let Ok(Some(key)) = gcp_auth::CustomServiceAccount::from_env() {
        return Some(Arc::new(key));
    }
    if let Ok(user) = gcp_auth::ConfigDefaultCredentials::new().await {
        return Some(Arc::new(user));
    }
    let metadata = gcp_auth::MetadataServiceAccount::new().await.ok()?;
    Some(Arc::new(metadata))
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

#[cfg(test)]
mod tests {
    use super::azure_access_token;
    use std::env;

    /// The only test that sets `AZURE_ACCESS_TOKEN`, so it needs no lock on the environment.
    #[tokio::test]
    async fn azure_env_token_is_used_unless_empty() {
        unsafe {
            env::set_var("AZURE_ACCESS_TOKEN", "token");
        }
        assert_eq!(azure_access_token().await.unwrap(), "token");
        unsafe {
            env::set_var("AZURE_ACCESS_TOKEN", "");
        }
        assert!(azure_access_token().await.is_err());
        unsafe {
            env::remove_var("AZURE_ACCESS_TOKEN");
        }
    }
}
