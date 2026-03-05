from __future__ import annotations

import importlib.util
import tempfile
import unittest
from pathlib import Path


SCRIPT_PATH = Path(__file__).resolve().parents[1] / "validate_pr_body.py"
SPEC = importlib.util.spec_from_file_location("validate_pr_body", SCRIPT_PATH)
MODULE = importlib.util.module_from_spec(SPEC)
assert SPEC is not None and SPEC.loader is not None
SPEC.loader.exec_module(MODULE)


class ValidatePrBodyTests(unittest.TestCase):
    def test_infer_class_a_for_docs_only_changes(self) -> None:
        changed = ["docs/engineering/HARNESS_ENGINEERING_SPEC.md", "README.md", ".github/workflows/ci.yml"]
        policy = MODULE.load_policy_config(None)
        required, reason = MODULE.infer_required_class(changed, policy)
        self.assertEqual(required, 1)
        self.assertIn("Docs/process-only", reason)

    def test_infer_class_d_for_protocol_paths(self) -> None:
        changed = ["crates/manager/src/protocol/tangle/metadata.rs"]
        policy = MODULE.load_policy_config(None)
        required, reason = MODULE.infer_required_class(changed, policy)
        self.assertEqual(required, 4)
        self.assertIn("High-risk", reason)

    def test_infer_class_c_for_cross_crate_changes(self) -> None:
        changed = ["crates/manager/src/lib.rs", "crates/clients/src/lib.rs"]
        policy = MODULE.load_policy_config(None)
        required, reason = MODULE.infer_required_class(changed, policy)
        self.assertEqual(required, 3)
        self.assertIn("Multiple crates", reason)

    def test_load_policy_config_from_toml(self) -> None:
        content = """
[classification]
docs_only_patterns = ["docs/**"]
class_d_prefixes = ["crates/custom/"]
class_d_patterns = ["special/**"]
class_c_multi_crate = false
class_c_cli_and_crate = false
""".strip()
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "policy.toml"
            path.write_text(content, encoding="utf-8")
            policy = MODULE.load_policy_config(str(path))
            self.assertEqual(policy["class_d_prefixes"], ["crates/custom/"])
            self.assertEqual(policy["class_d_patterns"], ["special/**"])
            self.assertFalse(policy["class_c_multi_crate"])
            self.assertFalse(policy["class_c_cli_and_crate"])


if __name__ == "__main__":
    unittest.main()
