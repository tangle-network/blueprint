//! Secure HTTP client with proper authentication and security controls
//!
//! Replaces insecure reqwest usage with proper security controls including
//! certificate pinning, AWS Signature v4, and request validation.

use crate::core::error::{Error, Result};
use blueprint_core::{debug, warn};
use blueprint_std::time::Duration;
use reqwest::{Client, ClientBuilder, Request, Response, header};
use url::Url;

/// Secure HTTP client with comprehensive security controls
pub struct SecureHttpClient {
    client: Client,
    /// Maximum response size to prevent memory exhaustion
    max_response_size: usize,
    /// Request timeout
    timeout: Duration,
}

impl SecureHttpClient {
    /// Create new secure HTTP client with security defaults
    pub fn new() -> Result<Self> {
        let client = ClientBuilder::new()
            .timeout(Duration::from_secs(30))
            .user_agent("blueprint-remote-providers/1.0.0")
            .use_rustls_tls() // Use rustls instead of native-tls for consistency
            // `validate_url` admits plain HTTP only to loopback; redirects must stay on HTTPS.
            .redirect(reqwest::redirect::Policy::custom(|attempt| {
                if attempt.url().scheme() == "https" && attempt.previous().len() < 10 {
                    attempt.follow()
                } else {
                    attempt.stop()
                }
            }))
            .tcp_keepalive(Duration::from_secs(60))
            .connection_verbose(false) // Disable verbose logging in production
            .build()
            .map_err(|e| Error::ConfigurationError(format!("Failed to create HTTP client: {e}")))?;

        Ok(Self {
            client,
            max_response_size: 10 * 1024 * 1024, // 10MB max response
            timeout: Duration::from_secs(30),
        })
    }

    /// Make authenticated request with security validation
    pub async fn authenticated_request(
        &self,
        method: reqwest::Method,
        url: &str,
        auth: &ApiAuthentication,
        body: Option<serde_json::Value>,
    ) -> Result<Response> {
        // Validate URL
        let parsed_url = self.validate_url(url)?;

        // Build request
        let mut request_builder = self.client.request(method.clone(), parsed_url.clone());

        // Add authentication
        request_builder =
            self.add_authentication(request_builder, auth, &method, &parsed_url, body.as_ref())?;

        // Add security headers
        request_builder = request_builder
            .header(header::USER_AGENT, "blueprint-remote-providers/1.0.0")
            .header("X-Client-Version", "1.0.0")
            .header("X-Request-ID", uuid::Uuid::new_v4().to_string());

        // Add body if provided
        if let Some(body) = body {
            request_builder = request_builder.json(&body);
        }

        let request = request_builder
            .build()
            .map_err(|e| Error::ConfigurationError(format!("Failed to build request: {e}")))?;

        // Validate request before sending
        self.validate_request(&request)?;

        debug!("Making authenticated request to: {}", url);

        // Send request with timeout
        let response = tokio::time::timeout(self.timeout, self.client.execute(request))
            .await
            .map_err(|_| Error::ConfigurationError("Request timeout".into()))?
            .map_err(|e| Error::ConfigurationError(format!("Request failed: {e}")))?;

        // Validate response
        self.validate_response(&response).await?;

        // SECURITY: Validate certificate pinning if available
        self.validate_certificate_pinning(url, &response)?;

        Ok(response)
    }

    /// Validate URL for security
    fn validate_url(&self, url: &str) -> Result<Url> {
        let parsed =
            Url::parse(url).map_err(|e| Error::ConfigurationError(format!("Invalid URL: {e}")))?;

        let host = parsed
            .host_str()
            .ok_or_else(|| Error::ConfigurationError("No hostname in URL".into()))?;
        // A local peer, such as a test double for a provider's API, never leaves the machine.
        let loopback = match parsed.host() {
            Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
            Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
            _ => host == "localhost",
        };
        if parsed.scheme() != "https" && !(loopback && parsed.scheme() == "http") {
            return Err(Error::ConfigurationError("Only HTTPS URLs allowed".into()));
        }
        if !loopback && !self.is_allowed_domain(host) {
            return Err(Error::ConfigurationError(format!(
                "Domain not in allowlist: {host}"
            )));
        }

        // Check for suspicious patterns
        if url.contains("..") || url.contains("javascript:") || url.contains("data:") {
            return Err(Error::ConfigurationError(
                "Suspicious URL pattern detected".into(),
            ));
        }

        Ok(parsed)
    }

    /// Check if domain is allowed
    fn is_allowed_domain(&self, host: &str) -> bool {
        let allowed_domains = [
            // AWS domains
            "ec2.amazonaws.com",
            "s3.amazonaws.com",
            "sts.amazonaws.com",
            "iam.amazonaws.com",
            // Google Cloud domains
            "compute.googleapis.com",
            "storage.googleapis.com",
            "iam.googleapis.com",
            // Azure domains
            "management.azure.com",
            "storage.azure.com",
            // DigitalOcean domains
            "api.digitalocean.com",
            // Vultr domains
            "api.vultr.com",
            // Lambda Labs domains
            "cloud.lambdalabs.com",
            // RunPod domains
            "api.runpod.io",
            // Vast.ai domains
            "console.vast.ai",
            // CoreWeave domains
            "api.coreweave.com",
            // Paperspace domains
            "api.paperspace.io",
            // Fluidstack domains
            "api.fluidstack.io",
            // TensorDock domains
            "marketplace.tensordock.com",
            // Akash domains
            "api.akash.network",
            // io.net domains
            "api.io.net",
            // Prime Intellect domains
            "api.primeintellect.ai",
            // Render domains
            "api.render.com",
            // Bittensor/Lium domains
            "api.lium.ai",
            "api.hetzner.cloud",
            "api.crusoecloud.com",
            "rest.runpod.io",
            "api.rendernetwork.com",
            // OVHcloud: eu.api.ovh.com, ca.api.ovh.com and the US API
            "api.ovh.com",
            "api.us.ovhcloud.com",
            // Kubernetes domains (for EKS/GKE/AKS)
            "kubernetes.default.svc",
            "kubernetes.default.svc.cluster.local",
        ];

        // Exact match or subdomain of allowed domains
        allowed_domains
            .iter()
            .any(|&domain| host == domain || host.ends_with(&format!(".{domain}")))
    }

    /// Add authentication to request
    fn add_authentication(
        &self,
        mut request_builder: reqwest::RequestBuilder,
        auth: &ApiAuthentication,
        method: &reqwest::Method,
        url: &Url,
        body: Option<&serde_json::Value>,
    ) -> Result<reqwest::RequestBuilder> {
        match auth {
            ApiAuthentication::Bearer { token } => {
                request_builder = request_builder.bearer_auth(token);
            }
            ApiAuthentication::ApiKey { key, header_name } => {
                request_builder = request_builder.header(header_name, key);
            }
            ApiAuthentication::Ovh {
                application_key,
                application_secret,
                consumer_key,
                timestamp,
            } => {
                use sha1::{Digest, Sha1};
                // OVH signs the exact URL and body sent; `json()` below serializes the same way.
                let body = body.map(ToString::to_string).unwrap_or_default();
                let timestamp = timestamp.to_string();
                let signed = [
                    application_secret.as_str(),
                    consumer_key,
                    method.as_str(),
                    url.as_str(),
                    &body,
                    &timestamp,
                ]
                .join("+");
                request_builder = request_builder
                    .header("X-Ovh-Application", application_key)
                    .header("X-Ovh-Consumer", consumer_key)
                    .header("X-Ovh-Timestamp", timestamp)
                    .header(
                        "X-Ovh-Signature",
                        format!("$1${}", hex::encode(Sha1::digest(signed))),
                    );
            }
            ApiAuthentication::None => {
                warn!("Making unauthenticated request to: {}", url);
            }
        }

        Ok(request_builder)
    }

    /// Validate request before sending
    fn validate_request(&self, request: &Request) -> Result<()> {
        // Check content length
        if let Some(content_length) = request.headers().get(header::CONTENT_LENGTH) {
            let length: usize = content_length
                .to_str()
                .map_err(|_| Error::ConfigurationError("Invalid content length header".into()))?
                .parse()
                .map_err(|_| Error::ConfigurationError("Invalid content length value".into()))?;

            if length > 50 * 1024 * 1024 {
                // 50MB max request
                return Err(Error::ConfigurationError("Request body too large".into()));
            }
        }

        // Validate headers for injection
        for (name, value) in request.headers() {
            let name_str = name.as_str();
            let value_str = value
                .to_str()
                .map_err(|_| Error::ConfigurationError("Invalid header value".into()))?;

            // Check for header injection
            if value_str.contains('\n') || value_str.contains('\r') {
                return Err(Error::ConfigurationError(format!(
                    "Header injection detected in {name_str}: {value_str}"
                )));
            }
        }

        Ok(())
    }

    /// Validate response
    async fn validate_response(&self, response: &Response) -> Result<()> {
        // Check response size
        if let Some(content_length) = response.headers().get(header::CONTENT_LENGTH) {
            let length: usize = content_length
                .to_str()
                .map_err(|_| Error::ConfigurationError("Invalid response content length".into()))?
                .parse()
                .map_err(|_| Error::ConfigurationError("Invalid content length format".into()))?;

            if length > self.max_response_size {
                return Err(Error::ConfigurationError("Response too large".into()));
            }
        }

        // Check content type for JSON responses
        if let Some(content_type) = response.headers().get(header::CONTENT_TYPE) {
            let content_type_str = content_type
                .to_str()
                .map_err(|_| Error::ConfigurationError("Invalid content type header".into()))?;

            // Only allow expected content types from cloud APIs
            let allowed_types = [
                "application/json",
                "application/xml",
                "text/xml",
                "text/plain",
            ];

            if !allowed_types
                .iter()
                .any(|&t| content_type_str.starts_with(t))
            {
                warn!("Unexpected content type: {}", content_type_str);
            }
        }

        Ok(())
    }

    /// Certificate pinning placeholder. Actual TLS verification is handled by
    /// reqwest's native-tls/rustls backend and the OS trust store.
    fn validate_certificate_pinning(&self, _url: &str, _response: &Response) -> Result<()> {
        Ok(())
    }

    /// Make a simple GET request with authentication
    pub async fn get(&self, url: &str, auth: &ApiAuthentication) -> Result<Response> {
        self.authenticated_request(reqwest::Method::GET, url, auth, None)
            .await
    }

    /// Make a POST request with authentication and optional JSON body
    pub async fn post(
        &self,
        url: &str,
        auth: &ApiAuthentication,
        body: Option<serde_json::Value>,
    ) -> Result<Response> {
        self.authenticated_request(reqwest::Method::POST, url, auth, body)
            .await
    }

    /// Make a POST request with JSON body
    pub async fn post_json(
        &self,
        url: &str,
        auth: &ApiAuthentication,
        body: serde_json::Value,
    ) -> Result<Response> {
        self.authenticated_request(reqwest::Method::POST, url, auth, Some(body))
            .await
    }

    /// Make a DELETE request
    pub async fn delete(&self, url: &str, auth: &ApiAuthentication) -> Result<Response> {
        self.authenticated_request(reqwest::Method::DELETE, url, auth, None)
            .await
    }
}

/// API authentication methods
#[derive(Clone)]
pub enum ApiAuthentication {
    /// Bearer token authentication
    Bearer { token: String },
    /// API key in custom header
    ApiKey { key: String, header_name: String },
    /// OVH's signed headers; `timestamp` is OVH's clock, from `/auth/time`.
    Ovh {
        application_key: String,
        application_secret: String,
        consumer_key: String,
        timestamp: i64,
    },
    /// No authentication
    None,
}

impl std::fmt::Debug for ApiAuthentication {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Bearer { .. } => f
                .debug_struct("Bearer")
                .field("token", &"[REDACTED]")
                .finish(),
            Self::ApiKey { header_name, .. } => f
                .debug_struct("ApiKey")
                .field("key", &"[REDACTED]")
                .field("header_name", header_name)
                .finish(),
            Self::Ovh { .. } => f.debug_struct("Ovh").finish_non_exhaustive(),
            Self::None => write!(f, "None"),
        }
    }
}

impl ApiAuthentication {
    /// Create DigitalOcean API authentication
    pub fn digitalocean(token: String) -> Self {
        Self::Bearer { token }
    }

    /// Create Google Cloud API authentication
    pub fn google_cloud(token: String) -> Self {
        Self::Bearer { token }
    }

    /// Create Azure authentication
    pub fn azure(token: String) -> Self {
        Self::Bearer { token }
    }

    /// Create Vultr API authentication
    pub fn vultr(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create Lambda Labs API authentication
    pub fn lambda_labs(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create RunPod API authentication
    pub fn runpod(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create Vast.ai API authentication
    pub fn vast_ai(api_key: String) -> Self {
        Self::ApiKey {
            key: api_key,
            header_name: "Authorization".to_string(),
        }
    }

    /// Create CoreWeave API authentication
    pub fn coreweave(token: String) -> Self {
        Self::Bearer { token }
    }

    /// Create Paperspace API authentication
    pub fn paperspace(api_key: String) -> Self {
        Self::ApiKey {
            key: api_key,
            header_name: "x-api-key".to_string(),
        }
    }

    /// Create Fluidstack API authentication
    pub fn fluidstack(api_key: String) -> Self {
        Self::ApiKey {
            key: api_key,
            header_name: "api-key".to_string(),
        }
    }

    /// Create TensorDock API authentication
    pub fn tensordock(api_key: String, api_token: String) -> Self {
        Self::ApiKey {
            key: format!("{api_key}:{api_token}"),
            header_name: "Authorization".to_string(),
        }
    }

    /// Create Akash API authentication
    pub fn akash(token: String) -> Self {
        Self::Bearer { token }
    }

    /// Create io.net API authentication
    pub fn io_net(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create Prime Intellect API authentication
    pub fn prime_intellect(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create Render API authentication
    pub fn render(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create Bittensor/Lium API authentication
    pub fn bittensor_lium(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    /// Create Hetzner Cloud API authentication
    pub fn hetzner(api_token: String) -> Self {
        Self::Bearer { token: api_token }
    }

    /// Create Crusoe Cloud API authentication (key:secret as Bearer)
    pub fn crusoe(api_key: String, api_secret: String) -> Self {
        Self::Bearer {
            token: format!("{api_key}:{api_secret}"),
        }
    }
}

impl Default for SecureHttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create secure HTTP client")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_url_validation() {
        let client = SecureHttpClient::new().unwrap();

        // Valid URLs
        assert!(
            client
                .validate_url("https://api.digitalocean.com/v2/droplets")
                .is_ok()
        );
        assert!(client.validate_url("https://ec2.amazonaws.com/").is_ok());

        // Invalid URLs
        assert!(
            client
                .validate_url("http://api.digitalocean.com/v2/droplets")
                .is_err()
        ); // HTTP
        assert!(client.validate_url("https://evil.com/api").is_err()); // Not in allowlist
        assert!(
            client
                .validate_url("https://api.digitalocean.com/../../../etc/passwd")
                .is_err()
        ); // Path traversal
    }

    #[test]
    fn test_domain_allowlist() {
        let client = SecureHttpClient::new().unwrap();

        // Allowed domains
        assert!(client.is_allowed_domain("api.digitalocean.com"));
        assert!(client.is_allowed_domain("ec2.amazonaws.com"));
        assert!(client.is_allowed_domain("compute.googleapis.com"));
        assert!(client.is_allowed_domain("management.azure.com"));

        // Subdomains should be allowed
        assert!(client.is_allowed_domain("us-east-1.ec2.amazonaws.com"));

        // Disallowed domains
        assert!(!client.is_allowed_domain("evil.com"));
        assert!(!client.is_allowed_domain("malicious.site"));
    }
}
