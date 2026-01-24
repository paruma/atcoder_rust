---
name: uv-manager
description: uv を用いた Python スクリプトの実行、依存関係管理、およびサンドボックス環境での制約（パス、キャッシュ、権限）を解決するための専門的な手順。
---

# uv-manager プロトコル

Python スクリプトの作成または実行が必要な場合、このプロトコルに従って uv を活用してください。

## 1. スクリプトの標準仕様
- **インラインメタデータ**: 全ての Python スクリプトの先頭に PEP 723 形式のインラインメタデータを記述してください。
  ```python
  # /// script
  # requires-python = ">=3.13"
  # dependencies = [
  #   "requests",
  # ]
  # ///
  ```
- **コーディングスタイル**: 現代的な Python (3.9+) の記法（組み込み型による型ヒント等）を用い、`typing` モジュールの使用を避けてください。
- **設計原則**: 入力、処理、出力を明確に分離し、保守性の高い SOLID な構造にしてください。

## 2. サンドボックス環境での実行手順
サンドボックス環境では権限とパスに制約があるため、`uv run` を実行する際は必ず以下の環境変数を設定してください。

- **必須の環境変数**:
  - `HOME=/home/node`
  - `UV_CACHE_DIR=/home/node/.cache/uv`
  - `PATH` への `/home/node/.local/bin` の追加
- **標準実行コマンド**:
  ```bash
  export HOME=/home/node && export UV_CACHE_DIR=/home/node/.cache/uv && export PATH="/home/node/.local/bin:$PATH" && uv run <script_path>
  ```

## 3. リソース情報
- `uv` バイナリの場所: `/home/node/.local/bin/uv`
- キャッシュディレクトリ: `/home/node/.cache/uv`