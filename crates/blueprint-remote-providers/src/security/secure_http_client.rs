//! Provider HTTP with origin-bound credentials, bounded bodies and no redirects.

use crate::managed::ProviderErrorCode;
use crate::{Error, Result};
use reqwest::{Client, Method, Response, header};
use std::time::Duration;
use url::{Host, Url};

pub struct SecureHttpClient {
    client: Client,
    origin: Option<String>,
}

impl SecureHttpClient {
    pub fn new() -> Result<Self> {
        Self::build(None)
    }

    pub fn with_endpoint(endpoint: &str) -> Result<Self> {
        let url = Url::parse(endpoint)
            .map_err(|_| Error::ConfigurationError("Invalid provider endpoint".into()))?;
        let loopback = match url.host() {
            Some(Host::Ipv4(ip)) => ip.is_loopback(),
            Some(Host::Ipv6(ip)) => ip.is_loopback(),
            Some(Host::Domain(host)) => host.eq_ignore_ascii_case("localhost"),
            None => false,
        };
        if !url.username().is_empty()
            || url.password().is_some()
            || url.query().is_some()
            || url.fragment().is_some()
            || !(url.scheme() == "https" || (url.scheme() == "http" && loopback))
        {
            return Err(Error::ConfigurationError(
                "Provider endpoint requires HTTPS or loopback HTTP".into(),
            ));
        }
        Self::build(Some(url.origin().ascii_serialization()))
    }

    fn build(origin: Option<String>) -> Result<Self> {
        let client = Client::builder()
            .https_only(origin.is_none())
            .use_rustls_tls()
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::none())
            .user_agent(concat!(
                "blueprint-remote-providers/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()
            .map_err(|_| Error::HttpError("Provider client initialization failed".into()))?;
        Ok(Self { client, origin })
    }

    fn url(&self, value: &str) -> Result<Url> {
        let url = Url::parse(value)
            .map_err(|_| Error::ConfigurationError("Invalid provider request URL".into()))?;
        if !url.username().is_empty() || url.password().is_some() || url.fragment().is_some() {
            return Err(Error::ConfigurationError(
                "Invalid provider request URL".into(),
            ));
        }
        let allowed = match &self.origin {
            Some(origin) => url.origin().ascii_serialization() == *origin,
            None => {
                url.scheme() == "https"
                    && url.host_str().is_some_and(|host| {
                        [
                            "ec2.amazonaws.com",
                            "s3.amazonaws.com",
                            "sts.amazonaws.com",
                            "iam.amazonaws.com",
                            "compute.googleapis.com",
                            "storage.googleapis.com",
                            "iam.googleapis.com",
                            "management.azure.com",
                            "storage.azure.com",
                            "api.digitalocean.com",
                            "api.vultr.com",
                            "cloud.lambdalabs.com",
                            "cloud.lambda.com",
                            "api.runpod.io",
                            "rest.runpod.io",
                            "console.vast.ai",
                            "api.coreweave.com",
                            "api.paperspace.io",
                            "api.fluidstack.io",
                            "marketplace.tensordock.com",
                            "dashboard.tensordock.com",
                            "api.akash.network",
                            "api.io.net",
                            "api.primeintellect.ai",
                            "api.render.com",
                            "api.lium.ai",
                            "lium.io",
                            "api.hetzner.cloud",
                            "api.crusoecloud.com",
                            "api.rendernetwork.com",
                            "api.ovh.com",
                            "api.us.ovhcloud.com",
                            "kubernetes.default.svc",
                            "kubernetes.default.svc.cluster.local",
                        ]
                        .iter()
                        .any(|domain| host == *domain || host.ends_with(&format!(".{domain}")))
                    })
            }
        };
        if !allowed {
            return Err(Error::ConfigurationError(
                "Request outside provider API origin".into(),
            ));
        }
        Ok(url)
    }

    fn authenticated(
        &self,
        method: Method,
        url: &str,
        auth: &ApiAuthentication,
        body: Option<&serde_json::Value>,
    ) -> Result<reqwest::RequestBuilder> {
        let url = self.url(url)?;
        let mut request = self.client.request(method.clone(), url.clone());
        if let Some(body) = body {
            request = request.json(body);
        }
        Ok(match auth {
            ApiAuthentication::Bearer { token } => request.bearer_auth(token),
            ApiAuthentication::ApiKey { key, header_name } => {
                let mut value = header::HeaderValue::from_str(key)
                    .map_err(|_| Error::Provider(ProviderErrorCode::InvalidRequest))?;
                value.set_sensitive(true);
                request.header(header_name, value)
            }
            ApiAuthentication::Ovh {
                application_key,
                application_secret,
                consumer_key,
                timestamp,
            } => {
                use sha1::{Digest, Sha1};
                // OVH signs the exact URL and body sent; `json()` serializes the same way.
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
                request
                    .header("X-Ovh-Application", application_key.as_str())
                    .header("X-Ovh-Consumer", consumer_key.as_str())
                    .header("X-Ovh-Timestamp", timestamp)
                    .header(
                        "X-Ovh-Signature",
                        format!("$1${}", hex::encode(Sha1::digest(signed))),
                    )
            }
            ApiAuthentication::None => request,
        })
    }

    async fn send(&self, request: reqwest::RequestBuilder) -> Result<Response> {
        const MAX_BODY: usize = 10 * 1024 * 1024;
        let mut response = request
            .send()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?;
        let status = response.status();
        if !status.is_success() && status != reqwest::StatusCode::NOT_FOUND {
            return Err(Error::Provider(match status.as_u16() {
                401 | 403 => ProviderErrorCode::AuthRejected,
                402 => ProviderErrorCode::Quota,
                429 => ProviderErrorCode::RateLimited,
                500..=599 => ProviderErrorCode::Indeterminate,
                _ => ProviderErrorCode::ProviderError,
            }));
        }
        if response
            .content_length()
            .is_some_and(|length| length > MAX_BODY as u64)
        {
            return Err(Error::Provider(ProviderErrorCode::ProviderError));
        }
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|_| Error::Provider(ProviderErrorCode::Indeterminate))?
        {
            if chunk.len() > MAX_BODY - body.len() {
                return Err(Error::Provider(ProviderErrorCode::ProviderError));
            }
            body.extend_from_slice(&chunk);
        }
        // Preserve status and headers for native parsing; error bodies never escape.
        if status == reqwest::StatusCode::NOT_FOUND {
            body.clear();
        }
        let mut bounded = http::Response::builder().status(status);
        *bounded.headers_mut().expect("valid HTTP response builder") = response.headers().clone();
        bounded
            .body(body)
            .map(Response::from)
            .map_err(|_| Error::Provider(ProviderErrorCode::ProviderError))
    }

    pub async fn authenticated_request(
        &self,
        method: Method,
        url: &str,
        auth: &ApiAuthentication,
        body: Option<serde_json::Value>,
    ) -> Result<Response> {
        self.send(self.authenticated(method, url, auth, body.as_ref())?)
            .await
    }

    pub async fn get(&self, url: &str, auth: &ApiAuthentication) -> Result<Response> {
        self.authenticated_request(Method::GET, url, auth, None)
            .await
    }

    pub async fn post(
        &self,
        url: &str,
        auth: &ApiAuthentication,
        body: Option<serde_json::Value>,
    ) -> Result<Response> {
        self.authenticated_request(Method::POST, url, auth, body)
            .await
    }

    pub async fn post_json(
        &self,
        url: &str,
        auth: &ApiAuthentication,
        body: serde_json::Value,
    ) -> Result<Response> {
        self.post(url, auth, Some(body)).await
    }

    pub async fn post_json_idempotent(
        &self,
        url: &str,
        auth: &ApiAuthentication,
        body: serde_json::Value,
        key: &str,
    ) -> Result<Response> {
        if key.len() != 64 || !key.bytes().all(|byte| byte.is_ascii_hexdigit()) {
            return Err(Error::Provider(ProviderErrorCode::InvalidRequest));
        }
        self.send(
            self.authenticated(Method::POST, url, auth, Some(&body))?
                .header("Idempotency-Key", key),
        )
        .await
    }

    pub async fn post_form(
        &self,
        url: &str,
        auth: &ApiAuthentication,
        form: &std::collections::HashMap<String, String>,
    ) -> Result<Response> {
        self.send(
            self.authenticated(Method::POST, url, auth, None)?
                .form(form),
        )
        .await
    }

    pub async fn delete(&self, url: &str, auth: &ApiAuthentication) -> Result<Response> {
        self.authenticated_request(Method::DELETE, url, auth, None)
            .await
    }
}

/// API authentication methods
#[derive(Clone, zeroize::Zeroize, zeroize::ZeroizeOnDrop)]
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
    pub fn digitalocean(token: String) -> Self {
        Self::Bearer { token }
    }

    pub fn google_cloud(token: String) -> Self {
        Self::Bearer { token }
    }

    pub fn azure(token: String) -> Self {
        Self::Bearer { token }
    }

    pub fn vultr(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn lambda_labs(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn runpod(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn vast_ai(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn coreweave(token: String) -> Self {
        Self::Bearer { token }
    }

    pub fn paperspace(api_key: String) -> Self {
        Self::ApiKey {
            key: api_key,
            header_name: "x-api-key".to_string(),
        }
    }

    pub fn fluidstack(api_key: String) -> Self {
        Self::ApiKey {
            key: api_key,
            header_name: "api-key".to_string(),
        }
    }

    pub fn tensordock(api_key: String, api_token: String) -> Self {
        Self::ApiKey {
            key: format!("{api_key}:{api_token}"),
            header_name: "Authorization".to_string(),
        }
    }

    pub fn akash(token: String) -> Self {
        Self::Bearer { token }
    }

    pub fn io_net(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn prime_intellect(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn render(api_key: String) -> Self {
        Self::Bearer { token: api_key }
    }

    pub fn bittensor_lium(api_key: String) -> Self {
        Self::ApiKey {
            key: api_key,
            header_name: "X-API-Key".into(),
        }
    }

    pub fn hetzner(api_token: String) -> Self {
        Self::Bearer { token: api_token }
    }

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
