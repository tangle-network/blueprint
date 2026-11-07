//! Public lifecycle calls must never acknowledge an unobserved resource.
use blueprint_remote_providers::{
    CloudProvider, Error, InstanceStatus, ProvisionedInstance,
    infra::traits::{BlueprintDeploymentResult, CloudProviderAdapter},
    managed::ProviderErrorCode,
    providers::coreweave::CoreWeaveAdapter,
};

#[test]
fn coreweave_does_not_acknowledge_placeholder_resources() {
    const CHILD: &str = "BPG_COREWEAVE_LIFECYCLE_CHILD";
    if std::env::var_os(CHILD).is_none() {
        let status = std::process::Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "coreweave_does_not_acknowledge_placeholder_resources",
                "--nocapture",
            ])
            .env_clear()
            .env(CHILD, "1")
            .env("COREWEAVE_TOKEN", "synthetic-key")
            .env("COREWEAVE_NAMESPACE", "synthetic-cell")
            .status()
            .unwrap();
        assert!(status.success(), "isolated CoreWeave consumer failed");
        return;
    }
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            let adapter = CoreWeaveAdapter::new().await.unwrap();
            assert!(
                matches!(
                    adapter
                        .provision_instance("H100", "synthetic-region", false)
                        .await,
                    Err(Error::Provider(ProviderErrorCode::Unsupported))
                ),
                "a local UUID is not a provider-confirmed VM"
            );
            assert!(
                matches!(
                    adapter.get_instance_status("synthetic-vm").await,
                    Err(Error::Provider(ProviderErrorCode::Unsupported))
                ),
                "a missing provider read cannot establish Running"
            );
            assert!(
                matches!(
                    adapter.terminate_instance("synthetic-vm").await,
                    Err(Error::Provider(ProviderErrorCode::Unsupported))
                ),
                "a missing provider delete cannot establish retirement"
            );
            let deployment = BlueprintDeploymentResult {
                instance: ProvisionedInstance {
                    id: "synthetic-vm".into(),
                    provider: CloudProvider::CoreWeave,
                    instance_type: "H100".into(),
                    region: "synthetic-region".into(),
                    public_ip: None,
                    private_ip: None,
                    status: InstanceStatus::Running,
                },
                blueprint_id: "synthetic-workload".into(),
                port_mappings: Default::default(),
                metadata: Default::default(),
            };
            assert!(
                matches!(
                    adapter.cleanup_blueprint(&deployment).await,
                    Err(Error::Provider(ProviderErrorCode::Unsupported))
                ),
                "a missing Kubernetes delete cannot establish cleanup"
            );
            assert!(
                adapter.managed().is_none(),
                "an incomplete lifecycle must not be advertised"
            );
        });
}
