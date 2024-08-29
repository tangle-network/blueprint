use alloy_network::EthereumWallet;
use alloy_primitives::{Address, FixedBytes};
use alloy_provider::ProviderBuilder;
use alloy_signer::k256::ecdsa::SigningKey;
use alloy_signer_local::PrivateKeySigner;
use color_eyre::{eyre::eyre, eyre::OptionExt, Result};
use gadget_sdk::{
    env::Protocol,
    events_watcher::{
        evm::{Config, EventWatcher},
        substrate::SubstrateEventWatcher,
        tangle::TangleEventsWatcher,
    },
    keystore::Backend,
    network::{
        gossip::NetworkService,
        setup::{start_p2p_network, NetworkConfig},
    },
    tangle_subxt::tangle_testnet_runtime::api::{
        self,
        runtime_types::{sp_core::ecdsa, tangle_primitives::services},
    },
    tx,
};
use std::net::IpAddr;

use incredible_squaring_blueprint::{self as blueprint, IncredibleSquaringTaskManager};

use gadget_common::config::DebugLogger;
use gadget_sdk::env::GadgetConfiguration;
use gadget_sdk::events_watcher::tangle::TangleConfig;
use gadget_sdk::keystore::BackendExt;
use gadget_sdk::network::gossip::GossipHandle;
use gadget_sdk::tangle_subxt::subxt;
use gadget_sdk::tangle_subxt::tangle_testnet_runtime::api::runtime_types::sp_core::ed25519;
use lock_api::{GuardSend, RwLock};
use std::sync::Arc;

#[async_trait::async_trait]
trait GadgetRunner {
    async fn register(&self) -> Result<()>;
    async fn run(&self) -> Result<()>;
}

struct TangleGadgetRunner {
    env: gadget_sdk::env::GadgetConfiguration<parking_lot::RawRwLock>,
}

#[async_trait::async_trait]
impl GadgetRunner for TangleGadgetRunner {
    async fn register(&self) -> Result<()> {
        // TODO: Use the function in blueprint-test-utils
        if self.env.test_mode {
            self.env.logger.info("Skipping registration in test mode");
            return Ok(());
        }

        let client = self.env.client().await.map_err(|e| eyre!(e))?;
        let signer = self.env.first_signer().map_err(|e| eyre!(e))?;
        let ecdsa_pair = self.env.first_ecdsa_signer().map_err(|e| eyre!(e))?;

        let xt = api::tx().services().register(
            self.env.blueprint_id,
            services::OperatorPreferences {
                key: ecdsa::Public(ecdsa_pair.public_key().0),
                approval: services::ApprovalPrefrence::None,
            },
            Default::default(),
        );

        // send the tx to the tangle and exit.
        let result = tx::tangle::send(&client, &signer, &xt).await?;
        tracing::info!("Registered operator with hash: {:?}", result);
        Ok(())
    }

    async fn run(&self) -> Result<()> {
        let env = gadget_sdk::env::load(None).map_err(|e| eyre!(e))?;
        let client = env.client().await.map_err(|e| eyre!(e))?;
        let signer = env.first_signer().map_err(|e| eyre!(e))?;

        let x_square = blueprint::XsquareEventHandler {
            service_id: env.service_id.unwrap(),
            signer,
        };

        tracing::info!("Starting the event watcher ...");

        SubstrateEventWatcher::run(
            &TangleEventsWatcher,
            client,
            // Add more handler here if we have more functions.
            vec![Box::new(x_square)],
        )
        .await?;

        Ok(())
    }
}

struct EigenlayerGadgetRunner<R: lock_api::RawRwLock> {
    env: GadgetConfiguration<R>,
}

struct EigenlayerEventWatcher<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Config> EventWatcher<T> for EigenlayerEventWatcher<T> {
    const TAG: &'static str = "eigenlayer";
    type Contract =
        IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance<T::T, T::P, T::N>;
    type Event = IncredibleSquaringTaskManager::NewTaskCreated;
    const GENESIS_TX_HASH: FixedBytes<32> = FixedBytes::from_slice(&[0; 32]);
}

#[async_trait::async_trait]
impl GadgetRunner for EigenlayerGadgetRunner<parking_lot::RawRwLock> {
    async fn register(&self) -> Result<()> {
        if self.env.test_mode {
            self.env.logger.info("Skipping registration in test mode");
            return Ok(());
        }

        // TODO: Register to become an operator
        let keystore = self.env.keystore().map_err(|e| eyre!(e))?;
        let ecdsa_key = self.env.first_ecdsa_signer().map_err(|e| eyre!(e))?;
        //let signer: PrivateKeySigner = PrivateKeySigner::from_signing_key(signing_key);
        // Implement Eigenlayer-specific registration logic here
        // For example:
        // let contract = YourEigenlayerContract::new(contract_address, provider);
        // contract.register_operator(signer).await?;

        tracing::info!("Registered operator for Eigenlayer");
        Ok(())
    }

    async fn run(&self) -> Result<()> {
        let env = gadget_sdk::env::load(Some(Protocol::Eigenlayer)).map_err(|e| eyre!(e))?;
        let keystore = env.keystore().map_err(|e| eyre!(e))?;
        // get the first ECDSA key from the keystore and register with it.
        let ecdsa_key = self.env.first_ecdsa_signer().map_err(|e| eyre!(e))?;
        let ed_key = keystore
            .iter_ed25519()
            .next()
            .ok_or_eyre("Unable to find ED25519 key")?;
        let ed_public_bytes = ed_key.as_ref(); // 32 byte len

        let ed_public = ed25519_zebra::VerificationKey::try_from(ed_public_bytes)
            .map_err(|e| eyre!("Unable to create ed25519 public key"))?;

        let signing_key = SigningKey::from(ecdsa_key.public_key());
        let priv_key_signer: PrivateKeySigner = PrivateKeySigner::from_signing_key(signing_key);
        let wallet = EthereumWallet::from(priv_key_signer.clone());
        // Set up eignelayer AVS
        let contract_address = Address::from_slice(&[0; 20]);
        // Set up the HTTP provider with the `reqwest` crate.
        let provider = ProviderBuilder::new()
            .with_recommended_fillers()
            .wallet(wallet)
            .on_http(env.rpc_endpoint.parse()?);

        let sr_secret = keystore
            .expose_ed25519_secret(&ed_public)
            .map_err(|e| eyre!(e))?
            .ok_or_eyre("Unable to find ED25519 secret")?;
        let mut sr_secret_bytes = sr_secret.as_ref().to_vec(); // 64 byte len

        let identity = libp2p::identity::Keypair::ed25519_from_bytes(&mut sr_secret_bytes)
            .map_err(|e| eyre!("Unable to construct libp2p keypair: {e:?}"))?;

        // TODO: Fill in and find the correct values for the network configuration
        // TODO: Implementations for reading set of operators from Tangle & Eigenlayer
        let network_config: NetworkConfig = NetworkConfig {
            identity,
            ecdsa_key,
            bootnodes: vec![],
            bind_ip: self.env.bind_addr,
            bind_port: self.env.bind_port,
            topics: vec!["__TESTING_INCREDIBLE_SQUARING".to_string()],
            logger: self.env.logger.clone(),
        };

        let network: GossipHandle = start_p2p_network(network_config).map_err(|e| eyre!(e))?;
        // let x_square_eigen = blueprint::XsquareEigenEventHandler {
        //     ctx: blueprint::MyContext { network, keystore },
        // };
        //
        // let contract: IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance = IncredibleSquaringTaskManager::IncredibleSquaringTaskManagerInstance::new(contract_address, provider);
        //
        // EventWatcher::run(
        //     &EigenlayerEventWatcher,
        //     contract,
        //     // Add more handler here if we have more functions.
        //     vec![Box::new(x_square_eigen)],
        // )
        // .await?;

        Ok(())
    }
}

fn create_gadget_runner(
    protocol: Protocol,
) -> (
    GadgetConfiguration<parking_lot::RawRwLock>,
    Arc<dyn GadgetRunner>,
) {
    let env = gadget_sdk::env::load(Some(protocol)).expect("Failed to load environment");
    match protocol {
        Protocol::Tangle => (env.clone(), Arc::new(TangleGadgetRunner { env })),
        Protocol::Eigenlayer => (env.clone(), Arc::new(EigenlayerGadgetRunner { env })),
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;
    let env_filter = tracing_subscriber::EnvFilter::from_default_env();
    let logger = tracing_subscriber::fmt()
        .compact()
        .with_target(true)
        .with_env_filter(env_filter);
    logger.init();

    // Load the environment and create the gadget runner
    let protocol = Protocol::from_env().unwrap_or(Protocol::Tangle);
    let (env, runner) = create_gadget_runner(protocol);
    // Register the operator if needed
    if env.should_run_registration() {
        runner.register().await?;
    }

    // Run the gadget / AVS
    runner.run().await?;

    Ok(())
}
