use alloy_network::EthereumWallet;
use alloy_primitives::{Address, FixedBytes};
use alloy_signer_local::PrivateKeySigner;
/// EigenLayer rewards claiming and management
///
/// Requires EigenLayer Sidecar API (<https://github.com/Layr-Labs/sidecar>)
use blueprint_eigenlayer_extra::sidecar::{Proof, SidecarClient};
use blueprint_evm_extra::util::get_wallet_provider_http;
use blueprint_keystore::backends::Backend;
use blueprint_keystore::backends::eigenlayer::EigenlayerBackend;
use blueprint_keystore::crypto::k256::K256Ecdsa;
use blueprint_keystore::{Keystore, KeystoreConfig};
use color_eyre::Result;
use eigensdk::utils::rewardsv2::core::rewards_coordinator::{
    IRewardsCoordinator, RewardsCoordinator,
};
use std::str::FromStr;

const DEFAULT_SIDECAR_URL_MAINNET: &str = "https://sidecar-rpc.eigenlayer.xyz/mainnet";
const DEFAULT_SIDECAR_URL_HOLESKY: &str = "https://sidecar-rpc.eigenlayer.xyz/holesky";

/// Show rewards summary for an earner address
///
/// # Arguments
/// * `earner_address` - Ethereum address of the earner
/// * `sidecar_url` - Optional Sidecar API URL (defaults to mainnet)
/// * `network` - Network name (mainnet or holesky)
///
/// # Errors
///
/// Returns error if:
/// - Sidecar client cannot be created
/// - Rewards data cannot be fetched from Sidecar API
pub async fn show_rewards(
    earner_address: &str,
    sidecar_url: Option<&str>,
    network: Option<&str>,
) -> Result<()> {
    let url = sidecar_url.unwrap_or(match network {
        Some("holesky") => DEFAULT_SIDECAR_URL_HOLESKY,
        _ => DEFAULT_SIDECAR_URL_MAINNET,
    });

    println!("Fetching rewards from Sidecar: {}", url);

    let client = SidecarClient::new(url.to_string())
        .map_err(|e| color_eyre::eyre::eyre!("Failed to create Sidecar client: {}", e))?;

    let rewards = client
        .get_summarized_rewards(earner_address, None)
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Failed to fetch rewards: {}", e))?;

    if rewards.is_empty() {
        println!("No rewards found for earner: {}", earner_address);
        return Ok(());
    }

    println!("\nRewards Summary for {}:", earner_address);
    println!("{:-<100}", "");
    println!(
        "{:<42} {:>15} {:>15} {:>15} {:>15}",
        "Token", "Earned", "Active", "Claimed", "Claimable"
    );
    println!("{:-<100}", "");

    for reward in rewards {
        println!(
            "{:<42} {:>15} {:>15} {:>15} {:>15}",
            reward.token, reward.earned, reward.active, reward.claimed, reward.claimable
        );
    }
    println!("{:-<100}", "");

    Ok(())
}

/// Claim rewards for an earner
///
/// # Arguments
/// * `earner_address` - Ethereum address of the earner
/// * `recipient_address` - Address to receive rewards (defaults to earner)
/// * `token_addresses` - Specific tokens to claim (empty = all claimable)
/// * `rewards_coordinator` - `RewardsCoordinator` contract address
/// * `sidecar_url` - Optional Sidecar API URL
/// * `network` - Network name
/// * `keystore_uri` - Keystore URI for signing
/// * `rpc_url` - Ethereum RPC endpoint
/// * `batch_file` - Optional YAML file for batch claiming
///
/// # Errors
///
/// Returns error if:
/// - Sidecar client cannot be created
/// - Invalid address format for earner or recipient
/// - Keystore cannot be loaded or accessed
/// - ECDSA key not found in keystore
/// - Failed to create wallet signer
/// - Proof generation fails
/// - Transaction submission or receipt retrieval fails
/// - Batch file cannot be read or parsed
#[allow(clippy::too_many_arguments)]
pub async fn claim_rewards(
    earner_address: &str,
    recipient_address: Option<&str>,
    token_addresses: Vec<String>,
    rewards_coordinator: Address,
    sidecar_url: Option<&str>,
    network: Option<&str>,
    keystore_uri: &str,
    rpc_url: &str,
    batch_file: Option<&str>,
) -> Result<()> {
    let url = sidecar_url.unwrap_or(match network {
        Some("holesky") => DEFAULT_SIDECAR_URL_HOLESKY,
        _ => DEFAULT_SIDECAR_URL_MAINNET,
    });

    let sidecar = SidecarClient::new(url.to_string())
        .map_err(|e| color_eyre::eyre::eyre!("Failed to create Sidecar client: {}", e))?;

    let recipient = if let Some(addr) = recipient_address {
        Address::parse_checksummed(addr, None)
            .map_err(|e| color_eyre::eyre::eyre!("Invalid recipient address: {}", e))?
    } else {
        Address::parse_checksummed(earner_address, None)
            .map_err(|e| color_eyre::eyre::eyre!("Invalid earner address: {}", e))?
    };

    // Load keystore for signing
    println!("Loading keystore from: {}", keystore_uri);
    let keystore_config = KeystoreConfig::new().fs_root(keystore_uri);
    let keystore = Keystore::new(keystore_config)?;

    let ecdsa_public = keystore
        .first_local::<K256Ecdsa>()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get ECDSA key: {}", e))?;

    let ecdsa_secret = keystore
        .expose_ecdsa_secret(&ecdsa_public)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to expose ECDSA secret: {}", e))?
        .ok_or_else(|| color_eyre::eyre::eyre!("No ECDSA secret found in keystore"))?;

    let private_key = alloy_primitives::hex::encode(ecdsa_secret.0.to_bytes());
    let wallet = PrivateKeySigner::from_str(&private_key)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to create signer: {}", e))?;

    let provider = get_wallet_provider_http(rpc_url, EthereumWallet::from(wallet));
    let rewards_contract = RewardsCoordinator::new(rewards_coordinator, provider);

    if let Some(file_path) = batch_file {
        // Batch claim from YAML file
        println!("Processing batch claims from file: {}", file_path);
        let yaml_content = std::fs::read_to_string(file_path)?;

        #[derive(serde::Deserialize)]
        struct BatchClaimEntry {
            earner_address: String,
            token_addresses: Vec<String>,
        }

        let entries: Vec<BatchClaimEntry> = serde_yaml::from_str(&yaml_content)?;
        let mut proofs = Vec::new();

        for entry in entries {
            println!("Generating proof for earner: {}", entry.earner_address);

            let proof = sidecar
                .generate_claim_proof(&entry.earner_address, entry.token_addresses, None)
                .await
                .map_err(|e| color_eyre::eyre::eyre!("Failed to generate proof: {}", e))?;

            proofs.push(convert_proof_to_contract_format(proof));
        }

        if proofs.is_empty() {
            return Err(color_eyre::eyre::eyre!("No valid proofs generated"));
        }

        println!("Submitting batch claim with {} proofs...", proofs.len());

        let receipt = rewards_contract
            .processClaims(proofs, recipient)
            .send()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Transaction failed: {}", e))?
            .get_receipt()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to get receipt: {}", e))?;

        println!("Batch claim successful!");
        println!("Transaction hash: {:?}", receipt.transaction_hash);
    } else {
        // Single claim
        println!("Generating claim proof for earner: {}", earner_address);

        let proof = sidecar
            .generate_claim_proof(earner_address, token_addresses, None)
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to generate proof: {}", e))?;

        let claim = convert_proof_to_contract_format(proof);

        println!("Submitting claim transaction...");

        let receipt = rewards_contract
            .processClaim(claim, recipient)
            .send()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Transaction failed: {}", e))?
            .get_receipt()
            .await
            .map_err(|e| color_eyre::eyre::eyre!("Failed to get receipt: {}", e))?;

        println!("Claim successful!");
        println!("Transaction hash: {:?}", receipt.transaction_hash);
    }

    Ok(())
}

/// Set the claimer address for the earner
///
/// # Arguments
/// * `claimer_address` - Address authorized to claim on behalf of earner
/// * `rewards_coordinator` - `RewardsCoordinator` contract address
/// * `keystore_uri` - Keystore URI for signing
/// * `rpc_url` - Ethereum RPC endpoint
///
/// # Errors
///
/// Returns error if:
/// - Invalid claimer address format
/// - Keystore cannot be loaded or accessed
/// - ECDSA key not found in keystore
/// - Failed to derive earner address
/// - Failed to create wallet signer
/// - Transaction submission or receipt retrieval fails
pub async fn set_claimer(
    claimer_address: &str,
    rewards_coordinator: Address,
    keystore_uri: &str,
    rpc_url: &str,
) -> Result<()> {
    let claimer = Address::parse_checksummed(claimer_address, None)
        .map_err(|e| color_eyre::eyre::eyre!("Invalid claimer address: {}", e))?;

    println!("Loading keystore from: {}", keystore_uri);
    let keystore_config = KeystoreConfig::new().fs_root(keystore_uri);
    let keystore = Keystore::new(keystore_config)?;

    let ecdsa_public = keystore
        .first_local::<K256Ecdsa>()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get ECDSA key: {}", e))?;

    let ecdsa_secret = keystore
        .expose_ecdsa_secret(&ecdsa_public)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to expose ECDSA secret: {}", e))?
        .ok_or_else(|| color_eyre::eyre::eyre!("No ECDSA secret found in keystore"))?;

    let earner_address = ecdsa_secret
        .alloy_address()
        .map_err(|e| color_eyre::eyre::eyre!("Failed to derive address: {}", e))?;

    let private_key = alloy_primitives::hex::encode(ecdsa_secret.0.to_bytes());
    let wallet = PrivateKeySigner::from_str(&private_key)
        .map_err(|e| color_eyre::eyre::eyre!("Failed to create signer: {}", e))?;

    let provider = get_wallet_provider_http(rpc_url, EthereumWallet::from(wallet));
    let rewards_contract = RewardsCoordinator::new(rewards_coordinator, provider);

    println!(
        "Setting claimer {} for earner {}...",
        claimer, earner_address
    );

    let receipt = rewards_contract
        .setClaimerFor(claimer)
        .send()
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Transaction failed: {}", e))?
        .get_receipt()
        .await
        .map_err(|e| color_eyre::eyre::eyre!("Failed to get receipt: {}", e))?;

    println!("Claimer set successfully!");
    println!("Transaction hash: {:?}", receipt.transaction_hash);

    Ok(())
}

/// Convert Sidecar Proof to `RewardsCoordinator` contract format
fn convert_proof_to_contract_format(proof: Proof) -> IRewardsCoordinator::RewardsMerkleClaim {
    let mut earner_token_root = [0u8; 32];
    earner_token_root.copy_from_slice(&proof.earner_leaf.earner_token_root);

    let earner = Address::parse_checksummed(&proof.earner_leaf.earner, None)
        .expect("Invalid earner address in proof");

    let token_leaves: Vec<IRewardsCoordinator::TokenTreeMerkleLeaf> = proof
        .token_leaves
        .into_iter()
        .map(|leaf| {
            let token = Address::parse_checksummed(&leaf.token, None)
                .expect("Invalid token address in proof");
            let cumulative_earnings = alloy_primitives::U256::from_str(&leaf.cumulative_earnings)
                .expect("Invalid cumulative earnings");

            IRewardsCoordinator::TokenTreeMerkleLeaf {
                token,
                cumulativeEarnings: cumulative_earnings,
            }
        })
        .collect();

    let token_tree_proofs: Vec<alloy_primitives::Bytes> = proof
        .token_tree_proofs
        .into_iter()
        .map(|p| {
            let bytes =
                hex::decode(p.trim_start_matches("0x")).expect("Invalid hex in token tree proof");
            alloy_primitives::Bytes::from(bytes)
        })
        .collect();

    IRewardsCoordinator::RewardsMerkleClaim {
        rootIndex: proof.root_index,
        earnerIndex: proof.earner_index,
        earnerTreeProof: proof.earner_tree_proof.into(),
        earnerLeaf: IRewardsCoordinator::EarnerTreeMerkleLeaf {
            earner,
            earnerTokenRoot: FixedBytes::from(earner_token_root),
        },
        tokenIndices: proof.token_indices,
        tokenTreeProofs: token_tree_proofs,
        tokenLeaves: token_leaves,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use blueprint_eigenlayer_extra::sidecar::{EarnerLeaf, TokenLeaf};

    #[test]
    fn test_convert_proof_to_contract_format() {
        let earner_token_root = vec![1u8; 32];
        let root = vec![2u8; 32];
        let earner_tree_proof = vec![3u8; 32];

        let proof = Proof {
            root,
            root_index: 1,
            earner_index: 2,
            earner_tree_proof,
            earner_leaf: EarnerLeaf {
                earner: "0x1234567890123456789012345678901234567890".to_string(),
                earner_token_root,
            },
            token_indices: vec![0, 1],
            token_tree_proofs: vec![
                "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa".to_string(),
                "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb".to_string(),
            ],
            token_leaves: vec![
                TokenLeaf {
                    token: "0xabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd".to_string(),
                    cumulative_earnings: "1000000000000000000".to_string(),
                },
                TokenLeaf {
                    token: "0x1111111111111111111111111111111111111111".to_string(),
                    cumulative_earnings: "2000000000000000000".to_string(),
                },
            ],
        };

        let claim = convert_proof_to_contract_format(proof);

        assert_eq!(claim.rootIndex, 1);
        assert_eq!(claim.earnerIndex, 2);
        assert_eq!(claim.tokenIndices, vec![0, 1]);
        assert_eq!(claim.tokenLeaves.len(), 2);
        assert_eq!(claim.tokenTreeProofs.len(), 2);
        assert_eq!(
            claim.earnerLeaf.earner,
            Address::parse_checksummed("0x1234567890123456789012345678901234567890", None).unwrap()
        );
    }

    #[test]
    fn test_proof_conversion_with_hex_prefixes() {
        let proof = Proof {
            root: vec![1u8; 32],
            root_index: 0,
            earner_index: 0,
            earner_tree_proof: vec![2u8; 32],
            earner_leaf: EarnerLeaf {
                earner: "0x1234567890123456789012345678901234567890".to_string(),
                earner_token_root: vec![3u8; 32],
            },
            token_indices: vec![0],
            token_tree_proofs: vec![
                "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc".to_string(),
            ],
            token_leaves: vec![TokenLeaf {
                token: "0xabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd".to_string(),
                cumulative_earnings: "1000".to_string(),
            }],
        };

        let claim = convert_proof_to_contract_format(proof);
        assert_eq!(claim.tokenTreeProofs.len(), 1);
        assert_eq!(claim.tokenTreeProofs[0].len(), 32);
    }

    #[test]
    fn test_proof_conversion_without_hex_prefixes() {
        let proof = Proof {
            root: vec![1u8; 32],
            root_index: 0,
            earner_index: 0,
            earner_tree_proof: vec![2u8; 32],
            earner_leaf: EarnerLeaf {
                earner: "0x1234567890123456789012345678901234567890".to_string(),
                earner_token_root: vec![3u8; 32],
            },
            token_indices: vec![0],
            token_tree_proofs: vec![
                "dddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddddd".to_string(),
            ],
            token_leaves: vec![TokenLeaf {
                token: "0xabcdabcdabcdabcdabcdabcdabcdabcdabcdabcd".to_string(),
                cumulative_earnings: "1000".to_string(),
            }],
        };

        let claim = convert_proof_to_contract_format(proof);
        assert_eq!(claim.tokenTreeProofs.len(), 1);
        assert_eq!(claim.tokenTreeProofs[0].len(), 32);
    }
}
