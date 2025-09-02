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

#[derive(Clone)]
pub struct OAuthBlueprintContext {
    pub tangle_client: Arc<TangleClient>,
}

pub const WHOAMI_JOB_ID: u32 = 0;
pub const CHECK_SCOPE_JOB_ID: u32 = 1;
pub const WRITE_DOC_JOB_ID: u32 = 2;
pub const READ_DOC_JOB_ID: u32 = 3;
pub const LIST_DOCS_JOB_ID: u32 = 4;
pub const ADMIN_PURGE_JOB_ID: u32 = 5;
pub const ECHO_JOB_ID: u32 = 6;

#[debug_job]
pub async fn whoami(
    Context(_ctx): Context<OAuthBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
) -> TangleResult<serde_json::Value> {
    TangleResult(serde_json::json!({
        "tenant": auth.tenant_hash,
        "scopes": auth.scopes,
        "context_present": true,
    }))
}

#[debug_job]
pub async fn check_scope(
    Context(_ctx): Context<OAuthBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
    TangleArg(scope): TangleArg<String>,
) -> TangleResult<serde_json::Value> {
    let ok = auth.has_scope(&scope);
    TangleResult(serde_json::json!({ "required": scope, "authorized": ok }))
}

#[derive(Clone)]
pub struct AuthEchoBackgroundService;

impl BackgroundService for AuthEchoBackgroundService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        tokio::spawn(async move {
            // Example generic server that could be used for health or metrics
            let app = Router::new().route("/health", get(|| async { Json("ok") }));
            let listener = tokio::net::TcpListener::bind((std::net::Ipv4Addr::LOCALHOST, 0))
                .await
                .unwrap();
            let _ = tx.send(Ok(()));
            let _ = axum::serve(listener, app).await;
        });
        Ok(rx)
    }
}

// In-memory tenant-scoped document store
type DocMap = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;

fn docs_store() -> &'static DocMap {
    static STORE: OnceLock<DocMap> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

#[debug_job]
pub async fn write_doc(
    Context(_ctx): Context<OAuthBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
    TangleArg((doc_id, content)): TangleArg<(String, String)>,
) -> TangleResult<serde_json::Value> {
    // require docs:write
    if !auth.has_scope("docs:write") {
        return TangleResult(serde_json::json!({"error":"missing_scopes","required":"docs:write"}));
    }
    let tenant = auth.tenant_hash.unwrap_or_default();
    let store = docs_store();
    let mut guard = store.write().await;
    let entry = guard.entry(tenant.clone()).or_default();
    entry.insert(doc_id.clone(), content.clone());
    TangleResult(serde_json::json!({"ok":true,"tenant":tenant,"doc_id":doc_id}))
}

#[debug_job]
pub async fn read_doc(
    Context(_ctx): Context<OAuthBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
    TangleArg(doc_id): TangleArg<String>,
) -> TangleResult<serde_json::Value> {
    if !auth.has_scope("docs:read") {
        return TangleResult(serde_json::json!({"error":"missing_scopes","required":"docs:read"}));
    }
    let tenant = auth.tenant_hash.unwrap_or_default();
    let store = docs_store();
    let guard = store.read().await;
    let content = guard
        .get(&tenant)
        .and_then(|m| m.get(&doc_id).cloned());
    TangleResult(serde_json::json!({"tenant":tenant,"doc_id":doc_id,"content":content}))
}

#[debug_job]
pub async fn list_docs(
    Context(_ctx): Context<OAuthBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
) -> TangleResult<serde_json::Value> {
    if !auth.has_scope("docs:read") {
        return TangleResult(serde_json::json!({"error":"missing_scopes","required":"docs:read"}));
    }
    let tenant = auth.tenant_hash.unwrap_or_default();
    let store = docs_store();
    let guard = store.read().await;
    let ids: Vec<String> = guard
        .get(&tenant)
        .map(|m| m.keys().cloned().collect())
        .unwrap_or_default();
    TangleResult(serde_json::json!({"tenant":tenant,"doc_ids":ids}))
}

#[debug_job]
pub async fn admin_purge(
    Context(_ctx): Context<OAuthBlueprintContext>,
    Extension(auth): Extension<AuthContext>,
    TangleArg(target_tenant): TangleArg<String>,
) -> TangleResult<serde_json::Value> {
    if !auth.has_scope("docs:admin") {
        return TangleResult(serde_json::json!({"error":"missing_scopes","required":"docs:admin"}));
    }
    let store = docs_store();
    let mut guard = store.write().await;
    let removed = guard.remove(&target_tenant).is_some();
    TangleResult(serde_json::json!({"purged":removed,"tenant":target_tenant}))
}

pub async fn echo(TangleArg(s): TangleArg<String>) -> TangleResult<String> {
    TangleResult(s)
}

mod tests;
mod state_validation_tests;
