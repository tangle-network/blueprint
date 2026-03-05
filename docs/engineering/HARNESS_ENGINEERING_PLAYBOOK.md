# Harness Engineering Playbook

This playbook defines how Blueprint teams turn unclear problems into verified, production-safe changes.

Companion documents:

- `docs/engineering/HARNESS_ENGINEERING_SPEC.md`
- `docs/engineering/HARNESS_REVIEW_CHECKLIST.md`

## Why This Exists

We work in high-risk surfaces:

- on-chain protocol integration
- operator runtime orchestration
- remote deployment and cloud provider behavior
- TEE policy enforcement

For these surfaces, "it compiles" is not enough. We require proof through targeted harnesses and explicit invariants.

## Roles

### Developer

- Defines behavior contract and invariants.
- Builds a minimal failing harness before or alongside the fix.
- Implements fail-closed logic for security-sensitive paths.
- Ships tests and docs with the code change.

### Reviewer

- Validates that the harness actually proves the claim.
- Looks for downgrade paths, silent fallback, and hidden API breaks.
- Challenges missing migration notes and weak test assertions.

### Operator

- Needs deterministic behavior, clear error states, and rollback guidance.
- Must understand whether behavior is required, optional, or best-effort.

## Design Contract (Required for Non-Trivial Changes)

Capture these in PR text:

1. Problem statement: what is wrong today?
2. Intended behavior: what should happen after merge?
3. Invariants: what must never happen?
4. Failure model: fail-open vs fail-closed; why?
5. Blast radius: which crates/roles/flows are affected?
6. Migration: what old data/config/metadata remains in circulation?

## Harness-First Workflow

1. Write or identify a small failing harness.
   - Unit test if logic is local.
   - Integration test if behavior crosses crate boundaries.
   - E2E harness only if protocol/runtime boundaries are involved.
2. Ensure the harness fails for the current bug.
3. Implement the fix.
4. Extend harness with negative-path assertions.
5. Verify with crate-scoped commands first, then workspace checks.

## Checklists

### Correctness Checklist

- [ ] Behavior is deterministic (ordering, source selection, tie-breaks).
- [ ] Input parsing rejects malformed structured payloads.
- [ ] Legacy payload behavior is explicit (supported with mapping, or rejected loudly).
- [ ] Defaults do not silently weaken security or policy.

### Security Checklist

- [ ] Sensitive policy changes are fail-closed where required.
- [ ] No silent downgrade from stricter to weaker execution modes.
- [ ] Runtime capability checks are explicit and auditable.
- [ ] Error messages are actionable and non-ambiguous.

### API and Compatibility Checklist

- [ ] Public API changes are documented.
- [ ] Compatibility aliases or migration notes are provided if needed.
- [ ] Breaking behavior changes are intentional and clearly surfaced.

### Test Checklist

- [ ] Positive-path test(s) added or updated.
- [ ] Negative-path test(s) added for invalid/missing/conflicting inputs.
- [ ] Regression test reproduces the original bug.
- [ ] Commands run are listed in the PR.

### Operations Checklist

- [ ] Operator behavior is documented for success and failure states.
- [ ] Logs/metrics expose enough detail to debug in production.
- [ ] Rollback or safe fallback path is defined when applicable.

## Review Rubric for Automated Audits

Treat AI/static audit output as input, not truth:

1. Classify each finding:
   - Real bug/regression
   - Design tradeoff
   - Out-of-scope/noise
2. Require concrete evidence:
   - file/line references
   - reproducer or failing test
   - explicit impact statement
3. Respond with one of:
   - fixed
   - accepted with rationale
   - rejected with evidence

## Definition of Done

A change is done when:

- behavior contract is explicit,
- tests prove both happy-path and negative-path behavior,
- operational impacts are documented,
- reviewers can verify claims without reverse engineering intent.
