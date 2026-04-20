# x402 Blueprint Tests

## Test tiers

### `x402_gateway.rs` (25 tests, always runs)
HTTP routing, price discovery, 402 responses, settlement options, MPP protocol,
credential verification, replay protection. Uses wiremock for the facilitator.
Fast, no Anvil needed.

### `x402_e2e_anvil.rs` (gated by `ANVIL_E2E=1`)
Real on-chain E2E: Anvil → deploy ERC-20 with EIP-3009 → real facilitator →
real gateway → real payment → real settlement → verify on-chain transfer +
job execution. No mocks. The production payment flow.

## Running

```bash
# Fast tests (wiremock facilitator)
cargo test -p x402-blueprint

# Full E2E (real Anvil, real ERC-20, real settlement)
ANVIL_E2E=1 cargo test -p x402-blueprint -- x402_e2e
```
