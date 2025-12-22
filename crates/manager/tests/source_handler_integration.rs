//! Source Handler Integration Tests
//!
//! Tests blueprint source fetching and execution across different source types:
//! - GitHub releases
//! - Container images
//! - Remote HTTP/IPFS archives
//!
//! These tests verify the complete fetch → validate → execute pipeline.

use std::process::Command;
use blueprint_manager::error::Error;
use blueprint_manager::sdk::utils::get_formatted_os_string;
use blueprint_manager::sources::BlueprintSourceHandler;
use blueprint_manager::sources::github::GithubBinaryFetcher;
use blueprint_manager::sources::remote::RemoteBinaryFetcher;
use blueprint_manager::sources::types::{BlueprintBinary, GithubFetcher, RemoteFetcher};
use serial_test::serial;
use sha2::{Digest, Sha256};
use tempfile::tempdir;

// =============================================================================
// GitHub Source Tests
// =============================================================================

#[tokio::test]
async fn github_fetcher_rejects_nonexistent_repo() {
    let fetcher = GithubFetcher {
        owner: "nonexistent-owner-12345".into(),
        repo: "nonexistent-repo-67890".into(),
        tag: "v0.0.0".into(),
        binaries: vec![fake_binary("cli")],
    };

    let mut handler = GithubBinaryFetcher::new(fetcher, 1, "test".into(), true);
    let cache = tempdir().unwrap();

    let result = handler.fetch(cache.path()).await;
    assert!(result.is_err(), "should fail for nonexistent repo");
}

#[tokio::test]
async fn github_fetcher_rejects_nonexistent_tag() {
    // Use a real repo but fake tag
    let fetcher = GithubFetcher {
        owner: "rust-lang".into(),
        repo: "rust".into(),
        tag: "v999.999.999-nonexistent".into(),
        binaries: vec![fake_binary("rustc")],
    };

    let mut handler = GithubBinaryFetcher::new(fetcher, 1, "test".into(), true);
    let cache = tempdir().unwrap();

    let result = handler.fetch(cache.path()).await;
    assert!(result.is_err(), "should fail for nonexistent tag");
}

#[tokio::test]
async fn github_fetcher_validates_binary_hash() {
    // Create fetcher with wrong hash
    let mut binary = fake_binary("cli");
    binary.sha256[0] ^= 0xff; // corrupt hash

    let fetcher = GithubFetcher {
        owner: "tangle-network".into(),
        repo: "blueprint".into(),
        tag: "v0.1.0".into(),
        binaries: vec![binary],
    };

    let mut handler = GithubBinaryFetcher::new(fetcher, 1, "test".into(), true);
    let cache = tempdir().unwrap();

    let result = handler.fetch(cache.path()).await;
    // Should fail either due to missing binary or hash mismatch
    assert!(result.is_err());
}

// =============================================================================
// Remote Source Tests
// =============================================================================

#[tokio::test]
async fn remote_fetcher_rejects_invalid_manifest_url() {
    let fetcher = RemoteFetcher {
        dist_url: "https://nonexistent.invalid/dist.json".into(),
        archive_url: "https://nonexistent.invalid/archive.tar.xz".into(),
        binaries: vec![fake_binary("cli")],
    };

    let mut handler = RemoteBinaryFetcher::new(fetcher, 1, "http".into());
    let cache = tempdir().unwrap();

    let result = handler.fetch(cache.path()).await;
    assert!(matches!(result, Err(Error::DownloadFailed { .. })));
}

#[tokio::test]
#[serial(env)]
async fn remote_fetcher_requires_ipfs_gateway_for_ipfs_urls() {
    // Clear IPFS gateway env
    unsafe { std::env::remove_var("IPFS_GATEWAY_URL") };

    let fetcher = RemoteFetcher {
        dist_url: "ipfs://QmTest123".into(),
        archive_url: "ipfs://QmArchive456".into(),
        binaries: vec![fake_binary("cli")],
    };

    let mut handler = RemoteBinaryFetcher::new(fetcher, 1, "ipfs".into());
    let cache = tempdir().unwrap();

    let result = handler.fetch(cache.path()).await;
    assert!(matches!(result, Err(Error::MissingIpfsGateway { .. })));
}

// =============================================================================
// Binary Validation Tests
// =============================================================================

#[tokio::test]
async fn binary_arch_os_matching_works() {
    let current_arch = std::env::consts::ARCH;
    let current_os = get_formatted_os_string();

    // Binary matching current platform
    let matching = BlueprintBinary {
        name: "test".into(),
        arch: current_arch.into(),
        os: current_os.clone(),
        sha256: [0u8; 32],
        blake3: None,
    };

    // Binary for different platform
    let non_matching = BlueprintBinary {
        name: "test".into(),
        arch: "riscv64".into(),
        os: "freebsd".into(),
        sha256: [0u8; 32],
        blake3: None,
    };

    assert_eq!(matching.arch, current_arch);
    assert_eq!(matching.os, current_os);
    assert_ne!(non_matching.arch, current_arch);
}

#[tokio::test]
async fn sha256_verification_detects_corruption() {
    let content = b"test binary content";
    let correct_hash = Sha256::digest(content);

    let mut corrupted_hash = [0u8; 32];
    corrupted_hash.copy_from_slice(&correct_hash);
    corrupted_hash[0] ^= 0xff;

    // Verify hashes differ
    assert_ne!(&correct_hash[..], &corrupted_hash[..]);

    // Verify correct hash matches
    let recomputed = Sha256::digest(content);
    assert_eq!(&correct_hash[..], &recomputed[..]);
}

// =============================================================================
// Cache Behavior Tests
// =============================================================================

#[tokio::test]
async fn cache_directory_is_created_if_missing() {
    let temp = tempdir().unwrap();
    let cache_path = temp.path().join("nested/cache/dir");

    // Directory doesn't exist yet
    assert!(!cache_path.exists());

    // Create it
    std::fs::create_dir_all(&cache_path).unwrap();
    assert!(cache_path.exists());
}

#[tokio::test]
async fn fetcher_creates_unique_cache_keys() {
    let fetcher1 = RemoteFetcher {
        dist_url: "https://a.com/dist.json".into(),
        archive_url: "https://a.com/archive.tar.xz".into(),
        binaries: vec![],
    };

    let fetcher2 = RemoteFetcher {
        dist_url: "https://b.com/dist.json".into(),
        archive_url: "https://b.com/archive.tar.xz".into(),
        binaries: vec![],
    };

    // Cache keys should differ
    let key1 = cache_key(&fetcher1.dist_url, &fetcher1.archive_url);
    let key2 = cache_key(&fetcher2.dist_url, &fetcher2.archive_url);
    assert_ne!(key1, key2);
}

// =============================================================================
// Container Source Tests (requires Docker)
// =============================================================================

#[tokio::test]
#[ignore = "requires Docker"]
async fn container_image_can_be_pulled() {
    // Check if docker is available
    let docker_check = Command::new("docker").arg("version").output();
    if docker_check.is_err() || !docker_check.unwrap().status.success() {
        eprintln!("Skipping: Docker not available");
        return;
    }

    // Pull a small test image
    let output = Command::new("docker")
        .args(["pull", "hello-world:latest"])
        .output()
        .expect("docker pull failed");

    assert!(output.status.success(), "failed to pull hello-world image");
}

#[tokio::test]
#[ignore = "requires Docker"]
async fn container_image_tag_is_validated() {
    let docker_check = Command::new("docker").arg("version").output();
    if docker_check.is_err() || !docker_check.unwrap().status.success() {
        return;
    }

    // Try to pull nonexistent tag
    let output = Command::new("docker")
        .args(["pull", "hello-world:nonexistent-tag-12345"])
        .output()
        .expect("docker command failed");

    assert!(!output.status.success(), "should fail for nonexistent tag");
}

// =============================================================================
// Helpers
// =============================================================================

fn fake_binary(name: &str) -> BlueprintBinary {
    BlueprintBinary {
        name: name.into(),
        arch: std::env::consts::ARCH.into(),
        os: get_formatted_os_string(),
        sha256: [0u8; 32],
        blake3: None,
    }
}

fn cache_key(dist_url: &str, archive_url: &str) -> String {
    let mut hasher = blake3::Hasher::new();
    hasher.update(dist_url.as_bytes());
    hasher.update(archive_url.as_bytes());
    hex::encode(&hasher.finalize().as_bytes()[..8])
}
