# Facilitator Blueprint Spec

A decentralized x402 facilitator running as a Tangle Blueprint. Replaces
centralized facilitators (Coinbase CDP, PayAI) with a restaked operator set
that verifies payments and settles them on-chain.

## Problem

Every x402 payment on the internet currently flows through one of two
centralized services. If Coinbase CDP goes down, all x402-protected APIs
stop accepting payments. The facilitator is stateless and deterministic --
there is no reason it needs to be centralized.

## Design

### Jobs

Three jobs, mapping 1:1 to the x402 facilitator HTTP API:

| Job | ID | Input | Output | Type |
|-----|----|-------|--------|------|
| `verify` | 0 | `VerifyRequest` (JSON) | `VerifyResponse` (JSON) | Pure computation |
| `settle` | 1 | `SettleRequest` (JSON) | `SettleResponse` (JSON) | On-chain tx |
| `supported` | 2 | (none) | `SupportedResponse` (JSON) | Static config |

### Verify (job 0)

Pure computation. Every operator in the set produces the same result.

Steps:
1. Decode `paymentPayload` and `paymentRequirements` from the request.
2. Validate payload structure (correct scheme, version, fields).
3. Check `amount >= requirements.amount`.
4. Check `payTo == requirements.payTo`.
5. Check `asset == requirements.asset`.
6. Check `network == requirements.network` (CAIP-2 chain ID match).
7. Recover the signer from the EIP-3009 `transferWithAuthorization` signature.
8. Query the token contract for `balanceOf(signer)` to confirm sufficient funds.
9. Simulate the `transferWithAuthorization` call via `eth_call` to confirm it would succeed.
10. Return `{ isValid: true, payer: signer_address }` or `{ isValid: false, invalidReason: "..." }`.

Balance checks (steps 8-9) require RPC access. All operators reading from the
same block height produce the same result. The blueprint should pin to a
recent finalized block for determinism.

### Settle (job 1)

On-chain transaction. Only one operator submits.

Steps:
1. Run the full verify flow (steps 1-9 above).
2. If invalid, return `{ success: false, errorReason: "..." }`.
3. Build the `transferWithAuthorization(from, to, value, validAfter, validBefore, nonce, v, r, s)` transaction.
4. Submit via the operator's gas wallet.
5. Wait for confirmation (1 block for L2s, configurable for L1).
6. Return `{ success: true, payer, transaction: tx_hash, network }`.

Operator selection for settlement: round-robin among the active set, or the
first operator to respond (race). The operator who settles earns the
facilitator fee.

Gas cost is paid by the operator and recouped from the facilitator fee.
Typical gas for `transferWithAuthorization` on Base: ~60k gas = ~$0.001.
The fee should be configurable and at minimum cover gas + margin.

### Supported (job 2)

Returns static configuration. No computation or chain interaction.

```json
{
  "kinds": [
    { "x402Version": 2, "scheme": "exact", "network": "eip155:8453" },
    { "x402Version": 2, "scheme": "exact", "network": "eip155:1" }
  ],
  "signers": {
    "eip155:8453": ["0xOperator1...", "0xOperator2..."],
    "eip155:1": ["0xOperator1...", "0xOperator2..."]
  }
}
```

The `signers` map lists every operator's address on each supported chain.
Any of them can submit settlement transactions.

### HTTP Gateway

The facilitator blueprint exposes its jobs over HTTP (via the x402 gateway
or a dedicated HTTP layer) to match the standard facilitator API:

```
POST /verify    -> job 0
POST /settle    -> job 1
GET  /supported -> job 2
```

Existing x402 clients and middleware (x402-axum) work unmodified. They just
point `facilitator_url` at the blueprint's HTTP endpoint instead of
`https://x402.org/facilitator`.

## Configuration

```toml
# facilitator.toml

# Chains this facilitator supports.
[[chains]]
network = "eip155:8453"          # Base
rpc_url = "https://mainnet.base.org"
confirmations = 1

[[chains]]
network = "eip155:1"             # Ethereum
rpc_url = "https://eth.llamarpc.com"
confirmations = 2

# Fee charged per settlement transaction (in USD).
# Must cover gas + operator margin.
settlement_fee_usd = "0.002"

# Minimum operator stake required to participate (in TNT).
min_stake = "1000"
```

## Operator Requirements

- **Keys**: ECDSA key on each supported chain (for submitting settlement txs).
- **RPC access**: Archive or full node on each supported chain (for balance
  checks and tx simulation).
- **Stake**: Minimum TNT stake on Tangle (slashable if operator submits
  invalid settlements or censors payments).
- **Gas**: Sufficient native token balance on each chain to pay for
  settlement transactions.

## Security Model

### What operators can do wrong

1. **Submit a settlement for an invalid payment.** The `transferWithAuthorization`
   call will revert on-chain (the token contract validates the signature).
   No funds are lost. The operator wastes gas.

2. **Censor payments (refuse to settle).** Other operators in the set can
   settle instead. Persistent censorship is detectable and slashable.

3. **Return a false `isValid: true` for verification.** The settlement will
   still fail on-chain. The resource server loses nothing because it settles
   before executing (the blueprint x402 gateway uses `settle_before_execution`).

4. **Return a false `isValid: false`.** Denial of service. Detectable by
   having multiple operators verify independently and comparing results.

### Slashing conditions

- Operator signs a `VerifyResponse` with `isValid: true` for a payment that
  fails simulation at the same block height. Provable on-chain via the
  simulation result.
- Operator fails to respond to settlement requests for more than N consecutive
  rounds (liveness failure).

### Trust assumptions

- Verification is deterministic given the same block height. A 2/3 threshold
  of operators agreeing on the result is sufficient.
- Settlement requires exactly one honest operator willing to submit the
  transaction.
- The token contract (USDC, DAI) is trusted to correctly validate
  `transferWithAuthorization` signatures.

## Data Flow

```
Client (pays for API access)
  |
  | HTTP request + Payment-Signature header
  v
Resource Server (x402 gateway, e.g. the x402-blueprint example)
  |
  | POST /settle { paymentPayload, paymentRequirements }
  v
Facilitator Blueprint (Tangle operator set)
  |
  | 1. All operators verify (deterministic)
  | 2. One operator submits transferWithAuthorization on-chain
  | 3. Returns { success, transaction, payer }
  v
Resource Server
  |
  | Payment confirmed. Execute the job.
  v
Client receives result
```

## Dependencies

- `x402-types` for `VerifyRequest`, `SettleRequest`, `VerifyResponse`,
  `SettleResponse`, `SupportedResponse`, `PaymentRequirements`, `PaymentPayload`.
- `x402-chain-eip155` for EVM-specific signature recovery and
  `transferWithAuthorization` encoding.
- `alloy` for RPC calls (`eth_call`, `eth_sendRawTransaction`, balance queries).
- `blueprint-sdk` for the job system, router, runner, and operator key management.
- `blueprint-x402` for the HTTP gateway layer (reuse the same gateway infra).

## Implementation Phases

### Phase 1: Single-operator facilitator

- One operator runs all three jobs.
- No threshold verification (trust the single operator).
- Direct HTTP gateway, no Tangle submission.
- Validate against Coinbase CDP by running both in parallel and comparing results.

### Phase 2: Multi-operator verification

- Multiple operators verify independently.
- 2/3 threshold agreement required before settlement.
- Settlement assigned via round-robin.
- Slashing for liveness failures.

### Phase 3: Full production

- Multi-chain support (Base, Ethereum, Polygon, Arbitrum, Solana).
- Dynamic operator set (join/leave via restaking).
- On-chain fee distribution.
- Dashboard showing settlement volume, operator performance, uptime.

## Open Questions

1. **Fee collection mechanism.** Should the facilitator fee be added to the
   settlement amount (operator keeps the difference), or collected separately
   via Tangle's billing system?

2. **Solana support.** The x402 protocol supports Solana via `x402-chain-solana`.
   The facilitator blueprint should support it, but the settlement mechanism
   is different (SPL token transfer authority, not EIP-3009).

3. **Block height pinning for verification.** Should operators agree on a
   specific block height before verifying, or is "latest finalized" sufficient?

4. **Race vs. round-robin for settlement.** Race is faster (first operator to
   settle wins the fee) but wastes gas on duplicate submissions. Round-robin
   is orderly but adds latency if the assigned operator is slow.
