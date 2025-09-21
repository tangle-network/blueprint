use core::iter::once;
use core::ops::Add;
use std::path::Path;

use axum::Json;
use axum::http::uri;
use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    http::header,
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::any,
    routing::post,
};
use blueprint_core::{debug, error, info, warn};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor};
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::sensitive_headers::{
    SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer,
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::instrument;

use crate::api_keys::{ApiKeyGenerator, ApiKeyModel};
use crate::certificate_authority::{
    CertificateAuthority, ClientCertificate, CreateTlsProfileRequest, IssueCertificateRequest,
    TlsProfileResponse, validate_certificate_request,
};
use crate::db::RocksDb;
use crate::models::{ApiTokenModel, ServiceModel, TlsProfile};
use crate::paseto_tokens::PasetoTokenManager;
use crate::request_extensions::{AuthMethod, extract_client_cert_from_request};
use crate::tls_assets::TlsAssetManager;
use crate::tls_client::TlsClientManager;
use crate::tls_envelope::{TlsEnvelope, init_tls_envelope_key};
use crate::tls_listener::{TlsListener, TlsListenerConfig};
use pem;

/// Maximum size for binary metadata headers in bytes
/// Configurable via build-time environment variable GRPC_BINARY_METADATA_MAX_SIZE
/// Default value is 16384 bytes (16KB) if not specified
/// Note: This uses a static variable instead of const to allow runtime configuration
static GRPC_BINARY_METADATA_MAX_SIZE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();

fn get_max_binary_metadata_size() -> usize {
    *GRPC_BINARY_METADATA_MAX_SIZE.get_or_init(|| {
        std::env::var("GRPC_BINARY_METADATA_MAX_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(16384) // 16KB default
    })
}
use crate::types::{ServiceId, VerifyChallengeResponse};
use crate::validation;

type HTTPClient =
    hyper_util::client::legacy::Client<hyper_util::client::legacy::connect::HttpConnector, Body>;
type HTTP2Client =
    hyper_util::client::legacy::Client<hyper_util::client::legacy::connect::HttpConnector, Body>;

/// The default port for the authenticated proxy server
// T9 Mapping of TBPM (Tangle Blueprint Manager)
pub const DEFAULT_AUTH_PROXY_PORT: u16 = 8276;

pub struct AuthenticatedProxy {
    http_client: HTTPClient,
    http2_client: HTTP2Client,
    tls_client_manager: TlsClientManager,
    db: crate::db::RocksDb,
    paseto_manager: PasetoTokenManager,
    tls_envelope: TlsEnvelope,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedProxyState {
    http_client: HTTPClient,
    http2_client: HTTP2Client,
    tls_client_manager: TlsClientManager,
    db: crate::db::RocksDb,
    paseto_manager: PasetoTokenManager,
    tls_envelope: TlsEnvelope,
    mtls_listener_address: Option<std::net::SocketAddr>,
    #[cfg(feature = "standalone")]
    mtls_listener_handle: Option<std::sync::Arc<tokio::task::JoinHandle<()>>>,
}

impl AuthenticatedProxyState {
    pub fn db_ref(&self) -> &crate::db::RocksDb {
        &self.db
    }
    pub fn paseto_manager_ref(&self) -> &PasetoTokenManager {
        &self.paseto_manager
    }
    pub fn tls_envelope_ref(&self) -> &TlsEnvelope {
        &self.tls_envelope
    }
    pub fn tls_client_manager_ref(&self) -> &TlsClientManager {
        &self.tls_client_manager
    }

    #[cfg(feature = "standalone")]
    pub fn set_mtls_listener_address(&mut self, addr: std::net::SocketAddr) {
        self.mtls_listener_address = Some(addr);
    }

    #[cfg(not(feature = "standalone"))]
    pub fn set_mtls_listener_address(&mut self, _addr: std::net::SocketAddr) {
        // No-op when standalone feature is not enabled
    }

    pub fn get_mtls_listener_address(&self) -> Option<std::net::SocketAddr> {
        self.mtls_listener_address
    }
}

impl AuthenticatedProxy {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, crate::Error> {
        let executer = TokioExecutor::new();

        // Configure HTTP connector for HTTP/1.1 support (fallback for non-TLS)
        let mut http_connector = HttpConnector::new();
        http_connector.enforce_http(false); // Allow both HTTP and HTTPS
        http_connector.set_nodelay(true); // Improve performance

        // Build HTTP/1.1 client for REST requests (fallback for non-TLS)
        let http_client: HTTPClient = hyper_util::client::legacy::Builder::new(executer.clone())
            .http2_only(false) // Allow HTTP/1.1 only
            .build(http_connector.clone());

        // Configure HTTP connector for HTTP/2 support (fallback for non-TLS)
        let mut http2_connector = HttpConnector::new();
        http2_connector.enforce_http(false); // Allow both HTTP and HTTPS
        http2_connector.set_nodelay(true); // Improve performance for gRPC

        // Build HTTP/2 client for gRPC requests (fallback for non-TLS)
        let http2_client: HTTP2Client = hyper_util::client::legacy::Builder::new(executer)
            .http2_only(true) // Use HTTP/2 only for gRPC compatibility
            .http2_adaptive_window(true) // Enable adaptive flow control for better gRPC performance
            .build(http2_connector);

        let db_config = crate::db::RocksDbConfig::default();
        let db = crate::db::RocksDb::open(&db_path, &db_config)?;

        // Initialize TLS envelope with persistent key
        let tls_envelope = Self::init_tls_envelope(&db_path)?;

        // Initialize TLS asset manager
        let tls_assets = TlsAssetManager::new(db.clone(), tls_envelope.clone());

        // Initialize TLS client manager
        let tls_client_manager = TlsClientManager::new(db.clone(), tls_assets);

        // Initialize Paseto token manager with persistent key
        let paseto_manager = Self::init_paseto_manager(&db_path)?;

        Ok(AuthenticatedProxy {
            http_client,
            http2_client,
            tls_client_manager,
            db,
            paseto_manager,
            tls_envelope,
        })
    }

    /// Initialize Paseto token manager with persistent key
    fn init_paseto_manager<P: AsRef<Path>>(db_path: P) -> Result<PasetoTokenManager, crate::Error> {
        use std::fs;
        use std::io::{Read, Write};

        // Try to load key from environment variable first
        if let Ok(key_hex) = std::env::var("PASETO_SIGNING_KEY") {
            if let Ok(key_bytes) = hex::decode(&key_hex) {
                if key_bytes.len() == 32 {
                    let mut key_array = [0u8; 32];
                    key_array.copy_from_slice(&key_bytes);
                    let key = crate::paseto_tokens::PasetoKey::from_bytes(key_array);
                    return Ok(PasetoTokenManager::with_key(
                        key,
                        std::time::Duration::from_secs(15 * 60),
                    ));
                }
            }
            warn!("Invalid PASETO_SIGNING_KEY environment variable, generating new key");
        }

        // Try to load key from file in db directory
        let key_path = db_path.as_ref().join(".paseto_key");
        if key_path.exists() {
            let mut file = fs::File::open(&key_path).map_err(crate::Error::Io)?;
            let mut key_bytes = vec![];
            file.read_to_end(&mut key_bytes).map_err(crate::Error::Io)?;

            if key_bytes.len() == 32 {
                let mut key_array = [0u8; 32];
                key_array.copy_from_slice(&key_bytes);
                let key = crate::paseto_tokens::PasetoKey::from_bytes(key_array);
                info!("Loaded existing Paseto signing key from disk");
                return Ok(PasetoTokenManager::with_key(
                    key,
                    std::time::Duration::from_secs(15 * 60),
                ));
            }
        }

        // Generate new key and save it
        let manager = PasetoTokenManager::new(std::time::Duration::from_secs(15 * 60));
        let key = manager.get_key();

        // Save key to file
        let mut file = fs::File::create(&key_path).map_err(crate::Error::Io)?;
        let key_bytes = key.as_bytes();
        file.write_all(&key_bytes).map_err(crate::Error::Io)?;
        file.sync_all().map_err(crate::Error::Io)?;

        // Set restrictive permissions on the key file (Unix only)
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let permissions = std::fs::Permissions::from_mode(0o600);
            fs::set_permissions(&key_path, permissions).map_err(crate::Error::Io)?;
        }

        info!("Generated and saved new Paseto signing key");
        Ok(manager)
    }

    /// Initialize TLS envelope with persistent key
    fn init_tls_envelope<P: AsRef<Path>>(db_path: P) -> Result<TlsEnvelope, crate::Error> {
        let envelope_key = init_tls_envelope_key(&db_path)
            .map_err(|e| crate::Error::Io(std::io::Error::other(e.to_string())))?;

        Ok(TlsEnvelope::with_key(envelope_key))
    }

    /// Create and start TLS listener with dual socket support
    pub async fn start_tls_listener<P: AsRef<Path>>(
        db_path: P,
        config: Option<TlsListenerConfig>,
    ) -> Result<(), crate::Error> {
        let config = config.unwrap_or_default();
        let listener = TlsListener::new(db_path, config).await?;
        listener.serve().await
    }

    pub fn router(self) -> Router {
        let state = AuthenticatedProxyState {
            http_client: self.http_client,
            http2_client: self.http2_client,
            tls_client_manager: self.tls_client_manager,
            db: self.db,
            paseto_manager: self.paseto_manager,
            tls_envelope: self.tls_envelope,
            mtls_listener_address: None,
            #[cfg(feature = "standalone")]
            mtls_listener_handle: None,
        };
        Router::new()
            .nest("/v1", Self::internal_api_router_v1())
            .fallback(any(unified_proxy))
            .layer(SetRequestIdLayer::new(
                header::HeaderName::from_static("x-request-id"),
                MakeRequestUuid,
            ))
            // propagate the header to the response before the response reaches `TraceLayer`
            .layer(PropagateRequestIdLayer::new(
                header::HeaderName::from_static("x-request-id"),
            ))
            .layer(SetSensitiveRequestHeadersLayer::new(once(
                header::AUTHORIZATION,
            )))
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(DefaultMakeSpan::new().include_headers(true))
                    .on_response(DefaultOnResponse::new().include_headers(true)),
            )
            .layer(CorsLayer::permissive())
            .layer(SetSensitiveResponseHeadersLayer::new(once(
                header::AUTHORIZATION,
            )))
            .with_state(state)
    }

    pub fn db(&self) -> RocksDb {
        self.db.clone()
    }

    /// Internal API router for version 1
    pub fn internal_api_router_v1() -> Router<AuthenticatedProxyState> {
        Router::new()
            .route("/auth/challenge", post(auth_challenge))
            .route("/auth/verify", post(auth_verify))
            .route("/auth/exchange", post(auth_exchange))
            // OAuth 2.0 JWT Bearer Assertion token endpoint (RFC 7523)
            .route("/oauth/token", post(crate::oauth::token::oauth_token))
            // mTLS administration endpoints
            .route(
                "/admin/services/{service_id}/tls-profile",
                axum::routing::put(update_tls_profile),
            )
            .route("/auth/certificates", axum::routing::post(issue_certificate))
    }
}

/// Auth challenge endpoint that handles authentication challenges
async fn auth_challenge(
    service_id: ServiceId,
    State(s): State<AuthenticatedProxyState>,
    Json(payload): Json<crate::types::ChallengeRequest>,
) -> Result<Json<crate::types::ChallengeResponse>, StatusCode> {
    let mut rng = blueprint_std::BlueprintRng::new();
    let service = ServiceModel::find_by_id(service_id, &s.db)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::NOT_FOUND)?;

    let public_key = payload.pub_key;
    if !service.is_owner(payload.key_type, &public_key) {
        return Err(StatusCode::UNAUTHORIZED);
    }
    let challenge = crate::generate_challenge(&mut rng);
    let now = std::time::SystemTime::now();
    let expires_at = now
        .duration_since(std::time::UNIX_EPOCH)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .add(std::time::Duration::from_secs(30))
        .as_secs();
    Ok(Json(crate::types::ChallengeResponse {
        challenge,
        expires_at,
    }))
}

/// Auth verify endpoint that handles authentication verification
async fn auth_verify(
    service_id: ServiceId,
    State(s): State<AuthenticatedProxyState>,
    Json(payload): Json<crate::types::VerifyChallengeRequest>,
) -> impl IntoResponse {
    let mut rng = blueprint_std::BlueprintRng::new();
    let service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => {
            return (
                StatusCode::NOT_FOUND,
                Json(VerifyChallengeResponse::UnexpectedError {
                    message: "Service not found".to_string(),
                }),
            );
        }
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(crate::types::VerifyChallengeResponse::UnexpectedError {
                    message: format!("Internal server error: {e}"),
                }),
            );
        }
    };

    let public_key = payload.challenge_request.pub_key;
    if !service.is_owner(payload.challenge_request.key_type, &public_key) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(crate::types::VerifyChallengeResponse::Unauthorized),
        );
    }
    // Verify the challenge
    let result = crate::verify_challenge(
        &payload.challenge,
        &payload.signature,
        &public_key,
        payload.challenge_request.key_type,
    );
    match result {
        Ok(true) => {
            // Validate additional headers before storing
            let validated_headers = match validation::validate_headers(&payload.additional_headers)
            {
                Ok(headers) => headers,
                Err(e) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(VerifyChallengeResponse::UnexpectedError {
                            message: format!("Invalid headers: {e}"),
                        }),
                    );
                }
            };

            // Apply PII protection by hashing sensitive fields
            let protected_headers =
                validation::process_headers_with_pii_protection(&validated_headers);

            // Generate long-lived API key (90 days)
            let api_key_gen = ApiKeyGenerator::with_prefix(service.api_key_prefix());
            let expires_at = payload.expires_at.max(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    + (90 * 24 * 60 * 60), // 90 days
            );

            let api_key =
                api_key_gen.generate_key(service_id, expires_at, protected_headers, &mut rng);

            let mut api_key_model = ApiKeyModel::from(&api_key);
            if let Err(e) = api_key_model.save(&s.db) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(VerifyChallengeResponse::UnexpectedError {
                        message: format!("Internal server error: {e}"),
                    }),
                );
            }

            (
                StatusCode::CREATED,
                Json(VerifyChallengeResponse::Verified {
                    api_key: api_key.full_key().to_string(),
                    expires_at,
                }),
            )
        }
        Ok(false) => (
            StatusCode::UNAUTHORIZED,
            Json(crate::types::VerifyChallengeResponse::InvalidSignature),
        ),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(crate::types::VerifyChallengeResponse::UnexpectedError {
                message: format!("Internal server error: {e}"),
            }),
        ),
    }
}

/// Token exchange endpoint that converts API keys to Paseto access tokens
async fn auth_exchange(
    State(s): State<AuthenticatedProxyState>,
    headers: axum::http::HeaderMap,
    Json(payload): Json<crate::auth_token::TokenExchangeRequest>,
) -> impl IntoResponse {
    // Extract API key from Authorization header
    let auth_header = match headers.get(crate::types::headers::AUTHORIZATION) {
        Some(header_value) => {
            let header_str = match header_value.to_str() {
                Ok(s) => s,
                Err(_) => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(serde_json::json!({
                            "error": "invalid_authorization_header",
                            "message": "Authorization header is not valid UTF-8"
                        })),
                    );
                }
            };

            // Extract Bearer token
            if let Some(token) = header_str.strip_prefix("Bearer ") {
                token
            } else {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(serde_json::json!({
                        "error": "invalid_authorization_header",
                        "message": "Authorization header must use Bearer scheme with API key"
                    })),
                );
            }
        }
        None => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "missing_authorization_header",
                    "message": "Authorization header with Bearer API key is required"
                })),
            );
        }
    };

    // Parse API key format: "ak_xxxxx.yyyyy"
    let key_id = if let Some((key_id_part, _)) = auth_header.split_once('.') {
        key_id_part
    } else {
        return (
            StatusCode::BAD_REQUEST,
            Json(serde_json::json!({
                "error": "invalid_api_key_format",
                "message": "API key must have format ak_xxxxx.yyyyy"
            })),
        );
    };

    // Find API key in database
    let mut api_key_model = match crate::api_keys::ApiKeyModel::find_by_key_id(key_id, &s.db) {
        Ok(Some(model)) => model,
        Ok(None) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(serde_json::json!({
                    "error": "invalid_api_key",
                    "message": "API key not found"
                })),
            );
        }
        Err(e) => {
            error!("Database error looking up API key: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "internal_error",
                    "message": "Failed to validate API key"
                })),
            );
        }
    };

    // Validate the full key matches
    if !api_key_model.validates_key(auth_header) {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "invalid_api_key",
                "message": "API key validation failed"
            })),
        );
    }

    // Check if key is expired or disabled
    if api_key_model.is_expired() {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "expired_api_key",
                "message": "API key has expired"
            })),
        );
    }

    if !api_key_model.is_enabled {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "disabled_api_key",
                "message": "API key is disabled"
            })),
        );
    }

    // Update last used timestamp
    if let Err(e) = api_key_model.update_last_used(&s.db) {
        warn!("Failed to update API key last_used timestamp: {}", e);
    }

    // Merge default headers with request headers
    let mut headers = api_key_model.get_default_headers();
    for (key, value) in payload.additional_headers {
        headers.insert(key, value);
    }

    // Validate merged headers
    let validated_headers = match crate::validation::validate_headers(&headers) {
        Ok(headers) => headers,
        Err(e) => {
            return (
                StatusCode::BAD_REQUEST,
                Json(serde_json::json!({
                    "error": "invalid_headers",
                    "message": format!("Header validation failed: {}", e)
                })),
            );
        }
    };

    // Apply PII protection
    let protected_headers =
        crate::validation::process_headers_with_pii_protection(&validated_headers);

    // Generate Paseto access token
    let service_id = api_key_model.service_id();
    let tenant_id = protected_headers.get("X-Tenant-Id").cloned();
    let custom_ttl = payload.ttl_seconds.map(std::time::Duration::from_secs);

    let access_token = match s.paseto_manager.generate_token(
        service_id,
        api_key_model.key_id.clone(),
        tenant_id,
        protected_headers,
        custom_ttl,
        None,
    ) {
        Ok(token) => token,
        Err(e) => {
            error!("Failed to create Paseto token: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(serde_json::json!({
                    "error": "token_generation_failed",
                    "message": "Failed to generate access token"
                })),
            );
        }
    };

    // Calculate expiration time
    let expires_at = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        + custom_ttl
            .unwrap_or(s.paseto_manager.default_ttl())
            .as_secs();

    let response = crate::auth_token::TokenExchangeResponse::new(access_token, expires_at);

    (
        StatusCode::OK,
        Json(serde_json::to_value(response).unwrap()),
    )
}

/// Create test TLS certificate and key files for mTLS testing
/// Update TLS profile for a service
async fn update_tls_profile(
    service_id: ServiceId,
    State(s): State<AuthenticatedProxyState>,
    Json(payload): Json<CreateTlsProfileRequest>,
) -> Result<Json<TlsProfileResponse>, StatusCode> {
    // Find the service
    let mut service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Create TLS profile with default empty values for encrypted fields
    let tls_profile = TlsProfile {
        tls_enabled: true,
        require_client_mtls: payload.require_client_mtls,
        encrypted_server_cert: Vec::new(), // Will be populated when server cert is generated
        encrypted_server_key: Vec::new(),  // Will be populated when server key is generated
        encrypted_client_ca_bundle: Vec::new(), // Will be populated when CA is initialized
        encrypted_upstream_ca_bundle: Vec::new(), // Optional upstream CA bundle
        encrypted_upstream_client_cert: Vec::new(), // Optional upstream client cert
        encrypted_upstream_client_key: Vec::new(), // Optional upstream client key
        client_cert_ttl_hours: payload.client_cert_ttl_hours,
        sni: payload.subject_alt_name_template.clone(),
    };

    // Update service with TLS profile
    service.tls_profile = Some(tls_profile);

    if let Err(e) = service.save(service_id, &s.db) {
        error!("Failed to save TLS profile: {}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // Initialize certificate authority if mTLS is enabled
    if payload.require_client_mtls {
        info!("mTLS enabled for service {}", service_id);
    }

    Ok(Json(TlsProfileResponse {
        tls_enabled: true,
        require_client_mtls: payload.require_client_mtls,
        client_cert_ttl_hours: payload.client_cert_ttl_hours,
        mtls_listener: format!(
            "https://localhost:{}",
            TlsListenerConfig::default().mtls_port
        ),
        subject_alt_name_template: payload.subject_alt_name_template,
    }))
}

/// Issue a client certificate for mTLS authentication
async fn issue_certificate(
    State(s): State<AuthenticatedProxyState>,
    Json(payload): Json<IssueCertificateRequest>,
) -> Result<Json<ClientCertificate>, StatusCode> {
    let service_id = ServiceId::new(payload.service_id);

    // Find the service
    let service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Check if TLS profile is configured
    let tls_profile = match &service.tls_profile {
        Some(profile) => profile,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    // Validate the certificate request against the profile
    if let Err(e) = validate_certificate_request(&payload, tls_profile) {
        warn!("Certificate request validation failed: {}", e);
        return Err(StatusCode::BAD_REQUEST);
    }

    // Initialize certificate authority using persisted CA material if available
    let mut service_ref = service.clone();
    let ca = if !tls_profile.encrypted_client_ca_bundle.is_empty() {
        load_persisted_ca(&s.tls_envelope, tls_profile)?
    } else {
        let ca = CertificateAuthority::new(s.tls_envelope.clone()).map_err(|e| {
            error!("Failed to initialise certificate authority: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

        persist_new_ca(&s.db, service_id, &mut service_ref, &ca)?;
        ca
    };

    // Generate client certificate
    let mut client_cert = match ca.generate_client_certificate(
        payload.common_name,
        payload.subject_alt_names,
        payload.ttl_hours,
    ) {
        Ok(cert) => cert,
        Err(e) => {
            error!("Failed to generate client certificate: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    if client_cert.revocation_url.is_none() {
        client_cert.revocation_url = Some(format!(
            "/v1/auth/certificates/{}/revoke",
            client_cert.serial
        ));
    }

    info!(
        "Issued client certificate for service {} with serial {}",
        service_id, client_cert.serial
    );
    Ok(Json(client_cert))
}

fn load_persisted_ca(
    envelope: &TlsEnvelope,
    profile: &TlsProfile,
) -> Result<CertificateAuthority, StatusCode> {
    let decrypted = envelope
        .decrypt(&profile.encrypted_client_ca_bundle)
        .map_err(|e| {
            error!("Failed to decrypt stored CA bundle: {e}");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let pem_str = String::from_utf8(decrypted).map_err(|e| {
        error!("Stored CA bundle is not valid UTF-8: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let blocks = pem::parse_many(&pem_str).map_err(|e| {
        error!("Failed to parse stored CA bundle: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut ca_cert_pem: Option<String> = None;
    let mut ca_key_pem: Option<String> = None;

    for block in blocks {
        let encoded = pem::encode(&block);
        match block.tag.as_str() {
            "CERTIFICATE" if ca_cert_pem.is_none() => ca_cert_pem = Some(encoded),
            tag if tag.ends_with("PRIVATE KEY") && ca_key_pem.is_none() => {
                ca_key_pem = Some(encoded)
            }
            _ => {}
        }
    }

    let cert = ca_cert_pem.ok_or_else(|| {
        error!("Stored CA bundle missing certificate block");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;
    let key = ca_key_pem.ok_or_else(|| {
        error!("Stored CA bundle missing private key block");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    CertificateAuthority::from_components(&cert, &key, clone_envelope(envelope)).map_err(|e| {
        error!("Failed to rehydrate certificate authority: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

fn persist_new_ca(
    db: &RocksDb,
    service_id: ServiceId,
    service: &mut ServiceModel,
    ca: &CertificateAuthority,
) -> Result<(), StatusCode> {
    let mut bundle = ca.ca_certificate_pem();
    if !bundle.ends_with('\n') {
        bundle.push('\n');
    }
    bundle.push_str(&ca.ca_private_key_pem());

    let encrypted = ca.envelope().encrypt(bundle.as_bytes()).map_err(|e| {
        error!("Failed to encrypt CA bundle: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if let Some(profile) = service.tls_profile.as_mut() {
        profile.encrypted_client_ca_bundle = encrypted;
    } else {
        error!("TLS profile unexpectedly missing while persisting CA");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    service.save(service_id, db).map_err(|e| {
        error!("Failed to persist CA bundle to service: {e}");
        StatusCode::INTERNAL_SERVER_ERROR
    })
}

fn clone_envelope(envelope: &TlsEnvelope) -> TlsEnvelope {
    TlsEnvelope::with_key(envelope.key().clone())
}

/// Handle legacy API token validation
async fn handle_legacy_token(
    token: crate::api_tokens::ApiToken,
    db: &crate::db::RocksDb,
) -> Result<
    (
        crate::types::ServiceId,
        std::collections::BTreeMap<String, String>,
    ),
    StatusCode,
> {
    let (token_id, token_str) = (token.0, token.1.as_str());

    let api_token = match ApiTokenModel::find_token_id(token_id, db) {
        Ok(Some(token)) if token.is(token_str) && !token.is_expired() && token.is_enabled => token,
        Ok(Some(_)) | Ok(None) => {
            warn!("Invalid or expired legacy token");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    let additional_headers = api_token.get_additional_headers();
    Ok((api_token.service_id(), additional_headers))
}

/// Handle API key validation
async fn handle_api_key(
    api_key: &str,
    db: &crate::db::RocksDb,
) -> Result<
    (
        crate::types::ServiceId,
        std::collections::BTreeMap<String, String>,
    ),
    StatusCode,
> {
    // Parse key_id from "ak_xxxxx.yyyyy"
    let key_id = api_key
        .split_once('.')
        .map(|(key_id_part, _)| key_id_part)
        .ok_or(StatusCode::BAD_REQUEST)?;

    // Find and validate API key
    let mut api_key_model = match crate::api_keys::ApiKeyModel::find_by_key_id(key_id, db) {
        Ok(Some(model)) => model,
        Ok(None) => {
            warn!("API key not found: {}", key_id);
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            error!("Database error looking up API key: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Validate the full key
    if !api_key_model.validates_key(api_key) {
        warn!("API key validation failed: {}", key_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check expiration and enabled status
    if api_key_model.is_expired() {
        warn!("API key expired: {}", key_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !api_key_model.is_enabled {
        warn!("API key disabled: {}", key_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update last used timestamp
    if let Err(e) = api_key_model.update_last_used(db) {
        warn!("Failed to update API key last_used timestamp: {}", e);
    }

    let additional_headers = api_key_model.get_default_headers();
    Ok((api_key_model.service_id(), additional_headers))
}

/// Handle Paseto access token validation
async fn handle_paseto_token(
    token: &str,
    paseto_manager: &crate::paseto_tokens::PasetoTokenManager,
) -> Result<
    (
        crate::types::ServiceId,
        std::collections::BTreeMap<String, String>,
    ),
    StatusCode,
> {
    let claims = match paseto_manager.validate_token(token) {
        Ok(claims) => claims,
        Err(e) => {
            warn!("Paseto token validation failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Build upstream headers starting from token's additional headers
    let mut headers = claims.additional_headers.clone();

    // Inject canonical X-Scopes from claims.scopes if present
    if let Some(scopes_vec) = claims.scopes.clone() {
        if !scopes_vec.is_empty() {
            let mut set = std::collections::BTreeSet::new();
            for s in scopes_vec {
                set.insert(s.to_lowercase());
            }
            let scopes_str = set.into_iter().collect::<Vec<_>>().join(" ");
            headers.insert("x-scopes".to_string(), scopes_str);
        }
    }

    // Token expiration is already checked in validate_token
    Ok((claims.service_id, headers))
}

/// Check if a header name is forbidden for security reasons
/// Note: header_name may already be lowercase from validation, but we normalize again for safety
fn is_forbidden_header(header_name: &str) -> bool {
    // Normalized comparison (case-insensitive)
    let lower = header_name.to_lowercase();
    matches!(
        lower.as_str(),
        "host"
            | "content-length"
            | "transfer-encoding"
            | "connection"
            | "upgrade"
            | "proxy-authorization"
            | "proxy-authenticate"
            | "x-forwarded-host"
            | "x-real-ip"
            | "x-forwarded-for"
            | "x-forwarded-proto"
            | "forwarded"
    )
}

/// Check if a header name is auth-specific and should be sanitized from client requests
fn is_auth_header(header_name: &str) -> bool {
    let lower = header_name.to_lowercase();
    lower == "authorization"
        || lower.starts_with("x-tenant-")
        || lower == "x-scope"
        || lower == "x-scopes"
}

/// Check if a header is required for gRPC functionality
fn is_grpc_required_header(header_name: &str) -> bool {
    let lower = header_name.to_lowercase();
    matches!(
        lower.as_str(),
        "content-type" | "te" | "grpc-encoding" | "grpc-accept-encoding"
    )
}

/// Check if a header is allowed to be injected as upstream metadata
/// Dedicated allowlist to prevent sensitive internal headers from leaking
fn is_proxy_injected_header_allowed(header_name: &str) -> bool {
    let lower = header_name.to_lowercase();
    // Allow tenant and scope headers as they are meant to be forwarded
    lower.starts_with("x-tenant-") 
        || lower == "x-scope" 
        || lower == "x-scopes"
        // Allow specific safe headers that are commonly forwarded
        || lower == "x-request-id"
        || lower == "x-trace-id"
        || lower == "user-agent"
        // Allow gRPC-specific headers that are safe to forward
        || lower.starts_with("grpc-")
}

/// Validate binary metadata header according to gRPC specification
/// Binary metadata keys must end with -bin, be base64 encoded, and have size limits
fn validate_binary_metadata(header_name: &str, header_value: &str) -> bool {
    let lower = header_name.to_lowercase();

    // Check if header ends with -bin suffix
    if !lower.ends_with("-bin") {
        return false;
    }

    // Check size limit (configurable max size for binary metadata to prevent large payloads)
    if header_value.len() > get_max_binary_metadata_size() {
        warn!(
            "Binary metadata header {} exceeds size limit: {} bytes (max: {})",
            header_name,
            header_value.len(),
            get_max_binary_metadata_size()
        );
        return false;
    }

    // Validate base64 encoding
    if header_value.contains('\r') || header_value.contains('\n') {
        warn!(
            "Binary metadata header {} contains CRLF characters",
            header_name
        );
        return false;
    }

    // Check if it's valid base64 (allowing padding and standard/base64url variants)
    if !header_value.chars().all(|c| {
        c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=' || c == '-' || c == '_'
    }) {
        warn!(
            "Binary metadata header {} contains invalid base64 characters",
            header_name
        );
        return false;
    }

    true
}

/// Extract and validate authentication token from headers
/// Returns (service_id, additional_headers) on success
async fn extract_and_validate_auth(
    headers: &axum::http::HeaderMap,
    db: &crate::db::RocksDb,
    paseto_manager: &crate::paseto_tokens::PasetoTokenManager,
) -> Result<
    (
        crate::types::ServiceId,
        std::collections::BTreeMap<String, String>,
    ),
    StatusCode,
> {
    let auth_header = headers
        .get(crate::types::headers::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or_else(|| {
            warn!("Missing or invalid Authorization header");
            StatusCode::UNAUTHORIZED
        })?;

    debug!("Processing auth header: {}", auth_header);

    // Use unified token parsing
    match crate::auth_token::AuthToken::parse(auth_header) {
        Ok(crate::auth_token::AuthToken::Legacy(legacy_token)) => {
            handle_legacy_token(legacy_token, db).await
        }
        Ok(crate::auth_token::AuthToken::ApiKey(api_key)) => handle_api_key(&api_key, db).await,
        Ok(crate::auth_token::AuthToken::AccessToken(_)) => {
            // This shouldn't happen as parse() returns an error for Paseto tokens
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
        Err(_) if auth_header.starts_with("v4.local.") => {
            // Special case for Paseto tokens which need the manager to validate
            handle_paseto_token(auth_header, paseto_manager).await
        }
        Err(e) => {
            warn!("Token parsing error for '{}': {:?}", auth_header, e);
            Err(StatusCode::UNAUTHORIZED)
        }
    }
}

/// Apply additional headers to request with security validation
fn apply_additional_headers(
    req: &mut Request,
    additional_headers: std::collections::BTreeMap<String, String>,
    is_grpc: bool,
) {
    for (header_name, header_value) in additional_headers {
        // Re-validate header names against security-sensitive headers
        if is_forbidden_header(&header_name) {
            warn!("Attempted to inject forbidden header: {}", header_name);
            continue;
        }

        // For gRPC, allow gRPC-required headers even if they might be auth-related
        // For gRPC, also enforce dedicated allowlist for proxy-injected headers to prevent sensitive internal headers from leaking
        let is_tenant_header = header_name.to_lowercase().starts_with("x-tenant-");
        let is_scope_header =
            header_name.to_lowercase() == "x-scope" || header_name.to_lowercase() == "x-scopes";
        let is_allowed_proxy_header = is_proxy_injected_header_allowed(&header_name);

        // Skip (continue) if header is not allowed
        let is_allowed = !is_auth_header(&header_name)
            || (is_grpc && is_grpc_required_header(&header_name))
            || is_tenant_header
            || is_scope_header
            || (is_grpc && is_allowed_proxy_header);

        if !is_allowed {
            continue;
        }

        if let Ok(name) = header::HeaderName::from_bytes(header_name.as_bytes()) {
            // Additional validation for header values
            if let Ok(value) = header::HeaderValue::from_str(&header_value) {
                // For gRPC binary metadata, apply additional validation
                if is_grpc
                    && header_name.to_lowercase().ends_with("-bin")
                    && !validate_binary_metadata(&header_name, &header_value)
                {
                    warn!(
                        "Invalid binary metadata header {}: {}",
                        header_name, header_value
                    );
                    continue;
                }

                // Prevent header value injection attacks
                if header_value.contains('\r') || header_value.contains('\n') {
                    warn!("Header value contains CRLF: {}", header_name);
                    continue;
                }
                req.headers_mut().insert(name, value);
            } else {
                warn!("Invalid header value for {}: {}", header_name, header_value);
            }
        } else {
            warn!("Invalid header name: {}", header_name);
        }
    }
}

/// Sanitize request headers by removing auth-specific and forbidden headers
fn sanitize_request_headers(req: &mut Request, is_grpc: bool) {
    let mut to_remove: Vec<header::HeaderName> = Vec::new();
    for (name, _value) in req.headers().iter() {
        let name_str = name.as_str();
        if is_auth_header(name_str) || is_forbidden_header(name_str) {
            // For gRPC, don't remove gRPC-required headers
            if !(is_grpc && is_grpc_required_header(name_str)) {
                to_remove.push(name.clone());
            }
        }
    }
    for name in to_remove {
        req.headers_mut().remove(name);
    }
}

/// Detect if a request is a gRPC request based on headers and HTTP version
fn is_grpc_request(headers: &axum::http::HeaderMap, req: &Request) -> bool {
    // Check for content-type: application/grpc (case-insensitive)
    let content_type = headers.get("content-type");
    let is_grpc_content_type = content_type
        .and_then(|ct| ct.to_str().ok())
        .map(|ct| {
            debug!("Content-Type header: {}", ct);
            ct.to_lowercase().starts_with("application/grpc")
        })
        .unwrap_or(false);

    // Check for te: trailers header (required for gRPC)
    let te_header = headers.get("te");
    let has_te_trailers = te_header
        .and_then(|te| te.to_str().ok())
        .map(|te| {
            debug!("TE header: {}", te);
            te.to_lowercase() == "trailers"
        })
        .unwrap_or(false);

    debug!(
        "gRPC detection - content-type: {:?}, is_grpc: {}, te: {:?}, has_trailers: {}",
        content_type, is_grpc_content_type, te_header, has_te_trailers
    );

    // Check HTTP version - gRPC requires HTTP/2 to prevent HTTP/1.1 downgrade attempts
    let is_http2 = req.version() == axum::http::Version::HTTP_2;
    debug!("HTTP version: {:?}, is_http2: {}", req.version(), is_http2);

    let result = is_grpc_content_type && has_te_trailers && is_http2;
    debug!("gRPC request detected: {}", result);
    result
}

/// gRPC-aware proxy handler that can handle both HTTP and gRPC requests
#[instrument(skip_all)]
async fn unified_proxy(
    headers: axum::http::HeaderMap,
    State(s): State<AuthenticatedProxyState>,
    req: Request,
) -> Result<Response, StatusCode> {
    info!("Unified proxy called with headers: {:?}", headers);
    info!("Request method: {}, URI: {}", req.method(), req.uri());

    // Extract client certificate information from request extensions
    let client_cert = extract_client_cert_from_request(&req);

    // Determine authentication method based on available information
    let auth_method = if client_cert.is_some() {
        AuthMethod::Mtls
    } else if headers.get("authorization").is_some() {
        AuthMethod::AccessToken
    } else if headers.get("x-api-key").is_some() {
        AuthMethod::ApiKey
    } else {
        AuthMethod::OAuth
    };

    let is_grpc = is_grpc_request(&headers, &req);

    if is_grpc {
        info!("Detected gRPC request, using gRPC proxy path");
        grpc_proxy_with_mtls(headers, State(s), req, client_cert, auth_method).await
    } else {
        info!("Detected HTTP request, using HTTP proxy path");
        reverse_proxy_with_mtls(headers, State(s), req, client_cert, auth_method).await
    }
}

/// gRPC proxy handler that forwards gRPC requests while preserving gRPC-specific headers and trailers
#[instrument(skip_all)]
#[allow(dead_code)]
async fn grpc_proxy(
    headers: axum::http::HeaderMap,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    debug!("gRPC proxy called with headers: {:?}", headers);

    // Extract and validate authentication
    let (service_id, additional_headers) =
        extract_and_validate_auth(&headers, &s.db, &s.paseto_manager).await?;

    let service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let target_host = service
        .upstream_url()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    debug!("Target host: {:?}", target_host);

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let target_uri = Uri::builder()
        .scheme(target_host.scheme().cloned().unwrap_or(uri::Scheme::HTTP))
        .authority(
            target_host
                .authority()
                .cloned()
                .unwrap_or(uri::Authority::from_static("localhost")),
        )
        .path_and_query(path_query)
        .build()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    debug!("Target URI: {:?}", target_uri);

    // Set the target URI in the request
    *req.uri_mut() = target_uri;

    // Sanitize inbound headers and apply additional headers (gRPC-aware)
    sanitize_request_headers(&mut req, true);
    apply_additional_headers(&mut req, additional_headers, true);

    debug!("Forwarding gRPC request with headers: {:?}", req.headers());

    // Forward the request to the target server using HTTP/2 client for gRPC
    let response = s.http2_client.request(req).await.map_err(|e| {
        error!("Failed to forward gRPC request: {:?}", e);
        StatusCode::BAD_GATEWAY
    })?;

    debug!("gRPC response received: {:?}", response);

    Ok(response.into_response())
}

/// Reverse proxy handler that forwards requests to the target host based on the service ID
#[instrument(skip_all)]
#[allow(dead_code)]
async fn reverse_proxy(
    headers: axum::http::HeaderMap,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    // Extract and validate authentication
    let (service_id, additional_headers) =
        extract_and_validate_auth(&headers, &s.db, &s.paseto_manager).await?;

    let service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    let target_host = service
        .upstream_url()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let target_uri = Uri::builder()
        .scheme(target_host.scheme().cloned().unwrap_or(uri::Scheme::HTTP))
        .authority(
            target_host
                .authority()
                .cloned()
                .unwrap_or(uri::Authority::from_static("localhost")),
        )
        .path_and_query(path_query)
        .build()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Set the target URI in the request
    *req.uri_mut() = target_uri;

    // Sanitize inbound headers and apply additional headers
    sanitize_request_headers(&mut req, false);
    apply_additional_headers(&mut req, additional_headers, false);

    // Forward the request to the target server using HTTP/1.1 client for REST
    let response = s
        .http_client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(response.into_response())
}

/// gRPC proxy handler with mTLS support
#[instrument(skip_all)]
async fn grpc_proxy_with_mtls(
    headers: axum::http::HeaderMap,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
    client_cert: Option<crate::tls_listener::ClientCertInfo>,
    _auth_method: AuthMethod,
) -> Result<Response, StatusCode> {
    debug!("gRPC proxy with mTLS called with headers: {:?}", headers);

    // Extract and validate authentication
    let (service_id, mut additional_headers) =
        extract_and_validate_auth(&headers, &s.db, &s.paseto_manager).await?;

    // Check if service requires mTLS and enforce it
    let service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    // Enforce mTLS requirement if configured
    if let Some(tls_profile) = &service.tls_profile {
        if tls_profile.require_client_mtls && client_cert.is_none() {
            warn!(
                "mTLS required but no client certificate provided for service {}",
                service_id
            );
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // Add client certificate information to headers if available
    if let Some(cert) = &client_cert {
        additional_headers.insert("x-client-cert-subject".to_string(), cert.subject.clone());
        additional_headers.insert("x-client-cert-issuer".to_string(), cert.issuer.clone());
        additional_headers.insert("x-client-cert-serial".to_string(), cert.serial.clone());
        additional_headers.insert("x-auth-method".to_string(), "mtls".to_string());
    }

    let target_host = service
        .upstream_url()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    debug!("Target host: {:?}", target_host);

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let target_uri = Uri::builder()
        .scheme(target_host.scheme().cloned().unwrap_or(uri::Scheme::HTTP))
        .authority(
            target_host
                .authority()
                .cloned()
                .unwrap_or(uri::Authority::from_static("localhost")),
        )
        .path_and_query(path_query)
        .build()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    debug!("Target URI: {:?}", target_uri);

    // Set the target URI in the request
    *req.uri_mut() = target_uri;

    // Sanitize inbound headers and apply additional headers (gRPC-aware)
    sanitize_request_headers(&mut req, true);
    apply_additional_headers(&mut req, additional_headers, true);

    debug!("Forwarding gRPC request with headers: {:?}", req.headers());

    // Determine if we need TLS for the upstream connection
    let use_tls = target_host.scheme() == Some(&uri::Scheme::HTTPS);

    // Forward the request using appropriate client
    let response = if use_tls {
        // Use TLS client for HTTPS upstreams
        let tls_client = s
            .tls_client_manager
            .get_client_for_service(service_id)
            .await
            .map_err(|e| {
                error!(
                    "Failed to get TLS client for service {}: {:?}",
                    service_id, e
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        // Convert request body to Incoming type
        let (parts, body) = req.into_parts();
        let req_with_incoming = Request::from_parts(parts, body);
        tls_client
            .http2_client
            .request(req_with_incoming)
            .await
            .map_err(|e| {
                error!("Failed to forward gRPC request with TLS: {:?}", e);
                StatusCode::BAD_GATEWAY
            })?
    } else {
        // Use fallback HTTP/2 client for HTTP upstreams
        s.http2_client.request(req).await.map_err(|e| {
            error!("Failed to forward gRPC request: {:?}", e);
            StatusCode::BAD_GATEWAY
        })?
    };

    debug!("gRPC response received: {:?}", response);

    Ok(response.into_response())
}

/// Reverse proxy handler with mTLS support
#[instrument(skip_all)]
async fn reverse_proxy_with_mtls(
    headers: axum::http::HeaderMap,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
    client_cert: Option<crate::tls_listener::ClientCertInfo>,
    _auth_method: AuthMethod,
) -> Result<Response, StatusCode> {
    // Extract and validate authentication
    let (service_id, mut additional_headers) =
        extract_and_validate_auth(&headers, &s.db, &s.paseto_manager).await?;

    // Check if service requires mTLS and enforce it
    let service = match ServiceModel::find_by_id(service_id, &s.db) {
        Ok(Some(service)) => service,
        Ok(None) => return Err(StatusCode::NOT_FOUND),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };

    if let Some(tls_profile) = &service.tls_profile {
        if tls_profile.require_client_mtls && client_cert.is_none() {
            warn!(
                "mTLS required but no client certificate provided for service {}",
                service_id
            );
            return Err(StatusCode::UNAUTHORIZED);
        }
    }

    // Add client certificate information to headers if available
    if let Some(cert) = &client_cert {
        additional_headers.insert("x-client-cert-subject".to_string(), cert.subject.clone());
        additional_headers.insert("x-client-cert-issuer".to_string(), cert.issuer.clone());
        additional_headers.insert("x-client-cert-serial".to_string(), cert.serial.clone());
        additional_headers.insert("x-auth-method".to_string(), "mtls".to_string());
    }

    let target_host = service
        .upstream_url()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let target_uri = Uri::builder()
        .scheme(target_host.scheme().cloned().unwrap_or(uri::Scheme::HTTP))
        .authority(
            target_host
                .authority()
                .cloned()
                .unwrap_or(uri::Authority::from_static("localhost")),
        )
        .path_and_query(path_query)
        .build()
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    // Set the target URI in the request
    *req.uri_mut() = target_uri;

    // Sanitize inbound headers and apply additional headers
    sanitize_request_headers(&mut req, false);
    apply_additional_headers(&mut req, additional_headers, false);

    // Determine if we need TLS for the upstream connection
    let use_tls = target_host.scheme() == Some(&uri::Scheme::HTTPS);

    // Forward the request using appropriate client
    let response = if use_tls {
        // Use TLS client for HTTPS upstreams
        let tls_client = s
            .tls_client_manager
            .get_client_for_service(service_id)
            .await
            .map_err(|e| {
                error!(
                    "Failed to get TLS client for service {}: {:?}",
                    service_id, e
                );
                StatusCode::INTERNAL_SERVER_ERROR
            })?;

        // Convert request body to Incoming type
        let (parts, body) = req.into_parts();
        let req_with_incoming = Request::from_parts(parts, body);
        tls_client
            .http_client
            .request(req_with_incoming)
            .await
            .map_err(|e| {
                error!("Failed to forward HTTP request with TLS: {:?}", e);
                StatusCode::BAD_GATEWAY
            })?
    } else {
        // Use fallback HTTP/1.1 client for HTTP upstreams
        s.http_client
            .request(req)
            .await
            .map_err(|_| StatusCode::BAD_GATEWAY)?
    };

    Ok(response.into_response())
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::net::Ipv4Addr;

    use tempfile::tempdir;

    use super::*;
    use crate::{
        test_client::TestClient,
        types::{ChallengeRequest, ChallengeResponse, KeyType, VerifyChallengeResponse, headers},
    };

    #[tokio::test]
    async fn auth_flow_works() {
        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_line_number(true)
                .with_file(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .with_test_writer()
                .finish(),
        );
        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp = tempdir().unwrap();
        let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

        // Create a simple hello world http server using axum
        let hello_world_router =
            Router::new().route("/hello", axum::routing::get(|| async { "Hello, World!" }));

        // Start the simple hello world server in a separate task
        let (hello_world_server, local_addr) = {
            let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
                .await
                .expect("Failed to bind to address");
            let server = axum::serve(listener, hello_world_router);
            let local_address = server.local_addr().unwrap();
            let handle = tokio::spawn(async move {
                if let Err(e) = server.await {
                    eprintln!("Hello world server error: {e}");
                }
            });
            (handle, local_address)
        };

        // Create a service in the database first
        let service_id = ServiceId::new(0);
        let mut service = crate::models::ServiceModel {
            api_key_prefix: "test_".to_string(),
            owners: Vec::new(),
            upstream_url: format!("http://localhost:{}", local_addr.port()),
            tls_profile: None,
        };

        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        // Add the owner to the service
        service.add_owner(KeyType::Ecdsa, public_key.clone().into());
        service.save(service_id, &proxy.db).unwrap();

        let router = proxy.router();
        let client = TestClient::new(router);

        // Step 1
        let req = ChallengeRequest {
            pub_key: public_key.clone().into(),
            key_type: KeyType::Ecdsa,
        };

        let res = client
            .post("/v1/auth/challenge")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&req)
            .await;

        let res: ChallengeResponse = res.json().await;

        // Sign the challenge and send it back
        let (signature, _) = signing_key
            .sign_prehash_recoverable(&res.challenge)
            .unwrap();
        // sanity check
        assert!(
            crate::verify_challenge(
                &res.challenge,
                &signature.to_vec(),
                &public_key,
                KeyType::Ecdsa
            )
            .unwrap()
        );

        // Step 2
        let req = crate::types::VerifyChallengeRequest {
            challenge: res.challenge,
            signature: signature.to_bytes().into(),
            challenge_request: req,
            expires_at: 0,
            additional_headers: BTreeMap::new(),
        };

        let res = client
            .post("/v1/auth/verify")
            .header(headers::X_SERVICE_ID, ServiceId::new(0).to_string())
            .json(&req)
            .await;
        let res: VerifyChallengeResponse = res.json().await;

        assert!(matches!(res, VerifyChallengeResponse::Verified { .. }));
        let api_key = match res {
            VerifyChallengeResponse::Verified { api_key, .. } => api_key,
            _ => panic!("Expected a verified response"),
        };

        // Try to send a request to the reverse proxy with the token in the header
        let res = client
            .get("/hello")
            .header(headers::AUTHORIZATION, format!("Bearer {api_key}"))
            .await;
        assert!(
            res.status().is_success(),
            "Request to reverse proxy failed: {res:?}",
        );

        hello_world_server.abort(); // Stop the hello world server
    }

    #[tokio::test]
    async fn auth_flow_with_additional_headers() {
        use std::collections::BTreeMap;

        let _guard = tracing::subscriber::set_default(
            tracing_subscriber::fmt()
                .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
                .with_line_number(true)
                .with_file(true)
                .with_span_events(tracing_subscriber::fmt::format::FmtSpan::CLOSE)
                .with_test_writer()
                .finish(),
        );
        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp = tempdir().unwrap();
        let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

        // Create a test server that echoes back headers
        let echo_router = Router::new().route(
            "/echo",
            axum::routing::get(|headers: axum::http::HeaderMap| async move {
                let mut response_headers = BTreeMap::new();
                for (name, value) in headers.iter() {
                    if name.as_str().starts_with("x-tenant-")
                        || name.as_str().starts_with("X-Tenant-")
                    {
                        response_headers
                            .insert(name.to_string(), value.to_str().unwrap_or("").to_string());
                    }
                }
                axum::Json(response_headers)
            }),
        );

        // Start the echo server
        let (echo_server, local_addr) = {
            let listener = tokio::net::TcpListener::bind((Ipv4Addr::LOCALHOST, 0))
                .await
                .expect("Failed to bind to address");
            let server = axum::serve(listener, echo_router);
            let local_address = server.local_addr().unwrap();
            let handle = tokio::spawn(async move {
                if let Err(e) = server.await {
                    eprintln!("Echo server error: {e}");
                }
            });
            (handle, local_address)
        };

        // Create a service in the database
        let service_id = ServiceId::new(1);
        let mut service = crate::models::ServiceModel {
            api_key_prefix: "test_".to_string(),
            owners: Vec::new(),
            upstream_url: format!("http://localhost:{}", local_addr.port()),
            tls_profile: None,
        };

        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        service.add_owner(KeyType::Ecdsa, public_key.clone().into());
        service.save(service_id, &proxy.db).unwrap();

        let router = proxy.router();
        let client = TestClient::new(router);

        // Step 1: Get challenge
        let req = ChallengeRequest {
            pub_key: public_key.clone().into(),
            key_type: KeyType::Ecdsa,
        };

        let res = client
            .post("/v1/auth/challenge")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&req)
            .await;

        let res: ChallengeResponse = res.json().await;

        // Sign the challenge
        let (signature, _) = signing_key
            .sign_prehash_recoverable(&res.challenge)
            .unwrap();

        // Step 2: Verify with additional headers
        let mut additional_headers = BTreeMap::new();
        let user_id = "user123@example.com";
        let tenant_id = crate::validation::hash_user_id(user_id);
        additional_headers.insert("X-Tenant-Id".to_string(), tenant_id.clone());
        additional_headers.insert("X-Tenant-Name".to_string(), "Acme Corp".to_string());

        let req = crate::types::VerifyChallengeRequest {
            challenge: res.challenge,
            signature: signature.to_bytes().into(),
            challenge_request: req,
            expires_at: 0,
            additional_headers,
        };

        let res = client
            .post("/v1/auth/verify")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&req)
            .await;
        let res: VerifyChallengeResponse = res.json().await;

        assert!(matches!(res, VerifyChallengeResponse::Verified { .. }));
        let api_key = match res {
            VerifyChallengeResponse::Verified { api_key, .. } => api_key,
            _ => panic!("Expected a verified response"),
        };

        // Step 3: Make request with token and verify headers are forwarded
        let res = client
            .get("/echo")
            .header(headers::AUTHORIZATION, format!("Bearer {api_key}"))
            .await;

        assert!(res.status().is_success());

        let response_headers: BTreeMap<String, String> = res.json().await;
        assert_eq!(response_headers.get("x-tenant-id"), Some(&tenant_id));
        assert_eq!(
            response_headers.get("x-tenant-name"),
            Some(&"Acme Corp".to_string())
        );

        echo_server.abort();
    }

    #[tokio::test]
    async fn auth_flow_rejects_invalid_headers() {
        use std::collections::BTreeMap;

        let mut rng = blueprint_std::BlueprintRng::new();
        let tmp = tempdir().unwrap();
        let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

        let service_id = ServiceId::new(2);
        let mut service = crate::models::ServiceModel {
            api_key_prefix: "test_".to_string(),
            owners: Vec::new(),
            upstream_url: "http://localhost:9999".to_string(),
            tls_profile: None,
        };

        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        service.add_owner(KeyType::Ecdsa, public_key.clone().into());
        service.save(service_id, &proxy.db).unwrap();

        let router = proxy.router();
        let client = TestClient::new(router);

        // Get challenge
        let req = ChallengeRequest {
            pub_key: public_key.clone().into(),
            key_type: KeyType::Ecdsa,
        };

        let res = client
            .post("/v1/auth/challenge")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&req)
            .await;

        let res: ChallengeResponse = res.json().await;

        let (signature, _) = signing_key
            .sign_prehash_recoverable(&res.challenge)
            .unwrap();

        // Try to verify with forbidden headers
        let mut additional_headers = BTreeMap::new();
        additional_headers.insert("Connection".to_string(), "close".to_string());

        let req = crate::types::VerifyChallengeRequest {
            challenge: res.challenge,
            signature: signature.to_bytes().into(),
            challenge_request: req,
            expires_at: 0,
            additional_headers,
        };

        let res = client
            .post("/v1/auth/verify")
            .header(headers::X_SERVICE_ID, service_id.to_string())
            .json(&req)
            .await;

        let res: VerifyChallengeResponse = res.json().await;

        // Should fail with an error about invalid headers
        assert!(
            matches!(res, VerifyChallengeResponse::UnexpectedError { message } if message.contains("Invalid headers"))
        );
    }
}
