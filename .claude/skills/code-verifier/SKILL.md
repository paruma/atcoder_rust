---
name: code-verifier
description: src/mylib 編集後に実行すること。このスキルでは、テスト、リンターなどのコード品質を検証する
---

# code-verifier

競技プログラミング用 Rust ライブラリの品質を維持するための、自動検証スペシャリスト。

## Instructions

あなたは、`src/mylib` 配下のコードを編集する際、**ユーザーからの明示的な依頼がなくても、編集完了と同時に必ず**統合検証スクリプトを実行し、品質を保証する責務を負います。

### The Verification Flow

何らかの修正（ロジックの変更、コメントの修正、テストの追加）を行った後、統合検証スクリプトを実行してください。

1.  **統合検証スクリプトの実行**
    - コマンド: `mkdir -p .claude/tmp && python3 .claude/skills/code-verifier/scripts/verify_lib.py <パス> > .claude/tmp/verify_<名前>_$(TZ=Asia/Tokyo date +%Y%m%d_%H%M%S).log 2>&1`
    - 例 (ファイルパス): `mkdir -p .claude/tmp && python3 .claude/skills/code-verifier/scripts/verify_lib.py src/mylib/data_structure/segtree_lib/lazy_segtree.rs > .claude/tmp/verify_lazy_segtree_$(TZ=Asia/Tokyo date +%Y%m%d_%H%M%S).log 2>&1`
    - **Note**: 結果を必ず `.claude/tmp/` 配下のログファイルにリダイレクトしてください。ファイル名には必ず `$(TZ=Asia/Tokyo date +%Y%m%d_%H%M%S)` を含めてください。
    - **Note**: ファイルパスを指定すると自動的に内部でモジュールパスに変換されます。

2.  **スクリプトによる検証内容**
    スクリプトは以下の項目を全て実行し、最後に結果をまとめて報告します。
    - **Unit Tests**: 指定されたモジュールのテストを実行。
    - **Coverage**: 行カバレッジを測定。**100%でない場合、未実行行のソースコードが前後2行のコンテキスト付きで直接表示されます**。これを見て不足しているテストケースを特定してください。
    - **Format**: `cargo fmt` を実行し、コードを自動整形します。
    - **Clippy**: `cargo clippy` の警告（`-D warnings`）がないか。
    - **Snippet Consistency**: `snippet_linter` による整合性チェック。

3.  **検証ステータスの確認**
    - スクリプト末尾の `VERIFICATION SUMMARY` を確認し、全ての項目が `PASS` または `WARN`（カバレッジ）であることを確認してください。

4.  **最終確認 (Git Diff)**
    - 意図しないファイルや箇所に変更が及んでいないか、`git diff` で最終確認してください。

## Critical Rules
- **自動化の徹底**: 手動で個別のコマンドを実行するのではなく、必ず上記スクリプトを使用して一括検証してください。
- **再帰的検証**: 検証中にコードを修正した場合は、必ず再度スクリプトを実行してください。
- **妥協の禁止**: スクリプトが `FAIL` を報告した場合、それらを解消するまでタスクの完了を報告してはいけません。
- **ステータスの透明性**: 報告の際、スクリプトのサマリー出力を提示し、検証済みであることを示してください。
