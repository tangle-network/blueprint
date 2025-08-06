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
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor, rt::TokioTimer};
use tower_http::cors::CorsLayer;
use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};
use tower_http::sensitive_headers::{
    SetSensitiveRequestHeadersLayer, SetSensitiveResponseHeadersLayer,
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};

use crate::api_keys::{ApiKeyGenerator, ApiKeyModel};
use crate::db::RocksDb;
use crate::models::{ApiTokenModel, ServiceModel};
use crate::paseto_tokens::PasetoTokenManager;
use crate::types::{ServiceId, VerifyChallengeResponse};
use crate::validation;

type HTTPClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

/// The default port for the authenticated proxy server
// T9 Mapping of TBPM (Tangle Blueprint Manager)
pub const DEFAULT_AUTH_PROXY_PORT: u16 = 8276;

pub struct AuthenticatedProxy {
    client: HTTPClient,
    db: crate::db::RocksDb,
    paseto_manager: PasetoTokenManager,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedProxyState {
    client: HTTPClient,
    db: crate::db::RocksDb,
    paseto_manager: PasetoTokenManager,
}

impl AuthenticatedProxy {
    pub fn new<P: AsRef<Path>>(db_path: P) -> Result<Self, crate::Error> {
        let executer = TokioExecutor::new();
        let timer = TokioTimer::new();
        let client: HTTPClient = hyper_util::client::legacy::Builder::new(executer)
            .pool_idle_timeout(std::time::Duration::from_secs(60))
            .pool_timer(timer)
            .build(HttpConnector::new());
        let db_config = crate::db::RocksDbConfig::default();
        let db = crate::db::RocksDb::open(db_path, &db_config)?;

        // Initialize Paseto token manager with 15-minute TTL
        let paseto_manager = PasetoTokenManager::new(std::time::Duration::from_secs(15 * 60));

        Ok(AuthenticatedProxy {
            client,
            db,
            paseto_manager,
        })
    }

    pub fn router(self) -> Router {
        let state = AuthenticatedProxyState {
            db: self.db,
            client: self.client,
            paseto_manager: self.paseto_manager,
        };
        Router::new()
            .nest("/v1", Self::internal_api_router_v1())
            .fallback(any(reverse_proxy))
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
                    message: format!("Internal server error: {}", e),
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
                            message: format!("Invalid headers: {}", e),
                        }),
                    );
                }
            };

            // Generate long-lived API key (90 days)
            let api_key_gen = ApiKeyGenerator::with_prefix(service.api_key_prefix());
            let expires_at = payload.expires_at.max(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    + (90 * 24 * 60 * 60), // 90 days
            );

            let api_key = api_key_gen.generate_key(
                service_id,
                expires_at,
                validated_headers,
                "Generated via challenge verification".to_string(),
                &mut rng,
            );

            let mut api_key_model = ApiKeyModel::from(&api_key);
            if let Err(e) = api_key_model.save(&s.db) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(VerifyChallengeResponse::UnexpectedError {
                        message: format!("Internal server error: {}", e),
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
                message: format!("Internal server error: {}", e),
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
            tracing::error!("Database error looking up API key: {}", e);
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
        tracing::warn!("Failed to update API key last_used timestamp: {}", e);
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

    // Generate Paseto access token
    let service_id = api_key_model.service_id();
    let tenant_id = validated_headers.get("X-Tenant-Id").cloned();
    let custom_ttl = payload.ttl_seconds.map(std::time::Duration::from_secs);

    let access_token = match s.paseto_manager.generate_token(
        service_id,
        api_key_model.key_id.clone(),
        tenant_id,
        validated_headers,
        custom_ttl,
    ) {
        Ok(token) => token,
        Err(e) => {
            tracing::error!("Failed to create Paseto token: {}", e);
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
        Ok(Some(token)) if token.is(&token_str) && !token.is_expired() && token.is_enabled => token,
        Ok(Some(_)) | Ok(None) => {
            tracing::warn!("Invalid or expired legacy token");
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
            tracing::warn!("API key not found: {}", key_id);
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(e) => {
            tracing::error!("Database error looking up API key: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Validate the full key
    if !api_key_model.validates_key(api_key) {
        tracing::warn!("API key validation failed: {}", key_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check expiration and enabled status
    if api_key_model.is_expired() {
        tracing::warn!("API key expired: {}", key_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    if !api_key_model.is_enabled {
        tracing::warn!("API key disabled: {}", key_id);
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Update last used timestamp
    if let Err(e) = api_key_model.update_last_used(db) {
        tracing::warn!("Failed to update API key last_used timestamp: {}", e);
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
            tracing::warn!("Paseto token validation failed: {}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Token expiration is already checked in validate_token
    Ok((claims.service_id, claims.additional_headers))
}

/// Reverse proxy handler that forwards requests to the target host based on the service ID
#[tracing::instrument(skip_all)]
async fn reverse_proxy(
    headers: axum::http::HeaderMap,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    // Extract and validate token from Authorization header
    let auth_header = headers
        .get(crate::types::headers::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;

    // Parse token type and validate
    let (service_id, additional_headers) = if auth_header.contains('|') {
        // Legacy API Token format: "id|token"
        let legacy_token = crate::api_tokens::ApiToken::from_str(auth_header)
            .map_err(|_| StatusCode::BAD_REQUEST)?;
        handle_legacy_token(legacy_token, &s.db).await?
    } else if auth_header.contains('.') && !auth_header.starts_with("v4.local.") {
        // API Key format: "ak_xxxxx.yyyyy"
        handle_api_key(auth_header, &s.db).await?
    } else if auth_header.starts_with("v4.local.") {
        // Paseto access token format: "v4.local.xxxxx"
        handle_paseto_token(auth_header, &s.paseto_manager).await?
    } else {
        tracing::warn!("Unrecognized token format: {}", auth_header);
        return Err(StatusCode::UNAUTHORIZED);
    };

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

    // Inject additional headers into the request
    for (header_name, header_value) in additional_headers {
        if let Ok(name) = header::HeaderName::from_bytes(header_name.as_bytes()) {
            if let Ok(value) = header::HeaderValue::from_str(&header_value) {
                req.headers_mut().insert(name, value);
            } else {
                tracing::warn!("Invalid header value: {}", header_value);
            }
        } else {
            tracing::warn!("Invalid header name: {}", header_name);
        }
    }

    // Forward the request to the target server
    let response = s
        .client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

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
                    eprintln!("Hello world server error: {}", e);
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
            .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
            .await;
        assert!(
            res.status().is_success(),
            "Request to reverse proxy failed: {:?}",
            res
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
                    eprintln!("Echo server error: {}", e);
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
            .header(headers::AUTHORIZATION, format!("Bearer {}", api_key))
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
