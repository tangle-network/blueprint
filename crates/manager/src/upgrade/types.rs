use alloy_primitives::B256;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;

/// Per-service binary upgrade policy. Mirrors the on-chain enum exactly.
///
/// The numeric discriminants are load-bearing — they are the ABI encoding the
/// contract uses for `setServiceUpgradePolicy` / `getServiceUpgradePolicy`.
/// Reorder = wire-protocol break.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
#[repr(u8)]
pub enum UpgradePolicy {
    /// Operator must `ackBinaryVersion` before the manager swaps.
    /// This is the protocol default for a brand-new service.
    Approve = 0,
    /// Manager auto-swaps whenever the blueprint's active version moves
    /// (subject to sha256 + attestation gating).
    Auto = 1,
    /// Manager never swaps. Operator must switch policy first.
    Manual = 2,
}

impl UpgradePolicy {
    /// Decode the on-chain `uint8`. Unknown discriminants are conservatively
    /// mapped to `Manual` (the safest stance — never auto-upgrade something
    /// we don't understand).
    #[must_use]
    pub fn from_u8(value: u8) -> Self {
        match value {
            0 => Self::Approve,
            1 => Self::Auto,
            2 => Self::Manual,
            _ => Self::Manual,
        }
    }

    #[must_use]
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

impl Default for UpgradePolicy {
    /// Protocol default is APPROVE — a new operator joining a service does
    /// not auto-update.
    fn default() -> Self {
        Self::Approve
    }
}

impl std::fmt::Display for UpgradePolicy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Approve => f.write_str("APPROVE"),
            Self::Auto => f.write_str("AUTO"),
            Self::Manual => f.write_str("MANUAL"),
        }
    }
}

impl std::str::FromStr for UpgradePolicy {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_ascii_uppercase().as_str() {
            "APPROVE" => Ok(Self::Approve),
            "AUTO" => Ok(Self::Auto),
            "MANUAL" => Ok(Self::Manual),
            other => Err(format!(
                "invalid UpgradePolicy `{other}` (expected APPROVE | AUTO | MANUAL)"
            )),
        }
    }
}

/// Resolved binary version metadata as the manager sees it.
///
/// `sha256` is the canonical trust root — every download is gated against it.
/// `attestation_hash == B256::ZERO` means "no attestation bundle published"
/// (per contract docstring); a non-zero value MUST be verified for AUTO swaps.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BinaryVersionInfo {
    pub version_id: u64,
    #[serde(with = "hex_b256")]
    pub sha256: B256,
    pub binary_uri: String,
    #[serde(with = "hex_b256")]
    pub attestation_hash: B256,
    pub published_at: u64,
    pub deprecated: bool,
}

impl BinaryVersionInfo {
    /// True iff the contract documents an attestation bundle for this version.
    #[must_use]
    pub fn has_attestation(&self) -> bool {
        self.attestation_hash != B256::ZERO
    }
}

/// Entry exposed via `GET /upgrades/pending`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PendingUpgrade {
    pub service_id: u64,
    pub blueprint_id: u64,
    pub current_version_id: Option<u64>,
    #[serde(with = "hex_b256_opt")]
    pub current_sha256: Option<B256>,
    pub available_version_id: u64,
    #[serde(with = "hex_b256")]
    pub available_sha256: B256,
    pub available_uri: String,
    #[serde(with = "hex_b256")]
    pub available_attestation_hash: B256,
    pub policy: UpgradePolicy,
    /// Captured at the moment the watcher noticed the divergence; useful for
    /// alerting on "this upgrade has been pending for N hours."
    pub detected_at: SystemTime,
}

/// Entry exposed via `GET /upgrades/history`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeHistoryEntry {
    pub service_id: u64,
    pub blueprint_id: u64,
    pub from_version_id: Option<u64>,
    #[serde(with = "hex_b256_opt")]
    pub from_sha256: Option<B256>,
    pub to_version_id: u64,
    #[serde(with = "hex_b256")]
    pub to_sha256: B256,
    pub policy: UpgradePolicy,
    pub completed_at: SystemTime,
    pub outcome: UpgradeOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum UpgradeOutcome {
    /// Drain + swap succeeded; manager is now serving the new version.
    Swapped,
    /// Sha256 mismatch — binary discarded, manager kept old version.
    Sha256Mismatch,
    /// Attestation verify failed — manager kept old version and downgraded to
    /// APPROVE-style notification.
    AttestationFailed,
    /// Download or filesystem error.
    DownloadFailed,
    /// Policy was MANUAL — never swap.
    PolicyBlocked,
}

/// Serde helper for `B256 <-> 0x-prefixed hex string`.
mod hex_b256 {
    use alloy_primitives::B256;
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S: Serializer>(value: &B256, ser: S) -> Result<S::Ok, S::Error> {
        ser.serialize_str(&format!("0x{}", hex::encode(value.as_slice())))
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<B256, D::Error> {
        let raw = String::deserialize(de)?;
        let trimmed = raw.trim_start_matches("0x");
        let bytes = hex::decode(trimmed).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom(format!(
                "expected 32 bytes, got {}",
                bytes.len()
            )));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        Ok(B256::from(out))
    }
}

mod hex_b256_opt {
    use alloy_primitives::B256;
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    // serde's `#[serde(with = "...")]` calls `serialize(&Option<T>, _)`, so
    // the parameter shape is dictated by the framework — not a style choice.
    #[allow(clippy::ref_option)]
    pub fn serialize<S: Serializer>(value: &Option<B256>, ser: S) -> Result<S::Ok, S::Error> {
        match value {
            Some(v) => format!("0x{}", hex::encode(v.as_slice())).serialize(ser),
            None => ser.serialize_none(),
        }
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(de: D) -> Result<Option<B256>, D::Error> {
        let raw = Option::<String>::deserialize(de)?;
        let Some(s) = raw else {
            return Ok(None);
        };
        let trimmed = s.trim_start_matches("0x");
        let bytes = hex::decode(trimmed).map_err(serde::de::Error::custom)?;
        if bytes.len() != 32 {
            return Err(serde::de::Error::custom(format!(
                "expected 32 bytes, got {}",
                bytes.len()
            )));
        }
        let mut out = [0u8; 32];
        out.copy_from_slice(&bytes);
        Ok(Some(B256::from(out)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn upgrade_policy_roundtrips_through_u8() {
        for policy in [
            UpgradePolicy::Approve,
            UpgradePolicy::Auto,
            UpgradePolicy::Manual,
        ] {
            assert_eq!(UpgradePolicy::from_u8(policy.as_u8()), policy);
        }
    }

    #[test]
    fn unknown_policy_discriminant_falls_back_to_manual() {
        // Defensive: any future variant we don't know about MUST NOT cause us
        // to auto-swap. MANUAL is the only safe default.
        assert_eq!(UpgradePolicy::from_u8(7), UpgradePolicy::Manual);
        assert_eq!(UpgradePolicy::from_u8(255), UpgradePolicy::Manual);
    }

    #[test]
    fn upgrade_policy_default_is_approve() {
        // Matches the on-chain default and the spec's "new operator joining a
        // service does not auto-update" invariant.
        assert_eq!(UpgradePolicy::default(), UpgradePolicy::Approve);
    }

    #[test]
    fn upgrade_policy_parses_case_insensitively() {
        assert_eq!(
            "approve".parse::<UpgradePolicy>().unwrap(),
            UpgradePolicy::Approve
        );
        assert_eq!(
            "AUTO".parse::<UpgradePolicy>().unwrap(),
            UpgradePolicy::Auto
        );
        assert_eq!(
            "Manual".parse::<UpgradePolicy>().unwrap(),
            UpgradePolicy::Manual
        );
        assert!("rolling".parse::<UpgradePolicy>().is_err());
    }

    #[test]
    fn binary_version_info_attestation_flag() {
        let mut info = BinaryVersionInfo {
            version_id: 0,
            sha256: B256::ZERO,
            binary_uri: String::new(),
            attestation_hash: B256::ZERO,
            published_at: 0,
            deprecated: false,
        };
        assert!(!info.has_attestation());
        info.attestation_hash = B256::repeat_byte(0xab);
        assert!(info.has_attestation());
    }
}
