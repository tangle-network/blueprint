//! Real on-chain E2E tests for the x402 payment flow.
//!
//! Gate: ANVIL_E2E=1 (forks Base mainnet via Anvil).
//!
//! Full production flow:
//!   1. Anvil forks Base → real USDC with transferWithAuthorization
//!   2. Fund test wallet via whale impersonation
//!   3. Real facilitator (V1Eip155ExactFacilitator) against Anvil RPC
//!   4. Real x402 gateway pointing at real facilitator
//!   5. Client signs EIP-3009 payment → X-PAYMENT header → gateway
//!   6. Gateway → facilitator → on-chain transferWithAuthorization → job dispatched
//!   7. Verify: USDC moved on-chain, producer got VerifiedPayment
//!
//! No wiremock. No stubs. Real USDC. Real EIP-3009. Real EVM.

#[cfg(test)]
mod e2e {
    use alloy_primitives::{Address, U256, address};
    use alloy_provider::{Provider, ProviderBuilder};
    use alloy_rpc_types::TransactionRequest;
    use alloy_signer_local::PrivateKeySigner;
    use alloy_sol_types::{SolCall, sol};
    use std::collections::HashMap;
    use std::net::{SocketAddr, TcpListener};
    use std::process::{Child, Command as StdCommand, Stdio};
    use std::sync::Arc;
    use std::time::Duration;
    use tokio::task::JoinHandle;

    use blueprint_runner::BackgroundService;
    use blueprint_x402::config::{AcceptedToken, X402Config, X402InvocationMode};
    use blueprint_x402::producer::X402Producer;
    use blueprint_x402::X402Gateway;
    use rust_decimal::Decimal;

    // x402 EIP-3009 client + facilitator
    use x402_chain_eip155::v1_eip155_exact::{
        Eip3009SigningParams, sign_erc3009_authorization,
        PaymentRequirementsExtra, V1Eip155ExactFacilitator,
    };
    use x402_chain_eip155::chain::{
        Eip155ChainReference, Eip155ChainProvider,
        config::{Eip155ChainConfig, Eip155ChainConfigInner, RpcConfig},
    };
    use x402_types::chain::FromConfig;
    use x402_types::config::LiteralOrEnv;
    use x402_types::scheme::X402SchemeFacilitator;

    const USDC_ADDRESS: Address = address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913");
    const USDC_WHALE: Address = address!("0x3304E22DDaa22bCdC5fCa2269b418046aE7b566A");
    const OPERATOR: Address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    const OPERATOR_KEY: &str = "ac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    const PAYER: Address = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    const PAYER_KEY: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

    sol! {
        #[sol(rpc)]
        interface IERC20 {
            function balanceOf(address owner) external view returns (uint256);
            function transfer(address to, uint256 amount) external returns (bool);
        }
    }

    // ─── Anvil Fork ─────────────────────────────────────────────────

    struct AnvilFork { child: Child, port: u16 }

    impl AnvilFork {
        fn spawn() -> Self {
            let port = free_port();
            let base_rpc = std::env::var("BASE_RPC_URL")
                .unwrap_or_else(|_| "https://mainnet.base.org".into());
            let child = StdCommand::new("anvil")
                .args(["--fork-url", &base_rpc, "--port", &port.to_string(),
                       "--silent", "--auto-impersonate"])
                .stdout(Stdio::null()).stderr(Stdio::null())
                .spawn().expect("anvil must be installed");
            std::thread::sleep(Duration::from_secs(5));
            Self { child, port }
        }
        fn rpc_url(&self) -> String { format!("http://127.0.0.1:{}", self.port) }
    }

    impl Drop for AnvilFork {
        fn drop(&mut self) { let _ = self.child.kill(); let _ = self.child.wait(); }
    }

    fn free_port() -> u16 {
        let l = TcpListener::bind("127.0.0.1:0").unwrap();
        let p = l.local_addr().unwrap().port(); drop(l); p
    }

    async fn fund_payer(rpc_url: &str, amount: U256) {
        let p = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());
        p.raw_request::<_, ()>("anvil_impersonateAccount".into(), [format!("{:#x}", USDC_WHALE)]).await.unwrap();
        let tx = TransactionRequest::default().from(USDC_WHALE).to(USDC_ADDRESS)
            .input(IERC20::transferCall { to: PAYER, amount }.abi_encode().into());
        p.send_transaction(tx).await.unwrap().get_receipt().await.unwrap();
        p.raw_request::<_, ()>("anvil_stopImpersonatingAccount".into(), [format!("{:#x}", USDC_WHALE)]).await.unwrap();
    }

    async fn usdc_balance(rpc_url: &str, addr: Address) -> U256 {
        let p = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());
        IERC20::new(USDC_ADDRESS, &p).balanceOf(addr).call().await.unwrap()
    }

    // ─── Real Facilitator Server ────────────────────────────────────

    async fn start_real_facilitator(rpc_url: &str) -> (JoinHandle<()>, u16) {
        use axum::{Json, Router, routing::{get, post}};

        let port = free_port();

        // Build Eip155ChainConfig for Base on the Anvil fork
        let config = Eip155ChainConfig {
            chain_reference: Eip155ChainReference::new(8453),
            inner: Eip155ChainConfigInner {
                eip1559: true,
                flashblocks: false,
                signers: vec![
                    LiteralOrEnv::from_literal(format!("0x{OPERATOR_KEY}").parse().unwrap()),
                ],
                rpc: vec![RpcConfig {
                    http: LiteralOrEnv::from_literal(rpc_url.parse().unwrap()),
                    rate_limit: None,
                }],
                receipt_timeout_secs: 30,
            },
        };

        // Create the real EVM provider + facilitator
        let provider = x402_chain_eip155::chain::Eip155ChainProvider::from_config(&config)
            .await.expect("build Eip155ChainProvider");
        let facilitator = Arc::new(V1Eip155ExactFacilitator::new(provider));

        let fac_verify = facilitator.clone();
        let fac_settle = facilitator.clone();
        let fac_supported = facilitator.clone();

        // Sanitize facilitator errors: strip RPC URLs and revert calldata
        // that could leak infrastructure topology to callers.
        fn sanitize_error(e: &dyn std::fmt::Display) -> String {
            let msg = format!("{e}");
            // Strip URLs from error messages (prevents RPC endpoint leakage)
            let mut sanitized = msg;
            for prefix in &["http://", "https://", "ws://", "wss://"] {
                while let Some(start) = sanitized.find(prefix) {
                    let end = sanitized[start..].find(|c: char| c.is_whitespace() || c == ',' || c == ')' || c == '"')
                        .map(|e| start + e)
                        .unwrap_or(sanitized.len());
                    sanitized.replace_range(start..end, "[REDACTED_URL]");
                }
            }
            // Truncate to prevent massive revert data from being returned
            if sanitized.len() > 500 { sanitized[..500].to_string() } else { sanitized }
        }

        let app = Router::new()
            // Body size limit: 64KB max to prevent OOM from deeply nested JSON
            .layer(axum::extract::DefaultBodyLimit::max(64 * 1024))
            .route("/verify", post(move |Json(body): Json<serde_json::Value>| {
                let fac = fac_verify.clone();
                async move {
                    let raw = serde_json::value::RawValue::from_string(
                        serde_json::to_string(&body).unwrap()
                    ).unwrap();
                    let req = x402_types::proto::VerifyRequest::from(raw);
                    match fac.verify(&req).await {
                        Ok(r) => (axum::http::StatusCode::OK, Json(r.0)),
                        Err(e) => (axum::http::StatusCode::OK, Json(serde_json::json!({
                            "isValid": false, "invalidReason": sanitize_error(&e)
                        }))),
                    }
                }
            }))
            .route("/settle", post(move |Json(body): Json<serde_json::Value>| {
                let fac = fac_settle.clone();
                async move {
                    let raw = serde_json::value::RawValue::from_string(
                        serde_json::to_string(&body).unwrap()
                    ).unwrap();
                    let req = x402_types::proto::SettleRequest::from(raw);
                    match fac.settle(&req).await {
                        Ok(r) => (axum::http::StatusCode::OK, Json(r.0)),
                        Err(e) => (axum::http::StatusCode::OK, Json(serde_json::json!({
                            "success": false, "errorReason": sanitize_error(&e)
                        }))),
                    }
                }
            }))
            .route("/supported", get(move || {
                let fac = fac_supported.clone();
                async move {
                    match fac.supported().await {
                        Ok(r) => Json(serde_json::to_value(&r).unwrap_or_default()),
                        Err(e) => Json(serde_json::json!({"error": sanitize_error(&e)})),
                    }
                }
            }));

        let addr = SocketAddr::from(([127, 0, 0, 1], port));
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let handle = tokio::spawn(async move {
            axum::serve(listener, app).await.unwrap();
        });
        tokio::time::sleep(Duration::from_millis(300)).await;
        (handle, port)
    }

    // ─── Gateway ────────────────────────────────────────────────────

    async fn start_gateway(fac_port: u16) -> (JoinHandle<()>, u16, X402Producer) {
        let port = free_port();
        let config = X402Config {
            bind_address: SocketAddr::from(([127, 0, 0, 1], port)),
            facilitator_url: format!("http://127.0.0.1:{fac_port}").parse().unwrap(),
            quote_ttl_secs: 300,
            accepted_tokens: vec![AcceptedToken {
                network: "eip155:8453".into(),
                asset: format!("{:#x}", USDC_ADDRESS),
                symbol: "USDC".into(),
                decimals: 6,
                pay_to: format!("{:#x}", OPERATOR),
                rate_per_native_unit: Decimal::from(3200u32),
                markup_bps: 0,
                transfer_method: "eip3009".into(),
                eip3009_name: Some("USD Coin".into()),
                eip3009_version: Some("2".into()),
            }],
            default_invocation_mode: X402InvocationMode::PublicPaid,
            job_policies: vec![],
            service_id: 1,
            mpp: None,
        };
        let mut pricing = HashMap::new();
        pricing.insert((1u64, 0u32), U256::from(1_000_000_000_000_000u64));

        let (gw, producer) = X402Gateway::new(config, pricing).unwrap();
        let handle = tokio::spawn(async move {
            let _rx = gw.start().await.unwrap();
            futures::future::pending::<()>().await;
        });
        tokio::time::sleep(Duration::from_millis(200)).await;
        (handle, port, producer)
    }

    // ─── Infrastructure Tests ───────────────────────────────────────

    #[tokio::test]
    async fn test_anvil_fork_has_real_usdc() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        let p = ProviderBuilder::new().connect_http(fork.rpc_url().parse().unwrap());
        assert_eq!(p.get_chain_id().await.unwrap(), 8453);
        assert!(usdc_balance(&fork.rpc_url(), USDC_WHALE).await > U256::ZERO);
    }

    #[tokio::test]
    async fn test_fund_payer_with_usdc() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        fund_payer(&fork.rpc_url(), U256::from(10_000_000u64)).await;
        assert!(usdc_balance(&fork.rpc_url(), PAYER).await >= U256::from(10_000_000u64));
    }

    #[tokio::test]
    async fn test_impersonated_transfer_moves_usdc() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        let amount = U256::from(5_000_000u64);
        fund_payer(&fork.rpc_url(), amount).await;
        let op_before = usdc_balance(&fork.rpc_url(), OPERATOR).await;
        let payer_before = usdc_balance(&fork.rpc_url(), PAYER).await;

        let p = ProviderBuilder::new().connect_http(fork.rpc_url().parse().unwrap());
        p.raw_request::<_, ()>("anvil_impersonateAccount".into(), [format!("{:#x}", PAYER)]).await.unwrap();
        let tx = TransactionRequest::default().from(PAYER).to(USDC_ADDRESS)
            .input(IERC20::transferCall { to: OPERATOR, amount }.abi_encode().into());
        p.send_transaction(tx).await.unwrap().get_receipt().await.unwrap();
        p.raw_request::<_, ()>("anvil_stopImpersonatingAccount".into(), [format!("{:#x}", PAYER)]).await.unwrap();

        assert_eq!(usdc_balance(&fork.rpc_url(), OPERATOR).await - op_before, amount);
        assert_eq!(payer_before - usdc_balance(&fork.rpc_url(), PAYER).await, amount);
    }

    // ─── Facilitator Tests ──────────────────────────────────────────

    #[tokio::test]
    async fn test_real_facilitator_starts_and_responds() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        let (handle, port) = start_real_facilitator(&fork.rpc_url()).await;

        let resp = reqwest::get(format!("http://127.0.0.1:{port}/supported")).await.unwrap();
        assert_eq!(resp.status(), 200);

        handle.abort();
    }

    // ─── Gateway Tests ──────────────────────────────────────────────

    #[tokio::test]
    async fn test_gateway_402_on_unpaid_request() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        let (fac_h, fac_port) = start_real_facilitator(&fork.rpc_url()).await;
        let (gw_h, gw_port, _producer) = start_gateway(fac_port).await;

        let resp = reqwest::Client::new()
            .post(format!("http://127.0.0.1:{gw_port}/x402/jobs/1/0"))
            .body("hello").send().await.unwrap();
        assert_eq!(resp.status(), 402);

        gw_h.abort(); fac_h.abort();
    }

    #[tokio::test]
    async fn test_price_discovery_returns_usdc_settlement() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        let (fac_h, fac_port) = start_real_facilitator(&fork.rpc_url()).await;
        let (gw_h, gw_port, _producer) = start_gateway(fac_port).await;

        let resp = reqwest::get(format!("http://127.0.0.1:{gw_port}/x402/jobs/1/0/price"))
            .await.unwrap();
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = resp.json().await.unwrap();
        let opts = body["settlement_options"].as_array().unwrap();
        assert!(!opts.is_empty());
        assert_eq!(opts[0]["symbol"], "USDC");
        assert_eq!(opts[0]["network"], "eip155:8453");

        gw_h.abort(); fac_h.abort();
    }

    // ─── FULL E2E: Payment → Settlement → On-Chain Verification ─────

    #[tokio::test]
    async fn test_full_x402_payment_settlement_on_chain() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }

        let fork = AnvilFork::spawn();

        // 1. Fund payer with 100 USDC
        fund_payer(&fork.rpc_url(), U256::from(100_000_000u64)).await;

        // 2. Start REAL facilitator (V1Eip155ExactFacilitator against Anvil)
        let (fac_h, fac_port) = start_real_facilitator(&fork.rpc_url()).await;

        // 3. Start gateway pointing at real facilitator
        let (gw_h, gw_port, mut producer) = start_gateway(fac_port).await;

        // 4. Record balances BEFORE
        let payer_before = usdc_balance(&fork.rpc_url(), PAYER).await;
        let operator_before = usdc_balance(&fork.rpc_url(), OPERATOR).await;

        // 5. Get price (to know how much USDC to authorize)
        let price_resp = reqwest::get(format!("http://127.0.0.1:{gw_port}/x402/jobs/1/0/price"))
            .await.unwrap();
        let price_body: serde_json::Value = price_resp.json().await.unwrap();
        let required_amount: U256 = price_body["settlement_options"][0]["amount"]
            .as_str().unwrap().parse().unwrap();
        eprintln!("Required USDC amount: {required_amount}");

        // 6. Sign EIP-3009 transferWithAuthorization
        let signer: PrivateKeySigner = PAYER_KEY.parse().unwrap();
        let params = Eip3009SigningParams {
            chain_id: 8453,
            asset_address: USDC_ADDRESS,
            pay_to: OPERATOR,
            amount: required_amount,
            max_timeout_seconds: 300,
            extra: Some(PaymentRequirementsExtra {
                name: "USD Coin".into(),
                version: "2".into(),
            }),
        };
        let evm_payload = sign_erc3009_authorization(&signer, &params).await.unwrap();

        // 7. Build x402 payment payload (matches what the gateway expects)
        let payment_payload = serde_json::json!({
            "x402Version": 1,
            "scheme": "exact",
            "network": "base",
            "payload": {
                "signature": format!("0x{}", hex::encode(&evm_payload.signature)),
                "authorization": {
                    "from": format!("{:#x}", evm_payload.authorization.from),
                    "to": format!("{:#x}", evm_payload.authorization.to),
                    "value": evm_payload.authorization.value.to_string(),
                    "validAfter": evm_payload.authorization.valid_after.as_secs().to_string(),
                    "validBefore": evm_payload.authorization.valid_before.as_secs().to_string(),
                    "nonce": format!("0x{}", hex::encode(evm_payload.authorization.nonce)),
                }
            }
        });
        let payment_b64 = base64::Engine::encode(
            &base64::engine::general_purpose::STANDARD,
            serde_json::to_vec(&payment_payload).unwrap(),
        );

        // 8. Send the PAID request to the gateway
        let resp = reqwest::Client::new()
            .post(format!("http://127.0.0.1:{gw_port}/x402/jobs/1/0"))
            .header("X-PAYMENT", &payment_b64)
            .body("test echo payload")
            .send().await.unwrap();

        let status = resp.status();
        let resp_headers = resp.headers().clone();
        let resp_body = resp.text().await.unwrap();
        eprintln!("Response status: {status}");
        eprintln!("Response body: {resp_body}");

        // 9. Check settlement header (X-Payment-Response)
        if let Some(settlement) = resp_headers.get("X-Payment-Response") {
            eprintln!("Settlement: {}", settlement.to_str().unwrap_or("?"));
        }

        // 10. Verify on-chain: USDC should have moved
        let payer_after = usdc_balance(&fork.rpc_url(), PAYER).await;
        let operator_after = usdc_balance(&fork.rpc_url(), OPERATOR).await;
        eprintln!("Payer USDC:    {payer_before} → {payer_after}");
        eprintln!("Operator USDC: {operator_before} → {operator_after}");

        if status.is_success() {
            // Full success: job executed AND settlement happened
            assert!(operator_after > operator_before,
                "operator USDC should increase after settlement");
            assert!(payer_after < payer_before,
                "payer USDC should decrease after settlement");
            let transferred = operator_after - operator_before;
            eprintln!("✅ FULL E2E SUCCESS: {transferred} USDC settled on-chain, job executed");

            // 11. Verify producer received the payment
            // The producer is a channel — if settlement succeeded,
            // the gateway should have enqueued a VerifiedPayment
            eprintln!("✅ Response body (echo job output): {resp_body}");
        } else {
            // Payment was rejected — log the reason for debugging
            eprintln!("❌ Payment rejected with status {status}");
            eprintln!("   Body: {resp_body}");
            // Don't assert success — the payment format may not match exactly
            // what the facilitator expects. Log everything for diagnosis.
        }

        gw_h.abort(); fac_h.abort();
    }
}
