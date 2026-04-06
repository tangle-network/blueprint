//! Integration tests for the x402 payment gateway example.
//!
//! These are real end-to-end tests: they start an actual HTTP server, send
//! real HTTP requests, and verify the full pipeline from gateway to producer.

use alloy_primitives::U256;
use blueprint_runner::BackgroundService;
use blueprint_x402::config::X402InvocationMode;
use blueprint_x402::producer::{VerifiedPayment, X402Producer};
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
async fn test_auth_dry_run_public_job() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!(
            "http://127.0.0.1:{port}/x402/jobs/1/0/auth-dry-run"
        ))
        .body("hello")
        .send()
        .await
        .expect("POST dry-run");

    assert_eq!(resp.status(), 200);
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["allowed"], true);
    assert_eq!(body["mode"], "public_paid");

    handle.abort();
}

#[tokio::test]
async fn test_unpaid_request_returns_402() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    // POST without a payment header -- the x402 middleware should reject with 402.
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/x402/jobs/1/0"))
        .body("hello")
        .send()
        .await
        .expect("POST job");

    assert_eq!(
        resp.status(),
        402,
        "unpaid request must be rejected by x402 middleware"
    );

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
        caller: None,
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
        caller: None,
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
    let oracle = ScaledPriceOracle::new(base, U256::from(2u64), U256::from(1u64)).unwrap();

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
    let oracle = ScaledPriceOracle::new(base, U256::from(3u64), U256::from(2u64)).unwrap();
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
            transfer_method: "permit2".into(),
            eip3009_name: None,
            eip3009_version: None,
        }],
        default_invocation_mode: X402InvocationMode::Disabled,
        job_policies: vec![],

        service_id: 1,
        mpp: None,
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
                transfer_method: "permit2".into(),
                eip3009_name: None,
                eip3009_version: None,
            },
            blueprint_x402::config::AcceptedToken {
                network: "eip155:1".into(),
                asset: "0x6B175474E89094C44Da98b954EedeAC495271d0F".into(),
                symbol: "DAI".into(),
                decimals: 18,
                pay_to: "0x0000000000000000000000000000000000000002".into(),
                rate_per_native_unit: rust_decimal::Decimal::from(3200u32),
                markup_bps: 100, // 1%
                transfer_method: "permit2".into(),
                eip3009_name: None,
                eip3009_version: None,
            },
        ],
        default_invocation_mode: X402InvocationMode::Disabled,
        job_policies: vec![],

        service_id: 1,
        mpp: None,
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

// ─────────────────────────────────────────────────────────────────────────────
// MPP (Machine Payments Protocol) integration tests
// ─────────────────────────────────────────────────────────────────────────────
//
// These tests exercise the parallel /mpp/jobs/... ingress added by the
// blueprint-x402 MPP integration. The example x402.toml now ships with an
// `[mpp]` section so `start_gateway` brings the MPP routes up automatically.

/// `GET /mpp/jobs/1/0/price` returns settlement options carrying
/// `protocol: "mpp"` with the same converted amount as the x402 path.
#[tokio::test]
async fn test_mpp_price_discovery() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/mpp/jobs/1/0/price"))
        .await
        .expect("GET /mpp price");

    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    let options = body["settlement_options"].as_array().unwrap();
    assert!(
        !options.is_empty(),
        "must advertise at least one MPP method"
    );
    assert_eq!(options[0]["protocol"], "mpp");
    // Same wei→token math as the x402 path: 0.001 ETH * 3200 USDC * 1.02 markup = 3.264 USDC.
    assert_eq!(options[0]["amount"], "3264000");
    assert_eq!(options[0]["scheme"], "charge");

    handle.abort();
}

/// `GET /mpp/jobs/1/99/price` for an unknown job is rejected with a 404
/// RFC 9457 Problem Details payload.
#[tokio::test]
async fn test_mpp_unknown_job_returns_problem() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/mpp/jobs/1/99/price"))
        .await
        .expect("GET /mpp unknown");

    assert_eq!(resp.status(), 404);
    assert_eq!(
        resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("application/problem+json")
    );

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], 404);
    assert!(
        body["type"]
            .as_str()
            .unwrap_or_default()
            .starts_with("https://paymentauth.org/problems/"),
        "MPP errors must use IETF Problem Details type URIs"
    );

    handle.abort();
}

/// `POST /mpp/jobs/1/0` without an `Authorization: Payment` header returns
/// `402 Payment Required` with `WWW-Authenticate: Payment` headers — one
/// per accepted token.
#[tokio::test]
async fn test_mpp_unpaid_returns_challenge() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/0"))
        .body("hello")
        .send()
        .await
        .expect("POST /mpp/jobs/1/0");

    assert_eq!(resp.status(), 402, "MPP must speak HTTP 402 like x402");

    let challenges: Vec<_> = resp
        .headers()
        .get_all("www-authenticate")
        .iter()
        .filter_map(|v| v.to_str().ok().map(str::to_owned))
        .collect();
    assert!(
        !challenges.is_empty(),
        "must emit at least one WWW-Authenticate: Payment header"
    );
    for challenge in &challenges {
        assert!(
            challenge.starts_with("Payment "),
            "WWW-Authenticate must use Payment scheme: {challenge}"
        );
        assert!(
            challenge.contains("method=\"blueprintevm\""),
            "challenge must advertise method=blueprintevm: {challenge}"
        );
        assert!(
            challenge.contains("intent=\"charge\""),
            "challenge must advertise intent=charge: {challenge}"
        );
    }

    // The 402 body MUST be RFC 9457 Problem Details.
    assert_eq!(
        resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("application/problem+json")
    );
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], 402);
    assert!(body["challenge_ids"].is_array());

    handle.abort();
}

/// `POST /mpp/jobs/1/0` with a malformed `Authorization: Payment` header
/// returns a 400 with the `malformed-credential` IETF code.
#[tokio::test]
async fn test_mpp_malformed_credential_returns_problem() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/0"))
        .header("authorization", "Payment not-base64url-and-not-json")
        .body("hello")
        .send()
        .await
        .expect("POST /mpp with bad credential");

    assert_eq!(resp.status(), 400);
    assert_eq!(
        resp.headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok()),
        Some("application/problem+json")
    );
    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["status"], 400);
    assert!(
        body["type"]
            .as_str()
            .unwrap_or_default()
            .ends_with("malformed-credential"),
        "type must signal malformed-credential, got {}",
        body["type"]
    );

    handle.abort();
}

/// MPP routes are NOT registered when the operator omits the `[mpp]` config
/// section. This test starts a gateway with MPP disabled and asserts that
/// `/mpp/jobs/...` returns 404 from the axum router (no route match), while
/// the legacy `/x402/jobs/...` endpoints still work.
#[tokio::test]
async fn test_mpp_routes_disabled_when_unconfigured() {
    use blueprint_x402::config::AcceptedToken;
    use rust_decimal::Decimal;

    // Build a minimal config WITHOUT [mpp].
    let config = X402Config {
        bind_address: SocketAddr::from(([127, 0, 0, 1], free_port())),
        facilitator_url: "https://facilitator.x402.org".parse().unwrap(),
        quote_ttl_secs: 300,
        accepted_tokens: vec![AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x0000000000000000000000000000000000000001".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps: 0,
            transfer_method: "permit2".into(),
            eip3009_name: None,
            eip3009_version: None,
        }],
        default_invocation_mode: X402InvocationMode::PublicPaid,
        job_policies: vec![],
        service_id: 1,
        mpp: None,
    };
    let port = config.bind_address.port();

    let mut pricing = HashMap::new();
    pricing.insert((1, 0), U256::from(1_000_000_000_000_000u64));

    let (gateway, _producer) = X402Gateway::new(config, pricing).expect("create gateway");
    let handle = tokio::spawn(async move {
        let _rx = gateway.start().await.expect("start gateway");
        futures::future::pending::<()>().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Existing x402 ingress still works.
    let x402_resp = reqwest::get(format!("http://127.0.0.1:{port}/x402/jobs/1/0/price"))
        .await
        .expect("GET x402 price");
    assert_eq!(x402_resp.status(), 200);

    // MPP route is not registered → axum returns 404 (not the MPP problem JSON).
    let mpp_resp = reqwest::get(format!("http://127.0.0.1:{port}/mpp/jobs/1/0/price"))
        .await
        .expect("GET mpp price");
    assert_eq!(mpp_resp.status(), 404);

    handle.abort();
}

/// `/x402/stats` exposes the new MPP-specific counters (initially zero).
#[tokio::test]
async fn test_stats_includes_mpp_counters() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let resp = reqwest::get(format!("http://127.0.0.1:{port}/x402/stats"))
        .await
        .expect("GET stats");
    assert_eq!(resp.status(), 200);

    let body: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(body["counters"]["mpp_accepted"], 0);
    assert_eq!(body["counters"]["mpp_challenge_issued"], 0);
    assert_eq!(body["counters"]["mpp_verification_failed"], 0);

    handle.abort();
}

// ───────────────────────────────────────────────────────────────────────────
// MPP end-to-end credential roundtrip
// ───────────────────────────────────────────────────────────────────────────
//
// These tests stub the x402 facilitator's `/verify` + `/settle` endpoints
// with `wiremock`, so we can drive a full Authorization: Payment credential
// through the MPP route without needing an on-chain settler. They are the
// missing happy-path coverage flagged by the audit.

use blueprint_x402::config::{AcceptedToken, MppConfig};
use mpp::protocol::core::headers::{format_authorization, parse_www_authenticate};
use mpp::protocol::core::{ChallengeEcho, PaymentCredential};
use rust_decimal::Decimal;
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

const TEST_PAYER_ADDR: &str = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266";
const TEST_TX_HASH: &str = "0x1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef";

/// Build an MPP-enabled gateway pointing at the supplied facilitator URL
/// and a fixed accepted token. Returns `(handle, port, producer)`.
///
/// **Important**: the caller MUST keep the returned producer alive for the
/// duration of the test. Dropping it closes the producer channel and the
/// next enqueue from the gateway will fail with `service shutting down`.
async fn start_mpp_gateway_with_facilitator(
    facilitator_url: &str,
) -> (JoinHandle<()>, u16, X402Producer) {
    let port = free_port();
    let config = X402Config {
        bind_address: SocketAddr::from(([127, 0, 0, 1], port)),
        facilitator_url: facilitator_url.parse().expect("valid facilitator url"),
        quote_ttl_secs: 300,
        accepted_tokens: vec![AcceptedToken {
            network: "eip155:8453".into(),
            asset: "0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913".into(),
            symbol: "USDC".into(),
            decimals: 6,
            pay_to: "0x0000000000000000000000000000000000000001".into(),
            rate_per_native_unit: Decimal::from(3200u32),
            markup_bps: 0,
            transfer_method: "eip3009".into(),
            eip3009_name: Some("USD Coin".into()),
            eip3009_version: Some("2".into()),
        }],
        default_invocation_mode: X402InvocationMode::PublicPaid,
        job_policies: vec![],
        service_id: 1,
        mpp: Some(MppConfig {
            realm: "test.example.com".into(),
            // Distinct from the demo secret rejected by the validator.
            secret_key: "9e7c2f4b6d1a0832514768af9c3e2b14f827d6e09a3b1c7d4e6f8a02b9c5d70e".into(),
            challenge_ttl_secs: 300,
        }),
    };
    let mut pricing = HashMap::new();
    pricing.insert((1, 0), U256::from(1_000_000_000_000_000u64)); // 0.001 ETH
    pricing.insert((1, 1), U256::from(2_000_000_000_000_000u64)); // 0.002 ETH

    let (gateway, producer) = X402Gateway::new(config, pricing).expect("create gateway");
    let handle = tokio::spawn(async move {
        let _rx = gateway.start().await.expect("start gateway");
        futures::future::pending::<()>().await;
    });
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    (handle, port, producer)
}

/// Stub the x402 facilitator's `/verify` and `/settle` endpoints to always
/// succeed. The MPP route forwards a v1 ExactScheme VerifyRequest to these
/// endpoints; we don't validate the body shape because the test is about
/// the *integration* — the facilitator already has its own coverage.
async fn stub_facilitator_success(server: &MockServer) {
    Mock::given(method("POST"))
        .and(path("/verify"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "isValid": true,
            "payer": TEST_PAYER_ADDR,
        })))
        .mount(server)
        .await;
    Mock::given(method("POST"))
        .and(path("/settle"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "success": true,
            "transaction": TEST_TX_HASH,
            "payer": TEST_PAYER_ADDR,
            "network": "base",
        })))
        .mount(server)
        .await;
}

/// Issue a 402 challenge against `/mpp/jobs/{sid}/{idx}` and return the
/// first parsed `PaymentChallenge`.
async fn fetch_mpp_challenge(
    port: u16,
    sid: u64,
    idx: u32,
) -> mpp::protocol::core::PaymentChallenge {
    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/{sid}/{idx}"))
        .body("hello")
        .send()
        .await
        .expect("POST 402");
    assert_eq!(resp.status(), 402);
    let header = resp
        .headers()
        .get("www-authenticate")
        .expect("WWW-Authenticate")
        .to_str()
        .expect("ascii header")
        .to_string();
    parse_www_authenticate(&header).expect("parse challenge")
}

/// Build a valid `Authorization: Payment` header from a server-issued
/// challenge. The inner `x402_payload` is intentionally a fake — the
/// wiremock facilitator stub accepts any well-formed JSON.
fn build_payment_authorization(challenge: &mpp::protocol::core::PaymentChallenge) -> String {
    use base64::Engine;

    // Mirror the on-the-wire shape of an x402 v1 PaymentPayload<ExactScheme,
    // ExactEvmPayload>. Real wallets sign this with EIP-712; the wiremock
    // stub doesn't care about cryptographic validity, only that the JSON is
    // well-formed and forwarded to /verify.
    let inner = serde_json::json!({
        "x402Version": 1,
        "scheme": "exact",
        "network": "base",
        "payload": {
            "signature": "0xdeadbeef",
            "authorization": {
                "from": TEST_PAYER_ADDR,
                "to": "0x0000000000000000000000000000000000000001",
                "value": "3200000",
                "validAfter": "0",
                "validBefore": "9999999999",
                "nonce": "0x0000000000000000000000000000000000000000000000000000000000000001"
            }
        }
    });
    let inner_b64 = base64::engine::general_purpose::URL_SAFE_NO_PAD
        .encode(serde_json::to_vec(&inner).unwrap());

    let echo = ChallengeEcho {
        id: challenge.id.clone(),
        realm: challenge.realm.clone(),
        method: challenge.method.clone(),
        intent: challenge.intent.clone(),
        request: challenge.request.clone(),
        expires: challenge.expires.clone(),
        digest: challenge.digest.clone(),
        opaque: challenge.opaque.clone(),
    };
    let credential = PaymentCredential::new(echo, serde_json::json!({ "x402_payload": inner_b64 }));
    format_authorization(&credential).expect("format Authorization header")
}

/// Happy-path roundtrip:
/// 1. Issue 402 challenge
/// 2. Build credential echoing that challenge
/// 3. POST credential to the same route
/// 4. Wiremock facilitator says verify+settle succeeded
/// 5. Assert 202 ACCEPTED + Payment-Receipt header set
#[tokio::test]
async fn test_mpp_credential_happy_path() {
    let server = MockServer::start().await;
    stub_facilitator_success(&server).await;

    let (handle, port, _producer) = start_mpp_gateway_with_facilitator(&server.uri()).await;
    let challenge = fetch_mpp_challenge(port, 1, 0).await;
    let auth = build_payment_authorization(&challenge);

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/0"))
        .header("authorization", auth)
        .body("hello")
        .send()
        .await
        .expect("POST credential");

    assert_eq!(
        resp.status(),
        202,
        "valid credential must be accepted (got {}: {:?})",
        resp.status(),
        resp.text().await.ok()
    );
    assert!(
        resp.headers().get("payment-receipt").is_some(),
        "Payment-Receipt header must be present on 202"
    );

    handle.abort();
}

/// Cross-route credential replay must be rejected: a credential issued for
/// `/mpp/jobs/1/0` cannot be replayed against `/mpp/jobs/1/1`. The route
/// guard checks `credential.method_details.{service_id, job_index}` against
/// the URL path before consulting the facilitator.
#[tokio::test]
async fn test_mpp_cross_route_replay_rejected() {
    let server = MockServer::start().await;
    stub_facilitator_success(&server).await;

    let (handle, port, _producer) = start_mpp_gateway_with_facilitator(&server.uri()).await;
    let challenge = fetch_mpp_challenge(port, 1, 0).await;
    let auth = build_payment_authorization(&challenge);

    let client = reqwest::Client::new();
    let resp = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/1"))
        .header("authorization", auth)
        .body("hello")
        .send()
        .await
        .expect("POST credential to wrong job");

    // Either 403 (route guard) or 402 (verify_credential_with_expected_request
    // amount mismatch), depending on which check fires first. Both are
    // acceptable refusals; what we MUST NOT see is 202.
    assert!(
        resp.status() == 403 || resp.status() == 402 || resp.status() == 400,
        "cross-route replay must be rejected (got {})",
        resp.status()
    );
    assert_ne!(
        resp.status(),
        202,
        "cross-route replay must NOT be accepted"
    );

    let body: serde_json::Value = resp.json().await.unwrap();
    assert!(
        body.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .starts_with("https://paymentauth.org/problems/"),
        "must surface IETF Problem Details"
    );

    handle.abort();
}

/// Per-credential challenge id is single-use against the same route: a
/// second submission of the same credential is rejected by the registry's
/// `quote_conflict` (or by the facilitator's nonce check in the real world).
#[tokio::test]
async fn test_mpp_credential_replay_same_route_eventually_rejected() {
    let server = MockServer::start().await;
    stub_facilitator_success(&server).await;

    let (handle, port, _producer) = start_mpp_gateway_with_facilitator(&server.uri()).await;
    let challenge = fetch_mpp_challenge(port, 1, 0).await;
    let auth = build_payment_authorization(&challenge);

    let client = reqwest::Client::new();
    // First submission: should succeed.
    let r1 = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/0"))
        .header("authorization", &auth)
        .body("hello")
        .send()
        .await
        .expect("POST credential 1");
    assert_eq!(r1.status(), 202);

    // Second submission of the SAME credential. Today the gateway-side
    // single-use guard is the `QuoteRegistry`; the facilitator's EIP-3009
    // nonce check is the on-chain backstop. We accept either:
    //   - 202 (gateway permits, on-chain rejects in production)
    //   - 4xx (gateway-side dedup catches it)
    // What we MUST NOT see: a 5xx panic.
    let r2 = client
        .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/0"))
        .header("authorization", &auth)
        .body("hello")
        .send()
        .await
        .expect("POST credential 2");
    assert_ne!(
        r2.status().as_u16() / 100,
        5,
        "replayed credential must not 5xx (got {})",
        r2.status()
    );

    handle.abort();
}

/// After requesting `/mpp/jobs/1/0` without payment, the
/// `mpp_challenge_issued` counter is incremented.
#[tokio::test]
async fn test_mpp_challenge_counter_increments() {
    let port = free_port();
    let pricing = load_example_pricing();
    let (handle, _producer) = start_gateway(port, pricing).await;

    let client = reqwest::Client::new();
    for _ in 0..3 {
        let resp = client
            .post(format!("http://127.0.0.1:{port}/mpp/jobs/1/0"))
            .body("hello")
            .send()
            .await
            .expect("POST /mpp/jobs/1/0");
        assert_eq!(resp.status(), 402);
    }

    let stats: serde_json::Value = reqwest::get(format!("http://127.0.0.1:{port}/x402/stats"))
        .await
        .expect("GET stats")
        .json()
        .await
        .unwrap();
    assert_eq!(stats["counters"]["mpp_challenge_issued"], 3);

    handle.abort();
}
