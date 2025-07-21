use std::net::IpAddr;

use blueprint_auth::proxy::DEFAULT_AUTH_PROXY_PORT;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let db_path = tmp_dir.path().join("db");
    tokio::fs::create_dir_all(&db_path).await?;

    let auth_proxy_port = DEFAULT_AUTH_PROXY_PORT;
    let auth_proxy_host = IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);

    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path)?;

    let router = proxy.router();

    let listener = tokio::net::TcpListener::bind((auth_proxy_host, auth_proxy_port)).await?;
    eprintln!(
        "Auth proxy listening on {}:{}",
        auth_proxy_host, auth_proxy_port
    );
    let result = axum::serve(listener, router).await;
    if let Err(err) = result {
        eprintln!("Auth proxy error: {err}");
    }

    Ok(())
}
