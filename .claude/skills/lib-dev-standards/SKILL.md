---
name: lib-dev-standards
description: src/mylib を編集するときに適用する設計指針・コーディング規約
---

# Library Development Standards

`src/mylib` 配下のライブラリコードに適用する設計指針とコーディング規約。

## クイックガイド

詳細は以下のリファレンスを参照してください：

- [抽象代数設計](references/algebraic-design.md): ドメイン依存排除、作用ベースの設計
- [構造的対称性](references/structural-symmetry.md): ミラー構造の維持・検証
- [スニペット設計](references/snippet-design.md): 依存分離、可視性、import 規約
- [コーディング規約](references/coding-conventions.md): rand、テスト、コメント、doc コメント、trait 実装
- [開発安全原則](references/development-safety.md): 段階的修正、インクリメンタル実装、検証プロトコル
- [その他の指針](references/misc-guidelines.md): シンボル名、代数演算子、既存資産の尊重

## 核心原則

1. **抽象代数的設計**: 型のドメイン依存を排除し、代数的性質のみを要求
2. **構造的対称性**: 対になる概念は完全にミラーする
3. **スニペット依存分離**: 展開後のコンパイル成功を保証するため依存グラフを最小化
4. **開発安全原則**: 段階的修正と検証プロトコル（`code-verifier`）を徹底
5. **既存資産の尊重**: テスト・コメント・関数の意図しない削除を防止
