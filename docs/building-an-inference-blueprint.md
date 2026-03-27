# Building an Inference Blueprint

A complete walkthrough of building a production inference blueprint on Tangle, using the [vLLM inference blueprint](https://github.com/tangle-network/vllm-inference-blueprint) as a real-world reference. Covers contract design, operator registration, pricing, RFQ quoting, and shielded payment settlement.

## Architecture

```
                           ┌──────────────────────────────────────────┐
                           │              Tangle Chain                │
                           │                                         │
                           │  ┌─────────────┐   ┌────────────────┐  │
                           │  │  TNT Core    │   │ ShieldedCredits│  │
                           │  │  (Diamond)   │   │   (VAnchor)    │  │
                           │  └──────┬───────┘   └───────┬────────┘  │
                           │         │                   │           │
                           └─────────┼───────────────────┼───────────┘
                                     │                   │
              ┌──────────────────────┼───────────────────┼──────────┐
              │  Operator Node       │                   │          │
              │                      │                   │          │
              │  ┌───────────────┐   │   ┌────────────┐  │          │
              │  │ BlueprintRunner│◄──┘   │  Billing   │◄─┘          │
              │  │  (Rust)       │       │  Client    │             │
              │  └───────┬───────┘       └─────┬──────┘             │
              │          │                     │                    │
              │  ┌───────▼───────┐   ┌─────────▼──────┐            │
              │  │  HTTP Server  │   │ Pricing Engine │            │
              │  │  (Axum)       │   │  (gRPC sidecar)│            │
              │  └───────┬───────┘   └────────────────┘            │
              │          │                                         │
              │  ┌───────▼───────┐                                 │
              │  │  vLLM Process │                                 │
              │  │  (Python)     │                                 │
              │  └───────────────┘                                 │
              └────────────────────────────────────────────────────┘
                          ▲
                          │  POST /v1/chat/completions
                          │  + SpendAuth (EIP-712)
                          │
                      ┌───┴───┐
                      │ Client│
                      └───────┘
```

Two request paths exist:

1. **On-chain jobs** — customer submits a job to TNT Core, BlueprintRunner receives it via events, executes inference, posts result on-chain. Used for verifiable, recorded inference.
2. **x402 HTTP** — customer sends a request directly to the operator's HTTP endpoint with a SpendAuth signature. Operator serves immediately, settles payment asynchronously. Used for low-latency inference.

## Step 1: Design the Blueprint Service Manager (BSM)

The BSM is the on-chain contract that defines your blueprint's pricing, operator requirements, and lifecycle hooks. It extends `BlueprintServiceManagerBase` from TNT Core.

### Model configuration

Define what models your blueprint supports and their pricing:

```solidity
struct ModelConfig {
    uint32 maxContextLen;
    uint64 pricePerInputToken;    // base units (e.g., tsUSD wei)
    uint64 pricePerOutputToken;
    uint32 minGpuVramMib;
    bool enabled;
}

/// @notice Configure a model's pricing and GPU requirements.
/// @dev Only callable by the blueprint owner.
function configureModel(
    string calldata model,
    uint32 maxContextLen,
    uint64 pricePerInputToken,
    uint64 pricePerOutputToken,
    uint32 minGpuVramMib
) external onlyBlueprintOwner {
    bytes32 key = keccak256(bytes(model));
    modelConfigs[key] = ModelConfig({
        maxContextLen: maxContextLen,
        pricePerInputToken: pricePerInputToken,
        pricePerOutputToken: pricePerOutputToken,
        minGpuVramMib: minGpuVramMib,
        enabled: true
    });
    emit ModelConfigured(model, maxContextLen, pricePerInputToken, pricePerOutputToken);
}
```

This sets **default prices** per model. Operators can override these via RFQ quotes (see [Pricing](#step-3-configure-pricing)).

### Operator registration

Operators register with their hardware capabilities. The BSM validates they meet minimum requirements:

```solidity
/// @param registrationInputs abi.encode(string model, uint32 gpuCount, uint32 totalVramMib, string gpuModel, string endpoint)
function onRegister(
    address operator,
    bytes calldata registrationInputs
) external payable override onlyFromTangle {
    (
        string memory model,
        uint32 gpuCount,
        uint32 totalVramMib,
        string memory gpuModel,
        string memory endpoint
    ) = abi.decode(registrationInputs, (string, uint32, uint32, string, string));

    bytes32 modelKey = keccak256(bytes(model));
    ModelConfig storage mc = modelConfigs[modelKey];
    if (!mc.enabled) revert ModelNotSupported(model);
    if (totalVramMib < mc.minGpuVramMib) {
        revert InsufficientGpuCapability(mc.minGpuVramMib, totalVramMib);
    }

    operatorCaps[operator] = OperatorCapabilities({
        model: model,
        gpuCount: gpuCount,
        totalVramMib: totalVramMib,
        gpuModel: gpuModel,
        endpoint: endpoint,
        active: true
    });
}
```

**Key design decision:** One blueprint serves all model sizes. Model selection happens at registration time (which model the operator serves) and at service request time (which model the customer wants). The BSM's `configureModel()` sets per-model pricing and GPU requirements.

### Blueprint configuration

Set these in the `BlueprintConfig` passed during `createBlueprint`:

```solidity
BlueprintConfig({
    membership: MembershipModel.Open,          // anyone can be an operator
    pricing: PricingModel.EventDriven,         // pay per job (inference request)
    minOperators: 1,
    maxOperators: 100,
    subscriptionRate: 0,                       // unused for EventDriven
    subscriptionInterval: 0,
    eventRate: 1000000000000000                // 0.001 ETH base event rate
})
```

The three pricing models in TNT Core (`Types.sol:19-26`):
- **PayOnce** — single payment at service creation
- **Subscription** — recurring payments per interval
- **EventDriven** — payment per job/event (correct for inference)

## Step 2: Build the Operator Binary

The operator binary is a Rust program that uses the Blueprint SDK's `BlueprintRunner` to wire into the Tangle lifecycle.

### Registration payload

When an operator registers, generate the ABI-encoded payload matching your BSM's `onRegister` signature:

```rust
fn registration_payload(config: &OperatorConfig) -> Vec<u8> {
    (
        config.vllm.model.clone(),              // string model
        config.gpu.expected_gpu_count,           // uint32 gpuCount
        config.gpu.min_vram_mib,                // uint32 totalVramMib
        config.gpu.gpu_model.clone()
            .unwrap_or_else(|| "unknown".into()),// string gpuModel
        format!("http://{}:{}",
            config.server.host,
            config.server.port),                 // string endpoint
    ).abi_encode()
}
```

### Job handler

Define a job handler for on-chain inference requests:

```rust
#[debug_job]
pub async fn run_inference(
    TangleArg(request): TangleArg<InferenceRequest>,
) -> Result<TangleResult<InferenceResult>, RunnerError> {
    // Proxy to vLLM's /v1/chat/completions
    let response = vllm_client
        .chat_completion(&request.prompt, request.max_tokens)
        .await?;

    Ok(TangleResult(InferenceResult {
        text: response.choices[0].message.content.clone(),
        prompt_tokens: response.usage.prompt_tokens,
        completion_tokens: response.usage.completion_tokens,
    }))
}
```

### Background service

For x402 HTTP serving (the low-latency path), implement a background service that starts alongside the BlueprintRunner:

```rust
impl BackgroundService for InferenceServer {
    async fn start(&self) -> Result<()> {
        // 1. Spawn vLLM subprocess
        let vllm = VllmProcess::spawn(&self.config.vllm).await?;
        vllm.wait_ready().await?;

        // 2. Initialize billing client
        let billing = BillingClient::new(&self.config)?;

        // 3. Start HTTP server
        let app = Router::new()
            .route("/v1/chat/completions", post(chat_completions))
            .route("/v1/models", get(list_models))
            .route("/health", get(health_check));

        axum::serve(listener, app).await?;
        Ok(())
    }
}
```

### Wiring it together

```rust
BlueprintRunner::new(config, env)
    .producer(TangleProducer::new(client.clone()))
    .consumer(TangleConsumer::new(client.clone()))
    .job(0, run_inference)                    // job index 0 = inference
    .background_service(InferenceServer::new(op_config))
    .run()
    .await?;
```

## Step 3: Configure Pricing

Pricing operates at two levels. Both are optional — use whichever fits your needs.

### Level 1: BSM default prices

Set via `configureModel()` on-chain. These are the advertised prices that customers see when querying your blueprint. Good for simple, fixed pricing.

```
configureModel("meta-llama/Llama-3.1-8B-Instruct", 8192, 1, 2, 16000)
//                                                         │  │
//                                              input tok ─┘  └─ output tok
//                                              (tsUSD base units)
```

### Level 2: Pricing Engine sidecar (RFQ quotes)

For dynamic pricing based on hardware costs, demand, or custom logic, run the Pricing Engine as a gRPC sidecar. It calculates quotes and signs them with EIP-712.

**Configuration** (`config/pricing.toml`):

```toml
[default]
resources = [
  { kind = "GPU", count = 1, price_per_unit_rate = 0.005 },
  { kind = "CPU", count = 4, price_per_unit_rate = 0.001 },
  { kind = "MemoryMB", count = 24576, price_per_unit_rate = 0.00005 },
  { kind = "NetworkEgressMB", count = 1024, price_per_unit_rate = 0.00003 },
]

# Per-job pricing: job_index = price_in_wei
[0]
0 = "1000000000000000"    # Job 0 (inference): 0.001 ETH base
```

**Operator config** (`operator.toml`):

```toml
[keystore]
keystore_path = "/keys/operator.k256"    # EIP-712 signing key

[rpc]
rpc_bind_address = "0.0.0.0"
rpc_port = 50051

[quote]
quote_validity_duration_secs = 300       # quotes expire after 5 min
```

**Pricing formula** (from `pricing-engine/src/pricing.rs`):

```
total_cost = Σ (resource_count × price_per_unit_rate × ttl_blocks × block_time)
             × security_adjustment_factor
```

The engine exposes two gRPC endpoints (defined in `pricing.proto`):

- `GetPrice` — service-level quotes (for `createServiceFromQuotes`)
- `GetJobPrice` — per-job quotes (for `submitJobFromQuote`)

Both return EIP-712 signed quotes that customers submit on-chain.

## Step 4: Understand the RFQ Flow

Two RFQ flows exist for different use cases.

### Service-level RFQ (creating a service)

Used when a customer wants to create a long-running service instance with specific operators.

```
Customer                    Pricing Engine              TNT Core
    │                            │                         │
    │── GetPrice(blueprint, ──►  │                         │
    │   ttl, resources)          │                         │
    │                            │                         │
    │◄── SignedQuote ────────────│                         │
    │    (EIP-712 signed)        │                         │
    │                            │                         │
    │── createServiceFromQuotes(quotes[], config) ──────►  │
    │                                                      │
    │   • verifies all EIP-712 signatures                  │
    │   • checks replay protection (digest → used)         │
    │   • sums totalCost from all quotes                   │
    │   • collects payment                                 │
    │   • notifies BSM via onRequest()                     │
    │   • activates service (no approval step)             │
    │                                                      │
    │◄── serviceId ────────────────────────────────────────│
```

**On-chain function** (`QuotesCreate.sol:33-67`):

```solidity
function createServiceFromQuotes(
    uint64 blueprintId,
    Types.SignedQuote[] calldata quotes,
    bytes calldata config,             // opaque blueprint-specific params
    address[] calldata permittedCallers,
    uint64 ttl
) external payable returns (uint64 serviceId)
```

The `config` bytes are passed through to your BSM's `onRequest()` hook. Use them for model selection, context length, or any blueprint-specific parameters. The protocol does not interpret them — define your own schema via the `requestSchema` field in `BlueprintDefinition`.

### Per-job RFQ (within an active service)

Used for EventDriven pricing where each inference request has its own quoted price.

```
Customer                    Pricing Engine              TNT Core
    │                            │                         │
    │── GetJobPrice(service, ──► │                         │
    │   jobIndex)                │                         │
    │                            │                         │
    │◄── SignedJobQuote ─────────│                         │
    │    {serviceId, jobIndex,   │                         │
    │     price, expiry, sig}    │                         │
    │                            │                         │
    │── submitJobFromQuote(serviceId, jobIndex, ────────►  │
    │   inputs, signedQuotes[])                            │
    │                                                      │
    │   • verifies each operator's EIP-712 signature       │
    │   • records per-operator quoted prices                │
    │   • creates JobCall with isRFQ=true                  │
    │                                                      │
    │   (after job completion, each operator gets           │
    │    their individually-quoted price)                   │
```

**On-chain types** (`Types.sol:589-602`):

```solidity
struct JobQuoteDetails {
    uint64 serviceId;
    uint8 jobIndex;
    uint256 price;         // in native token (wei)
    uint64 timestamp;
    uint64 expiry;
}

struct SignedJobQuote {
    JobQuoteDetails details;
    bytes signature;       // EIP-712 signature
    address operator;
}
```

**EIP-712 domain** (shared by all quotes):

```
name:              "TangleQuote"
version:           "1"
chainId:           <current chain>
verifyingContract: <TNT Core diamond address>
```

## Step 5: Integrate Shielded Payments (x402)

For privacy-preserving inference, integrate with the ShieldedCredits contract and x402 protocol. This is orthogonal to RFQ — RFQ determines the price, x402 authorizes the payment.

### SpendAuth flow

```
Client                    Operator HTTP             ShieldedCredits
  │                            │                         │
  │── POST /v1/chat/completions                          │
  │   + SpendAuth {                                      │
  │       commitment,   ◄── privacy: identifies account  │
  │       amount,       ◄── price from RFQ or BSM       │
  │       nonce,        ◄── replay protection            │
  │       expiry,                                        │
  │       signature     ◄── EIP-712 signed               │
  │     }                                                │
  │                            │                         │
  │   (operator validates      │                         │
  │    EIP-712 signature,      │                         │
  │    checks nonce + expiry   │                         │
  │    + credit balance)       │                         │
  │                            │                         │
  │◄── inference response ─────│                         │
  │                            │                         │
  │                            │── authorizeSpend(auth)──►│
  │                            │◄── authHash ────────────│
  │                            │── claimPayment(hash) ──►│
  │                            │                         │
```

The operator serves inference **before** on-chain settlement. This is a deliberate design choice for latency — verification is fast (local ecrecover), settlement is async.

### Implementation

Validate the SpendAuth on every request:

```rust
// 1. Recover signer from EIP-712 signature
let signer = ecrecover_typed_data(&spend_auth)?;

// 2. Check the commitment maps to a valid credit account
let account = shielded_credits.getAccount(spend_auth.commitment).call().await?;
if account.spending_key != signer { return Err(Unauthorized) }

// 3. Check balance covers the request
if account.balance < spend_auth.amount { return Err(InsufficientBalance) }

// 4. Check nonce hasn't been used (replay protection)
if nonce_store.is_used(spend_auth.nonce) { return Err(NonceReused) }
nonce_store.mark_used(spend_auth.nonce);

// 5. Check expiry
if now > spend_auth.expiry { return Err(Expired) }

// 6. Serve inference...

// 7. Async: settle on-chain
tokio::spawn(async move {
    let auth_hash = shielded_credits.authorizeSpend(auth).call().await?;
    shielded_credits.claimPayment(auth_hash, operator_address).call().await?;
});
```

If no SpendAuth is provided, return a 402 response with payment requirements:

```
HTTP/1.1 402 Payment Required
X-Payment-Required: true
X-Payment-Contract: 0x...
X-Payment-Amount: <cost estimate>
X-Payment-Token: tsUSD
X-Payment-Chain-Id: 3799
```

## Step 6: Deploy and Register

### Prerequisites

1. TNT Core deployed on-chain
2. BSM contract deployed and linked to blueprint
3. Model(s) configured via `configureModel()`
4. Operator registered in Tangle staking (before blueprint registration)

### Registration order matters

```bash
# 1. Deploy BSM contract
forge create InferenceBSM --constructor-args $TANGLE_CORE

# 2. Create blueprint (BSM + sources + binaries array MUST not be empty)
cast send $TANGLE_CORE "createBlueprint(BlueprintDefinition)" $DEFINITION

# 3. Configure models on BSM (BEFORE operator registration)
cast send $BSM "configureModel(string,uint32,uint64,uint64,uint32)" \
    "meta-llama/Llama-3.1-8B-Instruct" 8192 1 2 16000

# 4. Register operator in staking (BEFORE registering for blueprint)
# ... staking registration ...

# 5. Register operator for blueprint
cast send $TANGLE_CORE "registerOperator(uint64,bytes)" \
    $BLUEPRINT_ID $REGISTRATION_PAYLOAD
```

**Common errors:**
- `BlueprintBinaryRequired` — the binaries array in sources must not be empty
- `ModelNotSupported` — call `configureModel()` before operator registration
- `OperatorNotActive` — register in staking before registering for blueprint
- `InsufficientGpuCapability` — operator's declared VRAM is below model's `minGpuVramMib`

### Local testing with Anvil

```bash
# Start anvil with increased limits (TNT Core contracts are large)
anvil --gas-limit 1000000000 --code-size-limit 100000

# Deploy with --skip-simulation (contract sizes exceed default)
forge script Deploy.s.sol --skip-simulation --broadcast

# Delete broadcast cache between anvil restarts
rm -rf broadcast/
```

## Putting It Together

The full lifecycle:

```
1. Blueprint owner deploys BSM + creates blueprint on TNT Core
2. Blueprint owner configures models (pricing, GPU requirements)
3. Operator registers in staking, then registers for blueprint
4. BlueprintRunner detects ServiceInitialized → starts background service
5. Operator's HTTP server starts accepting /v1/chat/completions
6. Customer sends request with SpendAuth (x402 path)
   — OR submits on-chain job via submitJobFromQuote (RFQ path)
7. Operator serves inference, settles payment asynchronously
```

## Reference

| Component | Location | Purpose |
|-----------|----------|---------|
| TNT Core Types | `tnt-core/src/libraries/Types.sol` | PricingModel, QuoteDetails, JobQuoteDetails |
| Quote creation | `tnt-core/src/core/QuotesCreate.sol` | `createServiceFromQuotes()` |
| Job RFQ | `tnt-core/src/core/JobsRFQ.sol` | `submitJobFromQuote()` |
| EIP-712 verification | `tnt-core/src/libraries/SignatureLib.sol` | `verifyAndMarkQuoteUsed()` |
| BSM base | `tnt-core/src/interfaces/IBlueprintServiceManager.sol` | Lifecycle hooks |
| Pricing docs | `tnt-core/docs/PRICING.md` | Payment models reference |
| Pricing engine | `blueprint-sdk/crates/pricing-engine/` | gRPC sidecar for quote signing |
| EIP-712 signer | `blueprint-sdk/crates/pricing-engine/src/signer.rs` | `sign_quote()`, `sign_job_quote()` |
| Job quotes (Rust) | `blueprint-sdk/crates/tangle-extra/src/job_quote.rs` | `JobQuoteSigner` |
| x402 registry | `blueprint-sdk/crates/x402/` | Quote tracking + settlement |
| vLLM BSM | `vllm-inference-blueprint/contracts/src/InferenceBSM.sol` | Reference BSM implementation |
| vLLM operator | `vllm-inference-blueprint/operator/src/` | Reference operator binary |

## Related

- [Pricing Engine README](../crates/pricing-engine/README.md) — TOML config, gRPC API, security
- [x402 Blueprint Example](../examples/x402-blueprint/README.md) — Minimal x402 integration
- [Hello Tangle Example](../examples/hello-tangle/README.md) — Minimal job handler
- [TNT Core PRICING.md](https://github.com/tangle-network/tnt-core/blob/main/docs/PRICING.md) — Payment model deep dive
