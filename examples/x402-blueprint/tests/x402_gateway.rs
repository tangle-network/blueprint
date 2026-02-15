//! Integration tests for the x402 payment gateway example.
//!
//! These are real end-to-end tests: they start an actual HTTP server, send
//! real HTTP requests, and verify the full pipeline from gateway to producer.

use alloy_primitives::U256;
use blueprint_runner::BackgroundService;
use blueprint_x402::producer::{VerifiedPayment, X402_ORIGIN_KEY, X402Producer};
use blueprint_x402::{X402Config, X402Gateway};
use bytes::Bytes;
use std::collections::HashMap;
use std::net::{SocketAddr, TcpListener};
use tokio::task::JoinHandle;
use tower::Service;
use x402_blueprint::{PriceOracle, ScaledPriceOracle, StaticPriceOracle, load_job_pricing, router};

// ---- Shared test infrastructure ----

/// Bind to port 0, grab the assigned port, close the socket.
fn free_port() -> u16 {
    let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
    let port = listener.local_addr().unwrap().port();
    drop(listener);
    port
}

/// Load configs from the example's config directory and start the gateway.
///
/// Returns the task handle and an `X402Producer` that receives verified
/// payments from the gateway.
async fn start_gateway(
    port: u16,
    pricing: HashMap<(u64, u32), U256>,
) -> (JoinHandle<()>, X402Producer) {
    let config_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/config");

    let mut config =
        X402Config::from_toml(format!("{config_dir}/x402.toml")).expect("load x402.toml");
    config.bind_address = SocketAddr::from(([127, 0, 0, 1], port));

    let (gateway, producer) = X402Gateway::new(config, pricing).expect("create gateway");

    let handle = tokio::spawn(async move {
        let _rx = gateway.start().await.expect("start gateway");
        // Keep alive until the task is aborted.
        futures::future::pending::<()>().await;
    });

    // Give the server a moment to bind.
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    (handle, producer)
}

fn load_example_pricing() -> HashMap<(u64, u32), U256> {
    let content = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/config/job_pricing.toml"
    ))
    .expect("read job_pricing.toml");
    load_job_pricing(&content).expect("parse job_pricing.toml")
}

// ---- Tests ----

#[tokio::test]
async fn test_health_check() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/x402/health"))
        .await
        .expect("GET /x402/health");

    assert_eq!(resp.status(), 200);
    assert_eq!(resp.text().await.unwrap(), "ok");

    handle.abort();
}

#[tokio::test]
async fn test_price_discovery() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/x402/jobs/1/0/price"))
        .await
        .expect("GET price");

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    let options = body["settlement_options"].as_array().unwrap();
    assert!(!options.is_empty());

    // 0.001 ETH * 3200 USDC/ETH * 1.02 markup = 3.264 USDC = 3264000 smallest units
    let amount = options[0]["amount"].as_str().unwrap();
    assert_eq!(amount, "3264000");

    handle.abort();
}

#[tokio::test]
async fn test_unknown_job_returns_404() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/x402/jobs/1/99/price"))
        .await
        .expect("GET unknown job price");

    assert_eq!(resp.status(), 404);

    handle.abort();
}

#[tokio::test]
async fn test_job_submission_produces_verified_payment() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, mut producer) = start_gateway(port, pricing).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/x402/jobs/1/0"))
        .body("hello")
        .send()
        .await
        .expect("POST job");

    assert_eq!(resp.status(), 202);

    // The gateway should have sent a VerifiedPayment to the producer channel.
    // Read it via the underlying receiver.
    use futures::StreamExt;
    let job_call = tokio::time::timeout(std::time::Duration::from_secs(2), producer.next())
        .await
        .expect("timeout waiting for job call")
        .expect("stream ended")
        .expect("job call error");

    assert_eq!(job_call.job_id(), blueprint_core::JobId::from(0u64));
    assert_eq!(job_call.body(), &Bytes::from("hello"));

    let origin = job_call.metadata().get(X402_ORIGIN_KEY).unwrap();
    assert_eq!(origin.as_bytes(), b"x402");

    handle.abort();
}

#[tokio::test]
async fn test_router_dispatches_echo_job() {
    let payment = VerifiedPayment {
        service_id: 1,
        job_index: 0,
        job_args: Bytes::from("hello world"),
        quote_digest: [0xAA; 32],
        payment_network: "eip155:8453".into(),
        payment_token: "USDC".into(),
        call_id: 1,
    };

    let job_call = payment.into_job_call();
    let mut r = router();
    let result = r.as_service().call(job_call).await.unwrap();

    let results = result.expect("router matched");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].body().unwrap(), &Bytes::from("hello world"));
}

#[tokio::test]
async fn test_router_dispatches_hash_job() {
    let input = b"test data";
    let expected = alloy_primitives::keccak256(input);

    let payment = VerifiedPayment {
        service_id: 1,
        job_index: 1,
        job_args: Bytes::from_static(input),
        quote_digest: [0xBB; 32],
        payment_network: "eip155:8453".into(),
        payment_token: "USDC".into(),
        call_id: 2,
    };

    let job_call = payment.into_job_call();
    let mut r = router();
    let result = r.as_service().call(job_call).await.unwrap();

    let results = result.expect("router matched");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].body().unwrap().as_ref(), expected.as_slice());
}

#[tokio::test]
async fn test_settlement_options_computed_correctly() {
    let config_dir = concat!(env!("CARGO_MANIFEST_DIR"), "/config");
    let config = X402Config::from_toml(format!("{config_dir}/x402.toml")).expect("load config");

    // 0.01 ETH = 10_000_000_000_000_000 wei
    let price_wei = U256::from(10_000_000_000_000_000u64);
    let options =
        X402Gateway::settlement_options(&config, 1, 1, &price_wei).expect("settlement options");

    assert_eq!(options.len(), 1);
    assert_eq!(options[0].symbol, "USDC");
    // 0.01 ETH * 3200 * 1.02 = 32.64 USDC = 32640000 smallest units
    assert_eq!(options[0].amount, "32640000");
}

// ---- Price oracle tests ----

#[test]
fn test_static_oracle_seeds_gateway() {
    let pricing = load_example_pricing();
    let oracle = StaticPriceOracle::new(pricing);

    assert_eq!(
        oracle.price_wei(1, 0),
        Some(U256::from(1_000_000_000_000_000u64))
    );
    assert_eq!(
        oracle.price_wei(1, 1),
        Some(U256::from(10_000_000_000_000_000u64))
    );
    assert_eq!(oracle.price_wei(1, 99), None);
}

#[test]
fn test_scaled_oracle_applies_multiplier() {
    let pricing = load_example_pricing();
    let base = StaticPriceOracle::new(pricing);
    // 2x surge pricing
    let oracle = ScaledPriceOracle::new(base, U256::from(2u64), U256::from(1u64));

    assert_eq!(
        oracle.price_wei(1, 0),
        Some(U256::from(2_000_000_000_000_000u64))
    );

    // Verify the snapshot is also scaled
    let snap = oracle.snapshot();
    assert_eq!(snap[&(1, 0)], U256::from(2_000_000_000_000_000u64));
    assert_eq!(snap[&(1, 1)], U256::from(20_000_000_000_000_000u64));
}

#[tokio::test]
async fn test_gateway_with_scaled_pricing() {
    let port = free_port();
    let pricing = load_example_pricing();
    let base = StaticPriceOracle::new(pricing);
    // 1.5x multiplier
    let oracle = ScaledPriceOracle::new(base, U256::from(3u64), U256::from(2u64));
    let scaled_pricing = oracle.snapshot();

    let (handle, _producer) = start_gateway(port, scaled_pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/x402/jobs/1/0/price"))
        .await
        .expect("GET price");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();

    // Base: 0.001 ETH. Scaled: 0.0015 ETH.
    // 0.0015 * 3200 * 1.02 = 4.896 USDC = 4896000 smallest units
    let amount = body["settlement_options"][0]["amount"].as_str().unwrap();
    assert_eq!(amount, "4896000");

    handle.abort();
}

// ---- Custom token config test ----

/// Build an X402Config with a custom token address and verify that
/// settlement options reference the correct contract address and amount.
#[test]
fn test_settlement_with_custom_token_address() {
    let token_address = "0x1234567890abcdef1234567890abcdef12345678";
    let pay_to = "0xdeadbeefdeadbeefdeadbeefdeadbeefdeadbeef";

    let config = X402Config {
        bind_address: "127.0.0.1:0".parse().unwrap(),
        facilitator_url: "https://facilitator.x402.org".parse().unwrap(),
        quote_ttl_secs: 300,
        accepted_tokens: vec![blueprint_x402::config::AcceptedToken {
            network: "eip155:31337".into(),
            asset: token_address.into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: pay_to.into(),
            rate_per_native_unit: rust_decimal::Decimal::from(3200u32),
            markup_bps: 0,
        }],
        job_overrides: Default::default(),
        service_id: 1,
    };

    let price_wei = U256::from(1_000_000_000_000_000u64); // 0.001 ETH
    let options =
        X402Gateway::settlement_options(&config, 1, 0, &price_wei).expect("settlement options");

    assert_eq!(options.len(), 1);
    assert_eq!(options[0].asset, token_address);
    assert_eq!(options[0].pay_to, pay_to);
    assert_eq!(options[0].network, "eip155:31337");
    assert_eq!(options[0].amount, "3200000"); // 3.2 USDC, no markup
}

/// Verify settlement works with multiple accepted tokens.
#[test]
fn test_settlement_with_multiple_tokens() {
    let config = X402Config {
        bind_address: "127.0.0.1:0".parse().unwrap(),
        facilitator_url: "https://facilitator.x402.org".parse().unwrap(),
        quote_ttl_secs: 300,
        accepted_tokens: vec![
            blueprint_x402::config::AcceptedToken {
                network: "eip155:8453".into(),
                asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
                symbol: "USDC".into(),
                decimals: 6,
                pay_to: "0x0000000000000000000000000000000000000001".into(),
                rate_per_native_unit: rust_decimal::Decimal::from(3200u32),
                markup_bps: 0,
            },
            blueprint_x402::config::AcceptedToken {
                network: "eip155:1".into(),
                asset: "0x6B175474E89094C44Da98b954EedeAC495271d0F".into(),
                symbol: "DAI".into(),
                decimals: 18,
                pay_to: "0x0000000000000000000000000000000000000002".into(),
                rate_per_native_unit: rust_decimal::Decimal::from(3200u32),
                markup_bps: 100, // 1%
            },
        ],
        job_overrides: Default::default(),
        service_id: 1,
    };

    let price_wei = U256::from(1_000_000_000_000_000u64); // 0.001 ETH
    let options =
        X402Gateway::settlement_options(&config, 1, 0, &price_wei).expect("settlement options");

    assert_eq!(options.len(), 2);

    // USDC: 0.001 * 3200 = 3.2 USDC = 3200000 (6 decimals, no markup)
    assert_eq!(options[0].symbol, "USDC");
    assert_eq!(options[0].amount, "3200000");

    // DAI: 0.001 * 3200 * 1.01 = 3.232 DAI = 3232000000000000000 (18 decimals, 1% markup)
    assert_eq!(options[1].symbol, "DAI");
    assert_eq!(options[1].amount, "3232000000000000000");
}
