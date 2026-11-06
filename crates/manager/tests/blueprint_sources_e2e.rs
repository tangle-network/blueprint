//! Public source-handler proof. Only HTTP peers and the gh child process are faked.
#![cfg(all(
    target_os = "linux",
    any(target_arch = "x86_64", target_arch = "aarch64")
))]

use axum::{
    Router,
    http::{StatusCode, Uri},
};
use blueprint_manager::{
    error::Error,
    sources::{
        BlueprintSourceHandler,
        github::GithubBinaryFetcher,
        remote::RemoteBinaryFetcher,
        types::{BlueprintBinary, GithubFetcher, RemoteFetcher},
    },
};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeMap,
    os::unix::fs::PermissionsExt,
    path::{Path, PathBuf},
    process::Command,
    sync::{Arc, Mutex},
};
use tar::{Builder, Header};
use tempfile::{TempDir, tempdir};
use tokio::task::JoinHandle;
use url::Url;
use xz2::write::XzEncoder;

const PAYLOAD: &[u8] = b"#!/bin/sh\nprintf 'fetched cargo-dist\\n'\n";
const RELEASE: &str = "/tangle-network/blueprint/releases/download/cargo-tangle-v0.5.0-alpha.28";
#[cfg(target_arch = "x86_64")]
const TARGET: &str = "x86_64-unknown-linux-gnu";
#[cfg(target_arch = "x86_64")]
const ARCHIVE: &str = "cargo-tangle-x86_64-unknown-linux-gnu.tar.xz";
#[cfg(target_arch = "aarch64")]
const TARGET: &str = "aarch64-unknown-linux-gnu";
#[cfg(target_arch = "aarch64")]
const ARCHIVE: &str = "cargo-tangle-aarch64-unknown-linux-gnu.tar.xz";

type Responses = Arc<Mutex<BTreeMap<String, (StatusCode, Vec<u8>)>>>;
struct Peer {
    base: Url,
    responses: Responses,
    requests: Arc<Mutex<Vec<String>>>,
    server: JoinHandle<()>,
}
impl Peer {
    async fn new() -> Self {
        let responses: Responses = Arc::default();
        let requests: Arc<Mutex<Vec<String>>> = Arc::default();
        let routes = responses.clone();
        let seen = requests.clone();
        let app = Router::new().fallback(move |uri: Uri| {
            let routes = routes.clone();
            let seen = seen.clone();
            async move {
                seen.lock().unwrap().push(uri.path().to_owned());
                routes
                    .lock()
                    .unwrap()
                    .get(uri.path())
                    .cloned()
                    .unwrap_or((StatusCode::NOT_FOUND, b"unknown artifact".to_vec()))
            }
        });
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let base = Url::parse(&format!("http://{}/", listener.local_addr().unwrap())).unwrap();
        let server = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        Self {
            base,
            responses,
            requests,
            server,
        }
    }
    fn serve(&self, path: &str, bytes: impl Into<Vec<u8>>) {
        self.responses
            .lock()
            .unwrap()
            .insert(path.to_owned(), (StatusCode::OK, bytes.into()));
    }
    fn seen(&self) -> Vec<String> {
        self.requests.lock().unwrap().clone()
    }
}
impl Drop for Peer {
    fn drop(&mut self) {
        self.server.abort();
    }
}

fn binary(name: &str) -> BlueprintBinary {
    BlueprintBinary {
        name: name.into(),
        arch: std::env::consts::ARCH.into(),
        os: "linux".into(),
        sha256: Sha256::digest(PAYLOAD).into(),
        blake3: None,
    }
}
fn archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
    let mut tar = Builder::new(XzEncoder::new(Vec::new(), 1));
    for (path, contents) in entries {
        let mut header = Header::new_gnu();
        header.set_size(contents.len() as u64);
        header.set_mode(0o755);
        header.set_cksum();
        tar.append_data(&mut header, path, *contents).unwrap();
    }
    tar.into_inner().unwrap().finish().unwrap()
}
fn gh_tool(folder: &Path) -> PathBuf {
    let tool = folder.join("gh");
    std::fs::write(&tool, "#!/bin/sh\nprintf '%s\\n' \"$@\" > \"$0.args\"\n[ \"$1\" = attestation ] && [ \"$2\" = verify ] && [ -f \"$3\" ] && [ \"$4\" = --repo ] && [ \"$5\" = tangle-network/blueprint ]\n").unwrap();
    std::fs::set_permissions(&tool, std::fs::Permissions::from_mode(0o700)).unwrap();
    tool
}
fn github_manifest(peer: &Peer) -> Value {
    // Names, target and asset layout follow the retained real cargo-dist0.28.7 plan.
    json!({"dist_version":"0.28.7-prerelease.1", "announcement_tag":"cargo-tangle-v0.5.0-alpha.28",
        "releases":[{"app_name":"cargo-tangle", "app_version":"0.5.0-alpha.28",
            "artifacts":[ARCHIVE, format!("{ARCHIVE}.sha256")],
            "hosting":{"github":{"artifact_base_url":peer.base.as_str().trim_end_matches('/'),
                "artifact_download_path":RELEASE, "owner":"tangle-network", "repo":"blueprint"}}}],
        "artifacts":{ARCHIVE:{"name":ARCHIVE, "kind":"executable-zip", "target_triples":[TARGET],
            "assets":[{"name":"cargo-tangle", "path":"cargo-tangle", "kind":"executable"}],
            "checksum":format!("{ARCHIVE}.sha256")},
            format!("{ARCHIVE}.sha256"):{"name":format!("{ARCHIVE}.sha256"),"kind":"checksum"}}})
}

#[tokio::test]
async fn github_fetches_the_manifest_target_archive_and_executes_the_pinned_asset() {
    let peer = Peer::new().await;
    let archive = archive(&[(
        &format!("{}/cargo-tangle", ARCHIVE.trim_end_matches(".tar.xz")),
        PAYLOAD,
    )]);
    peer.serve(
        &format!("{RELEASE}/dist-manifest.json"),
        serde_json::to_vec(&github_manifest(&peer)).unwrap(),
    );
    peer.serve(
        &format!("{RELEASE}/{ARCHIVE}.sha256"),
        format!("{}  {ARCHIVE}\n", hex::encode(Sha256::digest(&archive))),
    );
    peer.serve(&format!("{RELEASE}/{ARCHIVE}"), archive);
    let cache = tempdir().unwrap();
    let tools = tempdir().unwrap();
    let gh = gh_tool(tools.path());
    let fetcher = GithubFetcher {
        owner: "tangle-network".into(),
        repo: "blueprint".into(),
        tag: "cargo-tangle-v0.5.0-alpha.28".into(),
        binaries: vec![binary("cargo-tangle")],
    };
    let mut source = GithubBinaryFetcher::new(fetcher, 77, "public fetch".into(), false)
        .with_release_base_url(peer.base.clone())
        .with_http_client(reqwest::Client::new())
        .with_github_cli(gh.clone());
    let result = source.fetch(cache.path()).await;
    assert!(
        result.is_ok(),
        "manifest-defined native archive must fetch: {result:?}; requests={:?}",
        peer.seen()
    );
    let executable = result.unwrap();
    assert_eq!(std::fs::read(&executable).unwrap(), PAYLOAD);
    let run = Command::new(&executable).output().unwrap();
    assert!(run.status.success());
    assert_eq!(run.stdout, b"fetched cargo-dist\n");
    assert!(peer.seen().contains(&format!("{RELEASE}/{ARCHIVE}")));
    let args = std::fs::read_to_string(gh.with_extension("args")).unwrap();
    assert!(args.starts_with("attestation\nverify\n"));
    assert!(args.ends_with("--repo\ntangle-network/blueprint\n"));
}

async fn remote_fixture(name: &str) -> (Peer, RemoteBinaryFetcher, TempDir) {
    let peer = Peer::new().await;
    peer.serve(
        "/dist.json",
        serde_json::to_vec(&json!({"artifacts":{"bundle":{
        "kind":"executable-zip", "assets":[{"kind":"executable","name":name}]}}}))
        .unwrap(),
    );
    peer.serve("/archive.tar.xz", archive(&[(name, PAYLOAD)]));
    let source = RemoteBinaryFetcher::new(
        RemoteFetcher {
            dist_url: peer.base.join("dist.json").unwrap().to_string(),
            archive_url: peer.base.join("archive.tar.xz").unwrap().to_string(),
            binaries: vec![binary(name)],
        },
        1,
        "remote public HTTP".into(),
    );
    (peer, source, tempdir().unwrap())
}

#[tokio::test]
async fn remote_source_validates_checksum_and_reuses_verified_cache() {
    let (peer, mut source, cache) = remote_fixture("cached-bin").await;
    let path = source.fetch(cache.path()).await.unwrap();
    assert_eq!(std::fs::read(&path).unwrap(), PAYLOAD);
    assert_eq!(source.fetch(cache.path()).await.unwrap(), path);
    assert_eq!(peer.seen(), ["/dist.json", "/archive.tar.xz"]);
}

#[tokio::test]
async fn remote_source_rejects_bad_checksum() {
    let (peer, _, cache) = remote_fixture("bad-bin").await;
    let mut wrong = binary("bad-bin");
    wrong.sha256 = [0xff; 32];
    let mut source = RemoteBinaryFetcher::new(
        RemoteFetcher {
            dist_url: peer.base.join("dist.json").unwrap().to_string(),
            archive_url: peer.base.join("archive.tar.xz").unwrap().to_string(),
            binaries: vec![wrong],
        },
        1,
        "bad checksum".into(),
    );
    assert!(matches!(
        source.fetch(cache.path()).await,
        Err(Error::HashMismatch { .. })
    ));
}

#[tokio::test]
async fn remote_source_rejects_missing_binary() {
    let (peer, mut source, cache) = remote_fixture("wanted-bin").await;
    peer.serve(
        "/dist.json",
        serde_json::to_vec(&json!({"artifacts":{"bundle":{
        "kind":"executable-zip", "assets":[{"kind":"executable","name":"other-bin"}]}}}))
        .unwrap(),
    );
    assert!(matches!(
        source.fetch(cache.path()).await,
        Err(Error::NoMatchingBinary)
    ));
}
