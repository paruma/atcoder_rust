# /// script
# requires-python = ">=3.12"
# dependencies = []
# ///
import subprocess
import sys
import os
import json
import re
import argparse
from datetime import datetime
from pathlib import Path


class VerificationResult:
    def __init__(self) -> None:
        self.results: list[dict[str, str | bool]] = []
        self.has_failure: bool = False

    def add(self, name: str, success: bool, message: str = "") -> None:
        self.results.append({"name": name, "success": success, "message": message})
        if not success:
            self.has_failure = True

    def print_summary(self) -> None:
        print("\n" + "=" * 40)
        print(" VERIFICATION SUMMARY")
        print("=" * 40)
        for r in self.results:
            success = r["success"]
            name = r["name"]
            message = r["message"]

            status = "✅ PASS" if success else "❌ FAIL"
            # Special case for coverage warning
            if name == "Coverage" and success and message:
                status = "⚠️ WARN"

            print(f"{status} | {name}")
            if message and isinstance(message, str):
                for line in message.splitlines():
                    print(f"   > {line}")
        print("=" * 40)


def run_command(
    cmd: list[str] | str,
    description: str = "",
    shell: bool = False,
    capture: bool = False,
    verbose: bool = True,
) -> tuple[bool, str]:
    if description:
        print(f"\n>>> Step: {description}")

    if isinstance(cmd, list):
        print(f"Executing: {' '.join(cmd)}")
    else:
        print(f"Executing: {cmd}")

    try:
        if capture:
            result = subprocess.run(
                cmd, shell=shell, check=True, text=True, capture_output=True
            )
            if verbose and result.stdout:
                print(result.stdout)
            return True, result.stdout
        else:
            subprocess.run(cmd, shell=shell, check=True, text=True)
            return True, ""
    except subprocess.CalledProcessError as e:
        print("\n❌ Error during execution!")
        output = e.stdout + e.stderr if capture else str(e)
        if capture:
            print(output)
        return False, output


def main() -> None:
    parser = argparse.ArgumentParser(description="Unified library verification script.")
    parser.add_argument(
        "module_path",
        help="Rust module path (e.g., data_structure::segtree_lib::lazy_segtree)",
    )
    parser.add_argument("--skip-cov", action="store_true", help="Skip coverage check")
    parser.add_argument(
        "--no-status", action="store_true", help="Do not update status file"
    )

    args = parser.parse_args()
    module_path: str = args.module_path
    vr = VerificationResult()

    # 1. Test
    ok, _ = run_command(
        [
            "cargo",
            "test",
            "-p",
            "mylib",
            "--lib",
            module_path,
            "--",
            "--include-ignored",
            "--show-output",
        ],
        description="Test",
        capture=True,
    )
    vr.add("Test", ok)

    # 2. Coverage
    if not args.skip_cov:
        cov_cmd = [
            "cargo",
            "llvm-cov",
            "test",
            "-p",
            "mylib",
            "--lib",
            module_path,
            "--show-missing-lines",
            "--",
            "--include-ignored",
        ]
        # We want to parse this, so we capture. verbose=True will print it.
        ok, output = run_command(cov_cmd, description="Coverage", capture=True)

        if not ok:
            vr.add("Coverage", False, "cargo llvm-cov command failed.")
        else:
            file_parts = module_path.split("::")
            target_filename = file_parts[-1] + ".rs"
            found_file = False
            msg = ""
            for line in output.splitlines():
                if target_filename in line:
                    found_file = True
                    percentages = re.findall(r"(\d+\.\d+)%", line)
                    if percentages and any(p != "100.00" for p in percentages):
                        msg = f"{target_filename} coverage: {percentages}"
                    break
            if not found_file:
                msg = f"Could not find coverage data for {target_filename}"
            vr.add("Coverage", True, msg)
    else:
        print("\n>>> Step: Coverage (SKIPPED)")
        vr.add("Coverage", True, "Skipped")

    # 3. Format
    ok, _ = run_command(["cargo", "fmt"], description="Format", capture=True)
    vr.add("Format", ok)

    # 4. Clippy
    ok, _ = run_command(
        ["cargo", "clippy", "-p", "mylib", "--all-targets", "--", "-D", "warnings"],
        description="Clippy",
        capture=True,
    )
    vr.add("Clippy", ok)

    # 5. Snippet Linter
    ok, _ = run_command(
        ["cargo", "run", "--bin", "snippet_linter"],
        description="Snippet Linter",
        capture=True,
    )
    vr.add("Snippet Linter", ok)

    # Summary
    vr.print_summary()

    # Update Status File
    if not vr.has_failure and not args.no_status:
        status_file = Path(".gemini/.verification_status.json")
        status_data = {
            "module": module_path,
            "timestamp": datetime.now().isoformat(),
            "status": "passed",
        }
        os.makedirs(".gemini", exist_ok=True)
        with open(status_file, "w") as f:
            json.dump(status_data, f, indent=2)
        print(f"Status recorded in {status_file}")

    if vr.has_failure:
        sys.exit(1)


if __name__ == "__main__":
    main()
