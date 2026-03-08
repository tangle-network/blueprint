## Summary

- What changed and why?
- Which user/operator/developer flow is affected?

## Change Class

- `Class A` (docs/tooling only)
- `Class B` (single-crate behavior)
- `Class C` (cross-crate/runtime behavior)
- `Class D` (protocol/security/metadata semantics)
- Selected class:
- Why this class:

## Behavior Contract

- Current behavior:
- Intended behavior:
- Invariants that must hold:
- Failure mode choice (`fail-closed` or `fail-open`) and rationale:

## Risk and Scope

- Security impact:
- Compatibility impact (APIs, metadata format, configs):
- Migration notes (if any):
- Rollback plan:

## Verification

List the exact commands you ran and outcomes.

```bash
# Example:
# cargo test -p blueprint-manager
```

## Harness Evidence

- Reproducer added/updated:
- Negative-path coverage added:
- Key assertions that prove the fix:

## Checklist

- [ ] I followed [`docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md`](/docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md).
- [ ] I followed [`docs/engineering/HARNESS_ENGINEERING_SPEC.md`](/docs/engineering/HARNESS_ENGINEERING_SPEC.md).
- [ ] I used [`docs/engineering/HARNESS_REVIEW_CHECKLIST.md`](/docs/engineering/HARNESS_REVIEW_CHECKLIST.md) while implementing/reviewing.
- [ ] I added or updated tests that reproduce the original issue.
- [ ] I added or updated negative-path tests for invalid/missing/conflicting input.
- [ ] I documented behavior or interface changes in docs/examples when needed.
- [ ] I evaluated compatibility/migration impact explicitly.
- [ ] I validated local formatting, clippy, and relevant tests.
