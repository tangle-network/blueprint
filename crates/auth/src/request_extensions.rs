//! Request extension plumbing for client certificate identity
//! Provides mechanisms to extract and inject mTLS identity information

use axum::extract::FromRequestParts;
use axum::http::HeaderMap;
use axum::http::request::Parts;
use std::collections::HashMap;

use crate::tls_listener::ClientCertInfo;

/// Request extension that carries client certificate information
#[derive(Clone, Debug)]
pub struct ClientCertExtension {
    pub client_cert: Option<ClientCertInfo>,
    pub headers: HeaderMap,
}

impl ClientCertExtension {
    /// Create a new client certificate extension
    pub fn new(client_cert: Option<ClientCertInfo>, headers: HeaderMap) -> Self {
        Self {
            client_cert,
            headers,
        }
    }

    /// Extract client certificate subject if available
    pub fn subject(&self) -> Option<&str> {
        self.client_cert.as_ref().map(|cert| cert.subject.as_str())
    }

    /// Extract client certificate issuer if available
    pub fn issuer(&self) -> Option<&str> {
        self.client_cert.as_ref().map(|cert| cert.issuer.as_str())
    }

    /// Extract client certificate serial if available
    pub fn serial(&self) -> Option<&str> {
        self.client_cert.as_ref().map(|cert| cert.serial.as_str())
    }

    /// Check if client certificate is valid (not expired)
    pub fn is_valid(&self) -> bool {
        if let Some(cert) = &self.client_cert {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs();
            cert.not_before <= now && now <= cert.not_after
        } else {
            false
        }
    }

    /// Get additional headers to inject based on client certificate
    pub fn additional_headers(&self) -> HeaderMap {
        let mut headers = HeaderMap::new();

        if let Some(cert) = &self.client_cert {
            // Inject client certificate information as headers
            headers.insert("x-client-cert-subject", cert.subject.parse().unwrap());
            headers.insert("x-client-cert-issuer", cert.issuer.parse().unwrap());
            headers.insert("x-client-cert-serial", cert.serial.parse().unwrap());
            headers.insert(
                "x-client-cert-not-before",
                cert.not_before.to_string().parse().unwrap(),
            );
            headers.insert(
                "x-client-cert-not-after",
                cert.not_after.to_string().parse().unwrap(),
            );

            // Add authentication method header
            headers.insert("x-auth-method", "mtls".parse().unwrap());
        }

        headers
    }
}

/// Extractor for client certificate information from request
pub struct ClientCertExtractor {
    pub client_cert: Option<ClientCertInfo>,
}

impl<S> FromRequestParts<S> for ClientCertExtractor
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract client certificate information from request extensions
        let client_cert = parts.extensions.get::<ClientCertInfo>().cloned();

        Ok(Self { client_cert })
    }
}

/// Middleware to inject client certificate information into request extensions
pub struct ClientCertMiddleware<S> {
    inner: S,
}

impl<S> ClientCertMiddleware<S> {
    pub fn new(inner: S) -> Self {
        Self { inner }
    }
}

impl<S> tower::Service<axum::extract::Request> for ClientCertMiddleware<S>
where
    S: tower::Service<axum::extract::Request, Response = axum::response::Response>
        + Clone
        + Send
        + 'static,
    S::Future: Send + 'static,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = std::pin::Pin<
        Box<dyn std::future::Future<Output = Result<Self::Response, Self::Error>> + Send>,
    >;

    fn poll_ready(
        &mut self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: axum::extract::Request) -> Self::Future {
        // Extract client certificate information from the request
        // This would typically come from the TLS connection
        let client_cert = req.extensions().get::<ClientCertInfo>().cloned();

        // Add client certificate extension to the request
        if let Some(cert) = client_cert {
            req.extensions_mut().insert(cert);
        }

        let inner = self.inner.clone();
        let mut inner = std::mem::replace(&mut self.inner, inner);

        Box::pin(async move { inner.call(req).await })
    }
}

/// Helper function to create client certificate middleware
pub fn client_cert_middleware<S>(inner: S) -> ClientCertMiddleware<S> {
    ClientCertMiddleware::new(inner)
}

/// Request extension for authentication context
#[derive(Clone, Debug)]
pub struct AuthContext {
    pub service_id: u64,
    pub auth_method: AuthMethod,
    pub client_cert: Option<ClientCertInfo>,
    pub additional_headers: HashMap<String, String>,
}

impl AuthContext {
    pub fn new(service_id: u64, auth_method: AuthMethod) -> Self {
        Self {
            service_id,
            auth_method,
            client_cert: None,
            additional_headers: HashMap::new(),
        }
    }

    pub fn with_client_cert(mut self, client_cert: Option<ClientCertInfo>) -> Self {
        self.client_cert = client_cert;
        self
    }

    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.additional_headers = headers;
        self
    }

    pub fn is_mtls(&self) -> bool {
        matches!(self.auth_method, AuthMethod::Mtls)
    }

    pub fn client_cert_subject(&self) -> Option<&str> {
        self.client_cert.as_ref().map(|cert| cert.subject.as_str())
    }
}

/// Authentication method enum
#[derive(Clone, Debug, PartialEq)]
pub enum AuthMethod {
    ApiKey,
    AccessToken,
    Mtls,
    OAuth,
}

/// Extractor for authentication context
pub struct AuthContextExtractor {
    pub context: AuthContext,
}

impl<S> FromRequestParts<S> for AuthContextExtractor
where
    S: Send + Sync,
{
    type Rejection = axum::http::StatusCode;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract authentication context from request extensions
        let context = parts
            .extensions
            .get::<AuthContext>()
            .cloned()
            .ok_or(axum::http::StatusCode::UNAUTHORIZED)?;

        Ok(Self { context })
    }
}

/// Helper function to inject authentication context into request
pub fn inject_auth_context(
    mut req: axum::extract::Request,
    context: AuthContext,
) -> axum::extract::Request {
    req.extensions_mut().insert(context);
    req
}

/// Helper function to extract client certificate from request
pub fn extract_client_cert_from_request(req: &axum::extract::Request) -> Option<ClientCertInfo> {
    req.extensions().get::<ClientCertInfo>().cloned()
}

/// Helper function to extract authentication context from request
pub fn extract_auth_context_from_request(req: &axum::extract::Request) -> Option<AuthContext> {
    req.extensions().get::<AuthContext>().cloned()
}
