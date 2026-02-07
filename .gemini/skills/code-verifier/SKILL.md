---
name: code-verifier
description: src/mylib 配下の Rust ライブラリ編集時に、テスト・カバレッジ・静的解析・フォーマットを一貫して自動検証する標準プロトコル。
---

# code-verifier

競技プログラミング用 Rust ライブラリの品質を維持するための、自動検証スペシャリスト。

## Instructions

あなたは、`src/mylib` 配下のコードを編集する際、**ユーザーからの明示的な依頼がなくても、編集完了と同時に必ず**統合検証スクリプトを実行し、品質を保証する責務を負います。

### The Verification Flow

何らかの修正（ロジックの変更、コメントの修正、テストの追加）を行った後、統合検証スクリプトを実行してください。

1.  **統合検証スクリプトの実行**
    - コマンド: `python3 .gemini/skills/code-verifier/scripts/verify_lib.py <モジュールパス>`
    - 例: `python3 .gemini/skills/code-verifier/scripts/verify_lib.py data_structure::segtree_lib::lazy_segtree`

2.  **スクリプトによる検証内容**
    スクリプトは以下の項目を全て実行し、最後に結果をまとめて報告します。
    - **Unit Tests**: 指定されたモジュールのテストを実行し、全てパスするか。
    - **Coverage**: 行・関数カバレッジを測定します（100.00%を推奨しますが、異常系等の未実行は許容されます）。
    - **Format**: `cargo fmt` によるフォーマットが適用されているか。
    - **Clippy**: `cargo clippy` の警告（`-D warnings`）がないか。
    - **Snippet Consistency**: `snippet_linter` による整合性チェック。

3.  **検証ステータスの確認**
    - スクリプト末尾の `VERIFICATION SUMMARY` を確認し、全ての項目が `PASS` または `WARN`（カバレッジ）であることを確認してください。
    - 成功時、`.gemini/.verification_status.json` が自動更新されます。

4.  **最終確認 (Git Diff)**
    - 意図しないファイルや箇所に変更が及んでいないか、`git diff` で最終確認してください。

## Critical Rules
- **自動化の徹底**: 手動で個別のコマンドを実行するのではなく、必ず上記スクリプトを使用して一括検証してください。
- **再帰的検証**: 検証中にコードを修正した場合は、必ず再度スクリプトを実行してください。
- **妥協の禁止**: スクリプトが `FAIL` を報告した場合、それらを解消するまでタスクの完了を報告してはいけません。
- **ステータスの透明性**: 報告の際、スクリプトのサマリー出力を提示し、検証済みであることを示してください。
