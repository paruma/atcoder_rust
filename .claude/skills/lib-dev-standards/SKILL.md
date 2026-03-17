---
name: lib-dev-standards
description: src/mylib を編集するときに適用する設計指針・コーディング規約
---

# Library Development Standards

`src/mylib` 配下のライブラリコードに適用する設計指針とコーディング規約。

## mylib について

mylib は競プロ用のコードライブラリです。以下の流れで運用されます：

1. mylib で実装した関数・モジュールに `#[snippet(...)]` アトリビュートを付ける
2. `cargo-snippet` ツールがこれを自動抽出する
3. 抽出されたスニペットがコンテスト問題のソースファイル（`q.rs` など）に展開される
4. 最終的に提出するのは、スニペット展開後の単一ファイルである

つまり、コンテスト問題のコードからは mylib を直接参照することはなく、**スニペット化されたコードのみが使用されます**。
この特性を踏まえて、以下の設計指針を適用してください。

## こんなときはこのファイルを読む

- **mylib で実装するとき（常に読むべき）**
  → [コーディング規約](references/coding-conventions.md)
  → [スニペット設計](references/snippet-design.md)

- **ランダムテストを実装するとき** (ユーザから指示があった場合や、他の同様のファイルでランダムテストが実装されていた場合)
  → [ランダムテスト実装ガイドライン](references/random-testing.md)

- **抽象的な代数構造(Add, Sub など)を使って実装をするとき**
  → [抽象代数設計](references/algebraic-design.md)

- **min/max など対称なペア実装を行うとき**
  → [構造的対称性](references/structural-symmetry.md)

## 実装後

mylib のコードを編集・実装した後は、必ず `code-verifier` スキルを実行してください。
テスト、リンター、その他のコード品質を検証します。
