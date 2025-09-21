use std::net::IpAddr;

use blueprint_auth::proxy::DEFAULT_AUTH_PROXY_PORT;
use blueprint_auth::types::ServiceId;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = tempfile::tempdir()?;
    let db_path = tmp_dir.path().join("db");
    tokio::fs::create_dir_all(&db_path).await?;

    let auth_proxy_port = DEFAULT_AUTH_PROXY_PORT;
    let auth_proxy_host = IpAddr::V4(std::net::Ipv4Addr::UNSPECIFIED);

    let proxy = blueprint_auth::proxy::AuthenticatedProxy::new(&db_path)?;

    blueprint_auth::models::ServiceModel {
        api_key_prefix: String::from("mcp_test"),
        owners: vec![blueprint_auth::models::ServiceOwnerModel {
            key_type: blueprint_auth::types::KeyType::Ecdsa as _,
            // Alice's public key in hex format
            key_bytes: hex::decode(
                "020a1091341fe5664bfa1782d5e04779689068c916b04cb365ec3153755684d9a1",
            )
            .expect("Failed to decode hex public key"),
        }],
        // The URL of the upstream service that this proxy will forward requests to
        upstream_url: String::from("http://localhost:3000"),
        tls_profile: None,
    }
    .save(ServiceId::new(0), &proxy.db())?;

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
