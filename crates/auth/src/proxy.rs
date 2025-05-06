use axum::Extension;
use axum::{
    Router,
    body::Body,
    extract::{Request, State},
    http::StatusCode,
    http::uri::Uri,
    response::{IntoResponse, Response},
    routing::any,
};
use hyper_util::{client::legacy::connect::HttpConnector, rt::TokioExecutor, rt::TokioTimer};
type Client = hyper_util::client::legacy::Client<HttpConnector, Body>;

pub fn router() -> Router {
    let executer = TokioExecutor::new();
    let timer = TokioTimer::new();
    let client: Client = hyper_util::client::legacy::Builder::new(executer)
        .pool_idle_timeout(std::time::Duration::from_secs(60))
        .pool_timer(timer)
        .build(HttpConnector::new());
    Router::new()
        .route("/", any(reverse_proxy))
        .layer(axum::Extension(client))
}

async fn reverse_proxy(
    Extension(client): Extension<Client>,
    mut req: Request,
) -> Result<Response, StatusCode> {
    let path = req.uri().path();
    let path_query = req
        .uri()
        .path_and_query()
        .map(|v| v.as_str())
        .unwrap_or(path);
    let target_uri = format!("http://localhost:8080{}", path_query);
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
