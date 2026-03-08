---
name: rust-asm-analyzer
description: Rust の最適化結果（アセンブリ）を調査・比較するための専門的な手順。black_box や '#[inline(never)]' を用いた正確な解析環境の構築と、emit asm による出力の解析をサポートする。
---

# Rust Assembly Analyzer

Rust コンパイラ（LLVM）による最適化の挙動を、アセンブリレベルで正確に調査するためのガイド。

## 1. 調査用コードの作成原則

正確な比較を行うために、以下の 3 点を遵守する。

### 1.1. 定数畳み込みの回避 (`black_box`)
入力値がコンパイル時に定数として認識されると、コンパイラが計算結果を事前に算出してしまい、調査したいアルゴリズムのコードが消失する場合がある。これを防ぐために `core::hint::black_box` を使用する。

```rust
use core::hint::black_box;

fn main() {
    // 入力を black_box で包むことで、実行時まで値が不明であるとコンパイラに思わせる
    let n = black_box(12345);
    let res = test_target(n);
    black_box(res); // 結果も black_box に入れることで、計算自体が削除されるのを防ぐ
}
```

### 1.2. インライン展開の抑制 (`#[inline(never)]`)
調査対象の関数が呼び出し側にインライン展開されると、アセンブリ内で関数の境界が不明瞭になり、解析が困難になる。特定の関数を単独で調べたい場合は `#[inline(never)]` を付与する。

```rust
#[inline(never)]
pub fn test_target(n: i64) -> i64 {
    // 調査したいロジック
    n * 2
}
```

### 1.3. シンボル名の固定 (`#[unsafe(no_mangle)]`)
Rust のマングリング（名前修飾）により、アセンブリ内のラベルが複雑になるのを防ぐ。
※ `unsafe` 属性が必要な点に注意。

```rust
#[unsafe(no_mangle)]
pub fn test_target(n: i64) -> i64 { ... }
```

## 2. アセンブリの出力と抽出

### 2.1. コンパイルコマンド
リリースビルドでアセンブリを出力する。ターゲットがバイナリ (`--bin`) か例 (`--example`) かに応じて使い分ける。

```bash
# バイナリの場合
cargo rustc --release --bin <bin_name> -- --emit asm

# 特定のパッケージのバイナリの場合
cargo rustc -p <pkg_name> --release --bin <bin_name> -- --emit asm
```

### 2.2. ファイルの特定
出力された `.s` ファイルは通常 `target/release/deps/<bin_name>-<hash>.s` に配置される。最新のファイルを探すのが確実である。

```bash
# 最新のアセンブリファイルを探してコピーする例
find target/release/deps -name "<bin_name>-*.s" -exec ls -t {} + | head -n 1
```

### 2.3. 関数の抽出
`grep` 等を用いて、目的の関数ラベルから次のラベル（または `.cfi_endproc`）までを読み取る。

```bash
# ラベルから 100 行表示する例
grep -A 100 "test_target:" <asm_file>.s
```

## 3. 構成例（行列演算の調査）

```rust
// src/example/matrix_test.rs

use core::hint::black_box;

#[path = "../mylib/math/matrix4.rs"]
mod matrix4;
use matrix4::matrix44::Matrix44;

#[inline(never)]
#[unsafe(no_mangle)]
fn add_mat(m1: &Matrix44<i64>, m2: &Matrix44<i64>) -> Matrix44<i64> {
    *m1 + *m2
}

#[inline(never)]
#[unsafe(no_mangle)]
fn mul_mat(m1: &Matrix44<i64>, m2: &Matrix44<i64>) -> Matrix44<i64> {
    *m1 * *m2
}

fn main() {
    let m1: Matrix44<i64> = black_box(Matrix44::from_array([
        [1, 2, 3, 4],
        [5, 6, 7, 8],
        [9, 10, 11, 12],
        [13, 14, 15, 16],
    ]));
    let m2 = black_box(Matrix44::from_array([
        [16, 15, 14, 13],
        [12, 11, 10, 9],
        [8, 7, 6, 5],
        [4, 3, 2, 1],
    ]));

    black_box(add_mat(&m1, &m2));
    black_box(mul_mat(&m1, &m2));
}
```

## 4. 競技プログラミングにおける最適化の性質

### 4.1. クレート境界の消失とインライン化
通常のライブラリ開発では、ライブラリと呼び出し側が別クレートに分かれているため、LTO (Link Time Optimization) が無効な環境ではクレート境界を越えたインライン展開や定数伝播が阻害される。

しかし、競技プログラミングでは**自作ライブラリのスニペットを提出コードに直接コピペする**ことが一般的である。これにより、以下の強力な最適化が期待できる。

- **強力なインライン展開**: 全てのコードが同一クレート（同一ファイル）内に存在するため、コンパイラは関数やイテレータの境界を越えて最適化を行うことができる。
- **実行時変数の定数化**: ライブラリ関数が引数（変数）として受け取る値であっても、呼び出し側で定数（例：`10`）を渡していれば、インライン展開によって関数内部の演算が「定数除算の乗算置換」などの高度な最適化対象となる。

したがって、アセンブリを調査する際は「別クレートからの呼び出し」ではなく、**「同一ファイル内にライブラリを展開した状態（スニペット形式）」**で検証することが、実際の提出コードの挙動を最も正確に反映する。
