//! Per-gateway MPP state.
//!
//! Holds the configured `mpp::server::Mpp` handler, the facilitator client,
//! a snapshot of accepted tokens, and the challenge TTL. Constructed once
//! by [`X402Gateway::new`](crate::X402Gateway::new) when the operator's
//! [`MppConfig`](crate::config::MppConfig) is `Some`.

use std::sync::Arc;
use std::time::Duration;

use mpp::server::Mpp;
use url::Url;
use x402_axum::facilitator_client::FacilitatorClient;

use crate::config::{AcceptedToken, MppConfig};
use crate::error::X402Error;
use crate::mpp::method::BlueprintEvmChargeMethod;

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
    pub mpp: Arc<Mpp<BlueprintEvmChargeMethod, ()>>,
    /// MPP realm string (mirrors [`MppConfig::realm`]).
    pub realm: String,
    /// HMAC secret used to compute / verify challenge IDs. Same value the
    /// inner [`Mpp`] uses; kept here so the request handlers can synthesise
    /// new `PaymentChallenge`s with `with_secret_key_full`.
    pub secret_key: String,
    /// Challenge TTL in seconds (mirrors [`MppConfig::challenge_ttl_secs`]).
    pub challenge_ttl_secs: u64,
    /// Accepted tokens, used by the request handlers when building the
    /// per-network `methodDetails` and resolving payment attribution after
    /// successful verification.
    pub accepted_tokens: Arc<Vec<AcceptedToken>>,
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

        let charge_method = BlueprintEvmChargeMethod::new(facilitator, accepted_tokens.clone());
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
        })
    }
}
