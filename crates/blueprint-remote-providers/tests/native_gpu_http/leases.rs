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
    let peer = runpod_v2_peer().await;
    let credentials = || {
        serde_json::from_value(json!({
            "provider":"runpod", "api_key":"synthetic-local-peer-key", "endpoint":peer.uri()
        }))
        .unwrap()
    };
    let provisioner = CloudProvisioner::from_credentials(vec![credentials()]).unwrap();
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

    tokio::time::sleep(
        (short.resource.expires_at.unwrap() - chrono::Utc::now())
            .to_std()
            .unwrap_or_default()
            + Duration::from_millis(50),
    )
    .await;
    let restarted = CloudProvisioner::from_credentials(vec![credentials()]).unwrap();
    let report = restarted.retire_expired("lease-cell").await;
    assert!(report.failures.is_empty(), "{report:?}");
    assert_eq!(
        report.retired,
        vec![ManagedLeaseIdentity {
            namespace: "lease-cell".into(),
            lease_id: "short".into(),
        }],
        "a restarted allocator retires from the provider listing alone"
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
async fn a_refusal_after_the_rental_retires_it_instead_of_renting_again() {
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
    let provisioner = CloudProvisioner::from_credentials(vec![credentials]).unwrap();
    let result = provisioner
        .allocate(&lease_spec("throttled"), Duration::from_secs(3600))
        .await;
    assert!(result.is_err(), "{result:?}");
    let requests = peer.received_requests().await.unwrap();
    let count = |verb: &str| requests.iter().filter(|r| r.method == verb).count();
    assert_eq!((count("POST"), count("DELETE")), (1, 1));
    let listed = provisioner.list_managed("lease-cell").await.unwrap();
    assert!(listed.is_empty(), "{listed:?}");
}

#[tokio::test]
async fn restart_adopts_eight_b200_without_stock_or_a_new_price_ceiling() {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();
    let (peer, state) = runpod_peer_state().await;
    let provisioner = || {
        CloudProvisioner::from_credentials(vec![
            serde_json::from_value(json!({
                "provider":"runpod","api_key":"synthetic-local-peer-key","endpoint":peer.uri()
            }))
            .unwrap(),
        ])
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
