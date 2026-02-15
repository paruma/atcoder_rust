# BTree (BTreeMap / BTreeSet) の計算量と最適化

Rust の標準ライブラリにおける `BTreeMap` および `BTreeSet` は、ソート済みであることを活かした高度な最適化が行われています。

## 1. 最大値・最小値の取得

`BTreeSet` / `BTreeMap` のイテレータ（`iter()` や `range()`）において、`max()` や `min()` は **$O(\log N)$** で動作します。

### 最適化の仕組み
通常、Rust のイテレータの `max()` は全要素を走査して比較を行う $O(N)$ の操作ですが、BTree 関連のイテレータは `DoubleEndedIterator` を実装しており、かつ `max()` / `min()` が以下のように特殊化（オーバーライド）されています。

- **`max()`**: 内部で `next_back()` を呼び出し、比較を一切行わずに末尾の要素を直接返します。
- **`min()`**: 内部で `next()` を呼び出し、先頭の要素を直接返します。
- **`last()`**: `max()` と同様に `next_back()` を呼び出します。

### 検証エビデンス
比較演算をカウントする構造体を用いて `set.range(..).max()` を実行すると、比較回数は **0回** になります。

```rust
// 内部的な特殊化の例 (alloc/src/collections/btree/map.rs)
fn max(mut self) -> Option<Item> {
    self.next_back()
}
```
