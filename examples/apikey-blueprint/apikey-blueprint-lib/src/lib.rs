use alloy_primitives::Address;
use alloy_sol_types::sol;
use axum::body::Body;
use axum::http::{HeaderMap, Request};
use axum::{
    Json, Router as HttpRouter,
    extract::Path,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use blueprint_sdk::macros::debug_job;
use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle_evm::TangleEvmLayer;
use blueprint_sdk::tangle_evm::extract::{TangleEvmArg, TangleEvmResult};
use blueprint_sdk::{Job, Router as BlueprintRouter};
use serde_json::json;
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::sync::oneshot::{self, Receiver};

pub const WRITE_RESOURCE_JOB_ID: u8 = 0;
pub const PURCHASE_API_KEY_JOB_ID: u8 = 1;

sol! {
    struct WriteResourceResult {
        bool ok;
        string resourceId;
        string account;
    }

    struct PurchaseApiKeyResult {
        bool ok;
        string apiKeyHash;
    }
}

#[debug_job]
pub async fn write_resource(
    TangleEvmArg((resource_id, data, account)): TangleEvmArg<(String, String, Address)>,
) -> TangleEvmResult<WriteResourceResult> {
    let account_hex = format_address(account);
    let store = resource_store();
    let mut guard = store.write().await;
    guard
        .entry(account_hex.clone())
        .or_default()
        .insert(resource_id.clone(), data.clone());

    TangleEvmResult(WriteResourceResult {
        ok: true,
        resourceId: resource_id,
        account: account_hex,
    })
}

#[debug_job]
pub async fn purchase_api_key(
    TangleEvmArg((tier, account)): TangleEvmArg<(String, Address)>,
) -> TangleEvmResult<PurchaseApiKeyResult> {
    let api_key = format!("sk_{}_{}", tier, uuid::Uuid::new_v4());

    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    let account_hex = format_address(account);
    let store = api_key_store();
    let mut guard = store.write().await;
    guard.insert(
        hash.clone(),
        json!({
            "tier": tier,
            "account": account_hex,
            "active": true,
        }),
    );

    TangleEvmResult(PurchaseApiKeyResult {
        ok: true,
        apiKeyHash: hash,
    })
}

/// Router wiring the API key jobs onto the runner.
#[must_use]
pub fn router() -> BlueprintRouter {
    BlueprintRouter::new()
        .route(WRITE_RESOURCE_JOB_ID, write_resource.layer(TangleEvmLayer))
        .route(
            PURCHASE_API_KEY_JOB_ID,
            purchase_api_key.layer(TangleEvmLayer),
        )
}

#[derive(Clone)]
pub struct ApiKeyProtectedService;

impl BackgroundService for ApiKeyProtectedService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let app = HttpRouter::new()
                .route("/health", get(|| async { Json("ok") }))
                .route("/api/resources", post(create_resource))
                .route("/api/resources/{id}", get(get_resource))
                .layer(middleware::from_fn(api_auth));

            let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
                .await
                .expect("failed to bind listener");

            let _ = tx.send(Ok(()));
            let _ = axum::serve(listener, app).await;
        });

        Ok(rx)
    }
}

#[derive(Clone)]
struct ApiKeyAuth {
    account: String,
}

async fn api_auth(
    headers: HeaderMap,
    mut req: Request<Body>,
    next: middleware::Next,
) -> Result<impl IntoResponse, StatusCode> {
    let api_key = headers
        .get("X-API-Key")
        .and_then(|v| v.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    let store = api_key_store();
    let guard = store.read().await;
    let entry = guard.get(&hash).ok_or(StatusCode::UNAUTHORIZED)?;

    req.extensions_mut().insert(ApiKeyAuth {
        account: entry["account"].as_str().unwrap_or_default().to_string(),
    });

    Ok(next.run(req).await)
}

async fn create_resource(
    axum::Extension(auth): axum::Extension<ApiKeyAuth>,
    Json(payload): Json<HashMap<String, String>>,
) -> impl IntoResponse {
    let id = uuid::Uuid::new_v4().to_string();
    let data = payload.get("data").cloned().unwrap_or_default();

    let store = resource_store();
    let mut guard = store.write().await;
    guard
        .entry(auth.account.clone())
        .or_default()
        .insert(id.clone(), data);

    Json(json!({ "id": id, "account": auth.account }))
}

async fn get_resource(
    axum::Extension(auth): axum::Extension<ApiKeyAuth>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let store = resource_store();
    let guard = store.read().await;
    guard
        .get(&auth.account)
        .and_then(|r| r.get(&id))
        .map(|data| Json(json!({ "id": id, "data": data })))
        .ok_or(StatusCode::NOT_FOUND)
}

type ResourceStore = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;
type ApiKeyStore = Arc<RwLock<HashMap<String, serde_json::Value>>>;

fn resource_store() -> &'static ResourceStore {
    static STORE: OnceLock<ResourceStore> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

fn api_key_store() -> &'static ApiKeyStore {
    static STORE: OnceLock<ApiKeyStore> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

fn format_address(address: Address) -> String {
    format!("{address:#x}")
}

/// Produce a demo TLV payload describing how operators can request API keys.
#[must_use]
pub fn registration_payload() -> Vec<u8> {
    const ENDPOINT_TLV_TYPE: u8 = 0x01;
    const CONTACT_TLV_TYPE: u8 = 0x02;
    const CONTACT: &str = "ops@tangle.tools";
    const ENDPOINT: &str = "https://operator.example.com/api-keys/register";

    fn write_tlv(buffer: &mut Vec<u8>, tlv_type: u8, value: &str) {
        buffer.push(tlv_type);
        let len: u16 = value.len().try_into().unwrap_or(u16::MAX);
        buffer.extend_from_slice(&len.to_be_bytes());
        buffer.extend_from_slice(value.as_bytes());
    }

    let mut payload = Vec::with_capacity(ENDPOINT.len() + CONTACT.len() + 8);
    write_tlv(&mut payload, ENDPOINT_TLV_TYPE, ENDPOINT);
    write_tlv(&mut payload, CONTACT_TLV_TYPE, CONTACT);
    payload
}

/// Clear in-memory stores between tests/integration runs.
pub async fn reset_state_for_tests() {
    resource_store().write().await.clear();
    api_key_store().write().await.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy_primitives::Address;
    use axum::http::HeaderValue;
    use axum::routing::get;
    use tower::ServiceExt;

    #[tokio::test]
    async fn purchase_and_use_api_key() {
        api_key_store().write().await.clear();
        let result = purchase_api_key(TangleEvmArg(("pro".into(), Address::ZERO))).await;
        assert!(result.ok);
        let api_key_hash = result.apiKeyHash.clone();

        let store = api_key_store();
        let guard = store.read().await;
        assert!(guard.contains_key(&api_key_hash));
    }

    #[tokio::test]
    async fn write_resource_records_data() {
        resource_store().write().await.clear();
        write_resource(TangleEvmArg((
            "doc".into(),
            "payload".into(),
            Address::ZERO,
        )))
        .await;

        let guard = resource_store().read().await;
        assert_eq!(
            guard
                .get(&format_address(Address::ZERO))
                .unwrap()
                .get("doc")
                .unwrap(),
            "payload"
        );
    }

    #[tokio::test]
    async fn middleware_rejects_missing_key() {
        api_key_store().write().await.clear();
        let app = HttpRouter::new()
            .route(
                "/",
                get(
                    |axum::Extension(auth): axum::Extension<ApiKeyAuth>| async move {
                        Json(json!({ "account": auth.account }))
                    },
                ),
            )
            .layer(middleware::from_fn(api_auth));

        let response = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn middleware_accepts_valid_key() {
        api_key_store().write().await.clear();
        let api_key = "sk_test_valid";
        let mut hasher = Sha256::new();
        hasher.update(api_key.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        api_key_store()
            .write()
            .await
            .insert(hash.clone(), json!({ "account": "acct", "tier": "tier" }));

        let app = HttpRouter::new()
            .route(
                "/",
                get(
                    |axum::Extension(auth): axum::Extension<ApiKeyAuth>| async move {
                        Json(json!({ "account": auth.account }))
                    },
                ),
            )
            .layer(middleware::from_fn(api_auth));

        let response = app
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header("X-API-Key", HeaderValue::from_static(api_key))
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(response.status(), StatusCode::OK);
    }
}
