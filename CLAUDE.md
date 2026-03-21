# CLAUDE.md

## 1. プロジェクトの基本

### 1.1. 概要

これは、AtCoderのコンテスト問題をRust言語で解くためのリポジトリです。
コンテスト問題のコードに加えて、ライブラリも同梱されています。

### 1.2. プロジェクトの構造

本プロジェクトは Cargo ワークスペースで構成されており、以下の単位でパッケージ管理されています。

- **ワークスペースルート (`Cargo.toml`)**:
  - 全体の構成（members）と、共通の依存ライブラリのバージョンを一括管理。
- **年度別パッケージ (`src/contest/YYYY/Cargo.toml`)**:
  - 該当年度の各問題（`abcXXX/q/q.rs`）をバイナリターゲットとして定義・管理。
- **共通ライブラリパッケージ (`src/mylib/Cargo.toml`)**:
  - 競プロ用コードスニペットを `mylib` クレートとして管理。
- **サンプルパッケージ (`src/example/Cargo.toml`)**:
  - 特定のアルゴリズムやデータ構造の練習用コードを `example` クレートとして管理。
- **検証用パッケージ (`src/library_checker/Cargo.toml`)**:
  - Library Checker 用の検証コードを `library_checker` パッケージとして管理。
- **テンプレート用パッケージ (`src/contest/Cargo.toml`)**:
  - `src/contest/template.rs` / `src/contest/template2.rs` 等のテンプレートファイルをバイナリターゲットとして管理し、即座に実行・検証するためのパッケージ。


### 1.3. 支援ツール

コンテストの問題の回答・ライブラリ開発を補助するためのスクリプト群です。

#### 人間用

これらは人間が使うことを想定していて、AIエージェントが使うことは想定していません。

- `contest.py`: 指定したコンテスト・問題用のディレクトリ作成とファイル配置を自動化し、`Cargo.toml` 用の定義を出力します。
- `script.sh`: `oj test` や提出、ビルド・実行などの頻用コマンドをエイリアス（`ojt`, `ojs`, `exe` 等）として提供します（`source script.sh` で使用）。

#### 共用

- `snippet.sh`: `cargo snippet` で mylib のスニペットを生成し、VSCode 用スニペットファイルに出力します。

### 1.4. 環境

- `rust-toolchain` ファイルに Rust のバージョンが記載されています。
    - AtCoder における Rust のバージョンは 1.89.0 です。1.89.0 は古くて rust-analyzer が使えないので、少しバージョンを進めて使っています。
- `Cargo.toml` に利用可能なライブラリが記載されています

### 1.5. 本リポジトリにおける生成AIの役割

本リポジトリにおける生成AIの役割は主に以下のとおりです

* ライブラリの作成
* コンテスト後のコードのリファクタリングなど

コンテスト中は生成AIを利用できません。

## 2. 開発の進め方

### 2.1. コーディング規約

**Queue の使用について**: Queue の機能が必要な場合は、`std::collections::VecDeque` ではなく、`src/mylib/data_structure/queue.rs` で定義されている `mod_queue::Queue` を使用してください。

## 3. ドキュメントリンク

* `.claude/docs/math-writing-guide.md`: 数学的な内容を含むノートや解説を書くときに読む
* `.claude/docs/NOTION-EDITING.md`: Notion を編集するときに読む

## 4. ルール

* /tmp ディレクトリは使わない。プロジェクトディレクトリを使う

