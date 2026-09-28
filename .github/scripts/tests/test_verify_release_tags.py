from __future__ import annotations

import importlib.util
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


INDEX_BODY = "\n".join(
    json.dumps(entry)
    for entry in [
        {"name": "blueprint-std", "vers": "0.1.0", "yanked": False},
        {"name": "blueprint-std", "vers": "0.2.0", "yanked": False},
        {"name": "blueprint-std", "vers": "0.3.0", "yanked": True},
    ]
)


class SparseIndexPathTests(unittest.TestCase):
    def test_one_character_name(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("a"), "1/a")

    def test_two_character_name(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("ab"), "2/ab")

    def test_three_character_name(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("abc"), "3/a/abc")

    def test_long_name(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("blueprint-sdk"), "bl/ue/blueprint-sdk")

    def test_underscores_become_hyphens(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("my_crate"), "my/-c/my-crate")

    def test_case_is_normalized(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("MyCrate"), "my/cr/mycrate")

    def test_case_and_underscores_together(self) -> None:
        self.assertEqual(MODULE.sparse_index_path("My_Crate"), "my/-c/my-crate")


class ParseIndexTests(unittest.TestCase):
    def test_parses_versions_and_yanked_flags(self) -> None:
        self.assertEqual(
            MODULE.parse_index(INDEX_BODY),
            {"0.1.0": False, "0.2.0": False, "0.3.0": True},
        )

    def test_blank_lines_are_ignored(self) -> None:
        self.assertEqual(MODULE.parse_index("\n\n" + INDEX_BODY + "\n\n")["0.2.0"], False)

    def test_truncated_trailing_line_is_skipped_not_fatal(self) -> None:
        # A half-written shard must not read as "this crate is unpublished".
        truncated = INDEX_BODY + '\n{"name": "blueprint-std", "vers": "0.4'
        parsed = MODULE.parse_index(truncated)
        self.assertIn("0.2.0", parsed)
        self.assertNotIn("0.4", parsed)

    def test_entry_without_version_is_ignored(self) -> None:
        self.assertEqual(MODULE.parse_index('{"name": "x"}'), {})


class UnresolvedTests(unittest.TestCase):
    def setUp(self) -> None:
        self.versions = {"blueprint-std": "0.2.0", "blueprint-sdk": "1.0.0"}

    def test_all_live_returns_no_problems(self) -> None:
        shards = {"blueprint-std": MODULE.parse_index(INDEX_BODY), "blueprint-sdk": {"1.0.0": False}}
        self.assertEqual(MODULE.unresolved(["blueprint-std", "blueprint-sdk"], self.versions, shards.get), [])

    def test_tag_exists_but_crate_absent_is_reported(self) -> None:
        # The #1465 wedge: tag exists, crates.io never got the version.
        def fetch(name: str):
            return MODULE.parse_index(INDEX_BODY) if name == "blueprint-std" else None

        problems = MODULE.unresolved(["blueprint-std", "blueprint-sdk"], self.versions, fetch)
        self.assertEqual(problems, [("blueprint-sdk", "1.0.0", "not on crates.io")])

    def test_crate_present_but_wrong_version_is_reported(self) -> None:
        problems = MODULE.unresolved(
            ["blueprint-std"], {"blueprint-std": "0.9.9"}, lambda name: MODULE.parse_index(INDEX_BODY)
        )
        self.assertEqual(problems, [("blueprint-std", "0.9.9", "version absent from sparse index")])

    def test_yanked_version_is_reported(self) -> None:
        problems = MODULE.unresolved(
            ["blueprint-std"], {"blueprint-std": "0.3.0"}, lambda name: MODULE.parse_index(INDEX_BODY)
        )
        self.assertEqual(problems, [("blueprint-std", "0.3.0", "version is yanked")])

    def test_missing_cargo_metadata_version_is_reported(self) -> None:
        problems = MODULE.unresolved(["ghost-crate"], {}, lambda name: None)
        self.assertEqual(problems, [("ghost-crate", "?", "no version in cargo metadata")])


class ReportTests(unittest.TestCase):
    def test_report_names_every_broken_crate_and_its_tag(self) -> None:
        text = MODULE.report([("blueprint-sdk", "1.0.0", "not on crates.io")])
        self.assertIn("blueprint-sdk 1.0.0", text)
        self.assertIn("refs/tags/blueprint-sdk-v1.0.0", text)
        self.assertIn("1 crate(s) tagged but not publishable", text)

    def test_report_omits_tag_command_for_unknown_version(self) -> None:
        text = MODULE.report([("ghost-crate", "?", "no version in cargo metadata")])
        self.assertNotIn("refs/tags/ghost-crate", text)


class MainTests(unittest.TestCase):
    def _release_file(self, tmp: str, names: list[str]) -> str:
        path = Path(tmp) / "release-output.json"
        path.write_text(
            json.dumps([{"package_name": n} for n in names]), encoding="utf-8"
        )
        return str(path)

    def test_empty_release_set_is_a_no_op(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = self._release_file(tmp, [])
            self.assertEqual(MODULE.main([path]), 0)

    def test_verified_release_exits_zero(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = self._release_file(tmp, ["blueprint-std"])
            with mock.patch.object(
                MODULE, "workspace_versions", return_value={"blueprint-std": "0.2.0"}
            ), mock.patch.object(MODULE, "fetch_index", return_value=MODULE.parse_index(INDEX_BODY)):
                self.assertEqual(MODULE.main([path]), 0)

    def test_wedged_release_exits_nonzero(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = self._release_file(tmp, ["blueprint-sdk"])
            with mock.patch.object(
                MODULE, "workspace_versions", return_value={"blueprint-sdk": "1.0.0"}
            ), mock.patch.object(MODULE, "fetch_index", return_value=None):
                self.assertEqual(MODULE.main([path]), 1)


if __name__ == "__main__":
    unittest.main()
