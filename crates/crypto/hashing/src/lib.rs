//! Hashing and key derivation primitives for Tangle Blueprints.
//!
//! Provides feature-gated access to common cryptographic hash functions and KDFs:
//!
//! | Feature        | Function(s)                          |
//! |----------------|--------------------------------------|
//! | `sha2-hasher`  | [`sha2_256`], [`sha2_512`]           |
//! | `sha3-hasher`  | [`keccak_256`]                       |
//! | `blake3-hasher`| [`blake3_256`]                       |
//! | `kdf-hkdf`     | [`kdf::hkdf_sha256`]                 |
//! | `kdf-argon2`   | [`kdf::argon2id_derive`], [`kdf::argon2id_derive_with`] |

/// Compute SHA2-256 hash of `data`.
///
/// # Feature
/// Requires the `sha2-hasher` feature.
#[cfg(feature = "sha2")]
pub fn sha2_256(data: &[u8]) -> [u8; 32] {
    use sha2::Digest;

    let mut hasher = sha2::Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 32];
    hash.copy_from_slice(&result);
    hash
}

/// Compute SHA2-512 hash of `data`.
///
/// # Feature
/// Requires the `sha2-hasher` feature.
#[cfg(feature = "sha2")]
pub fn sha2_512(data: &[u8]) -> [u8; 64] {
    use sha2::Digest;

    let mut hasher = sha2::Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut hash = [0u8; 64];
    hash.copy_from_slice(&result);
    hash
}

/// Compute Keccak-256 hash of `data`.
///
/// # Feature
/// Requires the `sha3-hasher` feature.
#[cfg(feature = "sha3")]
pub fn keccak_256(data: &[u8]) -> [u8; 32] {
    use sha3::Digest;

    let mut hasher = sha3::Keccak256::new();
    hasher.update(data);
    let output = hasher.finalize();
    output.into()
}

/// Compute BLAKE3-256 hash of `data`.
///
/// # Feature
/// Requires the `blake3-hasher` feature.
#[cfg(feature = "blake3")]
pub fn blake3_256(data: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(data);
    let output = hasher.finalize();
    output.into()
}

/// Key derivation functions (HKDF, Argon2id).
///
/// See the [`kdf`] module documentation for usage guidance.
#[cfg(any(feature = "kdf-hkdf", feature = "kdf-argon2"))]
pub mod kdf;
