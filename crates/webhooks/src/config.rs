//! Configuration types for the webhook gateway.

use crate::error::WebhookError;
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::path::Path;

/// Top-level webhook gateway configuration.
///
/// ```toml
/// bind_address = "0.0.0.0:9090"
///
/// [[endpoints]]
/// path = "/hooks/tradingview"
/// job_id = 30
/// auth = "hmac-sha256"
/// secret = "my-secret-key"
///
/// [[endpoints]]
/// path = "/hooks/price-alert"
/// job_id = 7
/// auth = "bearer"
/// secret = "my-api-token"
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookConfig {
    /// Socket address to bind the HTTP server to.
    #[serde(default = "default_bind_address")]
    pub bind_address: SocketAddr,

    /// Configured webhook endpoints.
    pub endpoints: Vec<WebhookEndpoint>,

    /// Service ID (set at runtime, not from TOML).
    #[serde(default, skip_deserializing)]
    pub service_id: u64,
}

/// A single webhook endpoint configuration.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookEndpoint {
    /// URL path for this webhook (e.g. `/hooks/tradingview`).
    pub path: String,

    /// Job ID to trigger when this webhook fires.
    pub job_id: u64,

    /// Authentication method: `"none"`, `"bearer"`, `"hmac-sha256"`, or `"api-key"`.
    #[serde(default = "default_auth")]
    pub auth: String,

    /// Secret for authentication (bearer token, HMAC key, or API key value).
    /// Can also reference an env var with `env:VAR_NAME`.
    #[serde(default)]
    pub secret: Option<String>,

    /// Header name for API key auth (default: `X-API-Key`).
    #[serde(default)]
    pub api_key_header: Option<String>,

    /// Optional human-readable name for logging.
    #[serde(default)]
    pub name: Option<String>,
}

impl WebhookEndpoint {
    /// Resolve the secret, expanding `env:VAR_NAME` references.
    pub fn resolve_secret(&self) -> Option<String> {
        self.secret.as_ref().and_then(|s| {
            if let Some(var_name) = s.strip_prefix("env:") {
                std::env::var(var_name).ok()
            } else {
                Some(s.clone())
            }
        })
    }

    /// Display name for logging.
    pub fn display_name(&self) -> &str {
        self.name.as_deref().unwrap_or(&self.path)
    }
}

impl WebhookConfig {
    /// Load configuration from a TOML file.
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, WebhookError> {
        let content = std::fs::read_to_string(path.as_ref())
            .map_err(|e| WebhookError::Config(format!("failed to read config: {e}")))?;
        let config: Self = toml::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }

    /// Validate the configuration.
    pub fn validate(&self) -> Result<(), WebhookError> {
        if self.endpoints.is_empty() {
            return Err(WebhookError::Config(
                "at least one webhook endpoint must be configured".into(),
            ));
        }

        for ep in &self.endpoints {
            if !ep.path.starts_with('/') {
                return Err(WebhookError::Config(format!(
                    "endpoint path must start with '/': {}",
                    ep.path,
                )));
            }

            match ep.auth.as_str() {
                "none" => {}
                "bearer" | "hmac-sha256" | "api-key" => {
                    if ep.secret.is_none() {
                        return Err(WebhookError::Config(format!(
                            "endpoint {} with auth={} requires a secret",
                            ep.path, ep.auth,
                        )));
                    }
                }
                other => {
                    return Err(WebhookError::Config(format!(
                        "endpoint {}: unknown auth method '{}' (expected: none, bearer, hmac-sha256, api-key)",
                        ep.path, other,
                    )));
                }
            }
        }

        Ok(())
    }
}

fn default_bind_address() -> SocketAddr {
    SocketAddr::from(([0, 0, 0, 0], 9090))
}

fn default_auth() -> String {
    "none".into()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_endpoint(auth: &str, secret: Option<&str>) -> WebhookEndpoint {
        WebhookEndpoint {
            path: "/hooks/test".into(),
            job_id: 1,
            auth: auth.into(),
            secret: secret.map(|s| s.into()),
            api_key_header: None,
            name: None,
        }
    }

    #[test]
    fn test_valid_config() {
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![test_endpoint("bearer", Some("secret123"))],
            service_id: 0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_empty_endpoints_rejected() {
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![],
            service_id: 0,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_no_auth_needs_no_secret() {
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![test_endpoint("none", None)],
            service_id: 0,
        };
        assert!(config.validate().is_ok());
    }

    #[test]
    fn test_bearer_without_secret_rejected() {
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![test_endpoint("bearer", None)],
            service_id: 0,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_hmac_without_secret_rejected() {
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![test_endpoint("hmac-sha256", None)],
            service_id: 0,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_unknown_auth_rejected() {
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![test_endpoint("oauth2", Some("key"))],
            service_id: 0,
        };
        let err = config.validate().unwrap_err();
        assert!(err.to_string().contains("unknown auth method"), "{err}");
    }

    #[test]
    fn test_path_must_start_with_slash() {
        let mut ep = test_endpoint("none", None);
        ep.path = "hooks/test".into();
        let config = WebhookConfig {
            bind_address: default_bind_address(),
            endpoints: vec![ep],
            service_id: 0,
        };
        assert!(config.validate().is_err());
    }

    #[test]
    fn test_env_secret_resolution() {
        let ep = WebhookEndpoint {
            path: "/hooks/test".into(),
            job_id: 1,
            auth: "bearer".into(),
            secret: Some("env:WEBHOOK_TEST_SECRET_XYZ".into()),
            api_key_header: None,
            name: None,
        };
        // Env var not set → None
        assert!(ep.resolve_secret().is_none());

        // SAFETY: test-only, single-threaded
        unsafe { std::env::set_var("WEBHOOK_TEST_SECRET_XYZ", "resolved_value") };
        assert_eq!(ep.resolve_secret().unwrap(), "resolved_value");
        unsafe { std::env::remove_var("WEBHOOK_TEST_SECRET_XYZ") };
    }

    #[test]
    fn test_literal_secret_resolution() {
        let ep = test_endpoint("bearer", Some("literal-secret"));
        assert_eq!(ep.resolve_secret().unwrap(), "literal-secret");
    }

    #[test]
    fn test_toml_round_trip() {
        let toml_str = r#"
bind_address = "127.0.0.1:9091"

[[endpoints]]
path = "/hooks/alert"
job_id = 5
auth = "bearer"
secret = "my-token"
name = "Price Alert"
"#;
        let config: WebhookConfig = toml::from_str(toml_str).unwrap();
        assert_eq!(config.endpoints.len(), 1);
        assert_eq!(config.endpoints[0].job_id, 5);
        assert_eq!(config.endpoints[0].display_name(), "Price Alert");
        assert!(config.validate().is_ok());
    }
}
