use blueprint_sdk::extract::Context;
use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use blueprint_sdk::contexts::tangle::TangleClient;
use blueprint_sdk::macros::debug_job;
use tokio::sync::oneshot;
use tokio::sync::oneshot::Receiver;
use axum::{
    routing::{get, post, delete},
    Json, Router,
    http::StatusCode,
    response::IntoResponse,
    extract::Path,
    middleware,
};
use axum::http::{Request, HeaderMap};
use axum::body::Body;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct OAuthBlueprintContext {
    pub tangle_client: Arc<TangleClient>,
}

pub const WRITE_DOC_JOB_ID: u32 = 0;
pub const ADMIN_PURGE_JOB_ID: u32 = 1;

// ===== Tangle Jobs (NO AUTH - just blockchain events) =====

#[debug_job]
pub async fn write_doc(
    Context(_ctx): Context<OAuthBlueprintContext>,
    TangleArg((doc_id, content, account)): TangleArg<(String, String, String)>,
) -> TangleResult<serde_json::Value> {
    // Blockchain tx - no auth, just store data
    let store = docs_store();
    let mut guard = store.write().await;
    let entry = guard.entry(account.clone()).or_default();
    entry.insert(doc_id.clone(), content);
    
    TangleResult(serde_json::json!({
        "ok": true,
        "doc_id": doc_id,
        "account": account,
    }))
}

#[debug_job]
pub async fn admin_purge(
    Context(_ctx): Context<OAuthBlueprintContext>,
    TangleArg(target_account): TangleArg<String>,
) -> TangleResult<serde_json::Value> {
    // Admin action via blockchain - no OAuth here
    let store = docs_store();
    let mut guard = store.write().await;
    let removed = guard.remove(&target_account).is_some();
    
    TangleResult(serde_json::json!({
        "purged": removed,
        "target": target_account,
    }))
}

// ===== Off-chain OAuth Protected API Service =====

#[derive(Clone)]
pub struct OAuthProtectedApiService;

impl BackgroundService for OAuthProtectedApiService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();
        
        tokio::spawn(async move {
            let app = Router::new()
                .route("/health", get(|| async { Json("ok") }))
                .route("/docs", post(create_doc))
                .route("/docs/:id", get(read_doc))
                .route("/docs/:id", delete(delete_doc))
                .route("/docs", get(list_docs))
                .layer(middleware::from_fn(oauth_auth));
            
            let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
                .await
                .unwrap();
            
            let _ = tx.send(Ok(()));
            let _ = axum::serve(listener, app).await;
        });
        
        Ok(rx)
    }
}

#[derive(Clone)]
struct OAuthContext {
    tenant: String,
    scopes: HashSet<String>,
}

async fn oauth_auth(
    headers: HeaderMap,
    mut req: Request<Body>,
    next: middleware::Next,
) -> Result<impl IntoResponse, StatusCode> {
    let token = headers
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .and_then(|h| h.strip_prefix("Bearer "))
        .ok_or(StatusCode::UNAUTHORIZED)?;
    
    // In production: validate with OAuth provider
    // For demo: simple token parsing
    let ctx = if token.starts_with("admin_") {
        OAuthContext {
            tenant: "admin".to_string(),
            scopes: vec!["docs:read", "docs:write", "docs:admin"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
    } else {
        OAuthContext {
            tenant: token.to_string(),
            scopes: vec!["docs:read", "docs:write"]
                .into_iter()
                .map(String::from)
                .collect(),
        }
    };
    
    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}

async fn create_doc(
    axum::Extension(auth): axum::Extension<OAuthContext>,
    Json(payload): Json<HashMap<String, String>>,
) -> Result<impl IntoResponse, StatusCode> {
    if !auth.scopes.contains("docs:write") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let id = uuid::Uuid::new_v4().to_string();
    let content = payload.get("content").cloned().unwrap_or_default();
    
    let store = docs_store();
    let mut guard = store.write().await;
    let docs = guard.entry(auth.tenant.clone()).or_default();
    docs.insert(id.clone(), content);
    
    Ok(Json(serde_json::json!({"id": id, "tenant": auth.tenant})))
}

async fn read_doc(
    axum::Extension(auth): axum::Extension<OAuthContext>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    if !auth.scopes.contains("docs:read") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let store = docs_store();
    let guard = store.read().await;
    
    match guard.get(&auth.tenant).and_then(|d| d.get(&id)) {
        Some(content) => Ok(Json(serde_json::json!({"id": id, "content": content}))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

async fn delete_doc(
    axum::Extension(auth): axum::Extension<OAuthContext>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    if !auth.scopes.contains("docs:write") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let store = docs_store();
    let mut guard = store.write().await;
    
    let removed = guard.get_mut(&auth.tenant)
        .and_then(|d| d.remove(&id))
        .is_some();
    
    if removed {
        Ok(Json(serde_json::json!({"deleted": true})))
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}

async fn list_docs(
    axum::Extension(auth): axum::Extension<OAuthContext>,
) -> Result<impl IntoResponse, StatusCode> {
    if !auth.scopes.contains("docs:read") {
        return Err(StatusCode::FORBIDDEN);
    }
    
    let store = docs_store();
    let guard = store.read().await;
    
    let ids: Vec<String> = guard.get(&auth.tenant)
        .map(|d| d.keys().cloned().collect())
        .unwrap_or_default();
    
    Ok(Json(serde_json::json!({"documents": ids})))
}

// Storage
type DocMap = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;

fn docs_store() -> &'static DocMap {
    static STORE: OnceLock<DocMap> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_doc_job() {
        let result = write_doc(
            Context(OAuthBlueprintContext {
                tangle_client: Arc::new(TangleClient::default()),
            }),
            TangleArg((
                "doc1".to_string(),
                "content here".to_string(),
                "account_123".to_string(),
            )),
        ).await;
        
        assert_eq!(result.0["ok"], true);
        assert_eq!(result.0["doc_id"], "doc1");
        
        // Verify storage
        let store = docs_store();
        let guard = store.read().await;
        assert_eq!(guard.get("account_123").unwrap().get("doc1").unwrap(), "content here");
    }

    #[tokio::test]
    async fn test_admin_purge_job() {
        // First create a doc
        write_doc(
            Context(OAuthBlueprintContext {
                tangle_client: Arc::new(TangleClient::default()),
            }),
            TangleArg((
                "doc2".to_string(),
                "data".to_string(),
                "target_account".to_string(),
            )),
        ).await;
        
        // Now purge it
        let result = admin_purge(
            Context(OAuthBlueprintContext {
                tangle_client: Arc::new(TangleClient::default()),
            }),
            TangleArg("target_account".to_string()),
        ).await;
        
        assert_eq!(result.0["purged"], true);
        
        // Verify it's gone
        let store = docs_store();
        let guard = store.read().await;
        assert!(!guard.contains_key("target_account"));
    }
}