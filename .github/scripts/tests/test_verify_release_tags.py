from __future__ import annotations

import contextlib
import importlib.util
import io
import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock


SCRIPT_PATH = Path(__file__).resolve().parents[1] / "verify_release_tags.py"
SPEC = importlib.util.spec_from_file_location("verify_release_tags", SCRIPT_PATH)
assert SPEC is not None and SPEC.loader is not None
MODULE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(MODULE)


def tagged_only(*names: str):
    """Pretend exactly these workspace crates carry a `<name>-v<version>` tag."""
    return mock.patch.object(
        MODULE,
        "tagged_workspace_names",
        side_effect=lambda versions: [n for n in names if n in versions],
    )


class AuditedNamesTests(unittest.TestCase):
    """A tag with no index entry must be audited even when release-plz skipped it."""

    VERSIONS = {"alpha-crate": "1.0.0", "beta-crate": "2.0.0"}

    def test_skipped_tagged_crate_is_added_to_the_audit_set(self) -> None:
        # release-plz listed only beta-crate; alpha-crate was skipped because its
        # tag already existed, which is how 19 crates stayed unpublished on
        # 2026-07-03 while the run reported success.
        with tagged_only("alpha-crate", "beta-crate"):
            names = MODULE.audited_names(["beta-crate"], self.VERSIONS)
        self.assertEqual(sorted(names), ["alpha-crate", "beta-crate"])

    def test_crate_in_both_sources_is_not_duplicated(self) -> None:
        with tagged_only("beta-crate"):
            names = MODULE.audited_names(["beta-crate"], self.VERSIONS)
        self.assertEqual(names, ["beta-crate"])

    def test_untagged_crates_are_not_audited(self) -> None:
        with tagged_only("beta-crate"):
            names = MODULE.audited_names([], self.VERSIONS)
        self.assertEqual(names, ["beta-crate"])

    def test_empty_release_manifest_still_falls_back_to_tags(self) -> None:
        # The pre-existing fallback behaviour must survive the change.
        with tagged_only("alpha-crate"):
            names = MODULE.audited_names([], self.VERSIONS)
        self.assertEqual(names, ["alpha-crate"])


class MainExitCodeTests(unittest.TestCase):
    """End-to-end on `main()`: an orphan tag has to turn the run red."""

    def test_orphan_tag_fails_the_run_even_when_the_manifest_lists_nothing(self) -> None:
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as handle:
            json.dump({"releases": []}, handle)
            manifest = handle.name

        buffer = io.StringIO()
        with mock.patch.object(MODULE, "workspace_versions", return_value={"alpha-crate": "1.0.0"}), \
             mock.patch.object(MODULE, "fetch_index", return_value={"0.9.0": False}), \
             tagged_only("alpha-crate"), \
             contextlib.redirect_stderr(buffer):
            code = MODULE.main([manifest])

        self.assertEqual(code, 1)
        self.assertIn("alpha-crate 1.0.0", buffer.getvalue())

    def test_published_crate_passes(self) -> None:
        with tempfile.NamedTemporaryFile("w", suffix=".json", delete=False) as handle:
            json.dump({"releases": []}, handle)
            manifest = handle.name

        with mock.patch.object(MODULE, "workspace_versions", return_value={"alpha-crate": "1.0.0"}), \
             mock.patch.object(MODULE, "fetch_index", return_value={"1.0.0": False}), \
             tagged_only("alpha-crate"), \
             contextlib.redirect_stderr(io.StringIO()):
            code = MODULE.main([manifest])

        self.assertEqual(code, 0)


if __name__ == "__main__":
    unittest.main()