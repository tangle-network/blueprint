//! Real on-chain E2E tests for the x402 payment flow.
//!
//! Gate: ANVIL_E2E=1 (these fork Base mainnet and take ~30s each).
//!
//! Production-faithful flow:
//!   1. Anvil forks Base mainnet → real USDC contract with transferWithAuthorization
//!   2. A real facilitator HTTP server wraps V1Eip155ExactFacilitator against Anvil RPC
//!   3. The x402 gateway starts, pointing at the real facilitator
//!   4. A test client constructs a valid EIP-3009 payment (signed by a test wallet)
//!   5. The client sends an HTTP request with X-PAYMENT header to the gateway
//!   6. Assertions: job executed, USDC transferred on-chain
//!
//! No wiremock. No stubs. Real USDC. Real EIP-3009. Real EVM execution.
//!
//! Requires: ANVIL_E2E=1, BASE_RPC_URL (or defaults to public Base RPC)

#[cfg(test)]
mod e2e {
    use alloy_primitives::{Address, U256, address};
    use alloy_provider::{Provider, ProviderBuilder};
    use alloy_sol_types::sol;
    use std::collections::HashMap;
    use std::net::{SocketAddr, TcpListener};
    use std::process::{Child, Command as StdCommand, Stdio};
    use std::time::Duration;
    use tokio::task::JoinHandle;

    use blueprint_x402::config::{AcceptedToken, X402Config, X402InvocationMode};
    use blueprint_x402::producer::X402Producer;
    use blueprint_x402::X402Gateway;
    use blueprint_runner::BackgroundService;
    use rust_decimal::Decimal;
    use x402_blueprint::{load_job_pricing, router};

    // Base mainnet USDC (FiatTokenV2 — supports transferWithAuthorization)
    const USDC_ADDRESS: Address = address!("0x833589fCD6eDb6E08f4c7C32D4f71b54bdA02913");
    // A known USDC holder on Base (Circle's address — has millions of USDC)
    const USDC_WHALE: Address = address!("0x3304E22DDaa22bCdC5fCa2269b418046aE7b566A");
    // Anvil default account #0 (operator — receives payment)
    const OPERATOR: Address = address!("0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266");
    // Anvil default account #1 (payer — sends payment)
    const PAYER: Address = address!("0x70997970C51812dc3A010C7d01b50e0d17dc79C8");
    const PAYER_KEY: &str = "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";

    // ERC-20 balanceOf ABI
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
                .args([
                    "--fork-url", &base_rpc,
                    "--port", &port.to_string(),
                    "--silent",
                    // Auto-impersonate so we can send txs from any address
                    "--auto-impersonate",
                ])
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .expect("anvil must be installed (foundry)");

            // Wait for fork to be ready (downloading state takes a few seconds)
            std::thread::sleep(Duration::from_secs(5));
            Self { child, port }
        }

        fn rpc_url(&self) -> String {
            format!("http://127.0.0.1:{}", self.port)
        }
    }

    impl Drop for AnvilFork {
        fn drop(&mut self) {
            let _ = self.child.kill();
            let _ = self.child.wait();
        }
    }

    fn free_port() -> u16 {
        let listener = TcpListener::bind("127.0.0.1:0").expect("bind ephemeral port");
        let port = listener.local_addr().unwrap().port();
        drop(listener);
        port
    }

    /// Fund the payer with USDC by impersonating a whale and transferring.
    async fn fund_payer(rpc_url: &str, amount: U256) {
        let provider = ProviderBuilder::new()
            .connect_http(rpc_url.parse().unwrap());

        // Impersonate the whale
        provider.raw_request::<_, ()>(
            "anvil_impersonateAccount".into(),
            [format!("{:#x}", USDC_WHALE)],
        ).await.expect("impersonate whale");

        let usdc = IERC20::new(USDC_ADDRESS, &provider);

        // Check whale balance
        let whale_balance = usdc.balanceOf(USDC_WHALE).call().await.expect("whale balance");
        assert!(
            whale_balance >= amount,
            "whale has insufficient USDC: {whale_balance} < {amount}"
        );

        // Transfer USDC from whale to payer
        // We need to send the tx from the whale's address
        let tx = alloy_rpc_types::TransactionRequest::default()
            .from(USDC_WHALE)
            .to(USDC_ADDRESS)
            .input(
                IERC20::transferCall { to: PAYER, amount }.abi_encode().into()
            );

        let pending = provider.send_transaction(tx).await.expect("send transfer");
        let receipt = pending.get_receipt().await.expect("transfer receipt");
        assert!(receipt.status(), "USDC transfer to payer reverted");

        // Stop impersonating
        provider.raw_request::<_, ()>(
            "anvil_stopImpersonatingAccount".into(),
            [format!("{:#x}", USDC_WHALE)],
        ).await.expect("stop impersonating");

        // Verify payer balance
        let payer_balance = usdc.balanceOf(PAYER).call().await.expect("payer balance");
        assert!(payer_balance >= amount, "payer funding failed: {payer_balance} < {amount}");
    }

    // ──────────────────────────────────────────────────────────────
    // Tests
    // ──────────────────────────────────────────────────────────────

    #[tokio::test]
    async fn test_anvil_fork_has_real_usdc() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" {
            eprintln!("skipping x402 anvil e2e (set ANVIL_E2E=1)");
            return;
        }

        let fork = AnvilFork::spawn();
        let provider = ProviderBuilder::new()
            .connect_http(fork.rpc_url().parse().unwrap());

        // Verify we forked Base and USDC exists
        let chain_id = provider.get_chain_id().await.expect("chain id");
        assert_eq!(chain_id, 8453, "should be Base mainnet fork");

        let usdc = IERC20::new(USDC_ADDRESS, &provider);
        let whale_balance = usdc.balanceOf(USDC_WHALE).call().await.expect("whale balance");
        assert!(whale_balance > U256::ZERO, "USDC whale should have balance on Base fork");

        eprintln!("Base fork OK — whale USDC balance: {whale_balance}");
    }

    #[tokio::test]
    async fn test_fund_payer_with_usdc() {
        if std::env::var("ANVIL_E2E").unwrap_or_default() != "1" {
            eprintln!("skipping x402 anvil e2e (set ANVIL_E2E=1)");
            return;
        }

        let fork = AnvilFork::spawn();
        let amount = U256::from(10_000_000u64); // 10 USDC (6 decimals)

        fund_payer(&fork.rpc_url(), amount).await;

        // Verify
        let provider = ProviderBuilder::new()
            .connect_http(fork.rpc_url().parse().unwrap());
        let usdc = IERC20::new(USDC_ADDRESS, &provider);
        let balance = usdc.balanceOf(PAYER).call().await.expect("payer balance");
        assert!(balance >= amount, "payer should have ≥10 USDC");

        eprintln!("Payer funded with {balance} USDC (smallest units)");
    }

    // TODO: Full E2E test with real facilitator + gateway + payment
    //
    // Remaining implementation:
    //
    // 1. Start a local facilitator HTTP server:
    //    - Wrap V1Eip155ExactFacilitator<AnvilProvider> in axum routes
    //    - POST /verify → facilitator.verify()
    //    - POST /settle → facilitator.settle() (calls transferWithAuthorization on-chain)
    //    - GET /supported → facilitator.supported()
    //
    // 2. Start the x402 gateway pointing at the local facilitator
    //
    // 3. Construct an EIP-3009 payment:
    //    - Build EIP-712 domain for USDC ("USD Coin", version "2", chain 8453)
    //    - Sign transferWithAuthorization(from=PAYER, to=OPERATOR, value, validAfter=0, validBefore=MAX, nonce=random)
    //    - Encode as x402 PaymentPayload
    //
    // 4. Send HTTP request with X-PAYMENT header to the gateway
    //
    // 5. Assert:
    //    - Response 200 (job executed)
    //    - Producer received VerifiedPayment
    //    - USDC on-chain: operator balance increased, payer balance decreased
    //
    // This requires adding x402-chain-eip155 with "client" feature to the
    // example's dev-dependencies, which is a Cargo.toml change.
}
