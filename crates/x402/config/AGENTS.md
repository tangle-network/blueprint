# config

## Purpose
Contains the example TOML configuration file for the x402 payment gateway, documenting all operator-configurable settings including bind address, facilitator URL, job invocation policies, and accepted token definitions.

## Contents (one hop)
### Subdirectories
- (none)

### Files
- `x402.example.toml` - Annotated example configuration covering: `bind_address`, `facilitator_url`, `quote_ttl_secs`, `default_invocation_mode`, `[[job_policies]]` entries (public_paid and restricted_paid with auth_mode/tangle_rpc_url/tangle_contract), and `[[accepted_tokens]]` entries (network as CAIP-2, asset address, symbol, decimals, pay_to, rate_per_native_unit, markup_bps)

## Key APIs
- (not applicable -- configuration file only)

## Relationships
- Consumed by `X402Config::from_toml()` in `crates/x402/src/config.rs`
- Documents the schema for `X402Config`, `JobPolicyConfig`, and `AcceptedToken` structs
