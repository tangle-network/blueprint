# x402 Blueprint Example

Reference implementation showing the full x402 payment pipeline: TOML pricing
config, exchange rate oracles, HTTP gateway, and job dispatch via the Blueprint
router.

## What This Demonstrates

- Two jobs (echo, keccak256) priced in wei and served via x402.
- Static TOML-based pricing and dynamic oracle-based pricing via `PriceOracle`.
- `ScaledPriceOracle` for surge pricing multipliers.
- Exchange rate oracles: Chainlink, Uniswap V3 TWAP, Coinbase API.
- `CachedRateProvider` for TTL-based rate caching.
- `refresh_rates()` helper to update `X402Config` from live oracle data.
- Gateway startup, health checks, price discovery, and job submission.
- Real integration tests with HTTP requests against a running gateway.

## Architecture

```
                          Exchange Rate Oracles
                          (Chainlink / Uniswap / Coinbase)
                                    |
                                    v
                            refresh_rates()
                                    |
                                    v
Client                        X402Config
  |                      (rate_per_native_unit)
  | HTTP POST /x402/jobs/{service_id}/{job_index}
  v
X402Gateway (axum)
  |   GET  /health         -> "ok"
  |   GET  /jobs/.../price -> settlement options (USDC amount)
  |   POST /jobs/...       -> accept payment, inject job
  |
  | VerifiedPayment -> mpsc channel
  v
X402Producer (Stream<Item = JobCall>)
  |
  v
Router
  |-- job 0 -> echo(body)  -> body
  |-- job 1 -> hash(body)  -> keccak256(body)
  v
JobResult
```

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
```

`rate_per_native_unit` is how many token units equal 1 native unit (1 ETH =
3200 USDC). `markup_bps` is a percentage markup in basis points (200 = 2%).

## Running

```bash
# All tests (default features, no RPC needed)
cargo test -p x402-blueprint

# Include Uniswap tick math tests
cargo test -p x402-blueprint --features uniswap
```

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
