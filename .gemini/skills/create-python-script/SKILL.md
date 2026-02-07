---
name: create-python-script
description: 現代的な Python (3.10+) スクリプト作成のための標準プロトコル。argparse による CLI 設計、ruff による品質管理（uvx 経由）、および最新の型ヒント記法（Optional ではなく | None 等）を適用する。
---

# Create Python Script

このスキルは、プロジェクト内での Python スクリプトの品質と現代的な記述を保証するためのガイドラインを提供します。

## Core Requirements

1. **CLI 設計 (argparse)**:
   - `sys.argv` を直接参照せず、必ず `argparse` モジュールを使用すること。
   - ツールが「検索 (search)」「閲覧 (view)」「作成 (create)」などの異なる複数のアクションを持つ場合は、サブコマンド (`subparsers`) を活用し、`dest="command"` で制御することを推奨する。

2. **現代的な型ヒント (Python 3.10+)**:
   - 組み込み型を使い、`typing.List` や `typing.Dict` を使用しない（例: `list[str]`, `dict[str, int]`, `tuple[int, int]`）。
   - `Optional[T]` は使用せず、`T | None` と記述すること。
   - `Union[A, B]` は使用せず、`A | B` と記述すること。

3. **環境管理 (PEP 723)**:
   - スクリプトの冒頭に `script` メタデータを記述し、`uv run` で実行可能にすること。
   - 例:
     ```python
     # /// script
     # requires-python = ">=3.12"
     # dependencies = [
     #   "httpx",
     #   "python-dotenv",
     # ]
     # ///
     ```

4. **品質検証 (ruff)**:
   - 作成時および**編集（コード変更）を行うたびに必ず**、以下のコマンドを実行してコードを整えること：
   ```bash
   export HOME=/home/node && uvx ruff check --fix <script_path>
   export HOME=/home/node && uvx ruff format <script_path>
   ```
