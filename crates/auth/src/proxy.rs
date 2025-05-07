use axum::Json;
use axum::{
    Extension, Router,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::any,
    routing::post,
};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor, rt::TokioTimer};

use crate::types::ServiceId;

type HTTPClient = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub struct AuthenticatedProxy {
    client: HTTPClient,
    target_host_map: hashbrown::HashMap<ServiceId, Uri>,
}

#[derive(Clone, Debug)]
struct TargetMap(hashbrown::HashMap<ServiceId, Uri>);

impl AuthenticatedProxy {
    pub fn new(service_id: ServiceId, target_host: Uri) -> Self {
        let executer = TokioExecutor::new();
        let timer = TokioTimer::new();
        let client: HTTPClient = hyper_util::client::legacy::Builder::new(executer)
            .pool_idle_timeout(std::time::Duration::from_secs(60))
            .pool_timer(timer)
            .build(HttpConnector::new());
        AuthenticatedProxy {
            client,
            target_host_map: hashbrown::HashMap::from([(service_id, target_host)]),
        }
    }

    pub fn router(self) -> Router {
        Router::new()
            .route("/auth/challenge", post(auth_challenge))
            .route("/auth/verify", post(auth_verify))
            .fallback(any(reverse_proxy))
            .layer(axum::Extension(self.client))
            .layer(axum::Extension(TargetMap(self.target_host_map)))
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
    Json(payload): Json<crate::types::VerifyChallengeRequest>,
) -> impl IntoResponse {
    // TODO: check public key of the sender.
    // Verify the challenge
    let result = crate::verify_challenge(
        &payload.challenge,
        &payload.signature,
        &payload.challenge_request.pub_key,
        payload.challenge_request.key_type,
    );
    match result {
        Ok(true) => {
            // Generate an API token and send it to the user.
            let token = String::from("token");
            return (
                StatusCode::CREATED,
                Json(crate::types::VerifyChallengeResponse::Verified {
                    access_token: token,
                    expires_at: 0,
                }),
            );
        }
        Ok(false) => {
            return (
                StatusCode::UNAUTHORIZED,
                Json(crate::types::VerifyChallengeResponse::InvalidSignature),
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
}

/// Reverse proxy handler that forwards requests to the target host based on the service ID
async fn reverse_proxy(
    service_id: ServiceId,
    Extension(client): Extension<HTTPClient>,
    Extension(TargetMap(target_map)): Extension<TargetMap>,
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
    let response = client
        .request(req)
        .await
        .map_err(|_| StatusCode::BAD_GATEWAY)?;

    Ok(response.into_response())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        test_client::TestClient,
        types::{ChallengeRequest, ChallengeResponse, KeyType, VerifyChallengeResponse, headers},
    };

    #[tokio::test]
    async fn auth_flow_works() {
        let mut rng = rand::thread_rng();
        let proxy =
            AuthenticatedProxy::new(ServiceId::new(0), "http://localhost:8080".parse().unwrap());

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
