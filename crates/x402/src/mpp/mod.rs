//! MPP (Machine Payments Protocol) ingress for the Blueprint SDK x402 gateway.
//!
//! MPP is the IETF standards-track Payment HTTP Authentication Scheme defined
//! at <https://paymentauth.org> and documented at <https://mpp.dev>. It uses
//! `WWW-Authenticate: Payment` / `Authorization: Payment` / `Payment-Receipt`
//! headers and RFC 9457 Problem Details for errors.
//!
//! The blueprint-x402 MPP ingress is **additive** to the existing x402 ingress:
//! both routes share the same `job_pricing`, accepted-token table, restricted
//! caller policies (including on-chain `isPermittedCaller` checks and the
//! delegated-signature replay guard), quote registry, and producer stream.
//! Only the wire format on the request side differs.
//!
//! # Architecture
//!
//! ```text
//!                      ┌──────────────────────────────────────┐
//!                      │     blueprint-x402 X402Gateway       │
//!                      │                                      │
//! /x402/jobs/.. ──────► │  x402-axum middleware                │
//!  (X-PAYMENT)          │      │                               │
//!                      │      ▼                               │
//!                      │  handle_paid_job_inner ──┐           │
//!                      │      ▲                   │           │
//! /mpp/jobs/.. ───────► │  parse Authorization:    │           │
//!  (Authorization:     │  Payment ──► Mpp::verify  ▼           │
//!   Payment)           │  (HMAC + facilitator     enqueue ──► │  Producer
//!                      │   verify + settle)                    │  ──► Runner
//!                      └──────────────────────────────────────┘
//! ```
//!
//! # Method
//!
//! The Blueprint MPP ingress uses a custom MPP method named [`METHOD_NAME`]
//! (`"blueprintevm"`). This method's credential payload wraps the **same**
//! EIP-3009 / Permit2 `PaymentPayload` that x402 clients already produce,
//! base64url-encoded into the MPP credential's `payload.signature` field.
//! Verification delegates to the same x402 facilitator the legacy ingress
//! uses, so existing x402 wallets work over the MPP wire format unchanged.
//!
//! See [`method::BlueprintEvmChargeMethod`] for the implementation.

pub mod credential;
pub mod method;
pub(crate) mod routes;
pub mod state;

pub use credential::{Eip3009Extra, MppCredentialPayload, MppMethodDetails};
pub use method::METHOD_NAME;
pub use state::MppGatewayState;
