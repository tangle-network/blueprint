# Pursuit: SDK Quality — Principal Eng Standard

Generation: 1
Date: 2026-04-04
Status: building

## Diagnosis

The SDK has excellent core abstractions (job/router/extractor pattern) buried under alpha-quality hygiene. The problems are not architectural — the Tower-inspired design is sound. The problems are execution discipline: panics in hot paths, suppressed lints masking bugs, missing security fundamentals, dead code tolerance.

## Generation 1 Thesis

**Strip the slop, harden the core.** No new features. Remove dead weight, fix security criticals, restore lint discipline, eliminate panics from library code. Less code, fewer crates, tighter contracts.

## Changes (ordered by impact)

### P0 — Security Criticals
1. Fix deterministic RNG in `no_std` crypto — compile_error!, not silent fallback
2. Replace `todo!()` in job dispatch with proper error propagation
3. Fix timing-vulnerable auth comparison with constant-time eq
4. Add `Zeroize` on drop for secret key types

### P1 — Lint Discipline
5. Remove workspace-wide `dead_code = "allow"` and `unused_variables = "allow"`
6. Audit and tighten the 108 suppressed clippy lints — keep only justified ones
7. Fix resulting warnings (dead code removal = bloat reduction for free)

### P2 — Panic Elimination
8. Replace `unwrap()` chains in eigenlayer registration with `?` propagation
9. Replace `panic!()` in keystore remote config with `TryFrom` returning Result
10. Replace `todo!()` in runner config (Symbiotic) with proper error

### P3 — Bloat Reduction
11. Audit and remove dead code surfaced by lint changes
12. Clean up 55 TODO/FIXME — resolve or delete

## Success Criteria
- `cargo clippy -- -D warnings` passes with real lint coverage
- Zero `todo!()`/`panic!()` in non-test library code paths
- Zero `unwrap()` in error-handling paths of runner/registration
- Compile succeeds with no dead_code/unused_variable suppression
