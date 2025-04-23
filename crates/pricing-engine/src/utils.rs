//! Utility functions for the pricing engine

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_u128_to_bytes_roundtrip() {
        let original = 0x1234_5678_9ABC_DEF0_1234_5678_9ABC_DEF0_u128;
        let bytes = u128_to_bytes(original);
        let roundtrip = bytes_to_u128(&bytes);
        assert_eq!(original, roundtrip);
    }

    #[test]
    fn test_u32_to_u128_bytes() {
        let original = 0x1234_5678_u32;
        let bytes = u32_to_u128_bytes(original);
        let roundtrip = bytes_to_u128(&bytes);
        assert_eq!(roundtrip, original as u128);
    }
}
