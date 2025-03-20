//! Incredible Squaring TaskManager Monitor
//!
//! Monitors TaskManager events for task creation and completion.

pub mod config;
pub mod contexts;
pub mod contracts;
pub mod jobs;

#[cfg(test)]
mod tests {
    use crate::contracts::task_manager::SquaringTask;
    use crate::jobs::x_square::IncredibleSquaringClientContext;
    use crate::jobs::x_square_create_contract_router;

    use super::*;
    use blueprint_sdk::alloy::network::EthereumWallet;
    use blueprint_sdk::alloy::primitives::{address, Address, U256};
    use blueprint_sdk::alloy::providers::Provider;
    use blueprint_sdk::alloy::signers::local::PrivateKeySigner;
    use blueprint_sdk::eigensdk;
    use blueprint_sdk::evm::consumer::EVMConsumer;
    use blueprint_sdk::evm::producer::{PollingConfig, PollingProducer};
    use blueprint_sdk::evm::util::get_provider_http;
    use blueprint_sdk::runner::config::{BlueprintEnvironment, ProtocolSettings};
    use blueprint_sdk::runner::eigenlayer::bls::EigenlayerBLSConfig;
    use blueprint_sdk::runner::BlueprintRunner;
    use blueprint_sdk::testing::utils::anvil::keys::ANVIL_PRIVATE_KEYS;
    use blueprint_sdk::testing::utils::anvil::start_default_anvil_testnet;
    use blueprint_sdk::testing::utils::setup_log;
    use gadget_keystore::Keystore;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;
    use tokio_util::sync::CancellationToken;
    use tower::BoxError;

    const REGISTRY_COORDINATOR_ADDRESS: Address =
        address!("0xc3e53f4d16ae77db1c982e75a937b9f60fe63690");

    /// Address of the first account in the local anvil network
    const ANVIL_FIRST_ADDRESS: Address = address!("f39Fd6e51aad88F6F4ce6aB8827279cffFb92266");

    /// Submits squaring tasks periodically
    async fn submit_tasks<P: Provider>(
        provider: P,
        contract_address: Address,
    ) -> Result<(), BoxError> {
        let mut interval = tokio::time::interval(Duration::from_secs(3));
        let mut number: u64 = 1;

        loop {
            interval.tick().await;

            // Create task parameters
            let quorum_threshold = 66; // 66% threshold
            let quorum_numbers = vec![0u8]; // Example quorum number

            let contract = SquaringTask::new(contract_address, &provider);
            // Submit task
            match contract
                .createSquaringTask(U256::from(number), quorum_threshold, quorum_numbers.into())
                .send()
                .await
            {
                Ok(tx) => {
                    let receipt = tx.get_receipt().await?;
                    assert!(receipt.status(), "Failed to process receipt: {:?}", receipt);
                    tracing::info!(
                        "Submitted squaring task for number {} in tx {:?} (block: {:?})",
                        number,
                        receipt.transaction_hash,
                        receipt.block_number,
                    );
                    number += 1;
                }
                Err(e) => {
                    tracing::error!("Failed to submit task: {}", e);
                }
            }
        }
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn task_monitoring() -> Result<(), BoxError> {
        setup_log();
        let (anvil_container, rpc_url, ws_url) = start_default_anvil_testnet(false).await;
        let mut env = BlueprintEnvironment::default();
        env.http_rpc_endpoint = rpc_url.clone();
        env.ws_rpc_endpoint = ws_url.clone();
        let el_settings = Default::default();
        env.protocol_settings = ProtocolSettings::Eigenlayer(el_settings);
        let signer: PrivateKeySigner = ANVIL_PRIVATE_KEYS[0].parse().unwrap();

        let provider = blueprint_sdk::evm::util::get_provider_http(&rpc_url);

        // Deploy contracts and get addresses
        let generator = ANVIL_FIRST_ADDRESS;
        let aggregator = ANVIL_FIRST_ADDRESS;
        let owner = ANVIL_FIRST_ADDRESS;

        let register_coordinatior = REGISTRY_COORDINATOR_ADDRESS;
        let task_response_window = 1000;
        let avs_registry_chain_reader =
            eigensdk::client_avsregistry::reader::AvsRegistryChainReader::new(
                eigensdk::logging::get_test_logger(),
                register_coordinatior,
                el_settings.operator_state_retriever_address,
                rpc_url,
            )
            .await?;
        let (operator_info_service, _) = eigensdk::services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory::new(
            eigensdk::logging::get_test_logger(),
            avs_registry_chain_reader.clone(),
            ws_url,
        )
        .await?;
        let cancellation_token = CancellationToken::new();
        let token_clone = cancellation_token.clone();
        let provider = provider.clone();
        let current_block = provider.get_block_number().await?;
        let operator_info_clone = operator_info_service.clone();

        tokio::task::spawn(async move {
            operator_info_clone
                .start_service(&token_clone, 0, current_block)
                .await
        });
        let avs_registry_service_chain_caller =
            eigensdk::services_avsregistry::chaincaller::AvsRegistryServiceChainCaller::new(
                avs_registry_chain_reader,
                operator_info_service,
            );
        let bls_aggregator_service =
            eigensdk::services_blsaggregation::bls_agg::BlsAggregatorService::new(
                avs_registry_service_chain_caller,
                eigensdk::logging::get_test_logger(),
            );
        println!("Deploying contract");
        let contract = SquaringTask::deploy(
            provider.clone(),
            register_coordinatior,
            task_response_window,
        )
        .await?;
        println!("Contract deployed at: {:?}", contract.address());
        let receipt = contract
            .initialize(aggregator, generator, owner)
            .send()
            .await?
            .get_receipt()
            .await?;

        assert!(receipt.status(), "{receipt:?}");

        let provider = Arc::new(provider);
        // Create producer for task events
        let task_polling_producer = PollingProducer::new(
            provider.clone(),
            PollingConfig {
                poll_interval: Duration::from_millis(2000),
                start_block: 200,
                confirmations: 3,
                step: 1,
            },
        );

        let evm_consumer = EVMConsumer::new(provider.clone(), EthereumWallet::new(signer));

        // Submit task
        let handle = tokio::spawn(submit_tasks(provider.clone(), *contract.address()));
        let config = EigenlayerBLSConfig::new(ANVIL_FIRST_ADDRESS, ANVIL_FIRST_ADDRESS)
            .with_exit_after_register(false);

        // Create and run the blueprint
        BlueprintRunner::builder(config, env)
            .router(x_square_create_contract_router(ctx, *contract.address()))
            .producer(task_polling_producer)
            .consumer(evm_consumer)
            .with_shutdown_handler(async move {
                handle.abort();
                anvil_container.stop().await.unwrap();
                anvil_container.rm().await.unwrap();
            })
            .run()
            .await?;

        Ok(())
    }
}
