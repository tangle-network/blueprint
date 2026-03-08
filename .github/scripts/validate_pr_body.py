#!/usr/bin/env python3
"""Validate pull request description against repository quality gates."""

from __future__ import annotations

import argparse
import fnmatch
import json
import os
import re
import sys
import tomllib
from pathlib import Path
from typing import Any


REQUIRED_SECTIONS = [
    "summary",
    "change class",
    "behavior contract",
    "risk and scope",
    "verification",
    "harness evidence",
    "checklist",
]

PLACEHOLDER_LINES = {
    "- What changed and why?",
    "- Which user/operator/developer flow is affected?",
    "- Selected class:",
    "- Why this class:",
    "- Current behavior:",
    "- Intended behavior:",
    "- Invariants that must hold:",
    "- Failure mode choice (`fail-closed` or `fail-open`) and rationale:",
    "- Security impact:",
    "- Compatibility impact (APIs, metadata format, configs):",
    "- Migration notes (if any):",
    "- Rollback plan:",
    "- Reproducer added/updated:",
    "- Negative-path coverage added:",
    "- Key assertions that prove the fix:",
}

DOCS_ONLY_HINTS = ("docs only", "docs-only", "documentation only")
SKIP_TOKEN = "[skip-pr-quality-gate]"

DEFAULT_POLICY: dict[str, Any] = {
    "docs_only_patterns": [
        "docs/**",
        ".github/**",
        "README.md",
        "CONTRIBUTING.md",
        "CLAUDE.md",
        "*.md",
    ],
    "class_d_prefixes": [
        "crates/manager/src/protocol/",
        "crates/manager/src/rt/container/",
        "crates/manager/src/sources/",
        "crates/clients/tangle/src/",
        "crates/tee/src/",
        "cli/src/command/deploy/",
    ],
    "class_d_patterns": [],
    "class_c_multi_crate": True,
    "class_c_cli_and_crate": True,
}


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description="Validate PR body quality gates.")
    parser.add_argument(
        "--event-path",
        default=os.getenv("GITHUB_EVENT_PATH"),
        help="Path to GitHub event payload JSON.",
    )
    parser.add_argument(
        "--body-file",
        help="Path to file containing PR body text (for local testing).",
    )
    parser.add_argument(
        "--changed-files-file",
        help="Path to file containing newline-delimited changed file paths.",
    )
    parser.add_argument(
        "--config-file",
        default=".github/pr-quality-gate.toml",
        help="Policy configuration file.",
    )
    parser.add_argument(
        "--report-file",
        help="Optional output JSON report path.",
    )
    return parser.parse_args()


def write_report(path: str | None, report: dict[str, Any]) -> None:
    if not path:
        return
    target = Path(path)
    target.parent.mkdir(parents=True, exist_ok=True)
    target.write_text(json.dumps(report, indent=2, sort_keys=True), encoding="utf-8")


def load_pr_body(args: argparse.Namespace) -> tuple[str, bool]:
    if args.body_file:
        body = Path(args.body_file).read_text(encoding="utf-8")
        return (body, SKIP_TOKEN in body)

    if not args.event_path:
        raise ValueError("No --body-file provided and GITHUB_EVENT_PATH is unset.")

    payload = json.loads(Path(args.event_path).read_text(encoding="utf-8"))
    pull_request = payload.get("pull_request", {})
    title = (pull_request.get("title") or "").strip()
    body = pull_request.get("body") or ""
    skipped = SKIP_TOKEN in title or SKIP_TOKEN in body
    return (body, skipped)


def split_sections(body: str) -> dict[str, str]:
    matches = list(re.finditer(r"^##\s+(.+?)\s*$", body, flags=re.MULTILINE))
    sections: dict[str, str] = {}
    if not matches:
        return sections

    for idx, match in enumerate(matches):
        section_name = match.group(1).strip().lower()
        start = match.end()
        end = matches[idx + 1].start() if idx + 1 < len(matches) else len(body)
        section_text = body[start:end].strip()
        sections[section_name] = section_text

    return sections


def has_meaningful_content(content: str) -> bool:
    for line in content.splitlines():
        normalized = line.strip()
        if not normalized:
            continue
        if normalized in {"```bash", "```"}:
            continue
        return True
    return False


def validate_sections(sections: dict[str, str], body: str) -> list[str]:
    errors: list[str] = []

    for required in REQUIRED_SECTIONS:
        if required not in sections:
            errors.append(f"Missing required section: '## {required.title()}'")
            continue
        if not has_meaningful_content(sections[required]):
            errors.append(f"Section is empty: '## {required.title()}'")

    for line in body.splitlines():
        normalized = line.strip()
        if normalized in PLACEHOLDER_LINES:
            errors.append(f"Template placeholder still present: '{normalized}'")

    checklist = sections.get("checklist", "")
    unchecked = re.findall(r"^- \[ \].*$", checklist, flags=re.MULTILINE)
    if unchecked:
        errors.append("Checklist has unchecked items. Complete or mark them explicitly.")

    return errors


def extract_change_class(change_class_section: str) -> str | None:
    match = re.search(r"selected class:\s*(.+)$", change_class_section, flags=re.IGNORECASE | re.MULTILINE)
    if not match:
        return None
    return match.group(1).strip().lower()


def parse_change_class_rank(selected_class: str | None) -> int | None:
    if not selected_class:
        return None
    normalized = selected_class.strip().lower()
    if normalized.startswith("class a"):
        return 1
    if normalized.startswith("class b"):
        return 2
    if normalized.startswith("class c"):
        return 3
    if normalized.startswith("class d"):
        return 4
    return None


def rank_to_class(rank: int | None) -> str | None:
    if rank is None:
        return None
    if rank < 1 or rank > 4:
        return None
    return f"Class {'ABCD'[rank - 1]}"


def validate_change_class(change_class_section: str) -> list[str]:
    errors: list[str] = []
    selected = extract_change_class(change_class_section)
    if not selected:
        errors.append("Change Class section must specify 'Selected class: ...'")
        return errors

    valid_prefixes = ("class a", "class b", "class c", "class d")
    if not selected.startswith(valid_prefixes):
        errors.append("Selected class must start with one of: Class A, Class B, Class C, Class D.")
    return errors


def load_changed_files(args: argparse.Namespace) -> list[str]:
    if not args.changed_files_file:
        return []
    raw = Path(args.changed_files_file).read_text(encoding="utf-8")
    return [line.strip() for line in raw.splitlines() if line.strip()]


def load_policy_config(config_file: str | None) -> dict[str, Any]:
    policy = dict(DEFAULT_POLICY)
    if not config_file:
        return policy

    path = Path(config_file)
    if not path.exists():
        return policy

    parsed = tomllib.loads(path.read_text(encoding="utf-8"))
    classification = parsed.get("classification", {})
    if not isinstance(classification, dict):
        return policy

    docs_patterns = classification.get("docs_only_patterns")
    if isinstance(docs_patterns, list) and all(isinstance(x, str) for x in docs_patterns):
        policy["docs_only_patterns"] = docs_patterns

    class_d_prefixes = classification.get("class_d_prefixes")
    if isinstance(class_d_prefixes, list) and all(isinstance(x, str) for x in class_d_prefixes):
        policy["class_d_prefixes"] = class_d_prefixes

    class_d_patterns = classification.get("class_d_patterns")
    if isinstance(class_d_patterns, list) and all(isinstance(x, str) for x in class_d_patterns):
        policy["class_d_patterns"] = class_d_patterns

    class_c_multi_crate = classification.get("class_c_multi_crate")
    if isinstance(class_c_multi_crate, bool):
        policy["class_c_multi_crate"] = class_c_multi_crate

    class_c_cli_and_crate = classification.get("class_c_cli_and_crate")
    if isinstance(class_c_cli_and_crate, bool):
        policy["class_c_cli_and_crate"] = class_c_cli_and_crate

    return policy


def matches_any(path: str, patterns: list[str]) -> bool:
    return any(fnmatch.fnmatch(path, pattern) for pattern in patterns)


def is_docs_or_process_only(path: str, policy: dict[str, Any]) -> bool:
    patterns: list[str] = policy["docs_only_patterns"]
    return matches_any(path, patterns)


def infer_required_class(changed_files: list[str], policy: dict[str, Any]) -> tuple[int, str]:
    if not changed_files:
        return (1, "No changed files list provided; defaulting to Class A.")

    if all(is_docs_or_process_only(path, policy) for path in changed_files):
        return (1, "Docs/process-only changes.")

    class_d_prefixes: list[str] = policy["class_d_prefixes"]
    class_d_patterns: list[str] = policy["class_d_patterns"]

    for path in changed_files:
        if path.startswith(tuple(class_d_prefixes)) or matches_any(path, class_d_patterns):
            return (4, f"High-risk protocol/security/runtime path changed: {path}")

    crates_touched: set[str] = set()
    touches_cli = False
    for path in changed_files:
        if path.startswith("crates/"):
            parts = path.split("/")
            if len(parts) >= 2:
                crates_touched.add(parts[1])
        if path.startswith("cli/"):
            touches_cli = True

    if policy["class_c_multi_crate"] and len(crates_touched) > 1:
        return (3, "Multiple crates touched; cross-crate behavior likely.")
    if policy["class_c_cli_and_crate"] and touches_cli and crates_touched:
        return (3, "CLI + crate changes touched; cross-boundary behavior likely.")

    return (2, "Code changes detected with local blast radius.")


def validate_inferred_change_class(
    args: argparse.Namespace,
    selected_class: str | None,
    policy: dict[str, Any],
) -> tuple[list[str], int, str]:
    errors: list[str] = []
    selected_rank = parse_change_class_rank(selected_class)
    changed_files = load_changed_files(args)
    required_rank, reason = infer_required_class(changed_files, policy)

    if selected_rank is not None and selected_rank < required_rank:
        errors.append(
            "Selected class is too low for changed files. "
            f"Required minimum: {rank_to_class(required_rank)}. Reason: {reason}"
        )
    return (errors, required_rank, reason)


def validate_verification(verification: str, selected_class: str | None) -> list[str]:
    errors: list[str] = []
    text = verification.lower()
    has_command = bool(re.search(r"`[^`\n]+`", verification) or "```" in verification)
    docs_only = any(hint in text for hint in DOCS_ONLY_HINTS)

    if selected_class and selected_class.startswith("class a"):
        if not (has_command or docs_only):
            errors.append("Verification section must include commands or an explicit docs-only statement for Class A.")
        return errors

    if not has_command:
        errors.append("Verification section must include at least one command (inline or fenced).")
    return errors


def validate_harness_evidence(harness_evidence: str, selected_class: str | None) -> list[str]:
    errors: list[str] = []
    text = harness_evidence.lower()
    docs_only = any(hint in text for hint in DOCS_ONLY_HINTS)

    if selected_class and selected_class.startswith("class a"):
        if not has_meaningful_content(harness_evidence):
            errors.append("Harness Evidence section cannot be empty.")
        return errors

    if docs_only:
        errors.append("Harness Evidence cannot be docs-only for Class B/C/D changes.")
    return errors


def build_report(
    *,
    valid: bool,
    skipped: bool,
    selected_class: str | None,
    selected_rank: int | None,
    required_rank: int | None,
    required_reason: str,
    changed_files: list[str],
    errors: list[str],
) -> dict[str, Any]:
    return {
        "valid": valid,
        "skipped": skipped,
        "selected_class_raw": selected_class,
        "selected_class_normalized": rank_to_class(selected_rank),
        "required_class": rank_to_class(required_rank),
        "required_reason": required_reason,
        "changed_files_count": len(changed_files),
        "changed_files_sample": changed_files[:25],
        "errors": errors,
    }


def main() -> int:
    args = parse_args()
    policy = load_policy_config(args.config_file)
    changed_files = load_changed_files(args)

    try:
        body, skipped = load_pr_body(args)
    except ValueError as exc:
        report = build_report(
            valid=False,
            skipped=False,
            selected_class=None,
            selected_rank=None,
            required_rank=None,
            required_reason="Unable to load PR body.",
            changed_files=changed_files,
            errors=[f"ERROR: {exc}"],
        )
        write_report(args.report_file, report)
        print(f"ERROR: {exc}")
        return 1
    except FileNotFoundError as exc:
        report = build_report(
            valid=False,
            skipped=False,
            selected_class=None,
            selected_rank=None,
            required_rank=None,
            required_reason="Unable to read input file.",
            changed_files=changed_files,
            errors=[f"ERROR: Cannot read input file: {exc}"],
        )
        write_report(args.report_file, report)
        print(f"ERROR: Cannot read input file: {exc}")
        return 1

    if skipped:
        report = build_report(
            valid=True,
            skipped=True,
            selected_class=None,
            selected_rank=None,
            required_rank=None,
            required_reason=f"Skipped by token: {SKIP_TOKEN}",
            changed_files=changed_files,
            errors=[],
        )
        write_report(args.report_file, report)
        print(f"Skipping PR body validation due to token: {SKIP_TOKEN}")
        return 0

    if not body.strip():
        errors = ["ERROR: Pull request body is empty."]
        report = build_report(
            valid=False,
            skipped=False,
            selected_class=None,
            selected_rank=None,
            required_rank=None,
            required_reason="PR body is empty.",
            changed_files=changed_files,
            errors=errors,
        )
        write_report(args.report_file, report)
        print(errors[0])
        return 1

    sections = split_sections(body)
    errors = validate_sections(sections, body)

    change_class_section = sections.get("change class", "")
    selected_class = extract_change_class(change_class_section)
    selected_rank = parse_change_class_rank(selected_class)
    errors.extend(validate_change_class(change_class_section))

    class_errors, required_rank, required_reason = validate_inferred_change_class(args, selected_class, policy)
    errors.extend(class_errors)

    verification = sections.get("verification", "")
    harness_evidence = sections.get("harness evidence", "")
    errors.extend(validate_verification(verification, selected_class))
    errors.extend(validate_harness_evidence(harness_evidence, selected_class))

    valid = not errors
    report = build_report(
        valid=valid,
        skipped=False,
        selected_class=selected_class,
        selected_rank=selected_rank,
        required_rank=required_rank,
        required_reason=required_reason,
        changed_files=changed_files,
        errors=errors,
    )
    write_report(args.report_file, report)

    if valid:
        print("PR description validation passed.")
        return 0

    print("PR description validation failed:")
    for error in errors:
        print(f"- {error}")
    print(f"To bypass in emergency only, include {SKIP_TOKEN} in PR title/body.")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
