---
name: code-verifier
description: src/mylib 配下の Rust ライブラリ編集時に、テスト・カバレッジ・静が解析・フォーマットを一貫して自動検証する標準プロトコル。
---

# code-verifier

競技プログラミング用 Rust ライブラリの品質を維持するための、自動検証スペシャリスト。

## Instructions

あなたは、`src/mylib` 配下のコードを編集する際、**ユーザーからの明示的な依頼がなくても**、このスキルを有効化して以下の「The Verification Flow」を完遂する責務を負います。

### The Verification Flow

何らかの修正（ロジックの変更、コメントの修正、テストの追加）を行った後、必ず以下の順序で実行してください。

1.  **テストの実行**
    - 編集したファイルのテストのみを対象に実行。
    - コマンド: `cargo test --package mylib --lib <モジュールパス> -- --include-ignored --show-output`
    - **制約**: ファイル単位で 0.2 秒以内に終了することを確認してください。
2.  **カバレッジの確認**
    - `cargo llvm-cov` で `lcov.info` を生成し、未実行行（`DA:行番号,0`）がないか確認。
    - コマンド: `cargo llvm-cov test --lcov --output-path lcov.info --package mylib --lib <モジュールパス> -- --include-ignored`
    - 未実行行がある場合、その理由を分析し、可能な限りテストケースを追加してステップ 1 に戻ってください。
    - 実行後、`lcov.info` は即座に削除してください。
3.  **フォーマットの適用**
    - コマンド: `cargo fmt`
4.  **静的解析 (Clippy)**
    - 警告をエラー（`-D warnings`）として扱い、編集したファイルに指摘がないか確認。
    - コマンド: `cargo clippy -p mylib --all-targets -- -D warnings 2>&1 | grep -A 10 "<ファイル名>.rs"`
5.  **スニペット制約のチェック**
    - `cargo-snippet` 展開時のパス不整合を防ぐ。
    - コマンド: `cargo run --bin snippet_linter`
6.  **最終確認 (Git Diff)**
    - 意図しないファイルや箇所に変更が及んでいないか、`git diff` で最終確認する。

## Critical Rules
- **再帰的検証**: ステップ 2〜5 の過程でコードを修正した場合は、**必ずステップ 1（テスト）からやり直してください**。
- **妥協の禁止**: 全てのステップが「成功」または「指摘なし」になるまで、タスクの完了を報告しないでください。
- **透明性**: 検証の各ステップを開始する前に、何を確認しようとしているかを簡潔に宣言してください。
