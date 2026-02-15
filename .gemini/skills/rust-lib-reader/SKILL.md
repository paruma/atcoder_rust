---
name: rust-lib-reader
description: Rust の標準ライブラリ（std, core, alloc）およびサードパーティライブラリ（crates.io）のソースコードを探索・閲覧するためのスキル。
---

# rust-lib-reader

Rust の内部実装や依存ライブラリの仕様を確認するために、システムの `/home/node/.rustup` や `/home/node/.cargo` 配下にあるソースコードを探索し、読み取ります。

## ワークフロー

1. **ソース場所の特定**:
  - `python3 .gemini/skills/rust-lib-reader/scripts/locate_rust_source.py <クレート名>` を実行して、そのクレートのベースディレクトリを取得します。
  - 入力例: `std`, `core`, `alloc`, `itertools`, `ac-library-rs`
  - **重要**: 標準ライブラリ（std, core, alloc）のソースが見つからない場合は、`rustup component add rust-src` を実行してソースコードをインストールしてください。

2. **シンボルの探索**:
  - `grep` を使用して、目的のシンボル（`struct BTreeMap`, `fn next_back`, `impl.*Iterator` など）が定義されているファイルを特定します。
  - 例: `grep -r "struct BTreeMap" <探索ディレクトリ>`
  - ファイルが特定できたら、`read_file` ツールを使用して読み取ります。

## 具体的なパス構造（参考）

- **標準ライブラリ**:
  - `/home/node/.rustup/toolchains/<toolchain>/lib/rustlib/src/rust/library/`
  - 主要モジュール: `core/src/`, `alloc/src/`, `std/src/`
- **サードパーティライブラリ**:
  - `/home/node/.cargo/registry/src/index.crates.io-<hash>/<crate-name>-<version>/`

## 注意事項

- サードパーティライブラリを読む前に、`cargo build` を実行して依存関係がダウンロードされていることを確認してください。