# Harness Engineering Spec

This spec defines the engineering operating model for high-risk changes in Blueprint.

## Objectives

1. Detect regressions before merge with targeted harnesses.
2. Minimize silent downgrades and fail-open behavior in security-sensitive paths.
3. Preserve velocity by matching verification depth to risk tier.
4. Make code review evidence-based instead of preference-based.

## Design Principles

1. Harnesses are product infrastructure, not temporary test code.
2. Evals and harnesses are a first-class competency.
3. Optimize for entropy: code, metadata, and environments drift over time.
4. Favor explicit contracts over implied behavior.
5. Prefer fail-closed semantics for security and policy constraints.

## Role Responsibilities

### Developer

- Declare behavior contract in PR.
- Add or update reproducer harnesses.
- Provide negative-path coverage.
- Document compatibility and migration outcomes.

### Reviewer

- Validate behavior contract against implementation and tests.
- Reject silent downgrade paths.
- Require evidence for compatibility assumptions.
- Classify audit findings with explicit disposition.

### Operator

- Validate runtime assumptions and observability.
- Confirm rollback or containment path exists.
- Ensure operational docs remain accurate.

## Change Classes

Use one class per PR:

- `Class A`: docs/tooling only, no runtime or API behavior change.
- `Class B`: single-crate behavior changes with local blast radius.
- `Class C`: cross-crate/runtime behavior changes (manager/source/runtime/protocol glue).
- `Class D`: protocol, security, on-chain metadata semantics, or policy enforcement changes.

## Required Evidence Matrix

| Requirement | Class A | Class B | Class C | Class D |
| --- | --- | --- | --- | --- |
| Behavior contract in PR | Optional | Required | Required | Required |
| Reproducer test/harness | Optional | Required | Required | Required |
| Negative-path assertions | Optional | Required | Required | Required |
| Compatibility/migration analysis | Optional | Recommended | Required | Required |
| Rollback/containment notes | Optional | Recommended | Required | Required |
| Targeted crate tests listed | Optional | Required | Required | Required |

## Automation

Quality gates are enforced automatically in CI:

- PR body/checklist + class validation: `.github/scripts/validate_pr_body.py`
- Classification policy config: `.github/pr-quality-gate.toml`
- Workflow and PR summary comment: `.github/workflows/pr-quality-gate.yml`

## PR Contract Fields

Every non-draft PR should include:

1. Summary.
2. Change class.
3. Behavior contract.
4. Risk and scope.
5. Verification commands and outcomes.
6. Harness evidence.
7. Checklist completion.

## Verification Policy

1. Start with minimal crate-scoped checks that exercise changed behavior.
2. Add cross-crate checks when behavior crosses boundaries.
3. Promote to workspace checks for broad or high-risk refactors.
4. Explicitly state skipped checks and why.

## Compatibility Policy

When parsing or semantics change:

1. Define legacy payload behavior explicitly.
2. Avoid silent defaulting that weakens security or policy.
3. If behavior is breaking, surface it as intentional and documented.
4. Tie migration plan to concrete code paths and owners.

## Audit Triage Standard

For each human/AI audit finding:

1. Categorize as `bug`, `tradeoff`, or `noise`.
2. Provide evidence (file reference, test, or reproducer).
3. Record resolution: `fixed`, `accepted-with-rationale`, or `rejected-with-evidence`.

## Exit Criteria

A PR meets this spec when:

1. Required matrix evidence exists for its class.
2. Reviewer can validate claims without reverse engineering intent.
3. Risks and migration paths are explicit and test-backed.
