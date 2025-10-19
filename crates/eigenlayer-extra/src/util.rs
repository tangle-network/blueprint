use alloy_primitives::{Address, keccak256};
use blueprint_crypto_bn254::ArkBlsBn254Secret;
use blueprint_evm_extra::util::get_provider_http;
use eigensdk::crypto_bls::{BlsKeyPair, OperatorId, error::BlsError};

/// Get the allocation manager address from the `DelegationManager` contract
///
/// # Returns
/// - [`Address`] - The allocation manager address
///
/// # Errors
/// - [`Error::Contract`] - If the call to the contract fails (i.e. the contract doesn't exist at the given address)
pub async fn get_allocation_manager_address(
    delegation_manager_addr: Address,
    http_endpoint: &str,
) -> Result<Address, alloy_contract::Error> {
    let provider = get_provider_http(http_endpoint);
    let delegation_manager =
        eigensdk::utils::slashing::core::delegation_manager::DelegationManager::DelegationManagerInstance::new(
            delegation_manager_addr,
            provider,
        );
    delegation_manager.allocationManager().call().await
}

/// Generate the Operator ID from the BLS Keypair
///
/// # Returns
/// - [`OperatorId`] - The operator ID
#[must_use]
pub fn operator_id_from_key(key: &BlsKeyPair) -> OperatorId {
    let pub_key = key.public_key();
    let pub_key_affine = pub_key.g1();

    let x_int: num_bigint::BigUint = pub_key_affine.x.into();
    let y_int: num_bigint::BigUint = pub_key_affine.y.into();

    let x_bytes = x_int.to_bytes_be();
    let y_bytes = y_int.to_bytes_be();

    keccak256([x_bytes, y_bytes].concat())
}

/// Generate the Operator ID from the Ark BLS Keypair
///
/// # Returns
/// - [`OperatorId`] - The operator ID
///
/// # Errors
/// - [`BlsError`] - If the key is invalid
pub fn operator_id_from_ark_bls_bn254(key: &ArkBlsBn254Secret) -> Result<OperatorId, BlsError> {
    BlsKeyPair::new(key.0.to_string()).map(|key| operator_id_from_key(&key))
}

/// Validate an operator address
///
/// Ensures the address is not zero and is properly formatted.
///
/// # Errors
/// - Returns error if address is zero
pub fn validate_operator_address(
    address: Address,
) -> Result<(), crate::error::EigenlayerExtraError> {
    if address == Address::ZERO {
        return Err(crate::error::EigenlayerExtraError::InvalidConfiguration(
            "Operator address cannot be zero".into(),
        ));
    }
    Ok(())
}

/// Format operator address for display
#[must_use]
pub fn format_operator_address(address: Address) -> alloc::string::String {
    alloc::format!("{address:#x}")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_operator_address_rejects_zero() {
        assert!(validate_operator_address(Address::ZERO).is_err());
    }

    #[test]
    fn test_validate_operator_address_accepts_valid() {
        let addr = Address::from([1u8; 20]);
        assert!(validate_operator_address(addr).is_ok());
    }

    #[test]
    fn test_format_operator_address() {
        let addr = Address::from([1u8; 20]);
        let formatted = format_operator_address(addr);
        assert!(formatted.starts_with("0x"));
    }
}
