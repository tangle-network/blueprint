//! Key Derivation Functions (KDFs) for deriving cryptographic keys from secrets.
//!
//! Provides two KDF families, feature-gated for minimal dependency footprint:
//!
//! - **HKDF** (`kdf-hkdf`): HMAC-based Extract-and-Expand Key Derivation Function
//!   ([RFC 5869](https://datatracker.ietf.org/doc/html/rfc5869)). Best for deriving
//!   keys from high-entropy input keying material (random secrets, DH shared secrets).
//!   Fast and suitable for non-password-based key derivation.
//!
//! - **Argon2id** (`kdf-argon2`): Memory-hard password-based KDF
//!   ([RFC 9106](https://datatracker.ietf.org/doc/html/rfc9106)). Best for deriving
//!   keys from low-entropy secrets (passwords, passphrases). Resistant to GPU/ASIC
//!   brute-force attacks.
//!
//! # Important
//!
//! These functions derive **raw key bytes**, not encoded password hashes. For password
//! *storage* (verification, parameter encoding), use a dedicated password hashing
//! library with PHC string output.

/// Errors that can occur during key derivation.
#[derive(Debug, Clone)]
pub enum KdfError {
    /// HKDF expand failed (output length exceeds 255 * HashLen = 8160 bytes for SHA-256).
    HkdfExpandFailed,
    /// Argon2 derivation failed (invalid parameters, salt too short, etc.).
    Argon2Failed(String),
}

impl core::fmt::Display for KdfError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::HkdfExpandFailed => write!(
                f,
                "HKDF expand failed: requested output length exceeds maximum (8160 bytes for SHA-256)"
            ),
            Self::Argon2Failed(msg) => write!(f, "Argon2id derivation failed: {msg}"),
        }
    }
}

/// Derive key bytes using HKDF-SHA256 ([RFC 5869](https://datatracker.ietf.org/doc/html/rfc5869)).
///
/// Uses the extract-then-expand paradigm to derive a fixed-length key from input
/// keying material. Best for high-entropy inputs (random secrets, DH shared secrets).
///
/// # Parameters
/// - `ikm`: Input keying material (the secret)
/// - `salt`: Optional salt value (a non-secret random value; if `None`, HKDF uses a
///   zero-filled array of hash length per RFC 5869 §2.2)
/// - `info`: Context and application-specific info string for domain separation
///
/// # Errors
/// Returns [`KdfError::HkdfExpandFailed`] if `N` exceeds 255 * 32 = 8160 bytes.
///
/// # Feature
/// Requires the `kdf-hkdf` feature.
#[cfg(feature = "kdf-hkdf")]
pub fn hkdf_sha256<const N: usize>(
    ikm: &[u8],
    salt: Option<&[u8]>,
    info: &[u8],
) -> Result<[u8; N], KdfError> {
    use hkdf::Hkdf;
    use sha2::Sha256;

    let hk = Hkdf::<Sha256>::new(salt, ikm);
    let mut okm = [0u8; N];
    hk.expand(info, &mut okm)
        .map_err(|_| KdfError::HkdfExpandFailed)?;
    Ok(okm)
}

/// Configuration for Argon2id key derivation.
///
/// Defaults to OWASP-recommended minimum parameters (19 MiB memory, 2 iterations,
/// 1 parallel lane) as of 2024.
#[cfg(feature = "kdf-argon2")]
#[derive(Debug, Clone)]
pub struct Argon2idConfig {
    /// Memory cost in KiB. Default: 19456 (19 MiB).
    pub memory_kib: u32,
    /// Number of iterations. Default: 2.
    pub iterations: u32,
    /// Degree of parallelism. Default: 1.
    pub parallelism: u32,
}

#[cfg(feature = "kdf-argon2")]
impl Default for Argon2idConfig {
    fn default() -> Self {
        Self {
            memory_kib: 19456,
            iterations: 2,
            parallelism: 1,
        }
    }
}

/// Derive key bytes using Argon2id ([RFC 9106](https://datatracker.ietf.org/doc/html/rfc9106))
/// with the default OWASP-recommended parameters.
///
/// This is a convenience wrapper around [`argon2id_derive_with`] using
/// [`Argon2idConfig::default()`]. For custom tuning, use [`argon2id_derive_with`] directly.
///
/// # Parameters
/// - `password`: The password or low-entropy secret
/// - `salt`: A unique, random salt (minimum 8 bytes; 16+ bytes recommended)
///
/// # Errors
/// Returns [`KdfError::Argon2Failed`] on invalid parameters or internal failure.
///
/// # Feature
/// Requires the `kdf-argon2` feature.
#[cfg(feature = "kdf-argon2")]
pub fn argon2id_derive<const N: usize>(password: &[u8], salt: &[u8]) -> Result<[u8; N], KdfError> {
    argon2id_derive_with(password, salt, &Argon2idConfig::default())
}

/// Derive key bytes using Argon2id with custom parameters.
///
/// # Parameters
/// - `password`: The password or low-entropy secret
/// - `salt`: A unique, random salt (minimum 8 bytes; 16+ bytes recommended)
/// - `config`: Argon2id tuning parameters (memory, iterations, parallelism)
///
/// # Errors
/// Returns [`KdfError::Argon2Failed`] on invalid parameters or internal failure.
///
/// # Feature
/// Requires the `kdf-argon2` feature.
#[cfg(feature = "kdf-argon2")]
pub fn argon2id_derive_with<const N: usize>(
    password: &[u8],
    salt: &[u8],
    config: &Argon2idConfig,
) -> Result<[u8; N], KdfError> {
    use argon2::{Algorithm, Argon2, Params, Version};

    let params = Params::new(
        config.memory_kib,
        config.iterations,
        config.parallelism,
        Some(N),
    )
    .map_err(|e| KdfError::Argon2Failed(e.to_string()))?;
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);

    let mut output = [0u8; N];
    argon2
        .hash_password_into(password, salt, &mut output)
        .map_err(|e| KdfError::Argon2Failed(e.to_string()))?;
    Ok(output)
}

#[cfg(test)]
mod tests {
    #[cfg(feature = "kdf-hkdf")]
    mod hkdf_tests {
        use super::super::hkdf_sha256;

        #[test]
        fn deterministic() {
            let key1: [u8; 32] = hkdf_sha256(b"secret", None, b"test-info").unwrap();
            let key2: [u8; 32] = hkdf_sha256(b"secret", None, b"test-info").unwrap();
            assert_eq!(key1, key2);
        }

        #[test]
        fn different_info_different_key() {
            let key1: [u8; 32] = hkdf_sha256(b"secret", None, b"info-a").unwrap();
            let key2: [u8; 32] = hkdf_sha256(b"secret", None, b"info-b").unwrap();
            assert_ne!(key1, key2);
        }

        #[test]
        fn different_salt_different_key() {
            let key1: [u8; 32] = hkdf_sha256(b"secret", Some(b"salt-a"), b"info").unwrap();
            let key2: [u8; 32] = hkdf_sha256(b"secret", Some(b"salt-b"), b"info").unwrap();
            assert_ne!(key1, key2);
        }

        #[test]
        fn variable_output_lengths() {
            let key16: [u8; 16] = hkdf_sha256(b"secret", None, b"info").unwrap();
            let key32: [u8; 32] = hkdf_sha256(b"secret", None, b"info").unwrap();
            let key64: [u8; 64] = hkdf_sha256(b"secret", None, b"info").unwrap();
            // First 16 bytes of each must match (HKDF expand property)
            assert_eq!(&key16[..], &key32[..16]);
            assert_eq!(&key16[..], &key64[..16]);
        }

        #[test]
        fn rfc5869_test_vector() {
            // RFC 5869 Test Case 1 (SHA-256)
            let ikm: [u8; 22] = [0x0b; 22];
            let salt: [u8; 13] = [
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c,
            ];
            let info: [u8; 10] = [0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9];
            let expected: [u8; 42] = [
                0x3c, 0xb2, 0x5f, 0x25, 0xfa, 0xac, 0xd5, 0x7a, 0x90, 0x43, 0x4f, 0x64, 0xd0, 0x36,
                0x2f, 0x2a, 0x2d, 0x2d, 0x0a, 0x90, 0xcf, 0x1a, 0x5a, 0x4c, 0x5d, 0xb0, 0x2d, 0x56,
                0xec, 0xc4, 0xc5, 0xbf, 0x34, 0x00, 0x72, 0x08, 0xd5, 0xb8, 0x87, 0x18, 0x58, 0x65,
            ];
            let okm: [u8; 42] = hkdf_sha256(&ikm, Some(&salt), &info).unwrap();
            assert_eq!(okm, expected);
        }
    }

    #[cfg(feature = "kdf-argon2")]
    mod argon2_tests {
        use super::super::{Argon2idConfig, argon2id_derive, argon2id_derive_with};

        #[test]
        fn deterministic() {
            let key1: [u8; 32] = argon2id_derive(b"password", b"unique-salt-16by").unwrap();
            let key2: [u8; 32] = argon2id_derive(b"password", b"unique-salt-16by").unwrap();
            assert_eq!(key1, key2);
        }

        #[test]
        fn different_password_different_key() {
            let key1: [u8; 32] = argon2id_derive(b"password-a", b"same-salt-16byte").unwrap();
            let key2: [u8; 32] = argon2id_derive(b"password-b", b"same-salt-16byte").unwrap();
            assert_ne!(key1, key2);
        }

        #[test]
        fn different_salt_different_key() {
            let key1: [u8; 32] = argon2id_derive(b"password", b"salt-aaaaaaaaaaaa").unwrap();
            let key2: [u8; 32] = argon2id_derive(b"password", b"salt-bbbbbbbbbbbb").unwrap();
            assert_ne!(key1, key2);
        }

        #[test]
        fn custom_config() {
            let config = Argon2idConfig {
                memory_kib: 4096,
                iterations: 1,
                parallelism: 1,
            };
            let key: [u8; 32] =
                argon2id_derive_with(b"password", b"custom-salt-16by", &config).unwrap();
            // Differs from default params
            let default_key: [u8; 32] = argon2id_derive(b"password", b"custom-salt-16by").unwrap();
            assert_ne!(key, default_key);
        }
    }
}
