use crate::bindings::{EmptyContract, ProxyAdmin, TransparentUpgradeableProxy};
use alloy_primitives::{Address, Bytes};
use alloy_provider::RootProvider;
use blueprint_sdk::testing::chain_setup::anvil::get_receipt;
use color_eyre::eyre::eyre;

/// Helper function to deploy an empty proxy
pub async fn deploy_empty_proxy(
    wallet: &RootProvider,
    proxy_admin: Address,
) -> color_eyre::eyre::Result<Address> {
    let data = Bytes::new();

    let empty_contract = EmptyContract::deploy(wallet).await?;
    let &empty_contract_addr = empty_contract.address();

    let proxy =
        TransparentUpgradeableProxy::deploy(wallet, empty_contract_addr, proxy_admin, data).await?;

    Ok(*proxy.address())
}

/// Helper function to upgrade a proxy with an implementation
pub async fn upgrade_proxy(
    wallet: &RootProvider,
    proxy_admin_addr: Address,
    proxy_addr: Address,
    implementation_addr: Address,
    data: Bytes,
) -> color_eyre::eyre::Result<()> {
    let proxy_admin = ProxyAdmin::new(proxy_admin_addr, wallet.clone());

    let receipt = if data.is_empty() {
        let call = proxy_admin.upgrade(proxy_addr, implementation_addr);
        get_receipt(call).await?
    } else {
        let call = proxy_admin.upgradeAndCall(proxy_addr, implementation_addr, data);
        get_receipt(call).await?
    };

    if !receipt.status() {
        return Err(eyre!("Failed to upgrade proxy"));
    }

    Ok(())
}
