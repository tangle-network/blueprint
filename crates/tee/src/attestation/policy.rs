//! Attestation verification policy.

use rand::RngCore;

/// Policy applied when verifying attestation evidence.
///
/// Empty/`None` fields mean "do not enforce this dimension". Production
/// defaults reject debug TEEs and enforce freshness; callers should additionally
/// pin nonce, audience, image digest, and/or measurement for workload binding.
#[derive(Debug, Clone, Default)]
pub struct AttestationPolicy {
    /// Expected audience claim (GCP/Azure JWT `aud`).
    pub expected_audience: Option<String>,
    /// Expected nonce echoed back in the attestation.
    pub expected_nonce: Option<String>,
    /// Expected workload/image digest.
    pub expected_image_digest: Option<String>,
    /// Expected primary measurement (Nitro PCR0, Azure launch measurement, ...).
    pub expected_measurement: Option<String>,
    /// Whether debug-mode TEEs are tolerated. Defaults to `false`.
    pub allow_debug: bool,
    /// Maximum tolerated age of the attestation, in seconds.
    pub max_age_secs: Option<u64>,
}

impl AttestationPolicy {
    /// Hard ceiling on requester-controlled freshness windows.
    pub const MAX_AGE_CEILING_SECS: u64 = 3600;

    /// Baseline production policy: reject debug TEEs and require freshness within
    /// ten minutes.
    pub fn production() -> Self {
        Self {
            allow_debug: false,
            max_age_secs: Some(600),
            ..Self::default()
        }
    }

    /// Pin the expected audience.
    pub fn with_audience(mut self, aud: impl Into<String>) -> Self {
        self.expected_audience = Some(aud.into());
        self
    }

    /// Pin the expected nonce.
    pub fn with_nonce(mut self, nonce: impl Into<String>) -> Self {
        self.expected_nonce = Some(nonce.into());
        self
    }

    /// Pin the expected workload image digest.
    pub fn with_image_digest(mut self, digest: impl Into<String>) -> Self {
        self.expected_image_digest = Some(digest.into());
        self
    }

    /// Pin the expected platform measurement / PCR0.
    pub fn with_measurement(mut self, measurement: impl Into<String>) -> Self {
        self.expected_measurement = Some(measurement.into());
        self
    }

    /// Whether this policy binds the attested VM to a workload or relying party.
    ///
    /// A nonce-only policy proves that a fresh confidential VM answered. Pinning
    /// audience and/or image digest upgrades the verdict to workload-bound.
    pub fn is_workload_bound(&self) -> bool {
        self.expected_audience.is_some() || self.expected_image_digest.is_some()
    }

    /// Build a policy from a provisioner's `custom_config` map.
    ///
    /// Recognised keys: `tee_expected_audience`, `tee_expected_image_digest`,
    /// `tee_expected_measurement`, `tee_allow_debug` (`true`/`false`),
    /// `tee_max_age_secs`. The nonce is supplied separately.
    pub fn from_custom_config(
        config: &std::collections::HashMap<String, String>,
        nonce: &str,
    ) -> Self {
        let mut policy = Self::production().with_nonce(nonce);
        if let Some(v) = config.get("tee_expected_audience") {
            policy.expected_audience = Some(v.clone());
        }
        if let Some(v) = config.get("tee_expected_image_digest") {
            policy.expected_image_digest = Some(v.clone());
        }
        if let Some(v) = config.get("tee_expected_measurement") {
            policy.expected_measurement = Some(v.clone());
        }
        if let Some(v) = config.get("tee_allow_debug") {
            if v.eq_ignore_ascii_case("true") {
                if cfg!(feature = "testing") {
                    policy.allow_debug = true;
                    tracing::warn!(
                        "tee_allow_debug=true honoured: accepting debug-mode TEEs \
                         (testing build only)"
                    );
                } else {
                    tracing::warn!(
                        "tee_allow_debug=true requested via deployment config but refused: \
                         debug-mode TEEs are only accepted in a `testing` build"
                    );
                }
            }
        }
        if let Some(v) = config.get("tee_max_age_secs") {
            if let Ok(secs) = v.parse::<u64>() {
                if secs > Self::MAX_AGE_CEILING_SECS {
                    tracing::warn!(
                        requested_secs = secs,
                        ceiling_secs = Self::MAX_AGE_CEILING_SECS,
                        "tee_max_age_secs exceeds the freshness ceiling; clamping"
                    );
                }
                policy.max_age_secs = Some(secs.min(Self::MAX_AGE_CEILING_SECS));
            }
        }
        policy
    }
}

/// Generate a fresh, unguessable attestation challenge nonce.
pub fn fresh_nonce() -> String {
    let mut bytes = [0_u8; 16];
    rand::thread_rng().fill_bytes(&mut bytes);
    hex::encode(bytes)
}

/// Current unix time in seconds.
pub(crate) fn now_unix() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}
