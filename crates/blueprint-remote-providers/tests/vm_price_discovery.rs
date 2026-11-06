//! VM selection through the exported discovery boundary, without provider effects.
use blueprint_remote_providers::CloudProvider;
use blueprint_remote_providers::monitoring::discovery::{CloudCredentials, MachineTypeDiscovery};

#[tokio::test]
async fn a_provider_without_a_live_quote_cannot_win_vm_selection() {
    let mut discovery = MachineTypeDiscovery::new();
    if let Ok(machines) = discovery
        .discover_machine_types(
            &CloudProvider::AWS,
            "us-east-1",
            &CloudCredentials::default(),
        )
        .await
    {
        assert!(
            machines
                .iter()
                .all(|machine| machine.hourly_price.is_none() && machine.spot_price.is_none()),
            "discovery presented handwritten fallback prices as provider quotes"
        );
    }
    assert!(
        discovery
            .find_best_match(&CloudProvider::AWS, 2, 1.0, false, Some(10.0))
            .is_none(),
        "an unpriced VM remained selectable with a caller spending cap"
    );
    assert!(
        discovery
            .find_best_match(&CloudProvider::AWS, 2, 1.0, false, None)
            .is_none(),
        "removing a spending cap made an unknown price selectable"
    );
}
