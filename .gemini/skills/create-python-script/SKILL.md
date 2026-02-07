# Create Python Script

このスキルは、プロジェクト内での Python スクリプトの品質と現代的な記述を保証するためのガイドラインを提供します。

## Core Requirements

1. **CLI 設計 (argparse)**:
   - `sys.argv` を直接参照せず、必ず `argparse` モジュールを使用すること。
   - ツールが「検索 (search)」「閲覧 (view)」「作成 (create)」などの異なる複数のアクションを持つ場合は、サブコマンド (`subparsers`) を活用し、`dest="command"` で制御することを推奨する。

2. **現代的な型ヒント (Python 3.10+)**:
   - **全ての関数引数と戻り値に型ヒントを記述すること**。
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
   - コマンド:
   ```bash
   export HOME=/home/node && uvx ruff check --fix <script_path>
   export HOME=/home/node && uvx ruff format <script_path>
   ```

## Critical Rules

- **即時検証の義務**: `write_file` や `replace` 等で Python コードを変更した直後、**他の作業（ユーザーへの報告を含む）を行う前に必ず** Ruff による品質検証を実行してください。
- **妥協の禁止**: Ruff がエラーを報告している間は、タスクの完了を報告してはいけません。
- **再帰的修正**: Ruff の指摘を修正するためにコードを変更した場合、再度 Ruff を実行して完全にクリーンな状態であることを確認してください。
