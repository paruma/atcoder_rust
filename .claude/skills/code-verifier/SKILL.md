---
name: code-verifier
description: このスキルでは、テスト、リンターなどのコード品質を検証する
---

# code-verifier

競技プログラミング用 Rust ライブラリの品質を維持するための、自動検証スペシャリスト。

### The Verification Flow

1.  **統合検証スクリプトの実行**
    - コマンド: `python3 .claude/skills/code-verifier/scripts/verify_lib.py <パス>`
    - 例 (ファイルパス): `python3 .claude/skills/code-verifier/scripts/verify_lib.py src/mylib/data_structure/segtree_lib/lazy_segtree.rs`
    - **Note**: ファイルパスを指定すると自動的に内部でモジュールパスに変換されます。

2.  **スクリプトによる検証内容**
    スクリプトは以下の項目を全て実行し、最後に結果をまとめて報告します。
    - **Unit Tests**: 指定されたモジュールのテストを実行。
    - **Coverage**: 行カバレッジを測定。**100%でない場合、未実行行のソースコードが前後2行のコンテキスト付きで直接表示されます**。これを見て不足しているテストケースを特定してください。
    - **Format**: `cargo fmt` を実行し、コードを自動整形します。
    - **Clippy**: `cargo clippy` の警告（`-D warnings`）がないか。
    - **Snippet Consistency**: `snippet_linter` による整合性チェック。
