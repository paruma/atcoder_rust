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


def normalize_module_path(path: str) -> str:
    """ファイルパス（src/mylib/...）をモジュールパス（...::...）に変換する。
    既にモジュールパス形式の場合はそのまま返す。
    """
    if "::" in path:
        return path

    # パスを正規化
    p = Path(path)
    if not p.exists():
        return path  # 存在しない場合はそのまま返す（エラーは呼び出し側で処理）

    # src/mylib からの相対パスを取得
    try:
        # プロジェクトルートからの相対パスかチェック
        if p.is_absolute():
            rel_p = p.relative_to(Path.cwd() / "src/mylib")
        else:
            # 相対パスの場合、src/mylib からの相対パスか、ルートからの相対パスか判断
            if str(p).startswith("src/mylib/"):
                rel_p = p.relative_to("src/mylib")
            else:
                rel_p = p
    except ValueError:
        return path

    # 拡張子を除去し、mod.rs なら親ディレクトリ名を、それ以外ならファイル名をモジュール名とする
    parts = list(rel_p.parts)
    if parts[-1] == "mod.rs":
        parts.pop()
    elif parts[-1].endswith(".rs"):
        parts[-1] = parts[-1][:-3]

    return "::".join(parts)


def get_source_context(file_path: str, missed_lines: str) -> str:
    """未実行行とその周辺のソースコードを取得する。
    missed_lines は "10, 15-20" のような形式を想定。
    """
    if not missed_lines or missed_lines == "0":
        return ""

    try:
        with open(file_path, "r") as f:
            lines = f.readlines()
    except Exception as e:
        return f"   [Error reading source: {e}]"

    # 行番号をリストに変換
    targets = set()
    for part in missed_lines.split(","):
        part = part.strip()
        if "-" in part:
            start, end = map(int, part.split("-"))
            for i in range(start, end + 1):
                targets.add(i)
        else:
            try:
                targets.add(int(part))
            except ValueError:
                continue

    if not targets:
        return ""

    output = ["   --- Missed Lines Context ---"]
    sorted_targets = sorted(list(targets))

    # 連続する行をグループ化して表示
    processed = set()
    for t in sorted_targets:
        if t in processed:
            continue

        # 前後2行のコンテキストを表示
        start_view = max(1, t - 2)
        end_view = min(len(lines), t + 2)

        # 直前のグループとの境界線
        if output[-1] != "   --- Missed Lines Context ---":
            output.append("   ...")

        for i in range(start_view, end_view + 1):
            marker = ">>" if i in targets else "  "
            line_content = lines[i - 1].rstrip()
            output.append(f"   {marker} {i:4}: {line_content}")
            if i in targets:
                processed.add(i)

    return "\n".join(output)


def verify_coverage(module_path: str) -> StepResult:
    """
    カバレッジを計測し、指定されたモジュールに関連する結果のみを表示・抽出する。

    1. cargo llvm-cov を実行してカバレッジを取得。
    2. 出力結果から、指定されたモジュールパスに関連するファイル行だけを抽出して表示（ノイズ削減）。
    3. 未実行行がある場合、該当するソースコードのコンテキストを表示。
    """
    name = "Coverage"
    # llvm-cov を実行
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
        return StepResult(
            name, VerificationStatus.FAIL, "cargo llvm-cov failed", output
        )

    # フィルタリング用のパス断片を作成 (例: data_structure::segtree -> data_structure/segtree)
    path_fragment = module_path.replace("::", "/")
    target_basename = module_path.split("::")[-1]

    lines = output.splitlines()
    header_lines = []
    data_lines = []

    # ヘッダーとターゲット行を抽出
    in_summary = False
    for line in lines:
        if line.startswith("Filename"):
            in_summary = True
            header_lines.append(line)
            continue
        if in_summary:
            if line.startswith("---"):
                header_lines.append(line)
                continue
            if line.startswith("TOTAL"):
                # TOTAL 行も一応保持するが、後で表示するか決める
                continue
            if path_fragment in line:
                data_lines.append(line)
                continue
            if not line.strip():
                in_summary = False
                continue

    if not data_lines:
        msg = f"Could not find specific coverage data for {module_path} in the report."
        print(f"\n⚠️ {msg}")
        return StepResult(name, VerificationStatus.WARN, msg, "")

    # 出力を表示（ノイズを排除したサマリー）
    filtered_summary = "\n".join(header_lines + data_lines)
    print("\n" + filtered_summary)

    # 詳細なカバレッジ判定とソースコード提示
    summary_msg_parts: list[str] = []
    status = VerificationStatus.PASS
    context_outputs: list[str] = []

    # 出力全体から "path/to/file.rs: 1, 2, 3" のような形式を探す
    missed_lines_map: dict[str, str] = {}
    for line in lines:
        if ":" in line and not line.startswith("  "):
            # /path/to/src/mylib/algorithm/math.rs: 10, 15-20 形式を想定
            parts = line.split(":", 1)
            if len(parts) == 2:
                fpath = parts[0].strip()
                m_lines = parts[1].strip()
                if not fpath.endswith(".rs"):
                    continue
                # 代表ファイル名 (例: algorithm/math.rs) に変換してマッチング
                for d_line in data_lines:
                    target_file = d_line.split()[0]
                    if fpath.endswith(target_file):
                        missed_lines_map[target_file] = m_lines

    for line in data_lines:
        cols = line.split()
        if len(cols) < 5:
            continue

        filename = cols[0]
        # ファイル名が期待するもの（target_basename）を含むかチェックする等の用途に使えるが、
        # 現状は filename をそのまま使用。ruff 指摘に基づき target_basename を消すか活用するか判断。
        # ここではサマリーの可読性のため活用する。
        display_name = (
            target_basename if filename.endswith(f"{target_basename}.rs") else filename
        )

        # 行内から全ての百分率を抽出
        percentages = re.findall(r"(\d+\.\d+)%", line)
        if not percentages:
            continue

        # 最後の百分率を Lines Coverage とみなす
        coverage_pct = percentages[-1]

        # Missed Lines Count を取得 (通常、Lines % の1つ前)
        pct_indices = [i for i, col in enumerate(cols) if "%" in col]
        missed_count = "0"
        if len(pct_indices) >= 3:
            lines_pct_idx = pct_indices[-1]
            if lines_pct_idx > 0:
                missed_count = cols[lines_pct_idx - 1]

        # 行番号リストを取得 (Mapから優先、なければ行末から)
        missed_lines_str = missed_lines_map.get(filename)
        if not missed_lines_str:
            # テーブル形式の末尾にあるかチェック (件数と等しい場合は件数とみなす)
            missed_lines_candidate = cols[-1]
            if (
                "%" not in missed_lines_candidate
                and any(c.isdigit() for c in missed_lines_candidate)
                and missed_lines_candidate != missed_count
            ):
                missed_lines_str = missed_lines_candidate
            else:
                missed_lines_str = "0"

        if coverage_pct != "100.00":
            status = VerificationStatus.WARN
            summary_msg_parts.append(f"{display_name}: {coverage_pct}%")

            # ソースコードのコンテキストを取得
            actual_file_path = os.path.join("src/mylib", filename)
            if context := get_source_context(actual_file_path, missed_lines_str):
                context_outputs.append(f"File: {filename}\n{context}")
            else:
                # 行番号が不明な場合は件数のみ表示
                msg = f"File: {filename}\n   [Missed Lines: {missed_count} (specific line numbers not found in report)]"
                context_outputs.append(msg)

    summary_msg = (
        "Coverage: " + ", ".join(summary_msg_parts) if summary_msg_parts else ""
    )
    full_output = filtered_summary + "\n\n" + "\n".join(context_outputs)

    if context_outputs:
        print("\n" + "\n".join(context_outputs))

    return StepResult(name, status, summary_msg, full_output)


def verify_static_analysis() -> list[StepResult]:
    """静的解析（Format, Clippy, Snippet Linter）の一括実行"""
    results = []

    # Format
    ok, _ = run_command(["cargo", "fmt"])
    results.append(
        StepResult(
            "Format", VerificationStatus.PASS if ok else VerificationStatus.FAIL, ""
        )
    )

    # Clippy
    ok, out = run_command(
        ["cargo", "clippy", "-p", "mylib", "--all-targets", "--", "-D", "warnings"]
    )
    results.append(
        StepResult(
            "Clippy",
            VerificationStatus.PASS if ok else VerificationStatus.FAIL,
            "",
            out,
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
    parser.add_argument(
        "module_path",
        help="Module path (e.g. math::gcd) or file path (e.g. src/mylib/math/gcd.rs)",
    )
    parser.add_argument("--skip-cov", action="store_true")
    parser.add_argument("--no-status", action="store_true")

    args = parser.parse_args()

    # パスを正規化 (file path -> module path)
    normalized_path = normalize_module_path(args.module_path)
    if normalized_path != args.module_path:
        print(f"Normalized path: {args.module_path} -> {normalized_path}")

    report = VerificationReport()

    # 1. Test
    print("\n>>> Step: Test")
    test_res = verify_unit_tests(normalized_path)
    report.add(test_res)
    if test_res.status == VerificationStatus.FAIL:
        print(f"\n❌ Test failed:\n{test_res.output}")

    # 2. Coverage
    if not args.skip_cov:
        print("\n>>> Step: Coverage")
        cov_res = verify_coverage(normalized_path)
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
        record_status(normalized_path, args.no_status)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
