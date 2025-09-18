use axum::body::Body;
use axum::http::{HeaderMap, Request};
use axum::{
    Json, Router,
    extract::Path,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{get, post},
};
use blueprint_sdk::contexts::tangle::TangleClient;
use blueprint_sdk::extract::Context;
use blueprint_sdk::macros::debug_job;
use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;

pub const WRITE_RESOURCE_JOB_ID: u32 = 0;
pub const PURCHASE_API_KEY_JOB_ID: u32 = 1;

#[derive(Clone)]
pub struct ApiKeyBlueprintContext {
    pub tangle_client: Arc<TangleClient>,
}

// ===== Tangle Jobs (NO AUTH - just blockchain events) =====

#[debug_job]
pub async fn write_resource(
    Context(_ctx): Context<ApiKeyBlueprintContext>,
    TangleArg((resource_id, data, account)): TangleArg<(String, String, String)>,
) -> TangleResult<serde_json::Value> {
    // This is triggered by blockchain tx - no auth needed
    // The account comes from the transaction data

    let store = resource_store();
    let mut guard = store.write().await;
    let entry = guard.entry(account.clone()).or_default();
    entry.insert(resource_id.clone(), data.clone());

    TangleResult(serde_json::json!({
        "ok": true,
        "resource_id": resource_id,
        "account": account,
    }))
}

#[debug_job]
pub async fn purchase_api_key(
    Context(_ctx): Context<ApiKeyBlueprintContext>,
    TangleArg((tier, account)): TangleArg<(String, String)>,
) -> TangleResult<serde_json::Value> {
    // Generate API key for the blockchain account
    let api_key = format!("sk_{}_{}", tier, uuid::Uuid::new_v4());

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let api_key_hash = format!("{:x}", hasher.finalize());

    let store = api_key_store();
    let mut guard = store.write().await;
    guard.insert(
        api_key_hash.clone(),
        serde_json::json!({
            "tier": tier,
            "account": account,
            "active": true,
        }),
    );

    TangleResult(serde_json::json!({
        "ok": true,
        "api_key_hash": api_key_hash,
        "tier": tier,
    }))
}

// ===== Off-chain API Service (WITH AUTH) =====

#[derive(Clone)]
pub struct ApiKeyProtectedService;

impl BackgroundService for ApiKeyProtectedService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let app = Router::new()
                .route("/health", get(|| async { Json("ok") }))
                .route("/api/resources", post(create_resource))
                .route("/api/resources/:id", get(get_resource))
                .layer(middleware::from_fn(api_auth));

            let listener = tokio::net::TcpListener::bind("127.0.0.1:8081")
                .await
                .unwrap();

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
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let hash = format!("{:x}", hasher.finalize());

    let store = api_key_store();
    let guard = store.read().await;
    let data = guard.get(&hash).ok_or(StatusCode::UNAUTHORIZED)?;

    let auth = ApiKeyAuth {
        account: data["account"].as_str().unwrap_or("").to_string(),
    };

    req.extensions_mut().insert(auth);
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
    let resources = guard.entry(auth.account.clone()).or_default();
    resources.insert(id.clone(), data);

    Json(serde_json::json!({"id": id, "account": auth.account}))
}

async fn get_resource(
    axum::Extension(auth): axum::Extension<ApiKeyAuth>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let store = resource_store();
    let guard = store.read().await;

    match guard.get(&auth.account).and_then(|r| r.get(&id)) {
        Some(data) => Json(serde_json::json!({"id": id, "data": data})),
        None => Json(serde_json::json!({"error": "not found"})),
    }
}

// Storage
type ResourceMap = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;
type ApiKeyMap = Arc<RwLock<HashMap<String, serde_json::Value>>>;

fn resource_store() -> &'static ResourceMap {
    static STORE: OnceLock<ResourceMap> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

fn api_key_store() -> &'static ApiKeyMap {
    static STORE: OnceLock<ApiKeyMap> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_purchase_api_key_job() {
        // Test the job directly - no auth needed
        let result = purchase_api_key(
            Context(ApiKeyBlueprintContext {
                tangle_client: Arc::new(TangleClient::default()),
            }),
            TangleArg(("premium".to_string(), "0x123".to_string())),
        )
        .await;

        assert_eq!(result.0["ok"], true);
        assert_eq!(result.0["tier"], "premium");
        assert!(
            result.0["api_key"]
                .as_str()
                .unwrap()
                .starts_with("sk_premium_")
        );
    }

    #[tokio::test]
    async fn test_write_resource_job() {
        let result = write_resource(
            Context(ApiKeyBlueprintContext {
                tangle_client: Arc::new(TangleClient::default()),
            }),
            TangleArg((
                "resource_1".to_string(),
                "test data".to_string(),
                "0x456".to_string(),
            )),
        )
        .await;

        assert_eq!(result.0["ok"], true);
        assert_eq!(result.0["resource_id"], "resource_1");
        assert_eq!(result.0["account"], "0x456");

        // Verify it was stored
        let store = resource_store();
        let guard = store.read().await;
        assert_eq!(
            guard.get("0x456").unwrap().get("resource_1").unwrap(),
            "test data"
        );
    }

    #[tokio::test]
    async fn test_api_service_auth() {
        // First create an API key
        let _ = purchase_api_key(
            Context(ApiKeyBlueprintContext {
                tangle_client: Arc::new(TangleClient::default()),
            }),
            TangleArg(("basic".to_string(), "test_account".to_string())),
        )
        .await;

        // Now test that the API key works in the service
        // (In a real test, we'd make HTTP requests to the service)
        let store = api_key_store();
        let guard = store.read().await;
        assert!(!guard.is_empty());
    }
}
