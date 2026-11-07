//! SDK lease allocation and deadline retirement through the RunPod HTTP peer.
use super::*;
use std::time::Duration;

fn lease_spec(lease_id: &str) -> ResourceSpec {
    let mut spec = ResourceSpec::basic();
    spec.ownership = Some(ManagedLeaseIdentity {
        namespace: "lease-cell".into(),
        lease_id: lease_id.into(),
    });
    spec.gpu_class = Some("nvidia-h200".into());
    spec.gpu_count = Some(2);
    spec.gpu_vram_gb = Some(140);
    spec.image = Some("example.invalid/workload:v1".into());
    spec.region = Some("US-TEST-1".into());
    spec
}

#[tokio::test]
async fn allocated_leases_carry_their_own_key_and_retire_from_provider_deadlines() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) = runpod_peer_state().await;
    let credentials = || {
        serde_json::from_value(json!({
            "provider":"runpod", "api_key":"synthetic-local-peer-key", "endpoint":peer.uri()
        }))
        .unwrap()
    };
    let directory = tempfile::tempdir().unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials()])
        .unwrap()
        .initialize_lease_state_dir(directory.path())
        .await
        .unwrap();
    assert!(
        provisioner
            .allocate(&lease_spec("zero"), Duration::from_millis(500))
            .await
            .is_err(),
        "a lease shorter than one second has no provider deadline"
    );

    let short = provisioner
        .allocate(&lease_spec("short"), Duration::from_secs(1))
        .await
        .unwrap();
    let generated = short.access.private_key().expect("the SDK generated a key");
    assert_eq!(
        ssh_key::PrivateKey::from_openssh(generated)
            .unwrap()
            .public_key()
            .to_openssh()
            .unwrap(),
        short.access.public_key
    );
    assert!(!format!("{short:?}").contains("PRIVATE KEY"));
    let launch: Value = peer
        .received_requests()
        .await
        .unwrap()
        .iter()
        .find(|request| request.method == "POST")
        .unwrap()
        .body_json()
        .unwrap();
    assert_eq!(launch["env"]["PUBLIC_KEY"], json!(short.access.public_key));
    assert!(!launch.to_string().contains("PRIVATE KEY"));

    let mut supplied = lease_spec("long");
    supplied.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    let long = provisioner
        .allocate(&supplied, Duration::from_secs(3600))
        .await
        .unwrap();
    assert!(long.access.private_key().is_none());

    state
        .lock()
        .unwrap()
        .1
        .get_mut(&short.resource.instance.id)
        .unwrap()["env"]["BPG_EXPIRES_AT"] =
        json!((chrono::Utc::now() + chrono::Duration::hours(1)).to_rfc3339());

    tokio::time::sleep(
        (short.resource.expires_at.unwrap() - chrono::Utc::now())
            .to_std()
            .unwrap_or_default()
            + Duration::from_millis(50),
    )
    .await;
    let restarted = CloudProvisioner::from_credentials(vec![credentials()])
        .unwrap()
        .with_lease_state_dir(directory.path())
        .unwrap();
    let report = restarted.retire_expired("lease-cell").await;
    assert!(report.failures.is_empty(), "{report:?}");
    assert_eq!(
        report.retired,
        vec![ManagedLeaseIdentity {
            namespace: "lease-cell".into(),
            lease_id: "short".into(),
        }],
        "a restarted allocator retires using its original authorization"
    );
    let remaining = restarted.list_managed("lease-cell").await.unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].lease_id, "long");
    assert_eq!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|request| request.method == "DELETE")
            .count(),
        1
    );
}

#[tokio::test]
async fn a_refusal_after_the_rental_fences_it_instead_of_renting_again() {
    use wiremock::matchers::path_regex;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = runpod_v2_peer().await;
    Mock::given(method("GET"))
        .and(path_regex("^/pods/.+$"))
        .respond_with(ResponseTemplate::new(429))
        .with_priority(1)
        .up_to_n_times(1)
        .mount(&peer)
        .await;
    let credentials = serde_json::from_value(json!({
        "provider":"runpod", "api_key":"synthetic-local-peer-key", "endpoint":peer.uri()
    }))
    .unwrap();
    let directory = tempfile::tempdir().unwrap();
    let provisioner = CloudProvisioner::from_credentials(vec![credentials])
        .unwrap()
        .initialize_lease_state_dir(directory.path())
        .await
        .unwrap();
    let result = provisioner
        .allocate(&lease_spec("throttled"), Duration::from_secs(3600))
        .await;
    assert!(result.is_err(), "{result:?}");
    let requests = peer.received_requests().await.unwrap();
    let count = |verb: &str| requests.iter().filter(|r| r.method == verb).count();
    assert_eq!((count("POST"), count("DELETE")), (1, 0));
    let listed = provisioner.list_managed("lease-cell").await.unwrap();
    assert_eq!(listed.len(), 1);
    assert!(
        provisioner
            .allocate(&lease_spec("throttled"), Duration::from_secs(3600))
            .await
            .is_err(),
        "lost generated key cannot rotate access"
    );
    assert_eq!(
        peer.received_requests()
            .await
            .unwrap()
            .iter()
            .filter(|request| request.method == "POST")
            .count(),
        1
    );
    provisioner
        .destroy_managed(
            &CloudProvider::RunPod,
            &ManagedLeaseIdentity {
                namespace: "lease-cell".into(),
                lease_id: "throttled".into(),
            },
            None,
        )
        .await
        .unwrap();
    assert!(
        provisioner
            .list_managed("lease-cell")
            .await
            .unwrap()
            .is_empty()
    );
}

#[tokio::test]
async fn restart_adopts_eight_b200_without_stock_or_a_new_price_ceiling() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) = runpod_peer_state().await;
    let directory = tempfile::tempdir().unwrap();
    CloudProvisioner::from_credentials(vec![
        serde_json::from_value(
            json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":peer.uri()}),
        )
        .unwrap(),
    ])
    .unwrap()
    .initialize_lease_state_dir(directory.path())
    .await
    .unwrap();
    let provisioner = || {
        CloudProvisioner::from_credentials(vec![
            serde_json::from_value(json!({
                "provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":peer.uri()
            }))
            .unwrap(),
        ])
        .unwrap()
        .with_lease_state_dir(directory.path())
        .unwrap()
    };
    let mut spec = lease_spec("restart-eight");
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.max_hourly_cost_usd = Some(40.0);
    spec.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    let ttl = Duration::from_secs(3600);
    let original = provisioner().allocate(&spec, ttl).await.unwrap();
    let stock = std::mem::take(&mut state.lock().unwrap().2);
    let recovered = provisioner()
        .allocate(&spec, ttl)
        .await
        .expect("owned capacity must recover without fresh stock");
    assert_eq!(
        recovered.resource.instance.id,
        original.resource.instance.id
    );
    assert_eq!(recovered.resource.expires_at, original.resource.expires_at);
    assert_eq!(recovered.resource.hourly_cost_usd, Some(36.0));
    state.lock().unwrap().2 = stock;
    for offer in &mut state.lock().unwrap().2 {
        offer["price"]["secure"] = json!(1.0);
    }
    let recovered = provisioner()
        .allocate(&spec, ttl)
        .await
        .expect("a cheaper new market quote cannot invalidate the original authorized rate");
    assert_eq!(recovered.resource.hourly_cost_usd, Some(36.0));
    assert_eq!(recovered.resource.expires_at, original.resource.expires_at);
    assert!(recovered.access.private_key().is_none());
    assert_eq!(
        state.lock().unwrap().0,
        1,
        "restart must never create another node"
    );
}

#[tokio::test]
async fn recovery_uses_private_authorization_and_keeps_the_attempt_fence() {
    use blueprint_remote_providers::managed::ProviderErrorCode;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) = runpod_peer_state().await;
    let directory = tempfile::tempdir().unwrap();
    let client = |key: &str| {
        CloudProvisioner::from_credentials(vec![
            serde_json::from_value(
                json!({"provider":"runpod","api_key":key,"endpoint":peer.uri()}),
            )
            .unwrap(),
        ])
        .unwrap()
    };
    let provisioner = client("synthetic-local-peer-key")
        .initialize_lease_state_dir(directory.path())
        .await
        .unwrap();
    let restart = || {
        client("synthetic-local-peer-key")
            .with_lease_state_dir(directory.path())
            .unwrap()
    };
    let mut spec = lease_spec("authorization");
    spec.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    spec.max_hourly_cost_usd = Some(20.0);
    spec.env
        .insert("SYNTHETIC_SECRET".into(), "synthetic-never-persist".into());
    let ttl = Duration::from_secs(3600);
    let original = provisioner.allocate(&spec, ttl).await.unwrap();
    let id = original.resource.instance.id.clone();
    let original_pod = state.lock().unwrap().1[&id].clone();
    let original_store =
        std::fs::read(directory.path().join("blueprint/lease-authorizations.json")).unwrap();
    let saved = String::from_utf8(original_store.clone()).unwrap();
    for secret in [
        "synthetic-local-peer-key",
        "synthetic-never-persist",
        "PRIVATE KEY",
    ] {
        assert!(!saved.contains(secret));
    }
    assert!(
        client("synthetic-local-peer-key")
            .initialize_lease_state_dir(directory.path())
            .await
            .is_err(),
        "initialization never clobbers existing authorization"
    );
    let other_directory = tempfile::tempdir().unwrap();
    let no_proof = client("synthetic-local-peer-key")
        .initialize_lease_state_dir(other_directory.path())
        .await
        .unwrap();
    assert!(
        no_proof.allocate(&spec, ttl).await.is_err(),
        "native ownership without original authorization cannot rent or adopt"
    );
    let changed_endpoint = MockServer::start().await;
    let changed_account=CloudProvisioner::from_credentials(vec![serde_json::from_value(json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":changed_endpoint.uri()})).unwrap()]).unwrap().with_lease_state_dir(directory.path()).unwrap();
    assert!(changed_account.recover_lease(&spec, ttl).await.is_err());
    assert!(
        changed_endpoint
            .received_requests()
            .await
            .unwrap()
            .is_empty(),
        "configuration mismatch fails before remote access"
    );
    // Account rotation may be the same vendor account, but this conservative binding cannot prove it.
    assert!(
        client("different-synthetic-key")
            .with_lease_state_dir(directory.path())
            .unwrap()
            .recover_lease(&spec, ttl)
            .await
            .is_err()
    );
    for mutate in 0..6 {
        let mut changed = spec.clone();
        match mutate {
            0 => changed.gpu_count = Some(1),
            1 => changed.image = Some("example.invalid/different:v2".into()),
            2 => changed.ownership.as_mut().unwrap().namespace = "foreign-namespace".into(),
            3 => changed.ssh_public_key = None,
            4 => changed.ssh_public_key = Some(
                "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAICYAqclxVqvBAc0OzItjBj4L68M4hqJPZbsnOUPuk0Ug"
                    .into(),
            ),
            _ => changed.env.clear(),
        }
        assert!(
            restart().allocate(&changed, ttl).await.is_err(),
            "request/key change {mutate}"
        );
    }
    for mutate in 0..5 {
        let mut pod = original_pod.clone();
        match mutate {
            0 => pod["cost"] = json!(10.0), // Original quote 9, caller cap 20: original ceiling still wins.
            1 => {
                pod["env"]["BPG_EXPIRES_AT"] = json!(
                    (original.resource.expires_at.unwrap() + chrono::Duration::hours(1))
                        .to_rfc3339()
                )
            }
            2 => pod["gpu"]["count"] = json!(1),
            3 => {
                pod["env"]
                    .as_object_mut()
                    .unwrap()
                    .remove("BPG_FINGERPRINT");
            }
            _ => pod["env"]["BPG_NAMESPACE"] = json!("foreign-namespace"),
        }
        state.lock().unwrap().1.insert(id.clone(), pod);
        assert!(
            restart().allocate(&spec, ttl).await.is_err(),
            "native mutation {mutate}"
        );
    }
    state
        .lock()
        .unwrap()
        .1
        .insert(id.clone(), original_pod.clone());
    assert_eq!(
        restart()
            .recover_lease(&spec, ttl)
            .await
            .unwrap()
            .unwrap()
            .resource
            .instance
            .id,
        id
    );
    Mock::given(method("GET"))
        .and(path("/pods"))
        .respond_with(ResponseTemplate::new(503))
        .with_priority(1)
        .up_to_n_times(1)
        .mount(&peer)
        .await;
    assert!(
        restart().recover_lease(&spec, ttl).await.is_err(),
        "listing error cannot prove absence"
    );
    state.lock().unwrap().1.clear();
    assert!(restart().recover_lease(&spec, ttl).await.unwrap().is_none());
    assert!(
        restart().allocate(&spec, ttl).await.is_err(),
        "even proven absence cannot cross an attempted fence"
    );
    assert_eq!(state.lock().unwrap().0, 1);
    state.lock().unwrap().1.insert(id.clone(), original_pod);
    let provisioner = provisioner.only_providers(&[]); // Selection cannot remove retained checked clients.
    let before_state_failure = peer.received_requests().await.unwrap().len();
    std::fs::write(
        directory.path().join("blueprint/lease-authorizations.json"),
        b"truncated",
    )
    .unwrap();
    assert!(
        client("synthetic-local-peer-key")
            .with_lease_state_dir(directory.path())
            .is_err()
    );
    let corrupt = provisioner.retire_expired("lease-cell").await;
    assert!(corrupt.retired.is_empty());
    assert!(
        corrupt
            .failures
            .iter()
            .any(|failure| failure.provider == CloudProvider::RunPod
                && failure.code == ProviderErrorCode::Conflict)
    );
    assert_eq!(
        peer.received_requests().await.unwrap().len(),
        before_state_failure,
        "corrupt state fails before native access even after selection narrows to no offers"
    );
    std::fs::write(
        directory.path().join("blueprint/lease-authorizations.json"),
        original_store,
    )
    .unwrap();
    std::fs::remove_file(directory.path().join("blueprint/lease-authorizations.json")).unwrap();
    assert!(
        client("synthetic-local-peer-key")
            .with_lease_state_dir(directory.path())
            .is_err(),
        "lost state cannot import mutable provider metadata"
    );
    let lost = provisioner.retire_expired("lease-cell").await;
    assert!(lost.retired.is_empty());
    assert!(
        lost.failures
            .iter()
            .any(|failure| failure.provider == CloudProvider::RunPod
                && failure.code == ProviderErrorCode::Conflict)
    );
    assert_eq!(
        peer.received_requests().await.unwrap().len(),
        before_state_failure,
        "lost state fails before native access"
    );
    assert!(
        state.lock().unwrap().1.contains_key(&id),
        "failed retirement preserves the original native capacity"
    );
    assert_eq!(state.lock().unwrap().0, 1);
    assert!(matches!(
        provisioner.allocate(&spec, ttl).await,
        Err(blueprint_remote_providers::Error::Provider(
            ProviderErrorCode::Conflict | ProviderErrorCode::InvalidRequest
        ))
    ));
}

#[tokio::test]
async fn lost_ack_recovers_once_and_selection_retains_original_accounts() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) = runpod_peer_state().await;
    let other = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/instances"))
        .respond_with(ResponseTemplate::new(200).set_body_json(json!({"instances":[]})))
        .mount(&other)
        .await;
    let credentials = || {
        vec![
        serde_json::from_value(json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":peer.uri()})).unwrap(),
        serde_json::from_value(json!({"provider":"vast-ai","api_key":"other-synthetic-key","endpoint":other.uri()})).unwrap(),
    ]
    };
    let directory = tempfile::tempdir().unwrap();
    let provisioner = CloudProvisioner::from_credentials(credentials())
        .unwrap()
        .initialize_lease_state_dir(directory.path())
        .await
        .unwrap()
        .only_providers(&[CloudProvider::RunPod]);
    let mut spec = lease_spec("lost-ack");
    spec.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    let ttl = Duration::from_secs(3600);
    // The peer commits the rental, then loses the response. This is a real HTTP side effect.
    let handle = state.clone();
    let authority_path = directory.path().join("blueprint/lease-authorizations.json");
    Mock::given(method("POST")).and(path("/pods")).respond_with(move |request:&wiremock::Request| {
        let body:Value=request.body_json().unwrap();
        let saved:Value=serde_json::from_slice(&std::fs::read(&authority_path).unwrap()).unwrap();
        assert_eq!(saved["entries"]["lost-ack"]["metadata"]["expires_at"].as_str().unwrap().parse::<chrono::DateTime<chrono::Utc>>().unwrap(),body["env"]["BPG_EXPIRES_AT"].as_str().unwrap().parse::<chrono::DateTime<chrono::Utc>>().unwrap());
        let mut state=handle.lock().unwrap();
        state.0+=1;
        state.1.insert("accepted-without-ack".into(),json!({"id":"accepted-without-ack","name":body["name"],"gpu":{"id":body["gpu"]["id"],"count":body["gpu"]["count"],"vcpuCount":8,"memory":64},"env":body["env"],"ports":body["ports"],"image":body["image"],"status":"RUNNING","dataCenterId":"US-TEST-1","cost":9.0,"ssh":{"direct":{"host":"192.0.2.10","port":22334,"username":"root"}}}));
        ResponseTemplate::new(503)
    }).with_priority(1).up_to_n_times(1).mount(&peer).await;
    assert!(provisioner.allocate(&spec, ttl).await.is_err());
    state.lock().unwrap().2.clear();
    let restart = || {
        CloudProvisioner::from_credentials(credentials())
            .unwrap()
            .with_lease_state_dir(directory.path())
            .unwrap()
            .only_providers(&[CloudProvider::RunPod])
    };
    let recovered = restart().recover_lease(&spec, ttl).await.unwrap().unwrap();
    assert_eq!(recovered.resource.instance.id, "accepted-without-ack");
    let deadline = recovered.resource.expires_at;
    assert_eq!(
        restart()
            .allocate(&spec, ttl)
            .await
            .unwrap()
            .resource
            .expires_at,
        deadline
    );
    let missing_account = CloudProvisioner::from_credentials(vec![credentials().remove(0)])
        .unwrap()
        .with_lease_state_dir(directory.path())
        .unwrap();
    assert!(missing_account.recover_lease(&spec, ttl).await.is_err());
    assert_eq!(state.lock().unwrap().0, 1);
    assert!(
        !other.received_requests().await.unwrap().is_empty(),
        "filtered recovery still accounts for the original second account"
    );
}

#[tokio::test]
async fn retirement_checks_original_accounts_before_listing_and_isolates_each_obligation() {
    use blueprint_remote_providers::managed::ProviderErrorCode;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (original, original_state) = runpod_peer_state().await;
    let (current, current_state) = runpod_peer_state().await;
    let wrong = runpod_v2_peer().await;
    let directory = tempfile::tempdir().unwrap();
    let client = |endpoint: String| {
        CloudProvisioner::from_credentials(vec![serde_json::from_value(json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":endpoint})).unwrap()]).unwrap()
    };
    let first = client(original.uri())
        .initialize_lease_state_dir(directory.path())
        .await
        .unwrap()
        .allocate(&lease_spec("original-account"), Duration::from_secs(1))
        .await
        .unwrap();
    // A new explicit intent can authorize another account without rewriting the old fence.
    let second = client(current.uri())
        .with_lease_state_dir(directory.path())
        .unwrap()
        .allocate(&lease_spec("current-account"), Duration::from_secs(1))
        .await
        .unwrap();
    let original_requests = original.received_requests().await.unwrap().len();
    tokio::time::sleep(
        (first
            .resource
            .expires_at
            .unwrap()
            .max(second.resource.expires_at.unwrap())
            - chrono::Utc::now())
        .to_std()
        .unwrap_or_default()
            + Duration::from_millis(50),
    )
    .await;
    let wrong_report = client(wrong.uri())
        .with_lease_state_dir(directory.path())
        .unwrap()
        .retire_expired("lease-cell")
        .await;
    assert!(wrong_report.retired.is_empty());
    assert!(!wrong_report.failures.is_empty());
    assert!(
        wrong_report
            .failures
            .iter()
            .all(|failure| failure.provider == CloudProvider::RunPod
                && failure.code == ProviderErrorCode::Conflict)
    );
    assert!(
        wrong.received_requests().await.unwrap().is_empty(),
        "a changed empty account is never an absence proof or contacted for original retirement"
    );
    assert_eq!(original_state.lock().unwrap().1.len(), 1);
    assert_eq!(current_state.lock().unwrap().1.len(), 1);

    let report = client(current.uri())
        .with_lease_state_dir(directory.path())
        .unwrap()
        .retire_expired("lease-cell")
        .await;
    assert_eq!(
        report.retired,
        vec![lease_spec("current-account").ownership.unwrap()]
    );
    assert!(
        report
            .failures
            .iter()
            .any(|failure| failure.provider == CloudProvider::RunPod
                && failure.code == ProviderErrorCode::Conflict)
    );
    assert!(
        current_state.lock().unwrap().1.is_empty(),
        "unchanged separately authorized account still retires despite another binding failure"
    );
    assert_eq!(original_state.lock().unwrap().1.len(), 1);
    assert_eq!(
        original.received_requests().await.unwrap().len(),
        original_requests
    );

    assert!(
        matches!(
            CloudProvisioner::from_credentials(Vec::new())
                .unwrap()
                .with_lease_state_dir(directory.path()),
            Err(blueprint_remote_providers::Error::Provider(
                ProviderErrorCode::ProviderNotConfigured
            ))
        ),
        "removing every usable native account rejects context setup"
    );
    let other = MockServer::start().await;
    let missing = CloudProvisioner::from_credentials(vec![
        serde_json::from_value(
            json!({"provider":"vast-ai","api_key":"other-synthetic-key","endpoint":other.uri()}),
        )
        .unwrap(),
    ])
    .unwrap()
    .with_regions(std::collections::HashMap::from([(
        CloudProvider::VastAi,
        "synthetic-region".into(),
    )]))
    .with_lease_state_dir(directory.path())
    .unwrap()
    .retire_expired("lease-cell")
    .await;
    assert!(
        other.received_requests().await.unwrap().is_empty(),
        "an unrelated valid native client cannot erase or satisfy the missing original obligation"
    );
    assert!(missing.retired.is_empty());
    assert!(
        missing
            .failures
            .iter()
            .any(|failure| failure.provider == CloudProvider::RunPod
                && failure.code == ProviderErrorCode::ProviderNotConfigured),
        "stored obligations survive removal from both clients and regions"
    );
    assert_eq!(original_state.lock().unwrap().1.len(), 1);
    assert_eq!(
        original.received_requests().await.unwrap().len(),
        original_requests
    );
}

#[tokio::test]
async fn concurrent_lease_identities_retain_both_authorizations_in_one_store() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) = runpod_peer_state().await;
    let directory = tempfile::tempdir().unwrap();
    let client = || {
        CloudProvisioner::from_credentials(vec![serde_json::from_value(json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":peer.uri()})).unwrap()]).unwrap()
    };
    client()
        .initialize_lease_state_dir(directory.path())
        .await
        .unwrap();
    let first = client().with_lease_state_dir(directory.path()).unwrap();
    let second = client().with_lease_state_dir(directory.path()).unwrap();
    let mut a = lease_spec("concurrent-a");
    a.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    let mut b = a.clone();
    b.ownership.as_mut().unwrap().lease_id = "concurrent-b".into();
    let ttl = Duration::from_secs(3600);
    let (a_rented, b_rented) = tokio::join!(first.allocate(&a, ttl), second.allocate(&b, ttl));
    let (a_rented, b_rented) = (a_rented.unwrap(), b_rented.unwrap());
    assert_ne!(a_rented.resource.instance.id, b_rented.resource.instance.id);
    let restarted = client().with_lease_state_dir(directory.path()).unwrap();
    assert_eq!(
        restarted
            .recover_lease(&a, ttl)
            .await
            .unwrap()
            .unwrap()
            .resource
            .instance
            .id,
        a_rented.resource.instance.id
    );
    assert_eq!(
        restarted
            .recover_lease(&b, ttl)
            .await
            .unwrap()
            .unwrap()
            .resource
            .instance
            .id,
        b_rented.resource.instance.id
    );
    assert_eq!(
        state.lock().unwrap().0,
        2,
        "both concurrent writes remain recoverable without additional create"
    );
}

#[cfg(unix)]
#[tokio::test]
async fn authorization_persistence_failure_precedes_even_native_template_creation() {
    use std::os::unix::fs::PermissionsExt;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    assert_ne!(
        unsafe { libc::geteuid() },
        0,
        "this permission-failure proof requires an unprivileged test process"
    );
    let peer = super::lium::lium_native_peer().await;
    let directory = tempfile::tempdir().unwrap();
    let provisioner=CloudProvisioner::from_credentials(vec![serde_json::from_value(json!({"provider":"bittensor-lium","api_key":"synthetic-local-peer-key","endpoint":peer.uri()})).unwrap()]).unwrap().initialize_lease_state_dir(directory.path()).await.unwrap();
    let mut spec = lease_spec("persist-before-template");
    spec.gpu_class = Some("nvidia-b200".into());
    spec.gpu_count = Some(8);
    spec.gpu_vram_gb = Some(180);
    spec.region = Some("NSW".into());
    spec.lease_duration_secs = Some(3600);
    spec.ssh_public_key = Some(
        "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIMo32AY0vFK7g5FBIBcyPdxaxSEM5rEc0kEzMVveA9b+".into(),
    );
    assert_eq!(
        provisioner.quotes(&spec).await.unwrap().quotes.len(),
        1,
        "the existing recorded Lium fixture must actually offer this request"
    );
    let before = peer.received_requests().await.unwrap().len();
    let sdk_dir = directory.path().join("blueprint");
    let authority = std::fs::read(sdk_dir.join("lease-authorizations.json")).unwrap();
    std::fs::set_permissions(&sdk_dir, std::fs::Permissions::from_mode(0o500)).unwrap();
    let result = provisioner.allocate(&spec, Duration::from_secs(3600)).await;
    std::fs::set_permissions(&sdk_dir, std::fs::Permissions::from_mode(0o700)).unwrap();
    assert!(result.is_err());
    let requests = peer.received_requests().await.unwrap();
    let attempted = &requests[before..];
    assert!(
        attempted
            .iter()
            .filter(|request| request.url.path() == "/public/v1/nodes")
            .count()
            >= 2,
        "allocation reached current-stock and pre-create revalidation before persistence failed"
    );
    assert!(
        attempted.iter().all(|request| request.method == "GET"),
        "failed persistence must precede templates, rent, and every other native mutation"
    );
    assert_eq!(
        std::fs::read(sdk_dir.join("lease-authorizations.json")).unwrap(),
        authority,
        "a failed snapshot write retains the previous complete authority"
    );
}

#[tokio::test]
async fn unsupported_lease_contexts_fail_before_files_or_provider_effects() {
    use blueprint_remote_providers::infra::traits::CloudProviderAdapter;
    use blueprint_remote_providers::providers::RunPodAdapter;
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let peer = runpod_v2_peer().await;
    // This is the real native adapter behind the public custom-client constructor,
    // which intentionally lacks the checked credential provenance of the typed factory.
    let unbound = || {
        CloudProvisioner::with_providers(std::collections::HashMap::from([(
            CloudProvider::RunPod,
            Box::new(
                RunPodAdapter::with_endpoint("synthetic-local-peer-key".into(), &peer.uri())
                    .unwrap(),
            ) as Box<dyn CloudProviderAdapter>,
        )]))
    };
    let empty = || CloudProvisioner::from_credentials(Vec::new()).unwrap();
    for unsupported in [empty(), unbound()] {
        let directory = tempfile::tempdir().unwrap();
        assert!(
            unsupported
                .initialize_lease_state_dir(directory.path())
                .await
                .is_err()
        );
        assert_eq!(
            std::fs::read_dir(directory.path()).unwrap().count(),
            0,
            "unsupported setup must not even create SDK directories or locks"
        );
    }
    let directory = tempfile::tempdir().unwrap();
    CloudProvisioner::from_credentials(vec![
        serde_json::from_value(
            json!({"provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":peer.uri()}),
        )
        .unwrap(),
    ])
    .unwrap()
    .initialize_lease_state_dir(directory.path())
    .await
    .unwrap();
    let authority =
        std::fs::read(directory.path().join("blueprint/lease-authorizations.json")).unwrap();
    for unsupported in [empty(), unbound()] {
        assert!(unsupported.with_lease_state_dir(directory.path()).is_err());
        assert_eq!(
            std::fs::read(directory.path().join("blueprint/lease-authorizations.json")).unwrap(),
            authority
        );
    }
    assert!(
        peer.received_requests().await.unwrap().is_empty(),
        "configuration validation precedes every provider request"
    );
}
