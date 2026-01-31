# Operator RFQ Pricing Server (Tangle EVM)

The pricing engine watches the Tangle v2 EVM contracts for blueprint/service events, benchmarks the node, computes prices from the local policy file, and responds to `GetPrice` RPC calls with ITangle-compatible signed quotes. It replaces the legacy Substrate/Tangle stack.

## Runtime Flow

1. **Bootstrap**
   - `pricing-engine-server` reads `operator.toml`, loads the pricing table, and initialises the RocksDB benchmark cache.
   - `init_operator_signer` ensures a `k256` key exists in the keystore and derives the operator’s Ethereum address.
   - The CLI arguments (or environment variables) provide the HTTP/WS RPC URLs plus the `ITangle`, `MultiAssetDelegation`, and `OperatorStatusRegistry` contract addresses.
2. **Event ingestion**
   - `EvmEventListener` polls `ITangle::ServiceActivated` / `ServiceTerminated` logs via `blueprint-client-tangle`.
   - For each activation we enqueue a benchmarking task so the cache always has a recent profile per blueprint.
3. **RPC handling**
   - `GetPriceRequest` includes PoW (`proof_of_work` + `challenge_timestamp`), TTL, and security requirements.
   - `PricingEngineService` verifies PoW, loads the cached benchmark profile, and runs `calculate_price`.
   - The resulting `QuoteDetails` are converted to both the proto response and the ABI payload expected by `ITangle::createServiceFromQuotes`.
4. **Signing**
   - `SignableQuote` hashes the ABI-encoded `ITangleTypes::QuoteDetails` (keccak256) and `OperatorSigner` signs with the EVM key (`K256Ecdsa`).
   - The response returns the proto `QuoteDetails`, the ECDSA signature (65 bytes), the operator address, and a fresh PoW solution for the client.

## Components

| Module | Responsibility |
| --- | --- |
| `app.rs` | Bootstraps the service, spawns the event listener and RPC server. |
| `service/blockchain/evm_listener.rs` | Polls Alloy logs for ITangle events and pushes `BlockchainEvent`s. |
| `benchmark/*` | Runs CPU/memory/storage benchmarks and persists them in `BenchmarkCache`. |
| `pricing.rs` | Applies the configured resource rates + TTL/security adjustments. |
| `signer.rs` | Builds ABI-compatible `QuoteDetails`, hashes them, and signs with `k256`. |
| `service/rpc/server.rs` | Implements the gRPC surface defined in `proto/pricing.proto`. |

## Quote Format

`proto/pricing.proto` now exposes the same structure as `ITangle.SignedQuote`. Security commitments are arrays, matching the Solidity ABI:

```proto
message QuoteDetails {
  uint64 blueprint_id = 1;
  uint64 ttl_blocks = 2;
  double total_cost_rate = 3;
  uint64 timestamp = 4;
  uint64 expiry = 5;
  repeated ResourcePricing resources = 6;
  repeated AssetSecurityCommitment security_commitments = 7;
}
```

The signer converts these proto structs into `ITangleTypes::QuoteDetails` with:

* `totalCost` stored as a scaled `U256` (10⁻⁹ precision via `decimal_to_scaled_amount`).
* `securityCommitments[i].asset` encoded as `{ kind: ERC20 (1), token: Address }`.
* `securityCommitments[i].exposureBps` derived from the requested exposure percent (×100).

## CLI and Environment

`pricing-engine-server --help` exposes all runtime inputs. The important env vars:

| Flag | Env | Description |
| --- | --- | --- |
| `--config` | `OPERATOR_CONFIG_PATH` | Path to `operator.toml`. |
| `--pricing-config` | `PRICING_CONFIG_PATH` | Resource pricing table (TOML). |
| `--http-rpc-endpoint` | `OPERATOR_HTTP_RPC` | HTTPS endpoint for the Tangle EVM RPC. |
| `--ws-rpc-endpoint` | `OPERATOR_WS_RPC` | WebSocket endpoint for real-time logs. |
| `--blueprint-id` | `OPERATOR_BLUEPRINT_ID` | Blueprint to watch for activations. |
| `--service-id` | `OPERATOR_SERVICE_ID` | Optional fixed service to benchmark. |
| `--tangle-contract` | `OPERATOR_TANGLE_CONTRACT` | ITangle contract address. |
| `--restaking-contract` | `OPERATOR_RESTAKING_CONTRACT` | MultiAssetDelegation contract. |
| `--status-registry-contract` | `OPERATOR_STATUS_REGISTRY_CONTRACT` | OperatorStatusRegistry contract. |

`operator.toml` controls the local node behaviour (cache paths, RPC bind addr, benchmark cadence, etc.). See the updated sample in `crates/pricing-engine/operator.toml`.

## Building and Testing

```bash
# Format proto + rebuild bindings
cargo fmt -p blueprint-pricing-engine

# Unit tests (signer + pricing config)
cargo test -p blueprint-pricing-engine signer_test

# Run the daemon with env vars
OPERATOR_HTTP_RPC=https://rpc.tangle.tools \
OPERATOR_WS_RPC=wss://rpc.tangle.tools \
OPERATOR_TANGLE_CONTRACT=0xCf7Ed3AccA5a467e9e704C703E8D87F634fB0Fc9 \
OPERATOR_RESTAKING_CONTRACT=0xe7f1725E7734CE288F8367e1Bb143E90bb3F0512 \
OPERATOR_STATUS_REGISTRY_CONTRACT=0xdC64a140Aa3E981100a9BecA4E685f962f0CF6C9 \
cargo run -p blueprint-pricing-engine --bin pricing-engine-server
```

## Security Notes

- Quotes are protected by:
  - Proof-of-work challenge (`sha2`-based) to prevent RPC abuse.
  - k256 ECDSA signatures hashed over the ABI payload (keccak256) so they can be submitted directly to `ITangle`.
  - Explicit TTL (`ttl_blocks`) and expiry timestamp to avoid replays.
- The keystore lives under `OperatorConfig.keystore_path`. Keys are generated with `blueprint-keystore` and never leave disk.

Keep the contract addresses and keystore directory private; the RPC server does **not** expose signing endpoints beyond `GetPrice`.
