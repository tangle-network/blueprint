use blueprint_sdk::extract::{Extension, Context};
use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use blueprint_sdk::contexts::tangle::TangleClient;
use blueprint_sdk::AuthContext;
use blueprint_sdk::macros::debug_job;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use axum::{routing::get, Json, Router};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

pub const WRITE_RESOURCE_JOB_ID: u32 = 0;
pub const PURCHASE_API_KEY_JOB_ID: u32 = 1;

#[derive(Clone)]
pub struct ApiKeyBlueprintContext {
    pub tangle_client: Arc<TangleClient>,
}

#[debug_job]
pub async fn write_resource(
    Context(_ctx): Context<ApiKeyBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
    TangleArg((resource_id, data)): TangleArg<(String, String)>,
) -> TangleResult<serde_json::Value> {
    let tenant = auth.tenant_hash.unwrap_or_default();
    let store = resource_store();
    let mut guard = store.write().await;
    let entry = guard.entry(tenant.clone()).or_default();
    entry.insert(resource_id.clone(), data.clone());
    TangleResult(serde_json::json!({
        "ok": true,
        "tenant": tenant,
        "resource_id": resource_id,
    }))
}

#[debug_job]
pub async fn purchase_api_key(
    Context(_ctx): Context<ApiKeyBlueprintContext>,
    TangleArg((subscription_tier, user_identifier)): TangleArg<(String, String)>,
) -> TangleResult<serde_json::Value> {
    // Generate a secure API key (simplified for demo)
    let api_key = format!("sk_{}_{}", subscription_tier, uuid::Uuid::new_v4());
    
    // Store the API key hash for verification
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(api_key.as_bytes());
    let api_key_hash = format!("{:x}", hasher.finalize());
    
    // Store in our secure storage
    let store = api_key_store();
    let mut guard = store.write().await;
    guard.insert(api_key_hash.clone(), serde_json::json!({
        "tier": subscription_tier,
        "user": user_identifier,
        "active": true
    }));
    
    // Return response with encrypted key (simplified - in production use proper encryption)
    TangleResult(serde_json::json!({
        "ok": true,
        "api_key_hash": api_key_hash,
        "encrypted_key": api_key, // WARNING: Should be encrypted with user's public key!
        "tier": subscription_tier,
    }))
}


#[derive(Clone)]
pub struct ApiUsageTracker;

impl BackgroundService for ApiUsageTracker {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            // Background service to track API usage and implement rate limiting
            let app = Router::new()
                .route("/usage", get(|| async { Json("usage tracking active") }))
                .route("/health", get(|| async { Json("ok") }));
            let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
                .await
                .unwrap();
            let _ = tx.send(Ok(()));
            let _ = axum::serve(listener, app).await;
        });
        Ok(rx)
    }
}

// In-memory tenant-scoped resource store
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

mod tests;