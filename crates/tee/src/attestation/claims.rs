//! Typed attestation claims.
//!
//! Provides structured claim types instead of raw byte vectors, enabling
//! type-safe policy evaluation and easier interoperability between providers.

use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Typed attestation claims extracted from a TEE attestation report.
///
/// Different providers populate different subsets of these fields. The
/// `custom` map allows provider-specific claims that don't fit the
/// standard fields.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AttestationClaims {
    /// The security version of the TEE platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_version: Option<String>,

    /// Whether the TEE is running in debug mode.
    /// Debug mode enclaves should never be trusted in production.
    #[serde(default)]
    pub debug_mode: bool,

    /// Boot-time measurements (PCR values for Nitro, RTMR for TDX, etc.).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub boot_measurements: Vec<String>,

    /// The signer/author identity of the enclave image.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signer_id: Option<String>,

    /// The product/enclave identity within a signer's namespace.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,

    /// User-supplied data included in the attestation request (e.g., nonce, public key hash).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_data: Option<Vec<u8>>,

    /// Provider-specific custom claims.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub custom: BTreeMap<String, serde_json::Value>,
}

impl AttestationClaims {
    /// Create empty claims.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set a custom claim.
    pub fn with_custom(
        mut self,
        key: impl Into<String>,
        value: impl Into<serde_json::Value>,
    ) -> Self {
        self.custom.insert(key.into(), value.into());
        self
    }

    /// Get a custom claim by key.
    pub fn get_custom(&self, key: &str) -> Option<&serde_json::Value> {
        self.custom.get(key)
    }
}
