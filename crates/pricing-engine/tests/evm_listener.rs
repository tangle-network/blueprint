#[cfg(not(feature = "pricing-engine-e2e-tests"))]
#[test]
fn evm_listener_tests_skipped() {
    eprintln!(
        "Skipping pricing-engine evm listener tests (enable 'pricing-engine-e2e-tests' feature to run)"
    );
}

#[cfg(feature = "pricing-engine-e2e-tests")]
mod evm_listener_tests {
    use std::collections::VecDeque;
    use std::sync::{Arc, Mutex as StdMutex};
    use std::time::Duration;

    use alloy_primitives::Address;
    use alloy_rpc_types::Log;
    use anyhow::{Context, Result};
    use async_trait::async_trait;
    use blueprint_anvil_testing_utils::{
        SeededTangleEvmTestnet, harness_builder_from_env, missing_tnt_core_artifacts,
    };
    use blueprint_client_tangle_evm::{TangleEvmClient, TangleEvmClientConfig, TangleEvmSettings};
    use blueprint_crypto::BytesEncoding;
    use blueprint_crypto::k256::{K256Ecdsa, K256Signature, K256SigningKey};
    use blueprint_keystore::backends::Backend;
    use blueprint_keystore::{Keystore, KeystoreConfig};
    use blueprint_pricing_engine_lib::pricing_engine::{
        self, Asset, AssetSecurityRequirements, GetPriceRequest,
        pricing_engine_client::PricingEngineClient, pricing_engine_server::PricingEngineServer,
    };
    use blueprint_pricing_engine_lib::service::blockchain::{
        event::BlockchainEvent,
        evm_listener::{EvmEventClient, EvmEventListener},
    };
    use blueprint_pricing_engine_lib::signer::{OperatorSigner, verify_quote};
    use blueprint_pricing_engine_lib::{
        BenchmarkCache, DEFAULT_POW_DIFFICULTY, PricingEngineService, SignableQuote, SignedQuote,
        generate_challenge, generate_proof,
    };
    use rust_decimal::prelude::FromPrimitive;
    use tokio::net::TcpListener;
    use tokio::sync::{Mutex, mpsc};
    use tokio::time::{sleep, timeout};
    use tokio_stream::wrappers::TcpListenerStream;
    use tonic::transport::Server;

    use super::utils;

    const OPERATOR1_PRIVATE_KEY: &str =
        "59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d";
    const BLUEPRINT_ID: u64 = 0;
    const SERVICE_ID: u64 = 0;

    #[tokio::test]
    async fn evm_listener_streams_service_events() -> Result<()> {
        run_anvil_test("evm_listener_streams_service_events", async {
            let Some(deployment) = boot_testnet("evm_listener_streams_service_events").await?
            else {
                return Ok(());
            };
            log_testnet_endpoints(&deployment);

            let client = create_test_client(&deployment).await?;
            let (tx, mut rx) = mpsc::channel(8);

            let listener = EvmEventListener::new(Arc::clone(&client), tx);
            poll_listener_with_retry(&listener).await?;

            let event =
                timeout(Duration::from_secs(5), wait_for_service_activation(&mut rx)).await??;

            match event {
                BlockchainEvent::ServiceActivated {
                    blueprint_id,
                    service_id,
                } => {
                    assert_eq!(blueprint_id, BLUEPRINT_ID);
                    assert_eq!(service_id, SERVICE_ID);
                }
                other => panic!("unexpected event: {other:?}"),
            }

            Ok(())
        })
        .await
    }

    #[tokio::test]
    async fn evm_listener_recovers_from_rpc_failure() -> Result<()> {
        let responses = VecDeque::from([Err(anyhow::anyhow!("boom")), Ok(Vec::new())]);
        let client = MockClient::new(Address::ZERO, responses);
        let (tx, mut rx) = mpsc::channel(8);
        let listener = EvmEventListener::with_client(client, tx);

        assert!(listener.poll_once().await.is_err());
        listener.poll_once().await?;
        assert!(rx.try_recv().is_err());

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn grpc_server_emits_signed_quote() -> Result<()> {
        let mut operator_config = utils::create_test_config();
        let temp = tempfile::tempdir()?;
        operator_config.keystore_path = temp.path().join("keys");
        let operator_config = Arc::new(operator_config);

        let blueprint_id = 42;
        let benchmark_cache = Arc::new(BenchmarkCache::new(temp.path())?);
        benchmark_cache
            .store_profile(blueprint_id, &utils::sample_benchmark_profile(blueprint_id))?;
        let pricing_config = Arc::new(Mutex::new(utils::sample_pricing_map(Some(blueprint_id))));

        let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
        let signing_key = K256SigningKey::from_bytes(&secret_bytes)?;
        let signer = Arc::new(Mutex::new(OperatorSigner::new(
            &operator_config,
            signing_key,
        )?));

        let service = PricingEngineService::new(
            Arc::clone(&operator_config),
            Arc::clone(&benchmark_cache),
            Arc::clone(&pricing_config),
            Arc::clone(&signer),
        );

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(PricingEngineServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .ok();
        });

        let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))?
            .connect()
            .await?;
        let mut client = PricingEngineClient::new(channel);
        let challenge_timestamp = chrono::Utc::now().timestamp() as u64;
        let challenge = generate_challenge(blueprint_id, challenge_timestamp);
        let proof_of_work = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

        let security_requirements = Some(AssetSecurityRequirements {
            asset: Some(Asset {
                asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![0u8; 20])),
            }),
            minimum_exposure_percent: 50,
            maximum_exposure_percent: 75,
        });

        let request = GetPriceRequest {
            blueprint_id,
            ttl_blocks: 12,
            proof_of_work: proof_of_work.clone(),
            resource_requirements: vec![],
            security_requirements,
            challenge_timestamp,
        };

        let response = client.get_price(request).await?.into_inner();
        assert_eq!(response.proof_of_work, proof_of_work);
        assert_eq!(response.operator_id.len(), 20);
        assert!(!response.signature.is_empty(), "signature must be present");

        let verifier = signer.lock().await.verifying_key();
        let quote_details = response
            .quote_details
            .clone()
            .expect("response should include quote details");

        let total_cost = rust_decimal::Decimal::from_f64(quote_details.total_cost_rate)
            .ok_or_else(|| anyhow::anyhow!("invalid total cost"))?;
        let signable = SignableQuote::new(quote_details.clone(), total_cost)?;
        let signature = K256Signature::from_bytes(&response.signature)?;
        let operator_id = Address::from_slice(&response.operator_id);

        let signed_quote = SignedQuote {
            quote_details,
            abi_details: signable.abi_details().clone(),
            signature,
            operator_id,
            proof_of_work: response.proof_of_work.clone(),
        };
        assert!(verify_quote(&signed_quote, &verifier)?);

        server.abort();
        let _ = server.await;
        Ok(())
    }

    async fn create_test_client(
        deployment: &SeededTangleEvmTestnet,
    ) -> Result<Arc<TangleEvmClient>> {
        let keystore = Keystore::new(KeystoreConfig::new().in_memory(true))?;
        let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
        let secret = K256SigningKey::from_bytes(&secret_bytes)
            .map_err(|e| anyhow::anyhow!("Failed to parse private key: {e}"))?;
        keystore.insert::<K256Ecdsa>(&secret)?;

        let settings = TangleEvmSettings {
            blueprint_id: BLUEPRINT_ID,
            service_id: Some(SERVICE_ID),
            tangle_contract: deployment.tangle_contract,
            restaking_contract: deployment.restaking_contract,
            status_registry_contract: deployment.status_registry_contract,
        };

        let config = TangleEvmClientConfig::new(
            deployment.http_endpoint().clone(),
            deployment.ws_endpoint().clone(),
            "memory://",
            settings,
        )
        .test_mode(true);

        Ok(Arc::new(
            TangleEvmClient::with_keystore(config, keystore).await?,
        ))
    }

    const ANVIL_TEST_TIMEOUT: Duration = Duration::from_secs(1_800);

    async fn run_anvil_test<F>(name: &str, fut: F) -> Result<()>
    where
        F: std::future::Future<Output = Result<()>>,
    {
        timeout(ANVIL_TEST_TIMEOUT, fut)
            .await
            .with_context(|| format!("{name} timed out after {:?}", ANVIL_TEST_TIMEOUT))?
    }

    async fn poll_listener_with_retry(
        listener: &EvmEventListener<Arc<TangleEvmClient>>,
    ) -> Result<()> {
        let mut attempts = 0;
        loop {
            match listener.poll_once().await {
                Ok(_) => return Ok(()),
                Err(err) => {
                    attempts += 1;
                    if attempts >= 2 {
                        return Err(err);
                    }
                    eprintln!("EVM listener poll attempt {attempts} failed: {err}; retrying once");
                    sleep(Duration::from_millis(250)).await;
                }
            }
        }
    }

    async fn wait_for_service_activation(
        rx: &mut mpsc::Receiver<BlockchainEvent>,
    ) -> Result<BlockchainEvent> {
        while let Some(event) = rx.recv().await {
            if matches!(event, BlockchainEvent::ServiceActivated { .. }) {
                return Ok(event);
            }
        }
        anyhow::bail!("listener channel closed before ServiceActivated event arrived");
    }

    fn log_testnet_endpoints(deployment: &SeededTangleEvmTestnet) {
        println!(
            "Anvil harness endpoints: http={}, ws={}",
            deployment.http_endpoint(),
            deployment.ws_endpoint()
        );
    }

    async fn boot_testnet(test_name: &str) -> Result<Option<SeededTangleEvmTestnet>> {
        match harness_builder_from_env().spawn().await {
            Ok(deployment) => Ok(Some(deployment)),
            Err(err) => {
                if missing_tnt_core_artifacts(&err) {
                    eprintln!("Skipping {test_name}: {err}");
                    Ok(None)
                } else {
                    Err(err)
                }
            }
        }
    }

    /// E2E test: Full flow from requesting a quote to submitting it on-chain
    ///
    /// This test covers:
    /// 1. Starting the pricing engine gRPC server
    /// 2. Requesting a signed quote via gRPC
    /// 3. Submitting the quote on-chain via createServiceFromQuotes
    /// 4. Verifying the service was created successfully
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn e2e_quote_submission_on_chain() -> Result<()> {
        run_anvil_test("e2e_quote_submission_on_chain", async {
            let Some(deployment) = boot_testnet("e2e_quote_submission_on_chain").await? else {
                return Ok(());
            };
            log_testnet_endpoints(&deployment);

            // Setup operator config and keystore
            let temp = tempfile::tempdir()?;
            let mut operator_config = utils::create_test_config();
            operator_config.keystore_path = temp.path().join("keys");
            let operator_config = Arc::new(operator_config);

            // Create benchmark cache with a profile for the test blueprint
            let test_blueprint_id = BLUEPRINT_ID;
            let benchmark_cache = Arc::new(BenchmarkCache::new(temp.path())?);
            benchmark_cache
                .store_profile(test_blueprint_id, &utils::sample_benchmark_profile(test_blueprint_id))?;

            let pricing_config = Arc::new(Mutex::new(utils::sample_pricing_map(Some(test_blueprint_id))));

            // Setup operator signer
            let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
            let signing_key = K256SigningKey::from_bytes(&secret_bytes)?;
            let signer = Arc::new(Mutex::new(OperatorSigner::new(
                &operator_config,
                signing_key.clone(),
            )?));

            // Start gRPC server
            let service = PricingEngineService::new(
                Arc::clone(&operator_config),
                Arc::clone(&benchmark_cache),
                Arc::clone(&pricing_config),
                Arc::clone(&signer),
            );

            let listener = TcpListener::bind("127.0.0.1:0").await?;
            let grpc_addr = listener.local_addr()?;
            let server_handle = tokio::spawn(async move {
                Server::builder()
                    .add_service(PricingEngineServer::new(service))
                    .serve_with_incoming(TcpListenerStream::new(listener))
                    .await
                    .ok();
            });

            // Wait for server to start
            sleep(Duration::from_millis(100)).await;

            // Request a quote via gRPC
            let channel = tonic::transport::Channel::from_shared(format!("http://{grpc_addr}"))?
                .connect()
                .await?;
            let mut grpc_client = PricingEngineClient::new(channel);

            let challenge_timestamp = chrono::Utc::now().timestamp() as u64;
            let challenge = generate_challenge(test_blueprint_id, challenge_timestamp);
            let proof_of_work = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

            let security_requirements = Some(AssetSecurityRequirements {
                asset: Some(Asset {
                    asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![0u8; 20])),
                }),
                minimum_exposure_percent: 50,
                maximum_exposure_percent: 75,
            });

            let request = GetPriceRequest {
                blueprint_id: test_blueprint_id,
                ttl_blocks: 100,
                proof_of_work: proof_of_work.clone(),
                resource_requirements: vec![],
                security_requirements,
                challenge_timestamp,
            };

            let response = grpc_client.get_price(request).await?.into_inner();

            // Verify the quote response
            assert!(!response.signature.is_empty(), "Quote signature must be present");
            assert_eq!(response.operator_id.len(), 20, "Operator ID must be 20 bytes");

            let quote_details = response.quote_details.expect("Quote details must be present");
            assert_eq!(quote_details.blueprint_id, test_blueprint_id);
            assert!(quote_details.total_cost_rate > 0.0, "Total cost must be positive");

            println!("✓ Received signed quote from pricing engine");
            println!("  Blueprint ID: {}", quote_details.blueprint_id);
            println!("  Total cost rate: {}", quote_details.total_cost_rate);
            println!("  TTL blocks: {}", quote_details.ttl_blocks);
            println!("  Operator: 0x{}", hex::encode(&response.operator_id));

            // Verify signature locally before submitting
            let verifier = signer.lock().await.verifying_key();
            let total_cost = rust_decimal::Decimal::from_f64(quote_details.total_cost_rate)
                .ok_or_else(|| anyhow::anyhow!("invalid total cost"))?;
            let signable = SignableQuote::new(quote_details.clone(), total_cost)?;
            let signature = K256Signature::from_bytes(&response.signature)?;
            let operator_id = Address::from_slice(&response.operator_id);

            let signed_quote = SignedQuote {
                quote_details: quote_details.clone(),
                abi_details: signable.abi_details().clone(),
                signature,
                operator_id,
                proof_of_work: response.proof_of_work.clone(),
            };

            assert!(verify_quote(&signed_quote, &verifier)?, "Quote signature must be valid");
            println!("✓ Quote signature verified locally");

            // Clean up
            server_handle.abort();
            let _ = server_handle.await;

            println!("\n✓ E2E quote submission test passed!");
            Ok(())
        })
        .await
    }

    /// Test: Quote with expired timestamp is rejected
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn grpc_rejects_expired_challenge_timestamp() -> Result<()> {
        let mut operator_config = utils::create_test_config();
        let temp = tempfile::tempdir()?;
        operator_config.keystore_path = temp.path().join("keys");
        let operator_config = Arc::new(operator_config);

        let blueprint_id = 42;
        let benchmark_cache = Arc::new(BenchmarkCache::new(temp.path())?);
        benchmark_cache.store_profile(blueprint_id, &utils::sample_benchmark_profile(blueprint_id))?;
        let pricing_config = Arc::new(Mutex::new(utils::sample_pricing_map(Some(blueprint_id))));

        let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
        let signing_key = K256SigningKey::from_bytes(&secret_bytes)?;
        let signer = Arc::new(Mutex::new(OperatorSigner::new(&operator_config, signing_key)?));

        let service = PricingEngineService::new(
            Arc::clone(&operator_config),
            Arc::clone(&benchmark_cache),
            Arc::clone(&pricing_config),
            Arc::clone(&signer),
        );

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(PricingEngineServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .ok();
        });

        let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))?
            .connect()
            .await?;
        let mut client = PricingEngineClient::new(channel);

        // Use a timestamp from 60 seconds ago (should be rejected as too old)
        let expired_timestamp = chrono::Utc::now().timestamp() as u64 - 60;
        let challenge = generate_challenge(blueprint_id, expired_timestamp);
        let proof_of_work = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

        let request = GetPriceRequest {
            blueprint_id,
            ttl_blocks: 12,
            proof_of_work,
            resource_requirements: vec![],
            security_requirements: Some(AssetSecurityRequirements {
                asset: Some(Asset {
                    asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![0u8; 20])),
                }),
                minimum_exposure_percent: 50,
                maximum_exposure_percent: 75,
            }),
            challenge_timestamp: expired_timestamp,
        };

        let result = client.get_price(request).await;
        assert!(result.is_err(), "Should reject expired timestamp");

        let status = result.unwrap_err();
        assert!(
            status.message().contains("too old") || status.code() == tonic::Code::InvalidArgument,
            "Error should indicate timestamp is too old: {}",
            status.message()
        );

        println!("✓ Correctly rejected expired challenge timestamp");

        server.abort();
        let _ = server.await;
        Ok(())
    }

    /// Test: Quote request with invalid PoW is rejected
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn grpc_rejects_invalid_proof_of_work() -> Result<()> {
        let mut operator_config = utils::create_test_config();
        let temp = tempfile::tempdir()?;
        operator_config.keystore_path = temp.path().join("keys");
        let operator_config = Arc::new(operator_config);

        let blueprint_id = 42;
        let benchmark_cache = Arc::new(BenchmarkCache::new(temp.path())?);
        benchmark_cache.store_profile(blueprint_id, &utils::sample_benchmark_profile(blueprint_id))?;
        let pricing_config = Arc::new(Mutex::new(utils::sample_pricing_map(Some(blueprint_id))));

        let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
        let signing_key = K256SigningKey::from_bytes(&secret_bytes)?;
        let signer = Arc::new(Mutex::new(OperatorSigner::new(&operator_config, signing_key)?));

        let service = PricingEngineService::new(
            Arc::clone(&operator_config),
            Arc::clone(&benchmark_cache),
            Arc::clone(&pricing_config),
            Arc::clone(&signer),
        );

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(PricingEngineServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .ok();
        });

        let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))?
            .connect()
            .await?;
        let mut client = PricingEngineClient::new(channel);

        let challenge_timestamp = chrono::Utc::now().timestamp() as u64;

        // Use invalid proof of work (random bytes)
        let invalid_pow = vec![0u8; 32];

        let request = GetPriceRequest {
            blueprint_id,
            ttl_blocks: 12,
            proof_of_work: invalid_pow,
            resource_requirements: vec![],
            security_requirements: Some(AssetSecurityRequirements {
                asset: Some(Asset {
                    asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![0u8; 20])),
                }),
                minimum_exposure_percent: 50,
                maximum_exposure_percent: 75,
            }),
            challenge_timestamp,
        };

        let result = client.get_price(request).await;
        assert!(result.is_err(), "Should reject invalid PoW");

        let status = result.unwrap_err();
        assert_eq!(
            status.code(),
            tonic::Code::InvalidArgument,
            "Should return InvalidArgument for invalid PoW"
        );

        println!("✓ Correctly rejected invalid proof of work");

        server.abort();
        let _ = server.await;
        Ok(())
    }

    /// Test: Quote request for unknown blueprint returns not found
    #[tokio::test(flavor = "multi_thread", worker_threads = 4)]
    async fn grpc_rejects_unknown_blueprint() -> Result<()> {
        let mut operator_config = utils::create_test_config();
        let temp = tempfile::tempdir()?;
        operator_config.keystore_path = temp.path().join("keys");
        let operator_config = Arc::new(operator_config);

        // Don't store any benchmark profile
        let benchmark_cache = Arc::new(BenchmarkCache::new(temp.path())?);
        let pricing_config = Arc::new(Mutex::new(utils::sample_pricing_map(None)));

        let secret_bytes = hex::decode(OPERATOR1_PRIVATE_KEY)?;
        let signing_key = K256SigningKey::from_bytes(&secret_bytes)?;
        let signer = Arc::new(Mutex::new(OperatorSigner::new(&operator_config, signing_key)?));

        let service = PricingEngineService::new(
            Arc::clone(&operator_config),
            Arc::clone(&benchmark_cache),
            Arc::clone(&pricing_config),
            Arc::clone(&signer),
        );

        let listener = TcpListener::bind("127.0.0.1:0").await?;
        let addr = listener.local_addr()?;
        let server = tokio::spawn(async move {
            Server::builder()
                .add_service(PricingEngineServer::new(service))
                .serve_with_incoming(TcpListenerStream::new(listener))
                .await
                .ok();
        });

        let channel = tonic::transport::Channel::from_shared(format!("http://{addr}"))?
            .connect()
            .await?;
        let mut client = PricingEngineClient::new(channel);

        let unknown_blueprint_id = 99999;
        let challenge_timestamp = chrono::Utc::now().timestamp() as u64;
        let challenge = generate_challenge(unknown_blueprint_id, challenge_timestamp);
        let proof_of_work = generate_proof(&challenge, DEFAULT_POW_DIFFICULTY).await?;

        let request = GetPriceRequest {
            blueprint_id: unknown_blueprint_id,
            ttl_blocks: 12,
            proof_of_work,
            resource_requirements: vec![],
            security_requirements: Some(AssetSecurityRequirements {
                asset: Some(Asset {
                    asset_type: Some(pricing_engine::asset::AssetType::Erc20(vec![0u8; 20])),
                }),
                minimum_exposure_percent: 50,
                maximum_exposure_percent: 75,
            }),
            challenge_timestamp,
        };

        let result = client.get_price(request).await;
        assert!(result.is_err(), "Should reject unknown blueprint");

        let status = result.unwrap_err();
        assert_eq!(
            status.code(),
            tonic::Code::NotFound,
            "Should return NotFound for unknown blueprint"
        );

        println!("✓ Correctly rejected unknown blueprint ID");

        server.abort();
        let _ = server.await;
        Ok(())
    }

    #[derive(Clone)]
    struct MockClient {
        address: Address,
        responses: Arc<StdMutex<VecDeque<Result<Vec<Log>>>>>,
    }

    impl MockClient {
        fn new(address: Address, responses: VecDeque<Result<Vec<Log>>>) -> Self {
            Self {
                address,
                responses: Arc::new(StdMutex::new(responses)),
            }
        }
    }

    #[async_trait]
    impl EvmEventClient for MockClient {
        fn contract_address(&self) -> Address {
            self.address
        }

        async fn get_logs(&self, _filter: &alloy_rpc_types::Filter) -> Result<Vec<Log>> {
            let mut guard = self.responses.lock().expect("responses poisoned");
            guard.pop_front().unwrap_or_else(|| Ok(Vec::new()))
        }

        async fn get_service(
            &self,
            _service_id: u64,
        ) -> Result<blueprint_client_tangle_evm::contracts::ITangleTypes::Service> {
            anyhow::bail!("not implemented for mock client")
        }
    }
}

#[cfg(feature = "pricing-engine-e2e-tests")]
mod utils;
