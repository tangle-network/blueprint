//! Exercise public auth APIs through real HTTP sockets and real EIP-191 signatures.
use axum::{
    Json, Router,
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    routing::{get, post},
};
use blueprint_auth::{
    owner::{
        OwnerAuthError, OwnerChallenge, OwnerDomain, OwnerGrant, OwnerPredicate, ResourceScope,
        ScopedOwnerAuth,
    },
    paseto_tokens::{PasetoKey, PasetoTokenManager},
    types::ServiceId,
};
use k256::ecdsa::SigningKey;
use serde::Deserialize;
use serde_json::{Value, json};
use std::{
    sync::{Arc, Mutex},
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use tiny_keccak::{Hasher, Keccak};

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}
fn hash(bytes: &[u8]) -> [u8; 32] {
    let mut out = [0; 32];
    let mut h = Keccak::v256();
    h.update(bytes);
    h.finalize(&mut out);
    out
}
fn address(key: &SigningKey) -> [u8; 20] {
    hash(&key.verifying_key().to_encoded_point(false).as_bytes()[1..])[12..]
        .try_into()
        .unwrap()
}
fn sign(key: &SigningKey, message: &str) -> Vec<u8> {
    let prefix = format!("\x19Ethereum Signed Message:\n{}", message.len());
    let (signature, recovery) = key
        .sign_prehash_recoverable(&hash(&[prefix.as_bytes(), message.as_bytes()].concat()))
        .unwrap();
    let mut bytes = signature.to_bytes().to_vec();
    bytes.push(recovery.to_byte() + 27);
    bytes
}
fn domain() -> OwnerDomain {
    OwnerDomain {
        audience: "gpu-lease".into(),
        chain_id: Some(31337),
        operator: Some([7; 20]),
        contract: Some([8; 20]),
    }
}
fn scope() -> ResourceScope {
    ResourceScope {
        service_id: ServiceId::new(11),
        resource: "lease-a".into(),
    }
}
fn manager() -> PasetoTokenManager {
    PasetoTokenManager::with_key(PasetoKey::from_bytes([9; 32]), Duration::from_secs(60))
}
fn error(error: OwnerAuthError) -> StatusCode {
    match error {
        OwnerAuthError::InvalidRequest => StatusCode::BAD_REQUEST,
        OwnerAuthError::InvalidChallenge | OwnerAuthError::InvalidSignature => {
            StatusCode::UNAUTHORIZED
        }
        OwnerAuthError::Unauthorized => StatusCode::FORBIDDEN,
        OwnerAuthError::Capacity => StatusCode::TOO_MANY_REQUESTS,
        OwnerAuthError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
    }
}
#[derive(Deserialize)]
struct ChallengeInput {
    resource: ResourceScope,
    owner: [u8; 20],
}
#[derive(Deserialize)]
struct VerifyInput {
    nonce: [u8; 32],
    signature: Vec<u8>,
}
fn router<P: OwnerPredicate + 'static>(auth: Arc<ScopedOwnerAuth<P>>) -> Router {
    Router::new()
        .route("/challenge",post(|State(auth):State<Arc<ScopedOwnerAuth<P>>>,Json(input):Json<ChallengeInput>| async move {
            auth.challenge(input.resource,input.owner).await.map(Json).map_err(error)
        }))
        .route("/verify",post(|State(auth):State<Arc<ScopedOwnerAuth<P>>>,Json(input):Json<VerifyInput>| async move {
            auth.verify(&input.nonce,&input.signature).await.map(Json).map_err(error)
        }))
        .route("/access/{service}/{resource}",get(|State(auth):State<Arc<ScopedOwnerAuth<P>>>,Path((service,resource)):Path<(u64,String)>,headers:HeaderMap| async move {
            let token=headers.get("authorization").and_then(|v|v.to_str().ok()).and_then(|v|v.strip_prefix("Bearer ")).ok_or(StatusCode::UNAUTHORIZED)?;
            auth.authorize(token,&ResourceScope{service_id:ServiceId::new(service),resource},"gpu:attach").await.map(|_|StatusCode::NO_CONTENT).map_err(error)
        }))
        .with_state(auth)
}
struct Server {
    url: String,
    task: tokio::task::JoinHandle<()>,
}
impl Drop for Server {
    fn drop(&mut self) {
        self.task.abort();
    }
}
async fn serve(router: Router) -> Server {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let url = format!("http://{}", listener.local_addr().unwrap());
    let task = tokio::spawn(async move {
        axum::serve(listener, router).await.unwrap();
    });
    Server { url, task }
}
#[derive(Clone)]
struct LeaseOwner {
    owner: [u8; 20],
    grant: Arc<Mutex<Option<(u64, Vec<String>)>>>,
}
impl OwnerPredicate for LeaseOwner {
    async fn authorize(
        &self,
        resource: &ResourceScope,
        owner: &[u8; 20],
    ) -> Result<OwnerGrant, OwnerAuthError> {
        if resource != &scope() || owner != &self.owner {
            return Err(OwnerAuthError::Unauthorized);
        }
        let (expires_at, scopes) = self
            .grant
            .lock()
            .unwrap()
            .clone()
            .ok_or(OwnerAuthError::Unauthorized)?;
        Ok(OwnerGrant { expires_at, scopes })
    }
}
async fn challenge(client: &reqwest::Client, server: &Server, key: &SigningKey) -> OwnerChallenge {
    client
        .post(format!("{}/challenge", server.url))
        .json(&json!({"resource":scope(),"owner":address(key)}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap()
}
async fn access(
    client: &reqwest::Client,
    server: &Server,
    token: &str,
    service: u64,
    resource: &str,
) -> StatusCode {
    client
        .get(format!("{}/access/{service}/{resource}", server.url))
        .bearer_auth(token)
        .send()
        .await
        .unwrap()
        .status()
}

#[tokio::test]
async fn scoped_owner_sessions_bind_domain_resource_and_live_policy() {
    let client = reqwest::Client::new();
    let key = SigningKey::from_slice(&[3; 32]).unwrap();
    let policy = LeaseOwner {
        owner: address(&key),
        grant: Arc::new(Mutex::new(Some((now() + 120, vec!["gpu:attach".into()])))),
    };
    let auth = Arc::new(ScopedOwnerAuth::with_owner(manager(), domain(), policy.clone()).unwrap());
    let server = serve(router(auth.clone())).await;
    let nonowner = SigningKey::from_slice(&[4; 32]).unwrap();
    assert_eq!(
        client
            .post(format!("{}/challenge", server.url))
            .json(&json!({"resource":scope(),"owner":address(&nonowner)}))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN
    );
    let request = challenge(&client, &server, &key).await;
    let verify = |signature: Vec<u8>| {
        client
            .post(format!("{}/verify", server.url))
            .json(&json!({"nonce":request.nonce,"signature":signature}))
            .send()
    };
    assert_eq!(
        verify(sign(&nonowner, &request.message))
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
    let signature = sign(&key, &request.message);
    let (first, second) = tokio::join!(verify(signature.clone()), verify(signature));
    let mut replies = vec![first.unwrap(), second.unwrap()];
    assert_eq!(
        replies
            .iter()
            .filter(|reply| reply.status() == StatusCode::OK)
            .count(),
        1
    );
    assert_eq!(
        replies
            .iter()
            .filter(|reply| reply.status() == StatusCode::UNAUTHORIZED)
            .count(),
        1
    );
    let session: Value = replies
        .swap_remove(
            replies
                .iter()
                .position(|r| r.status() == StatusCode::OK)
                .unwrap(),
        )
        .json()
        .await
        .unwrap();
    let token = session["token"].as_str().unwrap();
    assert!(
        session["expires_at"].as_u64().unwrap() <= policy.grant.lock().unwrap().as_ref().unwrap().0
    );
    assert_eq!(
        access(&client, &server, token, 11, "lease-a").await,
        StatusCode::NO_CONTENT
    );
    assert_eq!(
        access(&client, &server, token, 11, "lease-b").await,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        access(&client, &server, token, 12, "lease-a").await,
        StatusCode::FORBIDDEN
    );
    for field in 0..4 {
        let mut changed = domain();
        match field {
            0 => changed.audience = "other".into(),
            1 => changed.chain_id = Some(1),
            2 => changed.operator = Some([6; 20]),
            _ => changed.contract = Some([5; 20]),
        }
        let other = serve(router(Arc::new(
            ScopedOwnerAuth::with_owner(manager(), changed, policy.clone()).unwrap(),
        )))
        .await;
        assert_eq!(
            access(&client, &other, token, 11, "lease-a").await,
            StatusCode::FORBIDDEN
        );
    }
    *policy.grant.lock().unwrap() = Some((now() + 120, vec!["gpu:exec".into()]));
    assert_eq!(
        access(&client, &server, token, 11, "lease-a").await,
        StatusCode::FORBIDDEN
    );
    *policy.grant.lock().unwrap() = None;
    assert_eq!(
        access(&client, &server, token, 11, "lease-a").await,
        StatusCode::FORBIDDEN
    );
    *policy.grant.lock().unwrap() = Some((now() + 1, vec!["gpu:attach".into()]));
    let short = challenge(&client, &server, &key).await;
    let short_session: Value = client
        .post(format!("{}/verify", server.url))
        .json(&json!({"nonce":short.nonce,"signature":sign(&key,&short.message)}))
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let request = challenge(&client, &server, &key).await;
    tokio::time::sleep(Duration::from_secs(2)).await;
    assert_eq!(
        access(
            &client,
            &server,
            short_session["token"].as_str().unwrap(),
            11,
            "lease-a"
        )
        .await,
        StatusCode::FORBIDDEN
    );
    assert_eq!(
        client
            .post(format!("{}/verify", server.url))
            .json(&json!({"nonce":request.nonce,"signature":sign(&key,&request.message)}))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::UNAUTHORIZED
    );
    assert_eq!(
        access(&client, &server, token, 11, "lease-a").await,
        StatusCode::FORBIDDEN
    );
}

#[tokio::test]
async fn registered_service_owners_and_legacy_participant_routes_remain_available() {
    use blueprint_auth::{
        models::ServiceModel,
        proxy::AuthenticatedProxy,
        types::{ChallengeRequest, KeyType, VerifyChallengeRequest},
    };
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let temp = tempfile::tempdir().unwrap();
    let proxy = AuthenticatedProxy::new(temp.path()).unwrap();
    let db = proxy.db();
    let key = SigningKey::from_slice(&[3; 32]).unwrap();
    let public = key.verifying_key().to_sec1_bytes().to_vec();
    let mut service = ServiceModel::new("test_", "http://127.0.0.1:1");
    service.add_owner(KeyType::Ecdsa, public.clone());
    service.save(scope().service_id, &db).unwrap();
    let auth = ScopedOwnerAuth::new(manager(), domain(), db.clone()).unwrap();
    let server = serve(router(Arc::new(auth)).merge(proxy.router())).await;
    let client = reqwest::Client::new();
    let request = challenge(&client, &server, &key).await;
    assert_eq!(
        client
            .post(format!("{}/verify", server.url))
            .json(&json!({"nonce":request.nonce,"signature":sign(&key,&request.message)}))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::OK
    );
    let challenge_request = ChallengeRequest {
        pub_key: public.clone(),
        key_type: KeyType::Ecdsa,
    };
    let legacy: blueprint_auth::types::ChallengeResponse = client
        .post(format!("{}/v1/auth/challenge", server.url))
        .header("X-Service-Id", "11")
        .json(&challenge_request)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json()
        .await
        .unwrap();
    let signature = key
        .sign_prehash_recoverable(&legacy.challenge)
        .unwrap()
        .0
        .to_bytes()
        .into();
    assert_eq!(
        client
            .post(format!("{}/v1/auth/verify", server.url))
            .header("X-Service-Id", "11")
            .json(&VerifyChallengeRequest {
                challenge_request,
                challenge: legacy.challenge,
                signature,
                expires_at: 0,
                additional_headers: Default::default()
            })
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::CREATED
    );
    service.remove_owner(KeyType::Ecdsa, &public);
    service.save(scope().service_id, &db).unwrap();
    assert_eq!(
        client
            .post(format!("{}/challenge", server.url))
            .json(&json!({"resource":scope(),"owner":address(&key)}))
            .send()
            .await
            .unwrap()
            .status(),
        StatusCode::FORBIDDEN
    );
}
