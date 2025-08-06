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

use crate::api_tokens::{ApiToken, ApiTokenGenerator};
use crate::db::RocksDb;
use crate::models::{ApiTokenModel, ServiceModel};
use crate::types::{ServiceId, VerifyChallengeResponse};
use crate::validation;

type HTTPClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

/// The default port for the authenticated proxy server
// T9 Mapping of TBPM (Tangle Blueprint Manager)
pub const DEFAULT_AUTH_PROXY_PORT: u16 = 8276;

pub struct AuthenticatedProxy {
    client: HTTPClient,
    db: crate::db::RocksDb,
}

#[derive(Clone, Debug)]
pub struct AuthenticatedProxyState {
    client: HTTPClient,
    db: crate::db::RocksDb,
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
        Ok(AuthenticatedProxy { client, db })
    }

    pub fn router(self) -> Router {
        let state = AuthenticatedProxyState {
            db: self.db,
            client: self.client,
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
    let token_gen = ApiTokenGenerator::with_prefix(service.api_key_prefix());
    match result {
        Ok(true) => {
            // Validate additional headers before storing
            let validated_headers = match validation::validate_headers(&payload.additional_headers) {
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
            
            let token = token_gen.generate_token_with_expiration_and_headers(
                service_id,
                payload.expires_at,
                validated_headers,
                &mut rng,
            );
            let id = match ApiTokenModel::from(&token).save(&s.db) {
                Ok(id) => id,
                Err(e) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(crate::types::VerifyChallengeResponse::UnexpectedError {
                            message: format!("Internal server error: {}", e),
                        }),
                    );
                }
            };
            let plaintext = token.plaintext(id);
            (
                StatusCode::CREATED,
                Json(crate::types::VerifyChallengeResponse::Verified {
                    access_token: plaintext,
                    expires_at: payload.expires_at,
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

/// Reverse proxy handler that forwards requests to the target host based on the service ID
#[tracing::instrument(skip_all, fields(token_id = %token_id))]
async fn reverse_proxy(
    ApiToken(token_id, token_str): ApiToken,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let token = match ApiTokenModel::find_token_id(token_id, &s.db) {
        Ok(Some(token)) if token.is(&token_str) && !token.is_expired() && token.is_enabled => token,
        Ok(Some(_)) | Ok(None) => {
            tracing::warn!("Invalid or expired token");
            return Err(StatusCode::UNAUTHORIZED);
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
    
    // Get additional headers from the token
    let additional_headers = token.get_additional_headers();
    
    let service = match ServiceModel::find_by_id(token.service_id(), &s.db) {
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
        let access_token = match res {
            VerifyChallengeResponse::Verified { access_token, .. } => access_token,
            _ => panic!("Expected a verified response"),
        };

        let access_token = ApiToken::from_str(&access_token).expect("Failed to parse access token");
        // Try to send a request to the reverse proxy with the token in the header
        let res = client
            .get("/hello")
            .header(headers::AUTHORIZATION, format!("Bearer {}", access_token))
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
                    if name.as_str().starts_with("x-tenant-") || name.as_str().starts_with("X-Tenant-") {
                        response_headers.insert(
                            name.to_string(),
                            value.to_str().unwrap_or("").to_string(),
                        );
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
        let access_token = match res {
            VerifyChallengeResponse::Verified { access_token, .. } => access_token,
            _ => panic!("Expected a verified response"),
        };

        // Step 3: Make request with token and verify headers are forwarded
        let res = client
            .get("/echo")
            .header(headers::AUTHORIZATION, format!("Bearer {}", access_token))
            .await;
        
        assert!(res.status().is_success());
        
        let response_headers: BTreeMap<String, String> = res.json().await;
        assert_eq!(response_headers.get("x-tenant-id"), Some(&tenant_id));
        assert_eq!(response_headers.get("x-tenant-name"), Some(&"Acme Corp".to_string()));

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
        assert!(matches!(res, VerifyChallengeResponse::UnexpectedError { message } if message.contains("Invalid headers")));
    }
}
