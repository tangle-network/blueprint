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

use crate::api_tokens::ApiTokenGenerator;
use crate::models::ApiTokenModel;
use crate::types::ServiceId;

type HTTPClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

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
            .route("/auth/challenge", post(auth_challenge))
            .route("/auth/verify", post(auth_verify))
            .fallback(any(reverse_proxy))
            .with_state(state)
    }
}

/// Auth challenge endpoint that handles authentication challenges
async fn auth_challenge(
    service_id: ServiceId,
    Json(payload): Json<crate::types::ChallengeRequest>,
) -> Result<Json<crate::types::ChallengeResponse>, StatusCode> {
    let mut rng = rand::thread_rng();
    // TODO: check for the public key of the sender.
    let _public_key = payload.pub_key;
    let challenge = crate::generate_challenge(&mut rng);
    // Implement the logic for the auth challenge endpoint
    Ok(Json(crate::types::ChallengeResponse {
        challenge,
        // TODO: Support Expires_at
        expires_at: 0,
    }))
}

/// Auth verify endpoint that handles authentication verification
async fn auth_verify(
    service_id: ServiceId,
    State(s): State<AuthenticatedProxyState>,
    Json(payload): Json<crate::types::VerifyChallengeRequest>,
) -> impl IntoResponse {
    let mut rng = rand::thread_rng();
    // TODO: check public key of the sender.
    // Verify the challenge
    let result = crate::verify_challenge(
        &payload.challenge,
        &payload.signature,
        &payload.challenge_request.pub_key,
        payload.challenge_request.key_type,
    );
    // TODO: support API Token prefix
    let token_gen = ApiTokenGenerator::new();
    match result {
        Ok(true) => {
            let token = token_gen.generate_token(service_id, &mut rng);
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
                    expires_at: 0,
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
    State(s): State<AuthenticatedProxyState>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let target_host = target_map
        .get(&service_id)
        .ok_or(StatusCode::PRECONDITION_FAILED)?;

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

        let router = proxy.router();
        let client = TestClient::new(router);

        let signing_key = k256::ecdsa::SigningKey::random(&mut rng);
        let public_key = signing_key.verifying_key().to_sec1_bytes();

        // Step 1
        let req = ChallengeRequest {
            pub_key: public_key.clone().into(),
            key_type: KeyType::Ecdsa,
        };

        let res = client
            .post("/auth/challenge")
            .header(headers::X_SERVICE_ID, ServiceId::new(0).to_string())
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
        };

        let res = client
            .post("/auth/verify")
            .header(headers::X_SERVICE_ID, ServiceId::new(0).to_string())
            .json(&req)
            .await;
        let res: VerifyChallengeResponse = res.json().await;

        assert!(matches!(res, VerifyChallengeResponse::Verified { .. }));
    }
}
