use axum::{
    Json, Router,
    body::Body,
    extract::{Path, State},
    http::{Request, StatusCode},
    routing::{delete, get, post},
};
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tower::ServiceExt;
use uuid::Uuid;

#[tokio::test]
async fn test_http_health_endpoint() {
    let app = Router::new().route("/health", get(|| async { Json("ok") }));

    let response = app
        .oneshot(
            Request::builder()
                .uri("/health")
                .method("GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json, "ok");
}

#[tokio::test]
async fn test_http_create_doc_endpoint() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/docs")
                .method("POST")
                .header("Authorization", "Bearer test_user")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"content": "Test document content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("id").is_some());
    assert_eq!(json.get("tenant"), Some(&json!("test_user")));
}

#[tokio::test]
async fn test_http_create_doc_unauthorized() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/docs")
                .method("POST")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"content": "Test document content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::UNAUTHORIZED);
}

#[tokio::test]
async fn test_http_read_doc_endpoint() {
    let app = create_test_app();

    // First create a document
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/docs")
                .method("POST")
                .header("Authorization", "Bearer test_user")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"content": "Test document content"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let doc_id = create_json.get("id").unwrap().as_str().unwrap();

    // Now read the document
    let read_response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/docs/{}", doc_id))
                .method("GET")
                .header("Authorization", "Bearer test_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(read_response.status(), StatusCode::OK);

    let read_body = axum::body::to_bytes(read_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let read_json: Value = serde_json::from_slice(&read_body).unwrap();

    assert_eq!(read_json.get("id"), Some(&json!(doc_id)));
    assert_eq!(
        read_json.get("content"),
        Some(&json!("Test document content"))
    );
}

#[tokio::test]
async fn test_http_read_doc_not_found() {
    let app = create_test_app();

    let response = app
        .oneshot(
            Request::builder()
                .uri("/docs/nonexistent")
                .method("GET")
                .header("Authorization", "Bearer test_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_http_list_docs_endpoint() {
    let app = create_test_app();

    // Create multiple documents
    for i in 0..3 {
        let response = app
            .clone()
            .oneshot(
                Request::builder()
                    .uri("/docs")
                    .method("POST")
                    .header("Authorization", "Bearer test_user")
                    .header("Content-Type", "application/json")
                    .body(Body::from(format!(
                        r#"{{"content": "Document {} content"}}"#,
                        i
                    )))
                    .unwrap(),
            )
            .await
            .unwrap();

        assert_eq!(response.status(), StatusCode::OK);
    }

    // List documents
    let response = app
        .oneshot(
            Request::builder()
                .uri("/docs")
                .method("GET")
                .header("Authorization", "Bearer test_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let json: Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("docs").is_some());
    assert!(json.get("docs").unwrap().as_array().unwrap().len() >= 3);
}

#[tokio::test]
async fn test_http_delete_doc_endpoint() {
    let app = create_test_app();

    // Create a document
    let create_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri("/docs")
                .method("POST")
                .header("Authorization", "Bearer test_user")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"content": "Test document to delete"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(create_response.status(), StatusCode::OK);

    let create_body = axum::body::to_bytes(create_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let create_json: Value = serde_json::from_slice(&create_body).unwrap();
    let doc_id = create_json.get("id").unwrap().as_str().unwrap();

    // Delete the document
    let delete_response = app
        .clone()
        .oneshot(
            Request::builder()
                .uri(&format!("/docs/{}", doc_id))
                .method("DELETE")
                .header("Authorization", "Bearer test_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(delete_response.status(), StatusCode::OK);

    let delete_body = axum::body::to_bytes(delete_response.into_body(), usize::MAX)
        .await
        .unwrap();
    let delete_json: Value = serde_json::from_slice(&delete_body).unwrap();
    assert_eq!(delete_json.get("deleted"), Some(&json!(true)));

    // Verify the document is deleted
    let read_response = app
        .oneshot(
            Request::builder()
                .uri(&format!("/docs/{}", doc_id))
                .method("GET")
                .header("Authorization", "Bearer test_user")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(read_response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_admin_scope_access() {
    let app = create_test_app();

    // Test admin token has access to all scopes
    let response = app
        .oneshot(
            Request::builder()
                .uri("/docs")
                .method("POST")
                .header("Authorization", "Bearer admin_user")
                .header("Content-Type", "application/json")
                .body(Body::from(r#"{"content": "Admin document"}"#))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[derive(Clone)]
struct AppState {
    docs: Arc<RwLock<HashMap<String, serde_json::Value>>>,
}

#[derive(Clone)]
struct OAuthContext {
    tenant: String,
    scopes: std::collections::HashSet<String>,
}

fn create_test_app() -> Router {
    use axum::middleware;
    use std::collections::HashSet;

    let state = AppState {
        docs: Arc::new(RwLock::new(HashMap::new())),
    };

    async fn oauth_auth(
        headers: axum::http::HeaderMap,
        mut req: axum::http::Request<Body>,
        next: middleware::Next,
    ) -> Result<impl axum::response::IntoResponse, StatusCode> {
        let token = headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok())
            .and_then(|h| h.strip_prefix("Bearer "))
            .ok_or(StatusCode::UNAUTHORIZED)?;

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
        State(state): State<AppState>,
        axum::Extension(auth): axum::Extension<OAuthContext>,
        Json(payload): Json<serde_json::Value>,
    ) -> Result<impl axum::response::IntoResponse, StatusCode> {
        if !auth.scopes.contains("docs:write") {
            return Err(StatusCode::FORBIDDEN);
        }

        let id = Uuid::new_v4().to_string();
        let content = payload.get("content").cloned().unwrap_or_default();

        let mut doc = serde_json::Value::Object(serde_json::Map::new());
        doc.as_object_mut()
            .unwrap()
            .insert("id".to_string(), json!(id));
        doc.as_object_mut()
            .unwrap()
            .insert("tenant".to_string(), json!(auth.tenant));
        doc.as_object_mut()
            .unwrap()
            .insert("content".to_string(), content);

        state.docs.write().unwrap().insert(id.clone(), doc.clone());

        Ok(Json(doc))
    }

    async fn read_doc(
        State(state): State<AppState>,
        axum::Extension(auth): axum::Extension<OAuthContext>,
        Path(id): Path<String>,
    ) -> Result<impl axum::response::IntoResponse, StatusCode> {
        if !auth.scopes.contains("docs:read") {
            return Err(StatusCode::FORBIDDEN);
        }

        let docs = state.docs.read().unwrap();
        match docs.get(&id) {
            Some(doc) => Ok(Json(doc.clone())),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn delete_doc(
        State(state): State<AppState>,
        axum::Extension(auth): axum::Extension<OAuthContext>,
        Path(id): Path<String>,
    ) -> Result<impl axum::response::IntoResponse, StatusCode> {
        if !auth.scopes.contains("docs:write") {
            return Err(StatusCode::FORBIDDEN);
        }

        let mut docs = state.docs.write().unwrap();
        match docs.remove(&id) {
            Some(_) => Ok(Json(json!({"deleted": true}))),
            None => Err(StatusCode::NOT_FOUND),
        }
    }

    async fn list_docs(
        State(state): State<AppState>,
        axum::Extension(auth): axum::Extension<OAuthContext>,
    ) -> Result<impl axum::response::IntoResponse, StatusCode> {
        if !auth.scopes.contains("docs:read") {
            return Err(StatusCode::FORBIDDEN);
        }

        let docs = state.docs.read().unwrap();
        let docs_list: Vec<serde_json::Value> = docs.values().cloned().collect();

        Ok(Json(json!({"docs": docs_list})))
    }

    Router::new()
        .route("/health", get(|| async { Json("ok") }))
        .route("/docs", post(create_doc))
        .route("/docs/{id}", get(read_doc))
        .route("/docs/{id}", delete(delete_doc))
        .route("/docs", get(list_docs))
        .layer(middleware::from_fn(oauth_auth))
        .with_state(state)
}
