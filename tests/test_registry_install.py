import subprocess
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CLI = ROOT / "opaml"

class RegistryInstallTests(unittest.TestCase):
    def run_cli(self, cwd, *args):
        result = subprocess.run(["python3", str(CLI), *args], cwd=cwd, text=True, capture_output=True)
        if result.returncode != 0:
            self.fail(f"CLI failed: {result.stdout} {result.stderr}")
        return result

    def test_installs_example_package_and_verifies_lock(self):
        with tempfile.TemporaryDirectory() as temp:
            project = Path(temp) / "sample"
            self.run_cli(temp, "init", str(project))
            self.run_cli(project, "add", "hello", "--version", "1.0.0")
            self.run_cli(project, "install")
            self.run_cli(project, "verify")
            installed = project / ".opaml" / "packages" / "hello-1.0.0" / "README.md"
            self.assertTrue(installed.is_file())
            self.assertIn("hello 1.0.0", installed.read_text())

if __name__ == "__main__":
    unittest.main()
