use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use axum::Router;
use axum::body::Body;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::Response;
use axum::routing::get;
use blake3;
use blueprint_manager::error::Error;
use blueprint_manager::sdk::utils::get_formatted_os_string;
use blueprint_manager::sources::BlueprintSourceHandler;
use blueprint_manager::sources::remote::RemoteBinaryFetcher;
use blueprint_manager::sources::types::{BlueprintBinary, RemoteFetcher};
use hex;
use serde_json::json;
use serial_test::serial;
use sha2::{Digest, Sha256};
use tar::{Builder, Header};
use tempfile::tempdir;
use tokio::task::JoinHandle;
use xz2::write::XzEncoder;

#[tokio::test]
#[serial(env)]
async fn remote_fetcher_downloads_and_runs_binary() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary_name = "remote-cli";
    let binary_contents = b"#!/bin/sh\necho remote\n";
    let binary = blueprint_binary(binary_name, binary_contents);
    let manifest = manifest_with_binary(binary_name);
    let archive = build_archive(&[(binary_name, binary_contents)]);
    let server = MockServer::start(manifest, archive).await;

    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let binary_path = fetcher.fetch(cache_dir.path()).await.unwrap();

    let output = Command::new(&binary_path).output().unwrap();
    assert!(String::from_utf8_lossy(&output.stdout).contains("remote"));
    assert_eq!(server.manifest_hits(), 1);
    assert_eq!(server.archive_hits(), 1);
}

#[tokio::test]
#[serial(env)]
async fn manifest_missing_binary_is_rejected() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary_name = "missing-cli";
    let binary_contents = b"#!/bin/sh\necho test\n";
    let binary = blueprint_binary(binary_name, binary_contents);
    let manifest = manifest_without_binary();
    let archive = build_archive(&[(binary_name, binary_contents)]);
    let server = MockServer::start(manifest, archive).await;

    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let err = fetcher.fetch(cache_dir.path()).await.unwrap_err();
    assert!(matches!(err, Error::NoMatchingBinary));
}

#[tokio::test]
#[serial(env)]
async fn archive_missing_binary_is_rejected() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary = blueprint_binary("remote-cli", b"#!/bin/sh\necho hi\n");
    let manifest = manifest_with_binary("remote-cli");
    let archive = build_archive(&[("other-bin", b"hi\n")]);
    let server = MockServer::start(manifest, archive).await;

    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let err = fetcher.fetch(cache_dir.path()).await.unwrap_err();
    assert!(matches!(err, Error::NoMatchingBinary));
}

#[tokio::test]
#[serial(env)]
async fn checksum_mismatch_is_detected() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let mut binary = blueprint_binary("remote-cli", b"#!/bin/sh\necho hi\n");
    // Corrupt the checksum
    binary.sha256[0] ^= 0xff;
    let manifest = manifest_with_binary("remote-cli");
    let archive = build_archive(&[("remote-cli", b"#!/bin/sh\necho hi\n")]);
    let server = MockServer::start(manifest, archive).await;

    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let err = fetcher.fetch(cache_dir.path()).await.unwrap_err();
    assert!(matches!(err, Error::HashMismatch { .. }));
}

#[tokio::test]
#[serial(env)]
async fn download_failures_surface_clear_errors() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary = blueprint_binary("remote-cli", b"data");
    let manifest = manifest_with_binary("remote-cli");
    let archive = build_archive(&[("remote-cli", b"data")]);
    let server = MockServer::start_with_failures(manifest, archive, 5, 0).await;
    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let err = fetcher.fetch(cache_dir.path()).await.unwrap_err();
    assert!(matches!(err, Error::DownloadFailed { .. }));
}

#[tokio::test]
#[serial(env)]
async fn cache_is_reused_between_fetches() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary = blueprint_binary("remote-cli", b"#!/bin/sh\necho hi\n");
    let manifest = manifest_with_binary("remote-cli");
    let archive = build_archive(&[("remote-cli", b"#!/bin/sh\necho hi\n")]);
    let server = MockServer::start(manifest, archive).await;

    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let path_one = fetcher.fetch(cache_dir.path()).await.unwrap();
    let path_two = fetcher.fetch(cache_dir.path()).await.unwrap();
    assert_eq!(path_one, path_two);
    assert_eq!(server.manifest_hits(), 1);
    assert_eq!(server.archive_hits(), 1);
}

#[tokio::test]
#[serial(env)]
async fn corrupted_cache_triggers_redownload() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", None);
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary = blueprint_binary("remote-cli", b"#!/bin/sh\necho hi\n");
    let manifest = manifest_with_binary("remote-cli");
    let archive = build_archive(&[("remote-cli", b"#!/bin/sh\necho hi\n")]);
    let server = MockServer::start(manifest, archive).await;
    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let archive_path = cache_dir.path().join(archive_file_name(&server));
    let manifest_path = cache_dir
        .path()
        .join(format!("remote-{}-dist.json", cache_key(&server)));

    fetcher.fetch(cache_dir.path()).await.unwrap();
    std::fs::write(&archive_path, b"broken").unwrap();
    fetcher.fetch(cache_dir.path()).await.unwrap();
    assert_eq!(server.manifest_hits(), 2);
    assert_eq!(server.archive_hits(), 2);
    assert!(manifest_path.exists());
}

#[tokio::test]
#[serial(env)]
async fn archive_size_limits_are_enforced() {
    let _max_guard = EnvGuard::set("MAX_ARCHIVE_BYTES", Some("10"));
    let binary = blueprint_binary("remote-cli", b"0123456789abcdef");
    let manifest = manifest_with_binary("remote-cli");
    let archive = vec![0u8; 32];
    let server = MockServer::start(manifest, archive).await;
    let mut fetcher = build_fetcher(&server, vec![binary]);
    let cache_dir = tempdir().unwrap();
    let err = fetcher.fetch(cache_dir.path()).await.unwrap_err();
    assert!(matches!(err, Error::ArchiveTooLarge { .. }));
}

#[tokio::test]
#[serial(env)]
async fn ipfs_gateway_is_required_when_urls_use_ipfs_scheme() {
    let _ipfs_guard = EnvGuard::set("IPFS_GATEWAY_URL", None);
    let binary = blueprint_binary("remote-cli", b"payload");
    let fetcher = RemoteFetcher {
        dist_url: "ipfs://dist.json".into(),
        archive_url: "ipfs://archive.tar.xz".into(),
        binaries: vec![binary],
    };
    let mut remote = RemoteBinaryFetcher::new(fetcher, 1, "ipfs".into());
    let cache_dir = tempdir().unwrap();
    let err = remote.fetch(cache_dir.path()).await.unwrap_err();
    assert!(matches!(err, Error::MissingIpfsGateway { .. }));
}

#[tokio::test]
#[serial(env)]
async fn ipfs_gateway_supports_http_translation() {
    let binary = blueprint_binary("remote-cli", b"payload");
    let manifest = manifest_with_binary("remote-cli");
    let archive = build_archive(&[("remote-cli", b"payload")]);
    let server = MockServer::start(manifest, archive).await;
    let _guard = EnvGuard::set("IPFS_GATEWAY_URL", Some(&server.base_url));
    let mut fetcher = RemoteBinaryFetcher::new(
        RemoteFetcher {
            dist_url: "ipfs://dist.json".into(),
            archive_url: "ipfs://archive.tar.xz".into(),
            binaries: vec![binary],
        },
        99,
        "ipfs".into(),
    );
    let cache_dir = tempdir().unwrap();
    fetcher.fetch(cache_dir.path()).await.unwrap();
    assert_eq!(server.manifest_hits(), 1);
    assert_eq!(server.archive_hits(), 1);
}

fn manifest_with_binary(name: &str) -> Vec<u8> {
    serde_json::to_vec(&json!({
        "artifacts": {
            "bundle": {
                "kind": "executable-zip",
                "assets": [
                    {
                        "kind": "executable",
                        "name": name
                    }
                ]
            }
        }
    }))
    .unwrap()
}

fn manifest_without_binary() -> Vec<u8> {
    serde_json::to_vec(&json!({
        "artifacts": {
            "bundle": {
                "kind": "executable-zip",
                "assets": [
                    {
                        "kind": "executable",
                        "name": "other"
                    }
                ]
            }
        }
    }))
    .unwrap()
}

fn build_archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let cursor = Vec::new();
    let encoder = XzEncoder::new(cursor, 6);
    let mut builder = Builder::new(encoder);
    for (name, contents) in entries {
        let mut header = Header::new_gnu();
        header.set_path(name).unwrap();
        header.set_size(contents.len() as u64);
        header.set_mode(0o755);
        header.set_cksum();
        builder.append(&header, *contents).unwrap();
    }
    builder.finish().unwrap();
    let encoder = builder.into_inner().unwrap();
    encoder.finish().unwrap()
}

fn blueprint_binary(name: &str, contents: &[u8]) -> BlueprintBinary {
    let sha = Sha256::digest(contents);
    let mut sha_buf = [0u8; 32];
    sha_buf.copy_from_slice(&sha);
    let mut blake_buf = [0u8; 32];
    blake_buf.copy_from_slice(blake3::hash(contents).as_bytes());

    BlueprintBinary {
        arch: std::env::consts::ARCH.to_string(),
        os: get_formatted_os_string(),
        name: name.to_string(),
        sha256: sha_buf,
        blake3: Some(blake_buf),
    }
}

fn build_fetcher(server: &MockServer, binaries: Vec<BlueprintBinary>) -> RemoteBinaryFetcher {
    let fetcher = RemoteFetcher {
        dist_url: server.manifest_url(),
        archive_url: server.archive_url(),
        binaries,
    };
    RemoteBinaryFetcher::new(fetcher, 7, "demo".into())
}

fn cache_key(server: &MockServer) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(server.manifest_url().as_bytes());
    hasher.update(server.archive_url().as_bytes());
    hex::encode(&hasher.finalize().as_bytes()[..8])
}

fn archive_file_name(server: &MockServer) -> String {
    format!("remote-archive-{}", cache_key(server))
}

#[derive(Clone)]
struct EndpointState {
    body: Arc<Vec<u8>>,
    success_status: StatusCode,
    failure_status: StatusCode,
    remaining_failures: Arc<AtomicUsize>,
    hits: Arc<AtomicUsize>,
}

impl EndpointState {
    fn new(body: Vec<u8>, failures: usize) -> Self {
        Self {
            body: Arc::new(body),
            success_status: StatusCode::OK,
            failure_status: StatusCode::INTERNAL_SERVER_ERROR,
            remaining_failures: Arc::new(AtomicUsize::new(failures)),
            hits: Arc::new(AtomicUsize::new(0)),
        }
    }

    async fn respond(&self) -> Response {
        self.hits.fetch_add(1, Ordering::SeqCst);
        let should_fail = self
            .remaining_failures
            .fetch_update(Ordering::SeqCst, Ordering::SeqCst, |current| {
                if current > 0 { Some(current - 1) } else { None }
            })
            .is_ok();

        let (status, body) = if should_fail {
            (self.failure_status, b"failure".to_vec())
        } else {
            (self.success_status, self.body.as_ref().clone())
        };

        Response::builder()
            .status(status)
            .body(Body::from(body))
            .unwrap()
    }

    fn hits(&self) -> Arc<AtomicUsize> {
        self.hits.clone()
    }
}

#[derive(Clone)]
struct AppState {
    manifest: Arc<EndpointState>,
    archive: Arc<EndpointState>,
}

async fn manifest_handler(State(state): State<AppState>) -> Response {
    state.manifest.respond().await
}

async fn archive_handler(State(state): State<AppState>) -> Response {
    state.archive.respond().await
}

struct MockServer {
    base_url: String,
    manifest_hits: Arc<AtomicUsize>,
    archive_hits: Arc<AtomicUsize>,
    handle: JoinHandle<()>,
}

impl MockServer {
    async fn start(manifest_body: Vec<u8>, archive_body: Vec<u8>) -> Self {
        Self::start_with_failures(manifest_body, archive_body, 0, 0).await
    }

    async fn start_with_failures(
        manifest_body: Vec<u8>,
        archive_body: Vec<u8>,
        manifest_failures: usize,
        archive_failures: usize,
    ) -> Self {
        let manifest_state = Arc::new(EndpointState::new(manifest_body, manifest_failures));
        let archive_state = Arc::new(EndpointState::new(archive_body, archive_failures));
        let app_state = AppState {
            manifest: manifest_state.clone(),
            archive: archive_state.clone(),
        };
        let app = Router::new()
            .route("/dist.json", get(manifest_handler))
            .route("/archive.tar.xz", get(archive_handler))
            .with_state(app_state);

        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let handle = tokio::spawn(async move {
            if let Err(err) = axum::serve(listener, app.into_make_service()).await {
                eprintln!("mock server exited with error: {err}");
            }
        });

        Self {
            base_url: format!("http://{}", addr),
            manifest_hits: manifest_state.hits(),
            archive_hits: archive_state.hits(),
            handle,
        }
    }

    fn manifest_url(&self) -> String {
        format!("{}/dist.json", self.base_url)
    }

    fn archive_url(&self) -> String {
        format!("{}/archive.tar.xz", self.base_url)
    }

    fn manifest_hits(&self) -> usize {
        self.manifest_hits.load(Ordering::SeqCst)
    }

    fn archive_hits(&self) -> usize {
        self.archive_hits.load(Ordering::SeqCst)
    }
}

impl Drop for MockServer {
    fn drop(&mut self) {
        self.handle.abort();
    }
}

struct EnvGuard {
    key: &'static str,
    original: Option<String>,
}

impl EnvGuard {
    fn set(key: &'static str, value: Option<&str>) -> Self {
        let original = std::env::var(key).ok();
        match value {
            Some(val) => unsafe { std::env::set_var(key, val) },
            None => unsafe { std::env::remove_var(key) },
        }
        Self { key, original }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        if let Some(value) = &self.original {
            unsafe { std::env::set_var(self.key, value) };
        } else {
            unsafe { std::env::remove_var(self.key) };
        }
    }
}
