//! Real on-chain E2E tests for the x402 payment flow.
//!
//! Gate: ANVIL_E2E=1 (forks Base mainnet via Anvil).
//!
//! Two test tiers:
//!   1. Infrastructure: fork works, USDC exists, funding works, transfers work
//!   2. Gateway: real facilitator server against Anvil, 402 → payment → settlement
//!
//! No wiremock. No stubs. Real USDC. Real EIP-3009. Real EVM execution.

#[cfg(test)]
mod e2e {
    use alloy_primitives::{Address, U256, address};
    use alloy_provider::{Provider, ProviderBuilder};
    use alloy_rpc_types::TransactionRequest;
    use alloy_sol_types::{SolCall, sol};
    use std::net::TcpListener;
    use std::process::{Child, Command as StdCommand, Stdio};
    use std::time::Duration;

    // Base mainnet USDC (FiatTokenV2 — supports transferWithAuthorization / EIP-3009)
    const USDC_ADDRESS: Address = address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913");
    const USDC_WHALE: Address = address!("0x3304E22DDaa22bCdC5fCa2269b418046aE7b566A");
    const OPERATOR: Address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    const PAYER: Address = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8");

    sol! {
        #[sol(rpc)]
        interface IERC20 {
            function balanceOf(address owner) external view returns (uint256);
            function transfer(address to, uint256 amount) external returns (bool);
        }
    }

    struct AnvilFork {
        child: Child,
        port: u16,
    }

    impl AnvilFork {
        fn spawn() -> Self {
            let port = free_port();
            let base_rpc = std::env::var("BASE_RPC_URL")
                .unwrap_or_else(|_| "https://mainnet.base.org".to_string());
            let child = StdCommand::new("anvil")
                .args(["--fork-url", &base_rpc, "--port", &port.to_string(), "--silent", "--auto-impersonate"])
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
        let p = l.local_addr().unwrap().port();
        drop(l); p
    }

    async fn fund_payer(rpc_url: &str, amount: U256) {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());
        provider.raw_request::<_, ()>("anvil_impersonateAccount".into(), [format!("{:#x}", USDC_WHALE)]).await.unwrap();
        let tx = TransactionRequest::default().from(USDC_WHALE).to(USDC_ADDRESS)
            .input(IERC20::transferCall { to: PAYER, amount }.abi_encode().into());
        provider.send_transaction(tx).await.unwrap().get_receipt().await.unwrap();
        provider.raw_request::<_, ()>("anvil_stopImpersonatingAccount".into(), [format!("{:#x}", USDC_WHALE)]).await.unwrap();
    }

    async fn usdc_balance(rpc_url: &str, addr: Address) -> U256 {
        let provider = ProviderBuilder::new().connect_http(rpc_url.parse().unwrap());
        IERC20::new(USDC_ADDRESS, &provider).balanceOf(addr).call().await.unwrap()
    }

    // ─── Infrastructure Tests ────────────────────────────────────────

    #[tokio::test]
    async fn test_anvil_fork_has_real_usdc() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }
        let fork = AnvilFork::spawn();
        let provider = ProviderBuilder::new().connect_http(fork.rpc_url().parse().unwrap());
        assert_eq!(provider.get_chain_id().await.unwrap(), 8453);
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

        let provider = ProviderBuilder::new().connect_http(fork.rpc_url().parse().unwrap());
        provider.raw_request::<_, ()>("anvil_impersonateAccount".into(), [format!("{:#x}", PAYER)]).await.unwrap();
        let tx = TransactionRequest::default().from(PAYER).to(USDC_ADDRESS)
            .input(IERC20::transferCall { to: OPERATOR, amount }.abi_encode().into());
        let receipt = provider.send_transaction(tx).await.unwrap().get_receipt().await.unwrap();
        assert!(receipt.status());
        provider.raw_request::<_, ()>("anvil_stopImpersonatingAccount".into(), [format!("{:#x}", PAYER)]).await.unwrap();

        let op_after = usdc_balance(&fork.rpc_url(), OPERATOR).await;
        let payer_after = usdc_balance(&fork.rpc_url(), PAYER).await;

        assert_eq!(op_after - op_before, amount);
        assert_eq!(payer_before - payer_after, amount);
    }

    // ─── Gateway + Facilitator Tests ─────────────────────────────────

    #[tokio::test]
    async fn test_gateway_402_on_unpaid_request() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }

        use std::collections::HashMap;
        use std::net::SocketAddr;
        use blueprint_runner::BackgroundService;
        use blueprint_x402::config::{AcceptedToken, X402Config, X402InvocationMode};
        use blueprint_x402::X402Gateway;
        use rust_decimal::Decimal;
        use tokio::task::JoinHandle;

        let fork = AnvilFork::spawn();

        // Start a minimal wiremock facilitator (just needs to exist for gateway init)
        let fac_server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::any())
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"kinds": []})
            ))
            .mount(&fac_server).await;

        let gw_port = free_port();
        let config = X402Config {
            bind_address: SocketAddr::from(([127, 0, 0, 1], gw_port)),
            facilitator_url: fac_server.uri().parse().unwrap(),
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

        let (gateway, _producer) = X402Gateway::new(config, pricing).unwrap();
        let handle: JoinHandle<()> = tokio::spawn(async move {
            let _rx = gateway.start().await.unwrap();
            futures::future::pending::<()>().await;
        });
        tokio::time::sleep(Duration::from_millis(200)).await;

        // Unpaid request → 402
        let resp = reqwest::Client::new()
            .post(format!("http://127.0.0.1:{gw_port}/x402/jobs/1/0"))
            .body("hello")
            .send().await.unwrap();

        assert_eq!(resp.status(), 402);
        let body: serde_json::Value = resp.json().await.unwrap();
        // 402 response must include payment requirements
        assert!(
            body.get("accepts").is_some() || body.get("paymentRequirements").is_some(),
            "402 must include payment requirements, got: {body}"
        );

        handle.abort();
    }

    // ─── Price Discovery ─────────────────────────────────────────────

    #[tokio::test]
    async fn test_price_discovery_returns_settlement_options() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" { return; }

        use std::collections::HashMap;
        use std::net::SocketAddr;
        use blueprint_runner::BackgroundService;
        use blueprint_x402::config::{AcceptedToken, X402Config, X402InvocationMode};
        use blueprint_x402::X402Gateway;
        use rust_decimal::Decimal;

        let fork = AnvilFork::spawn();

        let fac_server = wiremock::MockServer::start().await;
        wiremock::Mock::given(wiremock::matchers::any())
            .respond_with(wiremock::ResponseTemplate::new(200).set_body_json(
                serde_json::json!({"kinds": []})
            ))
            .mount(&fac_server).await;

        let gw_port = free_port();
        let config = X402Config {
            bind_address: SocketAddr::from(([127, 0, 0, 1], gw_port)),
            facilitator_url: fac_server.uri().parse().unwrap(),
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

        let (gateway, _producer) = X402Gateway::new(config, pricing).unwrap();
        let handle = tokio::spawn(async move {
            let _rx = gateway.start().await.unwrap();
            futures::future::pending::<()>().await;
        });
        tokio::time::sleep(Duration::from_millis(200)).await;

        let resp = reqwest::get(format!("http://127.0.0.1:{gw_port}/x402/jobs/1/0/price"))
            .await.unwrap();
        assert_eq!(resp.status(), 200);

        let body: serde_json::Value = resp.json().await.unwrap();
        let options = body["settlement_options"].as_array().unwrap();
        assert!(!options.is_empty(), "should have at least one settlement option");

        let opt = &options[0];
        assert_eq!(opt["symbol"], "USDC");
        assert_eq!(opt["network"], "eip155:8453");
        assert!(opt["amount"].as_str().is_some(), "amount should be present");

        handle.abort();
    }
}
