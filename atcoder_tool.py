# /// script
# requires-python = ">=3.13"
# dependencies = []
# ///

"""AtCoder 開発補助ツール。

oj test / oj submit / oj download / exe を Python から実行する。
"""

import argparse
import re
import subprocess
import sys
from dataclasses import dataclass
from pathlib import Path


# ---------------------------------------------------------------------------
# Data classes
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class ContestInfo:
    """カレントディレクトリから取得したコンテスト情報。"""

    contest: str
    task: str


@dataclass(frozen=True)
class BuildConfig:
    """cargo build の設定。"""

    is_release: bool
    bin_suffix: str  # contest_<bin_suffix> がバイナリ名になる


@dataclass(frozen=True)
class OjTestConfig:
    """oj test の実行設定。"""

    bin_path: Path
    extra_arg: str  # "" or "-e 1e-6"
    bin_suffix: str  # ハイライト対象ファイル名（拡張子なし）兼バイナリ名サフィックス


# ---------------------------------------------------------------------------
# IO functions
# ---------------------------------------------------------------------------


def get_contest_info() -> ContestInfo:
    """カレントディレクトリのパスからコンテスト名とタスク名を取得する。

    前提: カレントディレクトリが <contest>/<task>/ の形式であること。

    Returns:
        コンテスト名とタスク名を持つ ContestInfo。
    """
    cwd = Path.cwd()
    task = cwd.name
    contest = cwd.parent.name
    return ContestInfo(contest=contest, task=task)


def get_repo_root() -> Path:
    """git リポジトリのルートディレクトリを返す。

    Returns:
        リポジトリルートの Path。
    """
    result = subprocess.run(
        ["git", "rev-parse", "--show-toplevel"],
        capture_output=True,
        text=True,
        check=True,
    )
    return Path(result.stdout.strip())


# ---------------------------------------------------------------------------
# Pure functions
# ---------------------------------------------------------------------------


def get_atcoder_url(info: ContestInfo) -> str:
    """AtCoder の問題 URL を生成する。

    Args:
        info: コンテスト情報。

    Returns:
        問題 URL 文字列。
    """
    return (
        f"https://atcoder.jp/contests/{info.contest}/tasks/{info.contest}_{info.task}"
    )


def get_bin_path(root: Path, config: BuildConfig, info: ContestInfo) -> Path:
    """cargo ビルド後の実行ファイルパスを返す。

    Args:
        root: リポジトリルート。
        config: ビルド設定。
        info: コンテスト情報。

    Returns:
        実行ファイルの Path。
    """
    build_type = "release" if config.is_release else "debug"
    bin_name = f"{info.contest}_{config.bin_suffix}"
    return root / "target" / build_type / bin_name


def build_cargo_cmd(config: BuildConfig, info: ContestInfo) -> list[str]:
    """cargo build コマンドを構築する。

    Args:
        config: ビルド設定。
        info: コンテスト情報。

    Returns:
        subprocess に渡すコマンドリスト。
    """
    bin_name = f"{info.contest}_{config.bin_suffix}"
    cmd = ["cargo", "build"]
    if config.is_release:
        cmd.append("--release")
    cmd += ["--bin", bin_name]
    return cmd


def build_oj_test_cmd(config: OjTestConfig) -> str:
    """oj test コマンド文字列を構築する（script -c に渡す用）。

    Args:
        config: oj test の実行設定。

    Returns:
        シェルコマンド文字列。
    """
    cmd = f"oj test -c '{config.bin_path}' --ignore-spaces-and-newline"
    if config.extra_arg:
        cmd = f"{cmd} {config.extra_arg}"
    return cmd


# ---------------------------------------------------------------------------
# Side-effecting functions
# ---------------------------------------------------------------------------


def run_download(info: ContestInfo) -> bool:
    """AtCoder からテストケースをダウンロードする。

    Args:
        info: コンテスト情報。

    Returns:
        ダウンロード成功なら True、失敗なら False。
    """
    url = get_atcoder_url(info)
    result = subprocess.run(["oj", "download", url], check=False)
    return result.returncode == 0


def run_cargo_build(cmd: list[str]) -> bool:
    """cargo build を実行する。

    Args:
        cmd: cargo build コマンドリスト。

    Returns:
        ビルド成功なら True、失敗なら False。
    """
    result = subprocess.run(cmd, check=False)
    return result.returncode == 0


def run_oj_test(config: OjTestConfig) -> None:
    """TTY シミュレーションありで oj test を実行し、ソース位置をハイライトする。

    Args:
        config: oj test の実行設定。
    """
    oj_cmd = build_oj_test_cmd(config)
    pattern = re.compile(rf"(\./{re.escape(config.bin_suffix)}\.rs:\d+:\d+)")

    proc = subprocess.Popen(
        ["script", "-q", "-e", "-c", oj_cmd, "/dev/null"],
        stdout=subprocess.PIPE,
    )
    assert proc.stdout is not None
    for raw_line in proc.stdout:
        line = raw_line.decode(errors="replace")
        highlighted = pattern.sub(r"\033[33m\1\033[0m", line)  # 黄色でハイライト
        sys.stdout.write(highlighted)
        sys.stdout.flush()
    proc.wait()


def check_dbg_output(task: str) -> bool:
    """ソースファイルに dbg!/lg! が残っていないかチェックする。

    コメントアウトされている行は無視する。

    Args:
        task: タスク名（ファイル名の拡張子なし部分）。

    Returns:
        dbg!/lg! が残っていなければ True、残っていれば False。
    """
    # 振り返り実装 (-b 指定時) もカレントディレクトリに <bin_suffix>.rs として置く前提
    src = Path(f"{task}.rs")
    if not src.exists():
        return True
    # (?<!:)lg で ::lg! (フルパス指定) を除外する
    result = subprocess.run(
        ["grep", "-Pq", r"^(?!.*//.*(dbg|lg)!).*(dbg|(?<!:)lg)!", str(src)],
        check=False,
    )
    # grep が 0 を返した = マッチあり = dbg! が残っている
    return result.returncode != 0


# ---------------------------------------------------------------------------
# Command handlers
# ---------------------------------------------------------------------------


def cmd_test(args: argparse.Namespace) -> None:
    """test サブコマンドのハンドラ。cargo build → oj test を実行する。

    Args:
        args: argparse でパースされた引数。
    """
    info = get_contest_info()
    bin_suffix: str = args.b or info.task
    config = BuildConfig(is_release=args.r, bin_suffix=bin_suffix)

    if not Path("test").is_dir():
        if not run_download(info):
            sys.exit(1)

    root = get_repo_root()
    cargo_cmd = build_cargo_cmd(config, info)
    if not run_cargo_build(cargo_cmd):
        sys.exit(1)

    bin_path = get_bin_path(root, config, info)
    extra_arg = "-e 1e-6" if args.f else ""
    oj_config = OjTestConfig(
        bin_path=bin_path,
        extra_arg=extra_arg,
        bin_suffix=bin_suffix,
    )
    run_oj_test(oj_config)

    if not check_dbg_output(bin_suffix):
        print()
        print("\033[33mWARNING: デバッグ出力が残ってる\033[0m")  # 黄色で警告


def cmd_submit(args: argparse.Namespace) -> None:  # noqa: ARG001
    """submit サブコマンドのハンドラ。oj submit を実行する。

    Args:
        args: argparse でパースされた引数（未使用）。
    """
    info = get_contest_info()
    url = get_atcoder_url(info)
    subprocess.run(
        ["oj", "submit", url, f"{info.task}.rs", "-w", "1", "--no-open"],
        check=False,
    )


def cmd_download(args: argparse.Namespace) -> None:  # noqa: ARG001
    """download サブコマンドのハンドラ。oj download を実行する。

    Args:
        args: argparse でパースされた引数（未使用）。
    """
    info = get_contest_info()
    if not run_download(info):
        sys.exit(1)


def cmd_exe(args: argparse.Namespace) -> None:
    """exe サブコマンドのハンドラ。cargo build → 実行ファイルを起動する。

    Args:
        args: argparse でパースされた引数。
    """
    info = get_contest_info()
    bin_suffix: str = args.b or info.task
    config = BuildConfig(is_release=args.r, bin_suffix=bin_suffix)
    root = get_repo_root()
    cargo_cmd = build_cargo_cmd(config, info)

    if not run_cargo_build(cargo_cmd):
        sys.exit(1)

    bin_path = get_bin_path(root, config, info)
    subprocess.run([str(bin_path)], check=False)


# ---------------------------------------------------------------------------
# Entry point
# ---------------------------------------------------------------------------


def build_parser() -> argparse.ArgumentParser:
    """argparse パーサーを構築する。

    Returns:
        ArgumentParser オブジェクト。
    """
    parser = argparse.ArgumentParser(
        prog="atcoder_tool",
        description="AtCoder 開発補助ツール",
    )
    subparsers = parser.add_subparsers(dest="command", required=True)

    # test
    p_test = subparsers.add_parser(
        "test",
        help="cargo build + oj test を実行する",
        description=(
            "Rust コードをビルドし、oj test でサンプルケースを検証する。\n"
            "test/ ディレクトリがなければ先に download を実行する。\n"
            "テスト後に dbg!/lg! の残存チェックを行う。"
        ),
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    p_test.add_argument(
        "-f", action="store_true", help="浮動小数点許容誤差 1e-6 を設定"
    )
    p_test.add_argument("-r", action="store_true", help="リリースビルドを使用する")
    p_test.add_argument(
        "-b", metavar="suffix", help="バイナリ名サフィックス（振り返り実装用）"
    )

    # submit
    subparsers.add_parser(
        "submit",
        help="oj submit を実行する",
        description="カレントディレクトリのタスクを AtCoder に提出する。",
    )

    # download
    subparsers.add_parser(
        "download",
        help="oj download を実行する",
        description="AtCoder からサンプルケースをダウンロードし test/ に保存する。",
    )

    # exe
    p_exe = subparsers.add_parser(
        "exe",
        help="cargo build + 実行ファイルを起動する",
        description="Rust コードをビルドし、標準入力を与えながら実行ファイルを起動する。",
    )
    p_exe.add_argument("-r", action="store_true", help="リリースビルドを使用する")
    p_exe.add_argument(
        "-b", metavar="suffix", help="バイナリ名サフィックス（振り返り実装用）"
    )

    return parser


def main() -> None:
    """エントリポイント。"""
    parser = build_parser()
    args = parser.parse_args()

    dispatch = {
        "test": cmd_test,
        "submit": cmd_submit,
        "download": cmd_download,
        "exe": cmd_exe,
    }
    dispatch[args.command](args)


if __name__ == "__main__":
    main()
