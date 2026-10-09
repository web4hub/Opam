import json
import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLI = ROOT / "opaml"

class OpamlCliTests(unittest.TestCase):
    def run_cli(self, cwd, *args, ok=True):
        result = subprocess.run(["python3", str(CLI), *args], cwd=cwd, text=True, capture_output=True)
        if ok and result.returncode != 0:
            self.fail(f"CLI failed: {result.stdout} {result.stderr}")
        if not ok and result.returncode == 0:
            self.fail("CLI unexpectedly succeeded")
        return result

    def test_init_and_empty_install(self):
        with tempfile.TemporaryDirectory() as temp:
            project = Path(temp) / "sample"
            self.run_cli(temp, "init", str(project))
            self.run_cli(project, "install")
            self.run_cli(project, "verify")
            lock = json.loads((project / "opaml.lock").read_text())
            self.assertEqual(lock["project"], "sample")
            self.assertEqual(lock["packages"], {})

    def test_missing_package_fails_cleanly(self):
        with tempfile.TemporaryDirectory() as temp:
            project = Path(temp) / "sample"
            self.run_cli(temp, "init", str(project))
            self.run_cli(project, "add", "not-in-registry", ok=True)
            result = self.run_cli(project, "install", ok=False)
            self.assertIn("no local registry version", result.stderr)

if __name__ == "__main__":
    unittest.main()
