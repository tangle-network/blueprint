# sidecar

## Purpose
HTTP client for the EigenLayer Sidecar API, which indexes rewards data, distribution roots, and generates Merkle proofs for on-chain rewards claiming. The Sidecar is EigenLayer's off-chain indexer replacing the deprecated S3-bucket approach.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `mod.rs` - Re-exports `SidecarClient` and all types from `types`.
- `client.rs` - `SidecarClient` struct wrapping a `reqwest::Client` with a base URL and 300-second timeout. Methods: `generate_claim_proof()` (POST to `/rewards/v1/claim-proof`, returns a `Proof` with Merkle tree data), `get_summarized_rewards()` (GET summarized earned/active/claimed/claimable amounts per token for an earner), `list_distribution_roots()` (GET distribution roots with optional block height filter). All methods set `x-sidecar-source: tangle-cli` header.
- `types.rs` - Request/response data types: `Proof` (root, earner/token tree proofs, leaves, indices), `EarnerLeaf`, `TokenLeaf`, `SummarizedEarnerReward` (token, earned, active, claimed, claimable), `DistributionRoot` (root hash, index, activation time, block info). Request types: `GenerateClaimProofRequest`, `GetSummarizedRewardsRequest`, `ListDistributionRootsRequest`. Custom hex serde module for byte fields.

## Key APIs
- `SidecarClient::new(base_url)` -- create client for a Sidecar endpoint
- `generate_claim_proof(earner, tokens, root_index)` -- generate Merkle proof for claiming
- `get_summarized_rewards(earner, block_height)` -- query reward summaries per token
- `list_distribution_roots(block_height)` -- list distribution roots
- `Proof` struct -- contains all data needed to submit an on-chain rewards claim
- `SummarizedEarnerReward` struct -- per-token reward breakdown
- `DistributionRoot` struct -- distribution root metadata

## Relationships
- Depends on `crate::error::EigenlayerExtraError` for error handling
- Complements `crate::services::rewards::RewardsManager` -- the Sidecar client fetches Merkle proofs that `RewardsManager.claim_rewards()` submits on-chain
- External dependency: EigenLayer Sidecar API (https://github.com/Layr-Labs/sidecar)
