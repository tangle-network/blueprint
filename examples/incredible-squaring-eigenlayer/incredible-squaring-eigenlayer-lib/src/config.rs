// use blueprint_sdk::alloy::primitives::Address;
// use blueprint_sdk::alloy::providers::{Provider, ProviderBuilder, RootProvider};
// use blueprint_sdk::alloy::signers::local::PrivateKeySigner;
// use blueprint_sdk::eigensdk::client_avsregistry::reader::AvsRegistryChainReader;
// use blueprint_sdk::eigensdk::crypto_bls::BlsKeyPair;
// use blueprint_sdk::eigensdk::services_avsregistry::chaincaller::AvsRegistryServiceChainCaller;
// use blueprint_sdk::eigensdk::services_blsaggregation::bls_agg::BlsAggregatorService;
// use blueprint_sdk::eigensdk::services_operatorsinfo::operatorsinfo_inmemory::OperatorInfoServiceInMemory;
// use blueprint_sdk::testing::utils::anvil::keys::ANVIL_PRIVATE_KEYS;

// pub type BlsAggServiceInMemory = BlsAggregatorService<
//     AvsRegistryServiceChainCaller<AvsRegistryChainReader, OperatorInfoServiceInMemory>,
// >;

// #[derive(Clone, Debug)]
// pub struct Keystore {
//     ecdsa_key: String,
//     bls_keypair: BlsKeyPair,
// }

// impl Default for Keystore {
//     fn default() -> Self {
//         // Use first Anvil private key
//         let ecdsa_key = ANVIL_PRIVATE_KEYS[0].to_string();

//         // Hardcoded BLS private key for testing
//         let bls_private_key =
//             "1371012690269088913462269866874713266643928125698382731338806296762673180359922";
//         let bls_keypair = BlsKeyPair::new(bls_private_key.to_string()).expect("Invalid BLS key");

//         Self {
//             ecdsa_key,
//             bls_keypair,
//         }
//     }
// }

// impl Keystore {
//     pub fn ecdsa_private_key(&self) -> &str {
//         &self.ecdsa_key
//     }

//     pub fn ecdsa_address(&self) -> Address {
//         let private_key: PrivateKeySigner = self.ecdsa_key.parse().unwrap();
//         private_key.address()
//     }

//     pub fn bls_keypair(&self) -> &BlsKeyPair {
//         &self.bls_keypair
//     }
// }
