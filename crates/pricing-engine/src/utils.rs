use blueprint_tangle_extra::serde::{BoundedVec, new_bounded_string};
use rust_decimal::Decimal;
use rust_decimal::prelude::ToPrimitive;
use tangle_subxt::{
    subxt::utils::H160,
    tangle_testnet_runtime::api::runtime_types::{
        sp_arithmetic::per_things::Percent,
        tangle_primitives::services::{
            pricing::{PricingQuote, ResourcePricing},
            types::{Asset, AssetSecurityCommitment},
        },
    },
};

use crate::pricing_engine::{QuoteDetails, asset::AssetType};

/// Pricing scale factor - used to convert decimal prices to integers
// Cannot use Decimal::new in const, define it as a function instead
fn pricing_scale() -> Decimal {
    Decimal::new(1_000_000_000, 0)
}

/// Convert a u128 value to a 16-byte Vec<u8> in little-endian byte order
///
/// # Arguments
///
/// * `value` - The u128 value to convert
///
/// # Returns
///
/// A Vec<u8> containing the 16-byte little-endian representation of the u128 value
pub fn u128_to_bytes(value: u128) -> Vec<u8> {
    value.to_le_bytes().to_vec()
}

/// Convert a byte slice to a u128 value, assuming little-endian byte order
///
/// # Arguments
///
/// * `bytes` - The byte slice to convert, must be exactly 16 bytes
///
/// # Returns
///
/// The u128 value represented by the bytes
///
/// # Panics
///
/// Panics if the byte slice is not exactly 16 bytes long
pub fn bytes_to_u128(bytes: &[u8]) -> u128 {
    let mut array = [0u8; 16];
    if bytes.len() != 16 {
        panic!("bytes_to_u128: Expected 16 bytes, got {}", bytes.len());
    }
    array.copy_from_slice(bytes);
    u128::from_le_bytes(array)
}

/// Convert a u32 value to a 16-byte Vec<u8> in little-endian byte order
/// This is useful when you have a u32 but need to store it as a u128 in bytes
///
/// # Arguments
///
/// * `value` - The u32 value to convert
///
/// # Returns
///
/// A Vec<u8> containing the 16-byte little-endian representation of the u32 value
/// (with the higher order bytes set to 0)
pub fn u32_to_u128_bytes(value: u32) -> Vec<u8> {
    let mut bytes = [0u8; 16];
    // Copy the u32 bytes into the first 4 bytes of the u128 array (little-endian)
    bytes[0..4].copy_from_slice(&value.to_le_bytes());
    bytes.to_vec()
}

/// Convert a QuoteDetails to a PricingQuote
///
/// # Arguments
///
/// * `quote_details` - The QuoteDetails to convert
///
/// # Returns
///
/// A PricingQuote containing the converted data
///
/// # Panics
///
/// Panics if any type conversions fails
pub fn create_on_chain_quote_type(quote_details: &QuoteDetails) -> PricingQuote {
    let security_commitment = quote_details.security_commitments.clone().unwrap();
    let scale = pricing_scale();

    let mapped_resources: Vec<tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::pricing::ResourcePricing> = quote_details.resources
                .iter()
                .map(|resource| {
                    // Convert f64 to Decimal for the calculation
                    let price_rate = Decimal::try_from(resource.price_per_unit_rate).unwrap_or(Decimal::ZERO);
                    tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::pricing::ResourcePricing {
                        kind: new_bounded_string(resource.kind.clone()),
                        count: resource.count,
                        price_per_unit_rate: (price_rate * scale).to_u128().unwrap_or(0),
                    }
                })
                .collect();
    let resources = BoundedVec::<ResourcePricing>(mapped_resources.clone());

    let inner_asset_type = security_commitment.asset.unwrap().asset_type.unwrap();
    let asset = match inner_asset_type {
        AssetType::Custom(asset) => {
            let asset_id = bytes_to_u128(&asset);
            Asset::Custom(asset_id)
        }
        AssetType::Erc20(address) => {
            let address_bytes: [u8; 20] = address
                .as_slice()
                .try_into()
                .expect("ERC20 address should be 20 bytes");
            Asset::Erc20(H160::from(address_bytes))
        }
    };
    let exposure_percent = Percent(security_commitment.exposure_percent as u8);
    let mapped_security_commitment =
                vec![tangle_subxt::tangle_testnet_runtime::api::runtime_types::tangle_primitives::services::types::AssetSecurityCommitment {
                    asset: asset.clone(),
                    exposure_percent,
                }];

    let security_commitments =
        BoundedVec::<AssetSecurityCommitment<u128>>(mapped_security_commitment.clone());

    // Convert f64 to Decimal for the calculation
    let total_cost_rate = Decimal::try_from(quote_details.total_cost_rate).unwrap_or(Decimal::ZERO);

    PricingQuote {
        blueprint_id: quote_details.blueprint_id,
        ttl_blocks: quote_details.ttl_blocks,
        resources,
        security_commitments,
        total_cost_rate: (total_cost_rate * scale).to_u128().unwrap_or(0),
        timestamp: quote_details.timestamp,
        expiry: quote_details.expiry,
    }
}
