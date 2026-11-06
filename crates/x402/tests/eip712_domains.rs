//! Golden EIP-712 domains, checked against values read from chain on 2026-10-27.
//!
//! A wrong domain makes every payment signature fail to verify, so these compare against
//! the deployed contracts' `DOMAIN_SEPARATOR()` rather than against our own encoding.

use alloy_primitives::{Address, B256, address, b256};
use alloy_sol_types::eip712_domain;
use blueprint_x402::config::AcceptedToken;
use rust_decimal::Decimal;
use x402_chain_eip155::chain::AssetTransferMethod;
use x402_chain_eip155::chain::permit2::PERMIT2_ADDRESS;

/// USDC on Base (`0x8335…2913`, chain 8453): `name()` "USD Coin", `version()` "2".
const USDC_BASE: Address = address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913");
const USDC_BASE_DOMAIN_SEPARATOR: B256 =
    b256!("0x02fa7265e7c5d81118673727957699e4d68f74cd74b7db77da710fe8a2c7834f");
/// Uniswap Permit2 (`0x0000…8BA3`) on Ethereum mainnet.
const PERMIT2_MAINNET_DOMAIN_SEPARATOR: B256 =
    b256!("0x866a5aba21966af95d6c7ab78eb2b2fc913915c28be3b9aa07cc04ff903e3f28");

fn usdc_base(transfer_method: &str) -> AcceptedToken {
    AcceptedToken {
        network: "eip155:8453".into(),
        asset: USDC_BASE.to_string(),
        symbol: "USDC".into(),
        decimals: 6,
        pay_to: "0x0000000000000000000000000000000000000001".into(),
        rate_per_native_unit: Decimal::from(3200u32),
        markup_bps: 0,
        transfer_method: transfer_method.into(),
        eip712_name: "USD Coin".into(),
        eip712_version: "2".into(),
    }
}

fn token_domain_separator(method: AssetTransferMethod) -> B256 {
    let (AssetTransferMethod::Eip3009 { name, version }
    | AssetTransferMethod::Permit2 { name, version }) = method;
    eip712_domain! {
        name: name,
        version: version,
        chain_id: 8453,
        verifying_contract: USDC_BASE,
    }
    .separator()
}

#[test]
fn usdc_on_base_token_domain_matches_chain_for_both_transfer_methods() {
    for token in [usdc_base("permit2"), usdc_base("eip3009")] {
        token.validate().expect("valid token");
        let method = token.asset_transfer_method();
        assert_eq!(token_domain_separator(method), USDC_BASE_DOMAIN_SEPARATOR);
    }
}

#[test]
fn a_domain_is_stated_never_guessed_and_unknown_methods_are_refused() {
    let token = |extra: &str| {
        toml::from_str::<AcceptedToken>(&format!(
            "network = \"eip155:8453\"\nasset = \"{USDC_BASE}\"\nsymbol = \"USDC\"\ndecimals = 6\n\
             pay_to = \"0x0000000000000000000000000000000000000001\"\nrate_per_native_unit = \"3200\"\n{extra}"
        ))
    };
    // No default domain for either method.
    assert!(token("transfer_method = \"eip3009\"").is_err());
    // Older configs keep loading through the eip3009_* aliases.
    let old = token("transfer_method = \"eip3009\"\neip3009_name = \"USD Coin\"\neip3009_version = \"2\"");
    assert_eq!(old.expect("aliases load").asset_transfer_method(), usdc_base("eip3009").asset_transfer_method());
    // A near-miss method never silently becomes permit2.
    let mut typo = usdc_base("permit2");
    typo.transfer_method = "EIP3009".into();
    assert!(typo.validate().is_err());
}

#[test]
fn library_permit2_domain_matches_mainnet_permit2() {
    // x402-chain-eip155 builds this domain inline in its signing functions (name "Permit2",
    // the chain id, its PERMIT2_ADDRESS, no version) and exports no builder, so this pins the
    // library's address constant and that domain shape to mainnet Permit2's separator.
    let domain = eip712_domain! {
        name: "Permit2",
        chain_id: 1,
        verifying_contract: PERMIT2_ADDRESS,
    };
    assert_eq!(domain.separator(), PERMIT2_MAINNET_DOMAIN_SEPARATOR);
}
