use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EarnerLeaf {
    pub earner: String,
    #[serde(with = "hex")]
    pub earner_token_root: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenLeaf {
    pub token: String,
    pub cumulative_earnings: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proof {
    #[serde(with = "hex")]
    pub root: Vec<u8>,
    pub root_index: u32,
    pub earner_index: u32,
    #[serde(with = "hex")]
    pub earner_tree_proof: Vec<u8>,
    pub earner_leaf: EarnerLeaf,
    pub token_indices: Vec<u32>,
    pub token_tree_proofs: Vec<String>,
    pub token_leaves: Vec<TokenLeaf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummarizedEarnerReward {
    pub token: String,
    pub earned: String,
    pub active: String,
    pub claimed: String,
    pub claimable: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DistributionRoot {
    pub root: String,
    pub root_index: u64,
    pub rewards_calculation_end: String,
    pub activated_at: String,
    pub created_at_block_number: u64,
    pub transaction_hash: String,
    pub block_height: u64,
    pub disabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GenerateClaimProofRequest {
    pub earner_address: String,
    pub tokens: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_index: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerateClaimProofResponse {
    pub proof: Proof,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSummarizedRewardsRequest {
    pub earner_address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSummarizedRewardsResponse {
    pub rewards: Vec<SummarizedEarnerReward>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDistributionRootsRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_height: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListDistributionRootsResponse {
    pub distribution_roots: Vec<DistributionRoot>,
}

mod hex {
    use serde::{Deserialize, Deserializer, Serializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&format!("0x{}", hex::encode(bytes)))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        let s = s.strip_prefix("0x").unwrap_or(&s);
        hex::decode(s).map_err(serde::de::Error::custom)
    }
}

