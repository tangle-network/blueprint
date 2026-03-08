# Harness Review Checklist

Use this checklist during implementation and review.

## 1. Scope and Contract

- [ ] PR has a declared change class (`A/B/C/D`).
- [ ] Current behavior and intended behavior are explicit.
- [ ] Invariants are explicit.
- [ ] Failure mode choice (`fail-open` or `fail-closed`) is explicit.

## 2. Correctness and Security

- [ ] Deterministic behavior is preserved (ordering, tie-breaks, source selection).
- [ ] Structured input parsing rejects malformed payloads.
- [ ] Legacy payload handling is explicit (mapped, rejected loudly, or migrated).
- [ ] No silent downgrade from stronger policy to weaker policy.

## 3. Test Evidence

- [ ] Reproducer exists for the bug/behavior.
- [ ] Positive-path assertions exist.
- [ ] Negative-path assertions exist.
- [ ] Verification commands and outcomes are listed in the PR.

## 4. Compatibility and Operations

- [ ] API/metadata/config compatibility impact is documented.
- [ ] Migration path is documented.
- [ ] Rollback/containment plan is documented.
- [ ] Operator-facing behavior changes are documented where relevant.

## 5. Audit and Review Discipline

- [ ] Audit findings are classified (`bug`, `tradeoff`, `noise`).
- [ ] Each accepted/rejected finding has evidence.
- [ ] Reviewer can verify claims from PR + tests without assumptions.
