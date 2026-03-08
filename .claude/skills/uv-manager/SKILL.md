---
name: uv-manager
description: uv を用いた Python スクリプトの実行、依存関係管理を解決するための専門的な手順。
---

# uv-manager プロトコル

Python スクリプトの実行が必要な場合、このプロトコルに従って uv を活用してください。

## 1. ローカル環境での実行手順

ローカル環境では `uv run` をそのまま実行できます。

- **標準実行コマンド**:
  ```bash
  uv run <script_path>
  ```

## 2. リソース情報
- `uv` バイナリの場所: `/home/linuxbrew/.linuxbrew/bin/uv`（Homebrew 経由、PATH 上に存在）
- `uv run` は inline script metadata（`# /// script` ブロック）で依存関係を自動解決します
