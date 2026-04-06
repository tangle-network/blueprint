# blueprint-x402

x402 + MPP payment gateway for Blueprint SDK.

This crate exposes paid HTTP job execution for Blueprint runners over **two parallel wire protocols**: clients settle via x402 *or* MPP, the gateway verifies the payment, and a verified `JobCall` is injected into the runner. Both ingresses share the same job pricing, accepted tokens, restricted-auth modes, and producer stream — only the wire format differs.

## What it provides

- `X402Gateway` background service (axum), exposing two parallel ingresses:
  - **x402** at `/x402/jobs/{service_id}/{job_index}` (`X-PAYMENT` / `X-Payment-Response` headers, via `x402-axum`)
  - **MPP** at `/mpp/jobs/{service_id}/{job_index}` (`WWW-Authenticate: Payment` / `Authorization: Payment` / `Payment-Receipt` headers, RFC 9457 Problem Details errors, via the `mpp` crate). Enabled when `[mpp]` is present in the config.
- `X402Producer` stream that converts verified payments into `JobCall`s (shared by both ingresses)
- Per-job invocation policy model:
  - `disabled`
  - `public_paid`
  - `restricted_paid`
- Restricted auth modes (apply to both ingresses):
  - `payer_is_caller`
  - `delegated_caller_signature`
- Auth dry-run endpoint for restricted policy checks:
  - `POST /x402/jobs/{service_id}/{job_index}/auth-dry-run`
- Discovery endpoints:
  - `GET /x402/jobs/{service_id}/{job_index}/price`
  - `GET /mpp/jobs/{service_id}/{job_index}/price`

## MPP integration

MPP (Machine Payments Protocol) is the IETF standards-track Payment HTTP Authentication Scheme defined at <https://paymentauth.org> and documented at <https://mpp.dev>. The Blueprint MPP ingress is **additive**: existing x402 clients keep working, MPP gives you standardised headers, RFC 9457 errors, and a path to multi-method payments.

The Blueprint MPP method is named `x402-evm`. Its credential payload wraps the **same** EIP-3009 / Permit2 `PaymentPayload` that x402 clients already produce, base64url-encoded into the MPP `PaymentCredential.payload`. Verification delegates to the same x402 facilitator the legacy ingress uses, so existing x402 wallets work over MPP unchanged.

To enable, add a `[mpp]` section to your `x402.toml`:

```toml
[mpp]
realm = "blueprint.example.com"
# At least 32 bytes of entropy. Rotate periodically.
secret_key = "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef"
challenge_ttl_secs = 300
```

When this section is present `X402Gateway::new` automatically constructs the MPP server state and registers the `/mpp/jobs/...` routes alongside the existing x402 ingress.

## Quick integration

```rust,ignore
use blueprint_x402::{X402Config, X402Gateway};
use blueprint_runner::BlueprintRunner;

let config = X402Config::from_toml("x402.toml")?;
let (gateway, producer) = X402Gateway::new(config, job_pricing)?;

BlueprintRunner::builder((), env)
    .router(router)
    .producer(producer)
    .background_service(gateway)
    .run()
    .await?;
```

`job_pricing` is a `(service_id, job_index) -> price_wei` map.

## Related links

- Source: https://github.com/tangle-network/blueprint/tree/main/crates/x402
- Example blueprint (end-to-end): https://github.com/tangle-network/blueprint/blob/main/examples/x402-blueprint/README.md
- Example x402 config: https://github.com/tangle-network/blueprint/blob/main/examples/x402-blueprint/config/x402.toml
- Delegated auth dry-run helper script: https://github.com/tangle-network/blueprint/blob/main/scripts/x402-auth-dry-run.sh
- Docs site page: https://docs.tangle.tools/developers/blueprint-runner/x402
- Trigger model context (cron/on-chain/webhooks/x402): https://docs.tangle.tools/developers/blueprint-runner/job-triggers
