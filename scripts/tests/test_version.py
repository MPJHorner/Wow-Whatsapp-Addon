import importlib.util
from pathlib import Path
import plistlib
import tempfile
import unittest

SCRIPT = Path(__file__).resolve().parents[1] / 'version.py'
spec = importlib.util.spec_from_file_location('version', SCRIPT)
version = importlib.util.module_from_spec(spec)
spec.loader.exec_module(version)


class VersionTests(unittest.TestCase):
    def fixture(self, root):
        (root / 'packaging').mkdir()
        (root / 'Cargo.toml').write_text('[package]\nname = "wow-chat"\nversion = "0.3.9"\n')
        (root / 'Cargo.lock').write_text('[[package]]\nname = "dependency"\nversion = "2.1.0"\n\n[[package]]\nname = "wow-chat"\nversion = "0.3.9"\n')
        (root / 'packaging/Info.plist').write_bytes(plistlib.dumps({'CFBundleShortVersionString':'0.3.9', 'CFBundleVersion':'12', 'LSUIElement':True}))

    def test_semver_resets_lower_components(self):
        for kind, expected in [('patch','0.3.10'), ('minor','0.4.0'), ('major','1.0.0')]:
            self.assertEqual(version.next_version('0.3.9', kind), expected)

    def test_updates_only_app_versions_and_build_number(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.fixture(root)
            self.assertEqual(version.bump(root, 'minor'), '0.4.0')
            actual, _, lock, info = version.read_state(root)
            self.assertEqual(actual, '0.4.0')
            self.assertIn('name = "dependency"\nversion = "2.1.0"', lock)
            self.assertEqual(info['CFBundleVersion'], '13')
            self.assertTrue(info['LSUIElement'])

    def test_dry_run_does_not_write(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.fixture(root)
            before = {p: p.read_bytes() for p in root.rglob('*') if p.is_file()}
            self.assertEqual(version.bump(root, 'patch', True), '0.3.10')
            self.assertTrue(all(p.read_bytes() == content for p, content in before.items()))

    def test_mismatch_fails_before_writing(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.fixture(root)
            lock = root / 'Cargo.lock'
            lock.write_text(lock.read_text().replace('0.3.9', '0.2.0'))
            before = (root / 'Cargo.toml').read_bytes()
            with self.assertRaises(ValueError):
                version.bump(root, 'patch')
            self.assertEqual(before, (root / 'Cargo.toml').read_bytes())


if __name__ == '__main__':
    unittest.main()
