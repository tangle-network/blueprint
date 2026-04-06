# x402 + MPP Blueprint Example

Reference implementation showing the full x402 / MPP payment pipeline: TOML
pricing config, exchange rate oracles, HTTP gateway with **two parallel wire
protocols**, and job dispatch via the Blueprint router.

## What This Demonstrates

- Two jobs (echo, keccak256) priced in wei and served via two payment ingresses:
  - `/x402/jobs/{sid}/{idx}` — the legacy x402 wire format (`X-PAYMENT` headers)
  - `/mpp/jobs/{sid}/{idx}` — the IETF Payment HTTP Authentication Scheme
    (`WWW-Authenticate: Payment` / `Authorization: Payment` / `Payment-Receipt`),
    documented at <https://paymentauth.org> and <https://mpp.dev>
- Both ingresses share the **same** job pricing, accepted-token table,
  restricted-caller policy, and producer/runner injection path. Only the
  wire format on the request side differs.
- Static TOML-based pricing and dynamic oracle-based pricing via `PriceOracle`.
- `ScaledPriceOracle` for surge pricing multipliers.
- Exchange rate oracles: Chainlink, Uniswap V3 TWAP, Coinbase API.
- `CachedRateProvider` for TTL-based rate caching.
- `refresh_rates()` helper to update `X402Config` from live oracle data.
- Gateway startup, health checks, price discovery, and job submission.
- Real integration tests with HTTP requests against a running gateway.

## When to use which protocol

| Use x402 (`/x402/jobs/...`) when... | Use MPP (`/mpp/jobs/...`) when... |
|---|---|
| You're integrating with an existing x402 wallet | You want the IETF standards-track wire format |
| You want the smallest possible client surface | You need RFC 9457 Problem Details errors for client branching |
| You're calling from a script with `X-PAYMENT` headers | You want the standard `WWW-Authenticate` / `Authorization` headers that browsers, proxies, and CDNs already understand |
| You don't want to manage an HMAC secret | You're OK with rotating an `mpp.secret_key` for stateless challenge verification |

The two ingresses are **functionally equivalent** for charge intent today —
the MPP method (`blueprintevm`) wraps the same EIP-3009 / Permit2 payload an
x402 wallet already produces. A single client could speak both; an operator
could enable just one.

## Architecture

```
                          Exchange Rate Oracles
                          (Chainlink / Uniswap / Coinbase)
                                    |
                                    v
                            refresh_rates()
                                    |
                                    v
                              X402Config
                          (rate_per_native_unit)
                                    |
                                    |
   ┌────────── x402 client ──────┐  │   ┌────────── mpp client ─────────┐
   │ POST /x402/jobs/{sid}/{idx} │  │   │ POST /mpp/jobs/{sid}/{idx}    │
   │ X-PAYMENT: <base64 payload> │  │   │ Authorization: Payment <b64u> │
   └──────────────┬──────────────┘  │   └────────────────┬──────────────┘
                  │                 v                    │
                  │       X402Gateway (axum)              │
                  │                                       │
                  │  x402-axum middleware     Mpp::verify_credential_with_
                  │  + facilitator /verify    expected_request
                  │  + facilitator /settle    + BlueprintEvmChargeMethod
                  │                            (-> facilitator /verify + /settle)
                  │           │    │                    │
                  │           ▼    ▼                    │
                  │     handle_paid_job_inner ◄─────────┘
                  │      (shared policy + quote
                  │       registry + producer)
                  │           │
                  │           ▼
                  │   VerifiedPayment -> mpsc channel
                  │           │
                  │           ▼
                  │     X402Producer (Stream<Item = JobCall>)
                  │           │
                  │           ▼
                  │       Router
                  │       |-- job 0 -> echo(body)  -> body
                  │       |-- job 1 -> hash(body)  -> keccak256(body)
                  │           │
                  │           ▼
                  │       JobResult
                  v
              (response)
```

The two ingresses converge at `handle_paid_job_inner`. Everything below
that point — policy, replay guard, quote registry, producer — is wire-
protocol-agnostic.

## Configuration

### `config/job_pricing.toml`

```toml
[1]
0 = "1000000000000000"       # echo: 0.001 ETH
1 = "10000000000000000"      # keccak256: 0.01 ETH
```

Section keys are service IDs. Inner keys are job indices. Values are prices in
wei as strings (to support large U256 values).

### `config/x402.toml`

```toml
bind_address = "127.0.0.1:0"
facilitator_url = "https://facilitator.x402.org"
quote_ttl_secs = 300

[[accepted_tokens]]
network = "eip155:8453"
asset = "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913"
symbol = "USDC"
decimals = 6
pay_to = "0x0000000000000000000000000000000000000001"
rate_per_native_unit = "3200.00"
markup_bps = 200

# Optional: enable the parallel MPP ingress.
[mpp]
realm = "x402-blueprint.example.com"
secret_key = "<openssl rand -hex 32>"
challenge_ttl_secs = 300
```

`rate_per_native_unit` is how many token units equal 1 native unit (1 ETH =
3200 USDC). `markup_bps` is a percentage markup in basis points (200 = 2%).

The `[mpp]` section is **optional**. When present, the gateway also exposes
`/mpp/jobs/{sid}/{idx}` and `/mpp/jobs/{sid}/{idx}/price`. The `secret_key`
MUST be 64 lowercase hex characters (32 bytes) generated with
`openssl rand -hex 32` — the validator rejects the example value, ASCII
patterns, and low-entropy keys at startup. Rotation invalidates outstanding
challenges; in-flight clients transparently retry on a fresh 402.

## Hitting the gateway with curl

### x402 path (legacy)

```bash
# 1. Discovery: how much does this job cost, on which chains?
curl -s http://127.0.0.1:8402/x402/jobs/1/0/price | jq

# 2. Pay (the X-PAYMENT header is built by an x402 wallet — see
#    https://github.com/coinbase/x402 for client libraries):
curl -X POST \
  -H "X-PAYMENT: $(x402_wallet sign --amount 3264000 --token USDC)" \
  -d 'hello' \
  http://127.0.0.1:8402/x402/jobs/1/0
```

### MPP path (IETF Payment Auth)

```bash
# 1. Discovery (note: protocol="mpp", scheme="charge"):
curl -s http://127.0.0.1:8402/mpp/jobs/1/0/price | jq

# 2. Issue a challenge by POSTing without an Authorization header. The 402
#    response carries one `WWW-Authenticate: Payment ...` header per
#    accepted token. Save the challenge ID for the next step.
curl -i -X POST -d 'hello' http://127.0.0.1:8402/mpp/jobs/1/0
# HTTP/1.1 402 Payment Required
# www-authenticate: Payment id="...", realm="...", method="blueprintevm", ...
# content-type: application/problem+json
# {"type":"https://paymentauth.org/problems/payment-required", ...}

# 3. Build a credential echoing the challenge and POST it. See
#    `scripts/mpp-challenge.sh` for a smoke-test helper that does
#    steps 1-2 automatically.
curl -X POST \
  -H 'Authorization: Payment <base64url-credential>' \
  -d 'hello' \
  http://127.0.0.1:8402/mpp/jobs/1/0
# HTTP/1.1 202 Accepted
# payment-receipt: <base64url-receipt>
# {"status":"accepted","receipt":"...","call_id":1}
```

The `examples/x402-blueprint/tests/x402_gateway.rs` file contains a real
end-to-end credential builder (`build_payment_authorization`) and a
wiremock-stubbed facilitator harness (`stub_facilitator_success`) that
operators can crib for their own client implementations.

## Running

```bash
# All tests (default features, no RPC needed)
cargo test -p x402-blueprint

# Include Uniswap tick math tests
cargo test -p x402-blueprint --features uniswap
```

## Restricted Auth Dry-Run (Signed + On-Chain Parity)

The gateway exposes:

- `POST /x402/jobs/{service_id}/{job_index}/auth-dry-run`

For `restricted_paid` jobs, this route runs the same caller-auth and
`eth_call isPermittedCaller` check as paid execution, but does not enqueue
work or settle payment.

Use the helper script:

```bash
scripts/x402-auth-dry-run.sh \
  --gateway-url http://127.0.0.1:8402 \
  --service-id 1 \
  --job-index 1 \
  --caller-private-key "$CALLER_PK" \
  --body '{"input":"hello"}' \
  --rpc-url http://127.0.0.1:8545 \
  --tangle-contract 0xYourTangleContract
```

Delegated signature payload format used by the gateway:

```text
x402-authorize:{service_id}:{job_index}:{keccak(body)_hex_no_0x}:{nonce}:{expiry_unix_secs}
```

Headers sent by delegated mode:

- `X-TANGLE-CALLER`
- `X-TANGLE-CALLER-SIG`
- `X-TANGLE-CALLER-NONCE`
- `X-TANGLE-CALLER-EXPIRY`

The delegated-signature mode is wire-protocol agnostic — it works on both
the x402 and the MPP ingress unchanged.

## MPP smoke-test

To verify your MPP-enabled gateway is wired correctly, use:

```bash
scripts/mpp-challenge.sh \
  --gateway-url http://127.0.0.1:8402 \
  --service-id 1 \
  --job-index 0
```

The script issues an unpaid `POST /mpp/jobs/...`, captures the
`WWW-Authenticate: Payment` headers, and asserts:

- HTTP status is 402 Payment Required
- `Content-Type` is `application/problem+json` (RFC 9457)
- At least one `WWW-Authenticate: Payment` header is present
- Each challenge advertises `method="blueprintevm"` and `intent="charge"`
- The `type` URI in the body points at `https://paymentauth.org/problems/...`

It does NOT build a real `Authorization: Payment` credential or settle a
payment — that requires an MPP wallet implementation. See
`tests/x402_gateway.rs::build_payment_authorization` for a reference
client-side credential builder, and `stub_facilitator_success` for a
wiremock harness operators can crib for their own implementation.

## How It Works

1. The operator writes `job_pricing.toml` and `x402.toml`.
2. At startup, `load_job_pricing()` parses the TOML into a
   `HashMap<(u64, u32), U256>`.
3. (Optional) An exchange rate oracle fetches live rates and
   `refresh_rates()` updates the config's `rate_per_native_unit`.
4. `X402Gateway::new(config, pricing)` creates the gateway and an
   `X402Producer`.
5. The gateway runs an axum server with three routes:
   - `GET /x402/health` returns "ok".
   - `GET /x402/jobs/{service_id}/{job_index}/price` returns settlement
     options (how much to pay, in which token, on which chain).
   - `POST /x402/jobs/{service_id}/{job_index}` accepts payment and injects
     a `VerifiedPayment` into the producer channel.
6. The `X402Producer` converts each `VerifiedPayment` into a `JobCall` with
   x402 metadata (origin, quote digest, payment network/token).
7. The `Router` dispatches the `JobCall` to the matching handler.

## Exchange Rate Oracles

Three oracle implementations are available behind feature flags. All implement
the `ExchangeRateProvider` trait which returns a `Decimal` rate for a trading
pair.

### Chainlink (feature: `chainlink`)

Reads from Chainlink AggregatorV3 price feed contracts on any EVM chain.

```rust
use alloy_primitives::address;
use x402_blueprint::oracle::ChainlinkOracle;

let oracle = ChainlinkOracle::new(
    provider,
    vec![
        ("ETH", "USD", address!("5f4eC3Df9cbd43714FE2740f5E3616155c5b8419")),
    ],
);
let rate = oracle.rate("ETH", "USD").await?;
```

### Uniswap V3 TWAP (feature: `uniswap`)

Reads tick cumulatives from a Uniswap V3 pool and computes the time-weighted
average price over a configurable window.

```rust
use alloy_primitives::address;
use x402_blueprint::oracle::{UniswapV3TwapOracle, uniswap::PoolConfig};

let oracle = UniswapV3TwapOracle::new(
    provider,
    vec![PoolConfig {
        base: "ETH".into(),
        quote: "USDC".into(),
        pool: address!("88e6A0c2dDD26FEEb64F039a2c41296FcB3f5640"),
        token0_decimals: 6,   // USDC
        token1_decimals: 18,  // WETH
        base_is_token0: false,
    }],
    600, // 10-minute TWAP
);
```

### Coinbase API (feature: `coinbase`)

Fetches spot exchange rates from the public Coinbase API. No API key required.

```rust
use x402_blueprint::oracle::CoinbaseOracle;

let oracle = CoinbaseOracle::new();
let rate = oracle.rate("ETH", "USDC").await?;
```

### Caching

Wrap any oracle in `CachedRateProvider` to avoid hitting the data source on
every call:

```rust
use x402_blueprint::oracle::CachedRateProvider;
use std::time::Duration;

let cached = CachedRateProvider::new(oracle, Duration::from_secs(60));
```

### Applying Rates to Config

Use `refresh_rates()` to update the gateway config before startup:

```rust
use x402_blueprint::oracle::refresh_rates;

refresh_rates(&mut config, &oracle, "ETH").await?;
let (gateway, producer) = X402Gateway::new(config, pricing)?;
```

## Job Pricing

The `PriceOracle` trait controls job prices in wei (independent of exchange
rates). Two built-in implementations:

- `StaticPriceOracle`: wraps a `HashMap` loaded from TOML.
- `ScaledPriceOracle<O>`: wraps any oracle with a multiplier for surge pricing.

```rust
let base = StaticPriceOracle::new(load_job_pricing(&toml_content)?);
let surge = ScaledPriceOracle::new(base, U256::from(3), U256::from(2)); // 1.5x
let (gateway, producer) = X402Gateway::new(config, surge.snapshot())?;
```

## Production Wiring

```rust
use blueprint_runner::BlueprintRunner;
use blueprint_runner::config::BlueprintEnvironment;
use x402_blueprint::router;
use x402_blueprint::oracle::{CoinbaseOracle, CachedRateProvider, refresh_rates};

// Load config
let mut config = X402Config::from_toml("x402.toml")?;
let pricing = load_job_pricing(&std::fs::read_to_string("job_pricing.toml")?)?;

// Fetch live exchange rate
let oracle = CachedRateProvider::new(CoinbaseOracle::new(), Duration::from_secs(60));
refresh_rates(&mut config, &oracle, "ETH").await?;

// Start
let (gateway, x402_producer) = X402Gateway::new(config, pricing)?;

BlueprintRunner::builder((), BlueprintEnvironment::default())
    .router(router())
    .producer(x402_producer)
    .background_service(gateway)
    .run()
    .await?;
```

## Adding Tokens

Add another `[[accepted_tokens]]` block to `x402.toml`:

```toml
[[accepted_tokens]]
network = "eip155:1"
asset = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"
symbol = "USDC"
decimals = 6
pay_to = "0xYourAddressOnEthereum"
rate_per_native_unit = "3200.00"
markup_bps = 100
```

Each token gets its own settlement option in the price discovery response.
Clients pick whichever chain/token they prefer.

## Feature Flags

| Feature | Deps | Description |
|---------|------|-------------|
| `chainlink` | alloy-* | Chainlink AggregatorV3 on-chain oracle |
| `uniswap` | alloy-* | Uniswap V3 TWAP on-chain oracle |
| `coinbase` | reqwest, serde_json | Coinbase REST API oracle |

None are enabled by default. The core example (jobs, router, TOML pricing,
gateway tests) compiles with zero optional dependencies.
