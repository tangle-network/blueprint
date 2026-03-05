## Summary

- What changed and why?
- Which user/operator/developer flow is affected?

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
- [ ] I added or updated tests that reproduce the original issue.
- [ ] I added or updated negative-path tests for invalid/missing/conflicting input.
- [ ] I documented behavior or interface changes in docs/examples when needed.
- [ ] I evaluated compatibility/migration impact explicitly.
- [ ] I validated local formatting, clippy, and relevant tests.
