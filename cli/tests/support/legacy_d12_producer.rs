//! Test-only source, compiled ONLY inside actual SDK827 by the native recipe.
//! Public old SDK APIs produce the candidate/store; no authority/hash algorithm copy.
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use blueprint_remote_providers::{CloudProvisioner, ResourceSpec};
use serde::Deserialize;
use serde_json::{Value, json};
use std::{io::Write, path::PathBuf};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Input {
    endpoint: String,
    state_dir: PathBuf,
    spec: ResourceSpec,
}

async fn run() -> anyhow::Result<Value> {
    let input: Input = serde_json::from_reader(std::io::stdin().lock())?;
    let url = url::Url::parse(&input.endpoint)?;
    anyhow::ensure!(
        url.scheme() == "http"
            && url
                .host_str()
                .and_then(|host| host.parse::<std::net::IpAddr>().ok())
                .is_some_and(|ip| ip.is_loopback()),
        "fixture origin required"
    );
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let credentials = serde_json::from_value(
        json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":input.endpoint}),
    )?;
    let provisioner = CloudProvisioner::from_credentials(vec![credentials])?
        .initialize_lease_state_dir(&input.state_dir)
        .await?;
    let quote = provisioner.quotes(&input.spec).await?.quotes.remove(0);
    // This must be the old private receipt API and real old full-arm encoding.
    anyhow::ensure!(
        quote
            .measurement
            .as_ref()
            .is_some_and(|row| row.raw_result_uri.contains("PRIVATE_URI")),
        "legacy row required"
    );
    let candidate: Value = serde_json::from_slice(&URL_SAFE_NO_PAD.decode(&quote.candidate)?)?;
    anyhow::ensure!(
        candidate["arm"]["measurements"]
            .as_array()
            .is_some_and(|rows| rows.len() == 2
                && rows.iter().all(|row| row["raw_result_uri"]
                    .as_str()
                    .is_some_and(|uri| uri.contains("PRIVATE_URI") && uri.contains("signature=")))),
        "legacy full arm required"
    );
    anyhow::ensure!(
        quote
            .offer
            .pricing_source
            .contains("PRIVATE_ENDPOINT_CANARY"),
        "legacy source path required"
    );
    let resource = provisioner
        .create_managed(&input.spec, &quote.candidate)
        .await?;
    let store: Value = serde_json::from_slice(&std::fs::read(
        input.state_dir.join("blueprint/lease-authorizations.json"),
    )?)?;
    anyhow::ensure!(store["version"] == 1, "legacy state version required");
    Ok(
        json!({"provider_id":resource.instance.id,"expires_at":resource.expires_at.map(|time|time.to_rfc3339()),"legacy_full_arm":true,"store_version":1}),
    )
}

#[tokio::main]
async fn main() {
    match run().await {
        Ok(value) => {
            if writeln!(std::io::stdout().lock(), "{value}").is_err() {
                std::process::exit(2);
            }
        }
        Err(_) => {
            eprintln!("legacy producer refused");
            std::process::exit(2);
        }
    }
}
