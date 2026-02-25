use alloy_sol_types::sol;
use axum::body::Body;
use axum::http::{HeaderMap, Request};
use axum::{
    Json, Router as HttpRouter,
    extract::Path,
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::{delete, get, post},
};
use blueprint_sdk::macros::debug_job;
use blueprint_sdk::runner::BackgroundService;
use blueprint_sdk::runner::error::RunnerError;
use blueprint_sdk::tangle::TangleLayer;
use blueprint_sdk::tangle::extract::{TangleArg, TangleResult};
use blueprint_sdk::{Job, Router as BlueprintRouter};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::sync::oneshot::{self, Receiver};

pub const WRITE_DOC_JOB_ID: u8 = 0;
pub const ADMIN_PURGE_JOB_ID: u8 = 1;

sol! {
    struct WriteDocResult {
        bool ok;
        string docId;
        string account;
    }

    struct AdminPurgeResult {
        bool purged;
        string target;
    }
}

// ===== On-chain jobs (triggered by contract events) =====

#[debug_job]
pub async fn write_doc(
    TangleArg((doc_id, content, account)): TangleArg<(String, String, String)>,
) -> TangleResult<WriteDocResult> {
    let store = docs_store();
    let mut guard = store.write().await;
    let entry = guard.entry(account.clone()).or_default();
    entry.insert(doc_id.clone(), content);

    TangleResult(WriteDocResult {
        ok: true,
        docId: doc_id,
        account,
    })
}

#[debug_job]
pub async fn admin_purge(
    TangleArg((target_account,)): TangleArg<(String,)>,
) -> TangleResult<AdminPurgeResult> {
    let store = docs_store();
    let mut guard = store.write().await;
    let removed = guard.remove(&target_account).is_some();

    TangleResult(AdminPurgeResult {
        purged: removed,
        target: target_account,
    })
}

/// Router wiring the OAuth jobs for reuse across binaries/tests.
#[must_use]
pub fn router() -> BlueprintRouter {
    BlueprintRouter::new()
        .route(WRITE_DOC_JOB_ID, write_doc.layer(TangleLayer))
        .route(ADMIN_PURGE_JOB_ID, admin_purge.layer(TangleLayer))
}

// ===== Off-chain OAuth-protected HTTP API =====

#[derive(Clone)]
pub struct OAuthProtectedApiService;

impl BackgroundService for OAuthProtectedApiService {
    async fn start(&self) -> Result<Receiver<Result<(), RunnerError>>, RunnerError> {
        let (tx, rx) = oneshot::channel();

        tokio::spawn(async move {
            let app = HttpRouter::new()
                .route("/health", get(|| async { Json("ok") }))
                .route("/docs", post(create_doc))
                .route("/docs/{id}", get(read_doc))
                .route("/docs/{id}", delete(delete_doc))
                .route("/docs", get(list_docs))
                .layer(middleware::from_fn(oauth_auth));

            let listener = tokio::net::TcpListener::bind("127.0.0.1:8080")
                .await
                .expect("failed to bind listener");

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

    let ctx = if token.starts_with("admin_") {
        OAuthContext {
            tenant: "admin".into(),
            scopes: HashSet::from([
                "docs:read".to_string(),
                "docs:write".to_string(),
                "docs:admin".to_string(),
            ]),
        }
    } else {
        OAuthContext {
            tenant: token.to_string(),
            scopes: HashSet::from(["docs:read".to_string(), "docs:write".to_string()]),
        }
    };

    req.extensions_mut().insert(ctx);
    Ok(next.run(req).await)
}

async fn create_doc(
    axum::Extension(auth): axum::Extension<OAuthContext>,
    Json(payload): Json<HashMap<String, String>>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !auth.scopes.contains("docs:write") {
        return Err(StatusCode::FORBIDDEN);
    }

    let id = uuid::Uuid::new_v4().to_string();
    let content = payload.get("content").cloned().unwrap_or_default();

    let store = docs_store();
    let mut guard = store.write().await;
    guard
        .entry(auth.tenant.clone())
        .or_default()
        .insert(id.clone(), content);

    Ok(Json(serde_json::json!({ "id": id, "tenant": auth.tenant })))
}

async fn read_doc(
    axum::Extension(auth): axum::Extension<OAuthContext>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !auth.scopes.contains("docs:read") {
        return Err(StatusCode::FORBIDDEN);
    }

    let store = docs_store();
    let guard = store.read().await;
    guard
        .get(&auth.tenant)
        .and_then(|docs| docs.get(&id))
        .map(|content| Json(serde_json::json!({ "id": id, "content": content })))
        .ok_or(StatusCode::NOT_FOUND)
}

async fn delete_doc(
    axum::Extension(auth): axum::Extension<OAuthContext>,
    Path(id): Path<String>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    if !auth.scopes.contains("docs:write") {
        return Err(StatusCode::FORBIDDEN);
    }

    let store = docs_store();
    let mut guard = store.write().await;
    let removed = guard
        .get_mut(&auth.tenant)
        .and_then(|docs| docs.remove(&id))
        .is_some();

    Ok(Json(serde_json::json!({ "deleted": removed })))
}

async fn list_docs(
    axum::Extension(auth): axum::Extension<OAuthContext>,
) -> Result<impl IntoResponse, StatusCode> {
    if !auth.scopes.contains("docs:read") {
        return Err(StatusCode::FORBIDDEN);
    }

    let store = docs_store();
    let guard = store.read().await;
    let docs = guard
        .get(&auth.tenant)
        .map(|d| d.keys().cloned().collect())
        .unwrap_or_else(Vec::new);

    Ok(Json(serde_json::json!({ "documents": docs })))
}

type DocStore = Arc<RwLock<HashMap<String, HashMap<String, String>>>>;

fn docs_store() -> &'static DocStore {
    static STORE: OnceLock<DocStore> = OnceLock::new();
    STORE.get_or_init(|| Arc::new(RwLock::new(HashMap::new())))
}

/// Clear the in-memory document store before running integration tests.
pub async fn reset_state_for_tests() {
    docs_store().write().await.clear();
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::http::HeaderValue;
    use axum::routing::get;
    use tokio::time::{Duration, timeout};
    use tower::ServiceExt;

    #[tokio::test]
    async fn background_service_starts() {
        let service = OAuthProtectedApiService;
        let signal = service.start().await.unwrap();
        assert!(
            timeout(Duration::from_secs(1), signal)
                .await
                .unwrap()
                .unwrap()
                .is_ok()
        );
    }

    #[tokio::test]
    async fn oauth_middleware_accepts_and_rejects_tokens() {
        let app = HttpRouter::new()
            .route(
                "/",
                get(
                    |axum::Extension(ctx): axum::Extension<OAuthContext>| async move {
                        Json(serde_json::json!({ "tenant": ctx.tenant }))
                    },
                ),
            )
            .layer(middleware::from_fn(oauth_auth));

        let authed = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/")
                    .header(
                        "Authorization",
                        HeaderValue::from_static("Bearer admin_token"),
                    )
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(authed.status(), StatusCode::OK);

        let rejected = app
            .oneshot(Request::builder().uri("/").body(Body::empty()).unwrap())
            .await
            .unwrap();
        assert_eq!(rejected.status(), StatusCode::UNAUTHORIZED);
    }

    #[tokio::test]
    async fn write_doc_and_purge_jobs_modify_state() {
        docs_store().write().await.clear();
        write_doc(TangleArg(("doc1".into(), "secret".into(), "tenant".into()))).await;
        {
            let guard = docs_store().read().await;
            assert_eq!(guard.get("tenant").unwrap().get("doc1").unwrap(), "secret");
        }

        admin_purge(TangleArg(("tenant".into(),))).await;
        let guard = docs_store().read().await;
        assert!(guard.get("tenant").is_none());
    }
}
