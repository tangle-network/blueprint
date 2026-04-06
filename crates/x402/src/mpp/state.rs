//! Per-gateway MPP state.
//!
//! Holds the configured `mpp::server::Mpp` handler, the facilitator client,
//! a snapshot of accepted tokens, and the challenge TTL. Constructed once
//! by [`X402Gateway::new`](crate::X402Gateway::new) when the operator's
//! [`MppConfig`](crate::config::MppConfig) is `Some`.

use std::sync::Arc;
use std::time::Duration;

use alloy_primitives::Address;
use dashmap::DashMap;
use mpp::server::Mpp;
use url::Url;
use x402_axum::facilitator_client::FacilitatorClient;

use crate::config::{AcceptedToken, MppConfig};
use crate::error::X402Error;
use crate::mpp::method::BlueprintEvmChargeMethod;

/// Side-channel cache used to surface the facilitator-verified payer from
/// [`BlueprintEvmChargeMethod::verify`] back to the MPP route handler.
///
/// The `mpp::ChargeMethod` trait returns only a `Receipt` from `verify`, but
/// the route needs the on-chain payer address to enforce restricted-caller
/// policies (`auth_mode = payer_is_caller`). We can't extend `Receipt`
/// without forking the upstream `mpp` crate, so we stash the payer in this
/// per-credential map keyed by the challenge id (which is HMAC-bound and
/// unique per challenge issuance), then drain the entry on the route side
/// immediately after `verify_credential_with_expected_request` returns.
pub type VerifiedPayerCache = Arc<DashMap<String, Address>>;

/// All MPP-specific state shared by the gateway and the MPP request handlers.
///
/// This is `Send + Sync + 'static` so it can live inside the axum router
/// state behind an `Arc`. Cloning is cheap; everything is reference-counted.
///
/// We keep a `secret_key` field in addition to handing it to `mpp::Mpp`
/// because [`PaymentChallenge::with_secret_key_full`](mpp::protocol::core::PaymentChallenge::with_secret_key_full)
/// — the API we use to build challenges — takes the secret as an explicit
/// parameter, and `mpp::server::Mpp` does not expose its private secret via
/// any getter. Both copies are guaranteed to be the same value because they
/// are constructed together in [`MppGatewayState::new`].
#[derive(Clone)]
pub struct MppGatewayState {
    /// The configured MPP handler. Owns the [`BlueprintEvmChargeMethod`]
    /// and is parameterized over no session method (charge intent only).
    pub(crate) mpp: Arc<Mpp<BlueprintEvmChargeMethod, ()>>,
    /// MPP realm string (mirrors [`MppConfig::realm`]).
    pub(crate) realm: String,
    /// HMAC secret used to compute / verify challenge IDs. Same value the
    /// inner [`Mpp`] uses; kept here so the request handlers can synthesise
    /// new `PaymentChallenge`s with `with_secret_key_full`. Held as
    /// `pub(crate)` to forbid external code from rotating one copy without
    /// the other.
    pub(crate) secret_key: String,
    /// Challenge TTL in seconds (mirrors [`MppConfig::challenge_ttl_secs`]).
    pub(crate) challenge_ttl_secs: u64,
    /// Accepted tokens, used by the request handlers when building the
    /// per-network `methodDetails` and resolving payment attribution after
    /// successful verification.
    pub(crate) accepted_tokens: Arc<Vec<AcceptedToken>>,
    /// Side-channel for the facilitator-verified payer. Populated by
    /// [`BlueprintEvmChargeMethod::verify`] on `Valid { payer }` and drained
    /// by the MPP route handler before consulting restricted-caller policy.
    /// See [`VerifiedPayerCache`] for the rationale.
    pub(crate) verified_payers: VerifiedPayerCache,
}

impl std::fmt::Debug for MppGatewayState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MppGatewayState")
            .field("realm", &self.realm)
            .field("challenge_ttl_secs", &self.challenge_ttl_secs)
            .field("accepted_tokens", &self.accepted_tokens.len())
            .finish_non_exhaustive()
    }
}

impl MppGatewayState {
    /// Build the MPP state from the operator's [`MppConfig`] and the
    /// (already-validated) facilitator URL + accepted tokens that the rest
    /// of the gateway uses.
    ///
    /// # Errors
    ///
    /// Returns [`X402Error::Mpp`] if the facilitator URL cannot be parsed
    /// into a [`FacilitatorClient`] or if the MPP handler cannot be built
    /// from the supplied charge method.
    pub fn new(
        config: &MppConfig,
        facilitator_url: Url,
        accepted_tokens: Vec<AcceptedToken>,
    ) -> Result<Self, X402Error> {
        let facilitator = FacilitatorClient::try_new(facilitator_url)
            .map_err(|e| X402Error::Mpp(format!("failed to construct facilitator client: {e}")))?
            // Reasonable default; the legacy x402-axum middleware also uses
            // a per-request timeout. Operators wanting longer timeouts can
            // tune this in a future config knob.
            .with_timeout(Duration::from_secs(15));

        let verified_payers: VerifiedPayerCache = Arc::new(DashMap::new());
        let charge_method = BlueprintEvmChargeMethod::new(
            facilitator,
            accepted_tokens.clone(),
            verified_payers.clone(),
        );
        let mpp = Mpp::new(
            charge_method,
            config.realm.clone(),
            config.secret_key.clone(),
        );

        Ok(Self {
            mpp: Arc::new(mpp),
            realm: config.realm.clone(),
            secret_key: config.secret_key.clone(),
            challenge_ttl_secs: config.challenge_ttl_secs,
            accepted_tokens: Arc::new(accepted_tokens),
            verified_payers,
        })
    }
}
