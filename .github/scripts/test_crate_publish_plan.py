import unittest
from importlib.machinery import SourceFileLoader

module = SourceFileLoader('plan', '.github/scripts/crate-publish-plan.py').load_module()


def entry(name, dependencies=(), version='1.0.0'):
    return {'id': name, 'name': name, 'version': version, 'dependencies': [
        {'name': dep, 'kind': 'normal', 'req': '^1'} for dep in dependencies]}


class PlanTest(unittest.TestCase):
    def test_actual_topology_beats_dependency_count(self):
        # A depends on B, yet both have one direct dependency.
        packages = [entry('a', ['b']), entry('b', ['z']), entry('z')]
        releases = [{'package_name': p['name'], 'version': p['version']} for p in packages]
        metadata = {'workspace_members': [p['id'] for p in packages], 'packages': packages}
        self.assertEqual([name for name, _ in module.plan(releases, metadata)], ['z', 'b', 'a'])

    def test_version_mismatch_is_refused(self):
        metadata = {'workspace_members': ['a'], 'packages': [entry('a')]}
        with self.assertRaisesRegex(ValueError, 'differs'):
            module.plan([{'package_name': 'a', 'version': '2.0.0'}], metadata)

    def test_cycle_is_refused(self):
        packages = [entry('a', ['b']), entry('b', ['a'])]
        metadata = {'workspace_members': ['a', 'b'], 'packages': packages}
        with self.assertRaisesRegex(ValueError, 'cycle'):
            module.plan([{'package_name': p['name'], 'version': p['version']} for p in packages], metadata)

    def test_duplicate_is_refused(self):
        metadata = {'workspace_members': ['a'], 'packages': [entry('a')]}
        with self.assertRaisesRegex(ValueError, 'duplicate'):
            module.plan([{'package_name': 'a', 'version': '1.0.0'}] * 2, metadata)


if __name__ == '__main__':
    unittest.main()
