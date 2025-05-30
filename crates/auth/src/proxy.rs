use std::ops::Add;
use std::path::Path;

use axum::Json;
use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::any,
    routing::post,
};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor, rt::TokioTimer};

use crate::api_tokens::{ApiToken, ApiTokenGenerator};
use crate::models::{ApiTokenModel, ServiceModel};
use crate::types::{ServiceId, VerifyChallengeResponse};

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
            .with_state(state)
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
    let mut rng = rand::thread_rng();
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
    let mut rng = rand::thread_rng();
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
            let token =
                token_gen.generate_token_with_expiration(service_id, payload.expires_at, &mut rng);
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
async fn reverse_proxy(
    ApiToken(token_id, token_str): ApiToken,
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let token = match ApiTokenModel::find_token_id(token_id, &s.db) {
        Ok(Some(token)) if token.is(&token_str) && !token.is_expired() && token.is_enabled => token,
        Ok(Some(_)) | Ok(None) => return Err(StatusCode::UNAUTHORIZED),
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    };
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
    let target_uri = format!("{}/{}", target_host, path_query);
    let target_uri: Uri = target_uri.parse().map_err(|_| StatusCode::BAD_REQUEST)?;

    // Set the target URI in the request
    *req.uri_mut() = target_uri;

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
    use tempfile::tempdir;

    use super::*;
    use crate::{
        test_client::TestClient,
        types::{ChallengeRequest, ChallengeResponse, KeyType, VerifyChallengeResponse, headers},
    };

    #[tokio::test]
    async fn auth_flow_works() {
        let mut rng = rand::thread_rng();
        let tmp = tempdir().unwrap();
        let proxy = AuthenticatedProxy::new(tmp.path()).unwrap();

        // Create a service in the database first
        let service_id = ServiceId::new(0);
        let mut service = crate::models::ServiceModel {
            api_key_prefix: "test_".to_string(),
            owners: Vec::new(),
            upstream_url: "http://localhost:8080".to_string(),
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
            signature: signature.to_vec(),
            challenge_request: req,
            expires_at: 0,
        };

        let res = client
            .post("/v1/auth/verify")
            .header(headers::X_SERVICE_ID, ServiceId::new(0).to_string())
            .json(&req)
            .await;
        let res: VerifyChallengeResponse = res.json().await;

        assert!(matches!(res, VerifyChallengeResponse::Verified { .. }));
    }
}
