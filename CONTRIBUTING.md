# Contributing to Blueprint

This document defines how we ship high-confidence changes in this repository.

## Development Setup

1. Install system dependencies.
   ```bash
   # Ubuntu/Debian
   sudo apt update && sudo apt install build-essential cmake libssl-dev pkg-config

   # macOS
   brew install openssl cmake
   ```
2. Clone and enter the repository, then use its Nix development shell or
   install the toolchain and components declared in `rust-toolchain.toml`.
   ```bash
   git clone https://github.com/tangle-network/blueprint.git
   cd blueprint
   nix develop
   ```

## Branches and Commits

- Branch prefixes:
  - `feature/` for features
  - `fix/` for bug fixes
  - `docs/` for docs-only changes
  - `refactor/` for internal code movement
  - `test/` for test-only updates
- Commit format: Conventional Commits (`feat(scope): ...`, `fix(scope): ...`).

## Local Verification

Run these before opening a PR:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --all-features -D warnings
cargo test --workspace --all-features
cargo build --workspace --all-features
```

For focused iteration, use `cargo test -p <crate>`.

## Harness Engineering Standard

Blueprint follows a harness-first workflow for risky changes:

1. Reproduce the problem with the smallest failing harness/test.
2. State invariants and fail-closed behavior before implementing.
3. Patch the code.
4. Prove the fix with targeted tests and negative-path coverage.
5. Document operational impact (operator/customer/developer) when behavior changes.

Use the playbook for full guidance:

- [`docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md`](docs/engineering/HARNESS_ENGINEERING_PLAYBOOK.md)
- [`docs/engineering/HARNESS_ENGINEERING_SPEC.md`](docs/engineering/HARNESS_ENGINEERING_SPEC.md)
- [`docs/engineering/HARNESS_REVIEW_CHECKLIST.md`](docs/engineering/HARNESS_REVIEW_CHECKLIST.md)

## Pull Request Requirements

Every PR must:

1. Fill out the PR template completely.
2. Include verification commands with pass/fail outcomes.
3. Call out risks, migration notes, and behavior changes.
4. Add tests for new logic and regressions.
5. Update docs/examples when interfaces or flows change.

Large changes may require an RFC in `docs/rfcs/`.

## Review Expectations

Reviewers should prioritize:

1. Correctness and security regressions.
2. Backward-compatibility and migration clarity.
3. Test quality (including negative/fail-closed paths).
4. Operational observability and rollback safety.

For questions, open an issue or use Discord: <https://discord.com/invite/cv8EfJu3Tn>.

## SDK and CLI Releases

[Release Plz](.github/workflows/release-plz.yml) owns version PRs, tags and
batched crate publication. Merge the generated version PR through the repository
gate. For recovery, dispatch that same workflow; do not publish crates by hand:

```bash
gh-drew workflow run release-plz.yml --ref main
```

[Release](.github/workflows/release.yml) keeps its CLI tag-push trigger. Release
Plz also dispatches it and records the accepted run URL. An artifact recovery uses
the same workflow with an existing CLI version tag:

```bash
gh-drew workflow run release.yml --ref main -f "tag=cargo-tangle-v<VERSION>"
```

Every job overlays this workflow revision's verifier and `dist-workspace.toml` on
the tag's source. A tag whose tree cannot build under that revision's workflow and
config needs a dispatch from a branch cut before the change. Each tag has one
active and one pending run; a newer dispatch replaces the pending run, whose recorded
URL then reads cancelled. `release-source.json` binds the source and workflow commits
to the planned digests. A complete matching release skips rebuilding; missing assets
rebuild. Unknown GitHub responses, a different recorded source, a tag moved mid-run,
or planned assets with no record, as on releases before #1624, fail closed. Inspect
the named run and artifact receipt before claiming delivery.
