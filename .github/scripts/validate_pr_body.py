#!/usr/bin/env python3
"""Validate pull request description against repository quality gates."""

from __future__ import annotations

import argparse
import json
import os
import re
import sys
from pathlib import Path


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
    return parser.parse_args()


def load_body(args: argparse.Namespace) -> str:
    if args.body_file:
        return Path(args.body_file).read_text(encoding="utf-8")

    if not args.event_path:
        raise ValueError("No --body-file provided and GITHUB_EVENT_PATH is unset.")

    payload = json.loads(Path(args.event_path).read_text(encoding="utf-8"))
    pull_request = payload.get("pull_request", {})
    title = (pull_request.get("title") or "").strip()
    body = pull_request.get("body") or ""

    if SKIP_TOKEN in title or SKIP_TOKEN in body:
        print(f"Skipping PR body validation due to token: {SKIP_TOKEN}")
        sys.exit(0)

    return body


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
        if normalized == "```bash" or normalized == "```":
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


def is_docs_or_process_only(path: str) -> bool:
    if path.startswith("docs/"):
        return True
    if path.startswith(".github/"):
        return True
    if path in {"README.md", "CONTRIBUTING.md", "CLAUDE.md"}:
        return True
    if path.endswith(".md"):
        return True
    return False


def infer_required_class(changed_files: list[str]) -> tuple[int, str]:
    if not changed_files:
        return (1, "No changed files list provided; defaulting to Class A.")

    if all(is_docs_or_process_only(path) for path in changed_files):
        return (1, "Docs/process-only changes.")

    class_d_prefixes = (
        "crates/manager/src/protocol/",
        "crates/manager/src/rt/container/",
        "crates/manager/src/sources/",
        "crates/clients/tangle/src/",
        "crates/tee/src/",
        "cli/src/command/deploy/",
    )
    for path in changed_files:
        if path.startswith(class_d_prefixes):
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

    if len(crates_touched) > 1:
        return (3, "Multiple crates touched; cross-crate behavior likely.")
    if touches_cli and crates_touched:
        return (3, "CLI + crate changes touched; cross-boundary behavior likely.")

    return (2, "Code changes detected with local blast radius.")


def validate_inferred_change_class(args: argparse.Namespace, selected_class: str | None) -> list[str]:
    errors: list[str] = []
    selected_rank = parse_change_class_rank(selected_class)
    if selected_rank is None:
        return errors

    changed_files = load_changed_files(args)
    required_rank, reason = infer_required_class(changed_files)
    if selected_rank < required_rank:
        errors.append(
            "Selected class is too low for changed files. "
            f"Required minimum: Class {'ABCD'[required_rank - 1]}. Reason: {reason}"
        )
    return errors


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


def main() -> int:
    args = parse_args()
    try:
        body = load_body(args)
    except ValueError as exc:
        print(f"ERROR: {exc}")
        return 1
    except FileNotFoundError as exc:
        print(f"ERROR: Cannot read input file: {exc}")
        return 1

    if not body.strip():
        print("ERROR: Pull request body is empty.")
        return 1

    sections = split_sections(body)
    errors = validate_sections(sections, body)

    change_class_section = sections.get("change class", "")
    selected_class = extract_change_class(change_class_section)
    errors.extend(validate_change_class(change_class_section))
    errors.extend(validate_inferred_change_class(args, selected_class))

    verification = sections.get("verification", "")
    harness_evidence = sections.get("harness evidence", "")
    errors.extend(validate_verification(verification, selected_class))
    errors.extend(validate_harness_evidence(harness_evidence, selected_class))

    if errors:
        print("PR description validation failed:")
        for error in errors:
            print(f"- {error}")
        print(f"To bypass in emergency only, include {SKIP_TOKEN} in PR title/body.")
        return 1

    print("PR description validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
