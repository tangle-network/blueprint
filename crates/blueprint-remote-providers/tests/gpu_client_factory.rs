//! A fresh SDK consumer constructs its provider client without global TLS setup.
use blueprint_remote_providers::{CloudProvisioner, managed::ProviderCredentials};

#[tokio::test]
async fn caller_owned_credentials_construct_the_native_client() {
    let result = CloudProvisioner::from_credentials(vec![ProviderCredentials::RunPod {
        api_key: "synthetic-caller-owned-key".into(),
        endpoint: Some("http://127.0.0.1:0".into()),
        region: None,
    }]);
    assert!(
        result.is_ok(),
        "public provider factory must construct its client"
    );
}
