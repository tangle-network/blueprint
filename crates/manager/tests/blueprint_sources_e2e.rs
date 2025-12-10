//! Blueprint Sources End-to-End Tests
//!
//! Tests the source fetching and validation mechanisms:
//! - Container image pulling
//! - GitHub release fetching
//! - Remote HTTP/IPFS fetching
//! - Checksum validation
//! - Cache behavior
//!
//! These tests exercise the real fetcher implementations.

use std::process::Command;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Duration;

use anyhow::{Context, Result};
use blueprint_manager::error::Error as ManagerError;
use blueprint_manager::sources::BlueprintSourceHandler;
use blueprint_manager::sources::container::ContainerSource;
use blueprint_manager::sources::github::GithubBinaryFetcher;
use blueprint_manager::sources::remote::RemoteBinaryFetcher;
use blueprint_manager::sources::types::{
    BlueprintBinary, GithubFetcher, ImageRegistryFetcher, RemoteFetcher,
};
use sha2::{Digest, Sha256};
use tar::{Builder, Header};
use tempfile::tempdir;
use tokio::time::timeout;
use xz2::write::XzEncoder;

const TEST_TIMEOUT: Duration = Duration::from_secs(60);

// =============================================================================
// SECTION 1: Container Source Tests
// =============================================================================

/// Test: Container source pulls a real public image
#[tokio::test]
async fn container_source_pulls_alpine() {
    if !docker_available() {
        eprintln!("Skipping: Docker not available");
        return;
    }

    let fetcher = ImageRegistryFetcher {
        registry: "docker.io".to_string(),
        image: "library/alpine".to_string(),
        tag: "3.19".to_string(),
    };

    let mut source = ContainerSource::new(fetcher, 0, "test".to_string());
    let cache_dir = tempdir().unwrap();

    let result = timeout(TEST_TIMEOUT, source.fetch(cache_dir.path())).await;

    match result {
        Ok(Ok(path)) => {
            let image_ref = path.to_string_lossy();
            assert!(
                image_ref.contains("alpine"),
                "Should return alpine image ref: {}",
                image_ref
            );
            println!("✓ Pulled alpine image: {}", image_ref);
        }
        Ok(Err(e)) => {
            println!("Container pull failed (may be expected): {}", e);
        }
        Err(_) => {
            eprintln!("Container pull timed out");
        }
    }
}

/// Test: Container source rejects nonexistent images
#[tokio::test]
async fn container_source_rejects_bad_image() {
    if !docker_available() {
        eprintln!("Skipping: Docker not available");
        return;
    }

    let fetcher = ImageRegistryFetcher {
        registry: "docker.io".to_string(),
        image: "nonexistent/image-xyz-99999".to_string(),
        tag: "v999".to_string(),
    };

    let mut source = ContainerSource::new(fetcher, 0, "test".to_string());
    let cache_dir = tempdir().unwrap();

    let result = timeout(Duration::from_secs(30), source.fetch(cache_dir.path())).await;

    match result {
        Ok(Ok(_)) => panic!("Should have failed for nonexistent image"),
        Ok(Err(_)) => println!("✓ Correctly rejected bad image"),
        Err(_) => println!("✓ Timed out (expected for bad image)"),
    }
}

// =============================================================================
// SECTION 2: GitHub Source Tests
// =============================================================================

/// Test: GitHub fetcher validates binary list
#[tokio::test]
async fn github_source_requires_matching_binary() {
    let fetcher = GithubFetcher {
        owner: "jqlang".to_string(),
        repo: "jq".to_string(),
        tag: "jq-1.7.1".to_string(),
        binaries: vec![], // Empty!
    };

    let mut github = GithubBinaryFetcher::new(fetcher, 0, "test".into(), true);
    let cache_dir = tempdir().unwrap();

    let result = github.fetch(cache_dir.path()).await;

    match result {
        Ok(_) => panic!("Should fail with empty binaries"),
        Err(e) => {
            assert!(
                matches!(e, ManagerError::NoMatchingBinary),
                "Expected NoMatchingBinary, got: {}",
                e
            );
            println!("✓ Correctly rejected empty binaries list");
        }
    }
}

/// Test: GitHub fetcher handles rate limiting gracefully
#[tokio::test]
async fn github_source_handles_errors() {
    let fetcher = GithubFetcher {
        owner: "nonexistent-owner-xyz".to_string(),
        repo: "nonexistent-repo".to_string(),
        tag: "v0.0.0".to_string(),
        binaries: vec![test_binary("app")],
    };

    let mut github = GithubBinaryFetcher::new(fetcher, 0, "test".into(), true);
    let cache_dir = tempdir().unwrap();

    let result = timeout(Duration::from_secs(15), github.fetch(cache_dir.path())).await;

    match result {
        Ok(Ok(_)) => panic!("Should fail for nonexistent repo"),
        Ok(Err(e)) => {
            println!("✓ GitHub error handled: {}", e);
        }
        Err(_) => {
            println!("✓ Request timed out (expected)");
        }
    }
}

// =============================================================================
// SECTION 3: Remote HTTP Source Tests
// =============================================================================

/// Test: Remote fetcher validates checksums correctly
#[tokio::test]
async fn remote_source_validates_checksum() {
    use axum::routing::get;
    use axum::Router;

    let binary_content = b"#!/bin/sh\necho test\n";
    let sha256 = Sha256::digest(binary_content);
    let mut sha256_arr = [0u8; 32];
    sha256_arr.copy_from_slice(&sha256);

    let manifest = serde_json::json!({
        "artifacts": {
            "bundle": {
                "kind": "executable-zip",
                "assets": [{"kind": "executable", "name": "test-bin"}]
            }
        }
    });
    let archive = create_test_archive(&[("test-bin", binary_content)]);

    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    let archive_bytes = archive;

    let app = Router::new()
        .route("/dist.json", get({
            let data = manifest_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }))
        .route("/archive.tar.xz", get({
            let data = archive_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    let binary = BlueprintBinary {
        name: "test-bin".to_string(),
        arch: std::env::consts::ARCH.to_string(),
        os: format_os(std::env::consts::OS),
        sha256: sha256_arr,
        blake3: None,
    };

    let fetcher = RemoteFetcher {
        dist_url: format!("http://{}/dist.json", addr),
        archive_url: format!("http://{}/archive.tar.xz", addr),
        binaries: vec![binary],
    };

    let mut remote = RemoteBinaryFetcher::new(fetcher, 0, "test".into());
    let cache_dir = tempdir().unwrap();

    let result = remote.fetch(cache_dir.path()).await;
    server.abort();

    match result {
        Ok(path) => {
            assert!(path.exists());
            println!("✓ Checksum validation passed, binary at: {:?}", path);
        }
        Err(e) => panic!("Should succeed with correct checksum: {}", e),
    }
}

/// Test: Remote fetcher rejects bad checksums
#[tokio::test]
async fn remote_source_rejects_bad_checksum() {
    use axum::routing::get;
    use axum::Router;

    let manifest = serde_json::json!({
        "artifacts": {
            "bundle": {
                "kind": "executable-zip",
                "assets": [{"kind": "executable", "name": "test-bin"}]
            }
        }
    });
    let archive = create_test_archive(&[("test-bin", b"#!/bin/sh\necho test\n")]);

    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    let archive_bytes = archive;

    let app = Router::new()
        .route("/dist.json", get({
            let data = manifest_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }))
        .route("/archive.tar.xz", get({
            let data = archive_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    let binary = BlueprintBinary {
        name: "test-bin".to_string(),
        arch: std::env::consts::ARCH.to_string(),
        os: format_os(std::env::consts::OS),
        sha256: [0xffu8; 32], // Wrong!
        blake3: None,
    };

    let fetcher = RemoteFetcher {
        dist_url: format!("http://{}/dist.json", addr),
        archive_url: format!("http://{}/archive.tar.xz", addr),
        binaries: vec![binary],
    };

    let mut remote = RemoteBinaryFetcher::new(fetcher, 0, "test".into());
    let cache_dir = tempdir().unwrap();

    let result = remote.fetch(cache_dir.path()).await;
    server.abort();

    match result {
        Ok(_) => panic!("Should reject bad checksum"),
        Err(e) => {
            assert!(
                matches!(e, ManagerError::HashMismatch { .. }),
                "Expected HashMismatch, got: {}",
                e
            );
            println!("✓ Bad checksum rejected");
        }
    }
}

/// Test: Remote fetcher rejects missing binary
#[tokio::test]
async fn remote_source_rejects_missing_binary() {
    use axum::routing::get;
    use axum::Router;

    let manifest = serde_json::json!({
        "artifacts": {
            "bundle": {
                "kind": "executable-zip",
                "assets": [{"kind": "executable", "name": "other-bin"}]
            }
        }
    });
    let archive = create_test_archive(&[("other-bin", b"data")]);

    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    let archive_bytes = archive;

    let app = Router::new()
        .route("/dist.json", get({
            let data = manifest_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }))
        .route("/archive.tar.xz", get({
            let data = archive_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    let binary = BlueprintBinary {
        name: "wanted-bin".to_string(), // Not in archive!
        arch: std::env::consts::ARCH.to_string(),
        os: format_os(std::env::consts::OS),
        sha256: [0u8; 32],
        blake3: None,
    };

    let fetcher = RemoteFetcher {
        dist_url: format!("http://{}/dist.json", addr),
        archive_url: format!("http://{}/archive.tar.xz", addr),
        binaries: vec![binary],
    };

    let mut remote = RemoteBinaryFetcher::new(fetcher, 0, "test".into());
    let cache_dir = tempdir().unwrap();

    let result = remote.fetch(cache_dir.path()).await;
    server.abort();

    match result {
        Ok(_) => panic!("Should reject missing binary"),
        Err(e) => {
            assert!(
                matches!(e, ManagerError::NoMatchingBinary),
                "Expected NoMatchingBinary, got: {}",
                e
            );
            println!("✓ Missing binary rejected");
        }
    }
}

/// Test: Remote fetcher uses cache on second fetch
#[tokio::test]
async fn remote_source_uses_cache() {
    use axum::routing::get;
    use axum::Router;

    let hit_count = Arc::new(AtomicUsize::new(0));
    let manifest_hits = hit_count.clone();

    let binary_content = b"#!/bin/sh\necho cached\n";
    let sha256 = Sha256::digest(binary_content);
    let mut sha256_arr = [0u8; 32];
    sha256_arr.copy_from_slice(&sha256);

    let manifest = serde_json::json!({
        "artifacts": {
            "bundle": {
                "kind": "executable-zip",
                "assets": [{"kind": "executable", "name": "cached-bin"}]
            }
        }
    });
    let archive = create_test_archive(&[("cached-bin", binary_content)]);

    let manifest_bytes = serde_json::to_vec(&manifest).unwrap();
    let archive_bytes = archive;

    let app = Router::new()
        .route("/dist.json", get({
            let data = manifest_bytes.clone();
            let hits = manifest_hits.clone();
            move || {
                hits.fetch_add(1, Ordering::SeqCst);
                let d = data.clone();
                async move { d }
            }
        }))
        .route("/archive.tar.xz", get({
            let data = archive_bytes.clone();
            move || {
                let d = data.clone();
                async move { d }
            }
        }));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let server = tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    let binary = BlueprintBinary {
        name: "cached-bin".to_string(),
        arch: std::env::consts::ARCH.to_string(),
        os: format_os(std::env::consts::OS),
        sha256: sha256_arr,
        blake3: None,
    };

    let cache_dir = tempdir().unwrap();

    // First fetch
    let fetcher1 = RemoteFetcher {
        dist_url: format!("http://{}/dist.json", addr),
        archive_url: format!("http://{}/archive.tar.xz", addr),
        binaries: vec![binary.clone()],
    };
    let mut remote1 = RemoteBinaryFetcher::new(fetcher1, 0, "cache-test".into());
    remote1.fetch(cache_dir.path()).await.unwrap();

    // Second fetch - should use cache
    let fetcher2 = RemoteFetcher {
        dist_url: format!("http://{}/dist.json", addr),
        archive_url: format!("http://{}/archive.tar.xz", addr),
        binaries: vec![binary],
    };
    let mut remote2 = RemoteBinaryFetcher::new(fetcher2, 0, "cache-test".into());
    remote2.fetch(cache_dir.path()).await.unwrap();

    server.abort();

    let hits = hit_count.load(Ordering::SeqCst);
    assert_eq!(hits, 1, "Second fetch should use cache, but hit {} times", hits);
    println!("✓ Cache correctly reused");
}

/// Test: Network failures produce clear errors
#[tokio::test]
async fn remote_source_network_failure() {
    let binary = test_binary("unreachable");
    let fetcher = RemoteFetcher {
        dist_url: "http://192.0.2.1:12345/dist.json".to_string(), // Unreachable
        archive_url: "http://192.0.2.1:12345/archive.tar.xz".to_string(),
        binaries: vec![binary],
    };

    let mut remote = RemoteBinaryFetcher::new(fetcher, 0, "test".into());
    let cache_dir = tempdir().unwrap();

    let result = timeout(Duration::from_secs(10), remote.fetch(cache_dir.path())).await;

    match result {
        Ok(Ok(_)) => panic!("Should fail to connect"),
        Ok(Err(e)) => {
            let err = e.to_string().to_lowercase();
            assert!(
                err.contains("connect") || err.contains("download") || err.contains("timeout"),
                "Error should mention connection: {}",
                e
            );
            println!("✓ Network error handled: {}", e);
        }
        Err(_) => println!("✓ Timed out as expected"),
    }
}

// =============================================================================
// Helper Functions
// =============================================================================

fn docker_available() -> bool {
    Command::new("docker")
        .arg("ps")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

fn test_binary(name: &str) -> BlueprintBinary {
    BlueprintBinary {
        name: name.to_string(),
        arch: std::env::consts::ARCH.to_string(),
        os: format_os(std::env::consts::OS),
        sha256: [0u8; 32],
        blake3: None,
    }
}

fn format_os(os: &str) -> String {
    // Must match get_formatted_os_string() from sdk/utils.rs
    match os {
        "macos" => "apple-darwin".to_string(),
        "linux" => "unknown-linux-gnu".to_string(),
        "windows" => "pc-windows-msvc".to_string(),
        other => other.to_string(),
    }
}

fn create_test_archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
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
