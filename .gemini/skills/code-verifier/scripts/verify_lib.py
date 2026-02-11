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
from dataclasses import dataclass
from enum import Enum, auto


class VerificationStatus(Enum):
    """検証ステップの結果状態を表す"""

    PASS = auto()
    WARN = auto()
    FAIL = auto()


@dataclass(frozen=True)
class StepResult:
    """各検証ステップの結果データを保持する"""

    name: str
    status: VerificationStatus
    message: str = ""
    output: str = ""


class VerificationReport:
    """検証結果を収集し、サマリーを表示するクラス"""

    def __init__(self):
        self.results: list[StepResult] = []
        self.has_failure = False

    def add(self, result: StepResult):
        self.results.append(result)
        if result.status == VerificationStatus.FAIL:
            self.has_failure = True

    def print_summary(self):
        # 状態に応じた表示用ラベルの定義
        labels = {
            VerificationStatus.PASS: "✅ PASS",
            VerificationStatus.WARN: "⚠️ WARN",
            VerificationStatus.FAIL: "❌ FAIL",
        }

        print("\n" + "=" * 40)
        print(" VERIFICATION SUMMARY")
        print("=" * 40)
        for r in self.results:
            label = labels.get(r.status, "???")
            print(f"{label} | {r.name}")
            if r.message:
                for line in r.message.splitlines():
                    print(f"   > {line}")
        print("=" * 40)


def run_command(cmd: list[str], capture: bool = True) -> tuple[bool, str]:
    """外部コマンドを実行し、成功判定と標準出力を取得する補助関数"""
    print(f"Executing: {' '.join(cmd)}")
    try:
        result = subprocess.run(cmd, check=True, text=True, capture_output=capture)
        return True, result.stdout if capture else ""
    except subprocess.CalledProcessError as e:
        output = e.stdout + e.stderr if capture else str(e)
        return False, output


# --- 各検証ステップの実装 ---


def verify_unit_tests(module_path: str) -> StepResult:
    """ユニットテストの実行と『0 tests』の検知"""
    name = "Test"
    ok, output = run_command(
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
        ]
    )

    if ok:
        if "running 0 tests" in output:
            return StepResult(
                name,
                VerificationStatus.FAIL,
                f"No tests found for path: {module_path}",
                output,
            )
        return StepResult(name, VerificationStatus.PASS, "", output)

    return StepResult(name, VerificationStatus.FAIL, "", output)


def verify_coverage(module_path: str) -> StepResult:
    """
    カバレッジを計測し、指定されたモジュールに関連する結果のみを表示・抽出する。

    1. cargo llvm-cov を実行してプロジェクト全ファイルのカバレッジを取得。
    2. 出力結果から、指定されたモジュールパス（例: a::b）に関連するファイル行だけを抽出。
    3. 抽出した結果から、そのモジュールの代表ファイル（a/b.rs 等）のカバレッジ率を確認。
    """
    name = "Coverage"
    # 全量が出ると見づらいため、結果をキャプチャしてフィルタリングする
    ok, output = run_command(
        [
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
        ],
        capture=True,
    )
    if not ok:
        return StepResult(name, VerificationStatus.FAIL, "cargo llvm-cov failed", output)

    # フィルタリング用のパス断片を作成 (例: data_structure::segtree -> data_structure/segtree)
    path_fragment = module_path.replace("::", "/")
    target_basename = module_path.split("::")[-1]

    lines = output.splitlines()
    # ヘッダー行を保持
    header_lines = [l for l in lines if l.startswith("Filename") or l.startswith("---")]
    # パス断片が含まれる行（データ行）を抽出
    data_lines = [l for l in lines if path_fragment in l and not l.startswith("TOTAL")]

    if not data_lines:
        msg = f"Could not find specific coverage data for {module_path} in the report."
        print(f"\n⚠️ {msg}")
        return StepResult(name, VerificationStatus.WARN, msg, "")

    # 抽出された関連ファイルの結果を画面に表示
    print("\n".join(header_lines + data_lines))

    # モジュールの代表ファイル（ディレクトリ名.rs または ディレクトリ名/mod.rs）のカバレッジ概要を特定
    target_patterns = [
        f"{path_fragment}.rs",
        f"{path_fragment}/mod.rs",
    ]
    summary_msg = ""
    status = VerificationStatus.PASS

    for line in data_lines:
        if any(pattern in line for pattern in target_patterns):
            # 行内から数値（%表示）をすべて抽出
            if percentages := re.findall(r"(\d+\.\d+)%", line):
                # 100% でない項目があればサマリーに表示するメッセージを作成
                if any(p != "100.00" for p in percentages):
                    summary_msg = f"{target_basename}.rs coverage summary: {percentages}"
                    status = VerificationStatus.WARN
            break  # 最初に見つかった代表ファイルで判定終了

    return StepResult(name, status, summary_msg, output)


def verify_static_analysis() -> list[StepResult]:
    """静的解析（Format, Clippy, Snippet Linter）の一括実行"""
    results = []

    # Format
    ok, _ = run_command(["cargo", "fmt"])
    results.append(
        StepResult("Format", VerificationStatus.PASS if ok else VerificationStatus.FAIL)
    )

    # Clippy
    ok, out = run_command(
        ["cargo", "clippy", "-p", "mylib", "--all-targets", "--", "-D", "warnings"]
    )
    results.append(
        StepResult(
            "Clippy", VerificationStatus.PASS if ok else VerificationStatus.FAIL, "", out
        )
    )

    # Snippet Linter
    ok, out = run_command(["cargo", "run", "--bin", "snippet_linter"])
    results.append(
        StepResult(
            "Snippet Linter",
            VerificationStatus.PASS if ok else VerificationStatus.FAIL,
            "",
            out,
        )
    )

    return results


def record_status(module_path: str, no_status: bool) -> None:
    """検証成功時にステータスファイルを更新"""
    if not no_status:
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


# --- メインフロー ---


def main():
    parser = argparse.ArgumentParser(description="Unified library verification script.")
    parser.add_argument("module_path")
    parser.add_argument("--skip-cov", action="store_true")
    parser.add_argument("--no-status", action="store_true")

    args = parser.parse_args()
    report = VerificationReport()

    # 1. Test
    print("\n>>> Step: Test")
    test_res = verify_unit_tests(args.module_path)
    report.add(test_res)
    if test_res.status == VerificationStatus.FAIL:
        print(f"\n❌ Test failed:\n{test_res.output}")

    # 2. Coverage
    if not args.skip_cov:
        print("\n>>> Step: Coverage")
        cov_res = verify_coverage(args.module_path)
        report.add(cov_res)
        if cov_res.status == VerificationStatus.FAIL:
            print(f"\n❌ Coverage check failed:\n{cov_res.output}")
    else:
        print("\n>>> Step: Coverage (SKIPPED)")
        report.add(StepResult("Coverage", VerificationStatus.PASS, "Skipped"))

    # 3. Static Analysis
    print("\n>>> Step: Static Analysis")
    for res in verify_static_analysis():
        report.add(res)
        if res.status == VerificationStatus.FAIL:
            print(f"\n❌ {res.name} failed:\n{res.output}")

    # 4. Finalize
    report.print_summary()

    if not report.has_failure:
        record_status(args.module_path, args.no_status)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
