# Data Structure Benchmarks

このディレクトリには、`mylib` に実装されている各種データ構造の性能比較ベンチマークが含まれています。
すべての数値は、AtCoder の環境に近い Release モード（`--release`）での実測値です。

## 計測環境・条件
- **規模**: $N = 1,000,000, Q = 1,000,000$ (10^6 回操作)
- **環境**: Linux (Gemini Sandbox)
- **手法**: 点更新・区間クエリを均等に混合したシナリオを計測

## 比較結果まとめ ($10^6$ operations)

### 1. Range Add Range Sum (BIT vs Lazy Segtree)
区間加算・区間和取得の性能比較。

| Implementation | Total Time | Speed Ratio |
| :--- | :---: | :---: |
| Fenwick Tree (BIT) | **151 ms** | 1.00x |
| Lazy Segment Tree | 435 ms | 2.89x slower |

**知見**: BIT が使えるシナリオでは、遅延セグ木よりも BIT の方が 3 倍近く高速。

### 2. Point Set Range Sum (BIT vs Std Segtree)
一点更新・区間和取得の性能比較。

| Implementation | Total Time | Speed Ratio |
| :--- | :---: | :---: |
| Fenwick Tree (BIT) | **49 ms** | 1.00x |
| Standard Segment Tree | 108 ms | 2.19x slower |

**知見**: シンプルな一点更新のケースでも、BIT は通常セグ木の 2 倍以上のパフォーマンスを発揮する。

### 3. Range Chmin/Chmax Add Action (Beats! vs Lazy)
複雑な区間作用の伝搬コスト比較。

| Implementation | Supported Query | Total Time | Speed Ratio |
| :--- | :---: | :---: | :---: |
| Lazy Segment Tree | Min / Max | **1172 ms** | 1.00x |
| Segtree Beats! | Sum / Min / Max | 2514 ms | 2.14x slower |

**知見**: Beats! は Range Sum を計算するために再帰的な探索が必要なため、通常の遅延セグ木の約 2 倍のコストがかかる。Sum が不要なら Lazy を優先すべき。

### 4. Range Add vs Range Affine (Specialized vs Generic)
遅延セグ木の作用素の汎用化によるコスト比較。

| Implementation | Total Time | Speed Ratio |
| :--- | :---: | :---: |
| Range Add Segtree | **501 ms** | 1.00x |
| Range Affine Segtree | 653 ms | 1.30x slower |

**知見**: 汎用的なアフィン変換の実装（$ax+b$）は、加算特化の実装（$x+b$）よりも 30% ほど定数倍が重い。

## 実行方法
各ベンチマークは以下のコマンドで実行可能です。

```bash
# Range Sum (BIT vs Segtree)
cargo run --release -p bench --bin range_sum

# Beats! vs Lazy
cargo run --release -p bench --bin beats_vs_lazy

# Add vs Affine
cargo run --release -p bench --bin add_vs_affine
```
